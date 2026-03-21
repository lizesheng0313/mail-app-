use crate::mail::types::{AttachmentData, EmailData, FetchResult, LoginResult};
use chrono::Utc;
use imap::{types::NameAttribute, Authenticator};
use log::{error, info, warn};
use native_tls::TlsStream;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

const TCP_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const IO_TIMEOUT: Duration = Duration::from_secs(30);

// ── 连接错误分类（决定是否回退到其他策略）──────────────────────

enum ConnectError {
    /// TCP 连接失败（端口不通、DNS 错误）→ 跳到下一个端口
    Connection(String),
    /// TLS 握手失败（证书、协议版本）→ 同端口放宽证书再试
    Tls(String),
    /// 认证失败（密码错误）→ 直接返回，不再重试
    Auth(String),
    /// 其他错误 → 继续尝试
    Other(String),
}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connection(s) | Self::Tls(s) | Self::Auth(s) | Self::Other(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

// ── 单次连接尝试 ──────────────────────────────────────────────

fn try_connect_and_login(
    host: &str,
    port: u16,
    email: &str,
    password: &str,
    accept_invalid_certs: bool,
) -> Result<(imap::Session<TlsStream<TcpStream>>, u16), ConnectError> {
    let use_starttls = port != 993;
    info!(
        "尝试 IMAP {}:{} ({}{})",
        host,
        port,
        if use_starttls { "STARTTLS" } else { "隐式TLS" },
        if accept_invalid_certs {
            ", 放宽证书"
        } else {
            ""
        }
    );

    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(accept_invalid_certs)
        .min_protocol_version(Some(native_tls::Protocol::Tlsv10))
        .build()
        .map_err(|e| ConnectError::Tls(format!("TLS 初始化失败: {}", e)))?;

    // DNS 解析
    let addr = (host, port)
        .to_socket_addrs()
        .map_err(|e| ConnectError::Connection(format!("DNS 解析失败 {}: {}", host, e)))?
        .next()
        .ok_or_else(|| ConnectError::Connection(format!("DNS 解析无结果: {}", host)))?;

    // TCP 连接（带超时）
    let tcp = TcpStream::connect_timeout(&addr, TCP_CONNECT_TIMEOUT)
        .map_err(|e| ConnectError::Connection(format!("TCP 连接失败 {}:{} - {}", host, port, e)))?;

    tcp.set_read_timeout(Some(IO_TIMEOUT)).ok();
    tcp.set_write_timeout(Some(IO_TIMEOUT)).ok();

    // 建立 IMAP TLS 客户端
    let client = if use_starttls {
        // 明文连接 → STARTTLS 升级
        let plain_client = imap::Client::new(tcp);
        plain_client.secure(host, &connector).map_err(|e| match &e {
            imap::Error::TlsHandshake(_) | imap::Error::Tls(_) => {
                ConnectError::Tls(format!("STARTTLS 握手失败: {}", e))
            }
            imap::Error::Io(_) | imap::Error::ConnectionLost => {
                ConnectError::Connection(format!("STARTTLS 连接错误: {}", e))
            }
            _ => ConnectError::Other(format!("STARTTLS 失败: {}", e)),
        })?
    } else {
        // 隐式 TLS（993）
        let tls = connector
            .connect(host, tcp)
            .map_err(|e| ConnectError::Tls(format!("TLS 握手失败: {}", e)))?;
        imap::Client::new(tls)
    };

    // 登录
    match client.login(email, password) {
        Ok(session) => {
            info!("✅ IMAP {}:{} 登录成功", host, port);
            Ok((session, port))
        }
        Err((e, _)) => {
            error!("IMAP 登录失败 {}:{}: {}", host, port, e);
            Err(match &e {
                imap::Error::No(_) | imap::Error::Bad(_) => {
                    ConnectError::Auth("邮箱或授权码错误，请检查后重试".to_string())
                }
                _ => {
                    let s = e.to_string().to_lowercase();
                    if s.contains("auth") || s.contains("credential") || s.contains("login") {
                        ConnectError::Auth("邮箱或授权码错误，请检查后重试".to_string())
                    } else {
                        ConnectError::Other(format!("登录失败: {}", e))
                    }
                }
            })
        }
    }
}

// ── 多策略连接：主端口 → 放宽证书 → 备用端口 → 放宽证书 ──────

fn connect_and_login(
    host: &str,
    port: u16,
    email: &str,
    password: &str,
) -> Result<(imap::Session<TlsStream<TcpStream>>, u16), String> {
    let alt_port: u16 = if port == 993 { 143 } else { 993 };
    let ports = [port, alt_port];
    let mut last_error = String::new();

    for &p in &ports {
        // 严格 TLS
        match try_connect_and_login(host, p, email, password, false) {
            Ok(r) => return Ok(r),
            Err(ConnectError::Auth(msg)) => return Err(msg),
            Err(ConnectError::Tls(msg)) => {
                warn!("{}", msg);
                // TLS 握手失败 → 放宽证书再试
                match try_connect_and_login(host, p, email, password, true) {
                    Ok(r) => {
                        warn!("⚠️ IMAP 放宽证书验证连接成功 {}:{}", host, p);
                        return Ok(r);
                    }
                    Err(ConnectError::Auth(msg)) => return Err(msg),
                    Err(e) => {
                        last_error = e.to_string();
                        continue;
                    }
                }
            }
            Err(e) => {
                last_error = e.to_string();
                continue;
            }
        }
    }

    Err(format!(
        "IMAP 连接失败（已尝试 {}:{} 和 {}:{}）: {}",
        host, port, host, alt_port, last_error
    ))
}

// ── XOAUTH2 认证 ─────────────────────────────────────────────

struct OAuth2Authenticator {
    user: String,
    access_token: String,
}

impl Authenticator for OAuth2Authenticator {
    type Response = String;
    fn process(&self, _data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

fn try_connect_and_xoauth2(
    host: &str,
    port: u16,
    email: &str,
    access_token: &str,
    accept_invalid_certs: bool,
) -> Result<(imap::Session<TlsStream<TcpStream>>, u16), ConnectError> {
    let use_starttls = port != 993;
    info!(
        "尝试 IMAP XOAUTH2 {}:{} ({}{})",
        host,
        port,
        if use_starttls { "STARTTLS" } else { "隐式TLS" },
        if accept_invalid_certs {
            ", 放宽证书"
        } else {
            ""
        }
    );

    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(accept_invalid_certs)
        .min_protocol_version(Some(native_tls::Protocol::Tlsv10))
        .build()
        .map_err(|e| ConnectError::Tls(format!("TLS 初始化失败: {}", e)))?;

    let addr = (host, port)
        .to_socket_addrs()
        .map_err(|e| ConnectError::Connection(format!("DNS 解析失败 {}: {}", host, e)))?
        .next()
        .ok_or_else(|| ConnectError::Connection(format!("DNS 解析无结果: {}", host)))?;

    let tcp = TcpStream::connect_timeout(&addr, TCP_CONNECT_TIMEOUT)
        .map_err(|e| ConnectError::Connection(format!("TCP 连接失败 {}:{} - {}", host, port, e)))?;

    tcp.set_read_timeout(Some(IO_TIMEOUT)).ok();
    tcp.set_write_timeout(Some(IO_TIMEOUT)).ok();

    let client = if use_starttls {
        let plain_client = imap::Client::new(tcp);
        plain_client.secure(host, &connector).map_err(|e| match &e {
            imap::Error::TlsHandshake(_) | imap::Error::Tls(_) => {
                ConnectError::Tls(format!("STARTTLS 握手失败: {}", e))
            }
            imap::Error::Io(_) | imap::Error::ConnectionLost => {
                ConnectError::Connection(format!("STARTTLS 连接错误: {}", e))
            }
            _ => ConnectError::Other(format!("STARTTLS 失败: {}", e)),
        })?
    } else {
        let tls = connector
            .connect(host, tcp)
            .map_err(|e| ConnectError::Tls(format!("TLS 握手失败: {}", e)))?;
        imap::Client::new(tls)
    };

    let auth = OAuth2Authenticator {
        user: email.to_string(),
        access_token: access_token.to_string(),
    };

    match client.authenticate("XOAUTH2", &auth) {
        Ok(session) => {
            info!("✅ IMAP XOAUTH2 {}:{} 认证成功", host, port);
            Ok((session, port))
        }
        Err((e, _)) => {
            error!("IMAP XOAUTH2 认证失败 {}:{}: {}", host, port, e);
            Err(match &e {
                imap::Error::No(_) | imap::Error::Bad(_) => {
                    ConnectError::Auth("OAuth2 认证失败，请重新授权".to_string())
                }
                _ => {
                    let s = e.to_string().to_lowercase();
                    if s.contains("auth") || s.contains("credential") || s.contains("login") {
                        ConnectError::Auth("OAuth2 认证失败，请重新授权".to_string())
                    } else {
                        ConnectError::Other(format!("XOAUTH2 认证失败: {}", e))
                    }
                }
            })
        }
    }
}

fn connect_and_xoauth2(
    host: &str,
    port: u16,
    email: &str,
    access_token: &str,
) -> Result<(imap::Session<TlsStream<TcpStream>>, u16), String> {
    let alt_port: u16 = if port == 993 { 143 } else { 993 };
    let ports = [port, alt_port];
    let mut last_error = String::new();

    for &p in &ports {
        match try_connect_and_xoauth2(host, p, email, access_token, false) {
            Ok(r) => return Ok(r),
            Err(ConnectError::Auth(msg)) => return Err(msg),
            Err(ConnectError::Tls(msg)) => {
                warn!("{}", msg);
                match try_connect_and_xoauth2(host, p, email, access_token, true) {
                    Ok(r) => {
                        warn!("⚠️ IMAP XOAUTH2 放宽证书验证连接成功 {}:{}", host, p);
                        return Ok(r);
                    }
                    Err(ConnectError::Auth(msg)) => return Err(msg),
                    Err(e) => {
                        last_error = e.to_string();
                        continue;
                    }
                }
            }
            Err(e) => {
                last_error = e.to_string();
                continue;
            }
        }
    }

    Err(format!(
        "IMAP XOAUTH2 连接失败（已尝试 {}:{} 和 {}:{}）: {}",
        host, port, host, alt_port, last_error
    ))
}

// ── 公开接口 ──────────────────────────────────────────────────

/// IMAP 登录验证
pub async fn verify_login(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
) -> Result<LoginResult, String> {
    info!("尝试 IMAP 登录验证: {} -> {}:{}", email, host, port);

    let email = email.to_string();
    let password = password.to_string();
    let host = host.to_string();

    tokio::task::spawn_blocking(move || imap_login_sync(&email, &password, &host, port))
        .await
        .map_err(|e| format!("任务执行失败: {}", e))?
}

fn imap_login_sync(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
) -> Result<LoginResult, String> {
    match connect_and_login(host, port, email, password) {
        Ok((mut session, actual_port)) => {
            let _ = session.logout();
            Ok(LoginResult {
                success: true,
                message: "登录验证成功".to_string(),
                protocol: Some("imap".to_string()),
                host: Some(host.to_string()),
                port: Some(actual_port),
                smtp_host: None,
                smtp_port: None,
                smtp_verified: false,
                smtp_error: None,
            })
        }
        Err(msg) => Ok(LoginResult {
            success: false,
            message: msg,
            protocol: Some("imap".to_string()),
            host: Some(host.to_string()),
            port: Some(port),
            smtp_host: None,
            smtp_port: None,
            smtp_verified: false,
            smtp_error: None,
        }),
    }
}

/// IMAP 收取邮件
pub async fn fetch_emails(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
    limit: usize,
    fetch_oldest: bool,
) -> Result<FetchResult, String> {
    info!("开始 IMAP 收取邮件: {} -> {}:{}", email, host, port);

    let email = email.to_string();
    let password = password.to_string();
    let host = host.to_string();

    tokio::task::spawn_blocking(move || {
        imap_fetch_sync(&email, &password, &host, port, limit, fetch_oldest)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

/// IMAP XOAUTH2 收取邮件
pub async fn fetch_emails_oauth2(
    email: &str,
    access_token: &str,
    host: &str,
    port: u16,
    limit: usize,
    fetch_oldest: bool,
) -> Result<FetchResult, String> {
    info!("开始 IMAP XOAUTH2 收取邮件: {} -> {}:{}", email, host, port);

    let email = email.to_string();
    let access_token = access_token.to_string();
    let host = host.to_string();

    tokio::task::spawn_blocking(move || {
        imap_fetch_oauth2_sync(&email, &access_token, &host, port, limit, fetch_oldest)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

fn imap_fetch_oauth2_sync(
    email: &str,
    access_token: &str,
    host: &str,
    port: u16,
    limit: usize,
    fetch_oldest: bool,
) -> Result<FetchResult, String> {
    let (mut session, _) = connect_and_xoauth2(host, port, email, access_token)?;

    // 选择收件箱
    session
        .select("INBOX")
        .map_err(|e| format!("选择收件箱失败: {}", e))?;

    let search_result = session.uid_search("ALL");
    let uids = if let Err(_) = search_result {
        session.select("INBOX").ok();
        session.uid_search("ALL").map_err(|e| format!("搜索邮件失败: {}", e))?
    } else {
        search_result.unwrap()
    };

    let mut emails: Vec<EmailData> = Vec::new();
    let uids_vec: Vec<u32> = uids.into_iter().collect();
    let total = uids_vec.len();

    let uids_to_fetch: Vec<u32> = if fetch_oldest {
        uids_vec.iter().take(limit).copied().collect()
    } else {
        let start = if total > limit { total - limit } else { 0 };
        uids_vec[start..].to_vec()
    };

    for uid in uids_to_fetch {
        match fetch_single_email(&mut session, uid) {
            Ok(email_data) => emails.push(email_data),
            Err(e) => {
                error!("获取邮件 {} 失败: {}", uid, e);
                continue;
            }
        }
    }

    let _ = session.logout();

    info!("✅ IMAP XOAUTH2 收取完成，共 {} 封邮件", emails.len());
    Ok(FetchResult {
        success: true,
        message: format!("收取成功，共 {} 封邮件", emails.len()),
        count: emails.len(),
        emails,
    })
}

/// IMAP 按 message_id 下载指定附件（密码认证）
pub async fn download_attachment(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
    message_id: &str,
    filename: &str,
    save_path: &str,
) -> Result<(), String> {
    let email = email.to_string();
    let password = password.to_string();
    let host = host.to_string();
    let message_id = message_id.to_string();
    let filename = filename.to_string();
    let save_path = save_path.to_string();

    tokio::task::spawn_blocking(move || {
        let (mut session, _) = connect_and_login(&host, port, &email, &password)?;
        imap_download_attachment_sync(&mut session, &message_id, &filename, &save_path)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

/// IMAP 按 message_id 下载指定附件（OAuth2 认证）
pub async fn download_attachment_oauth2(
    email: &str,
    access_token: &str,
    host: &str,
    port: u16,
    message_id: &str,
    filename: &str,
    save_path: &str,
) -> Result<(), String> {
    let email = email.to_string();
    let access_token = access_token.to_string();
    let host = host.to_string();
    let message_id = message_id.to_string();
    let filename = filename.to_string();
    let save_path = save_path.to_string();

    tokio::task::spawn_blocking(move || {
        let (mut session, _) = connect_and_xoauth2(&host, port, &email, &access_token)?;
        imap_download_attachment_sync(&mut session, &message_id, &filename, &save_path)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

fn imap_download_attachment_sync<T: Read + Write>(
    session: &mut imap::Session<T>,
    message_id: &str,
    filename: &str,
    save_path: &str,
) -> Result<(), String> {
    let selectable_mailboxes = list_selectable_mailboxes(session);
    info!(
        "开始按 Message-ID 查找附件邮件，候选文件夹数: {}, message_id={}",
        selectable_mailboxes.len(),
        message_id
    );

    let mut found_mailbox = None;
    let mut found_uid = None;
    for mailbox in selectable_mailboxes {
        match session.select(&mailbox) {
            Ok(_) => {}
            Err(e) => {
                warn!("选择文件夹失败: {} ({})", mailbox, e);
                continue;
            }
        }

        match find_message_uid_in_selected_mailbox(session, message_id) {
            Ok(Some(uid)) => {
                found_mailbox = Some(mailbox);
                found_uid = Some(uid);
                break;
            }
            Ok(None) => continue,
            Err(e) => {
                warn!("在文件夹中搜索邮件失败: {} ({})", mailbox, e);
                continue;
            }
        }
    }

    let mailbox_name = found_mailbox.ok_or_else(|| "邮件在服务器上未找到".to_string())?;
    let uid = found_uid.ok_or_else(|| "邮件在服务器上未找到".to_string())?;
    info!("附件所属邮件已定位: mailbox={}, uid={}", mailbox_name, uid);

    // 取完整邮件
    let uid_set = format!("{}", uid);
    let messages = session
        .uid_fetch(&uid_set, "RFC822")
        .map_err(|e| format!("获取邮件失败: {}", e))?;
    let message = messages.iter().next().ok_or("没有获取到邮件数据")?;
    let body = message.body().ok_or("邮件内容为空")?;

    let parsed = mailparse::parse_mail(body)
        .map_err(|e| format!("解析邮件失败: {}", e))?;

    let (_, _, attachments) = extract_content(&parsed);

    let att = attachments
        .iter()
        .find(|a| a.filename == filename)
        .ok_or_else(|| format!("附件 {} 未找到", filename))?;

    if att.data.is_empty() {
        return Err(format!("附件 {} 数据为空", filename));
    }

    std::fs::write(save_path, &att.data)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    let _ = session.logout();
    info!("✅ 附件下载成功: {} -> {}", filename, save_path);
    Ok(())
}

fn list_selectable_mailboxes<T: Read + Write>(session: &mut imap::Session<T>) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut mailboxes = Vec::new();

    let push_mailbox = |items: &mut Vec<String>, seen: &mut HashSet<String>, name: &str| {
        let normalized = name.trim().to_ascii_lowercase();
        if normalized.is_empty() || !seen.insert(normalized) {
            return;
        }
        items.push(name.to_string());
    };

    push_mailbox(&mut mailboxes, &mut seen, "INBOX");

    match session.list(None, Some("*")) {
        Ok(list) => {
            for mailbox in list.iter() {
                if mailbox
                    .attributes()
                    .iter()
                    .any(|attr| matches!(attr, NameAttribute::NoSelect))
                {
                    continue;
                }
                push_mailbox(&mut mailboxes, &mut seen, mailbox.name());
            }
        }
        Err(e) => warn!("获取 IMAP 文件夹列表失败，退回默认 INBOX: {}", e),
    }

    mailboxes
}

fn find_message_uid_in_selected_mailbox<T: Read + Write>(
    session: &mut imap::Session<T>,
    message_id: &str,
) -> Result<Option<u32>, String> {
    let search_query = format!("HEADER Message-ID \"{}\"", message_id);
    let uids = session
        .uid_search(&search_query)
        .map_err(|e| format!("搜索邮件失败: {}", e))?;

    if let Some(&uid) = uids.iter().next() {
        return Ok(Some(uid));
    }

    info!("Message-ID 精确搜索无结果，遍历当前文件夹全部邮件头部匹配");
    let all_uids = session
        .uid_search("ALL")
        .map_err(|e| format!("搜索邮件失败: {}", e))?;
    let uids_vec: Vec<u32> = all_uids.into_iter().collect();

    for &uid in uids_vec.iter().rev() {
        let uid_set = uid.to_string();
        if let Ok(messages) = session.uid_fetch(&uid_set, "BODY[HEADER.FIELDS (MESSAGE-ID)]") {
            if let Some(message) = messages.iter().next() {
                if let Some(body) = message.body() {
                    let header = String::from_utf8_lossy(body);
                    if header.contains(message_id) {
                        return Ok(Some(uid));
                    }
                }
            }
        }
    }

    Ok(None)
}

fn imap_fetch_sync(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
    limit: usize,
    fetch_oldest: bool,
) -> Result<FetchResult, String> {
    let (mut session, _) = connect_and_login(host, port, email, password)?;

    // 选择收件箱
    session
        .select("INBOX")
        .map_err(|e| format!("选择收件箱失败: {}", e))?;

    // 搜索所有邮件（部分服务器如163/189在select后可能踢回AUTH状态，重试一次）
    let search_result = session.uid_search("ALL");
    let uids = if let Err(_) = search_result {
        // 状态异常，重新 SELECT 再试
        session.select("INBOX").ok();
        session.uid_search("ALL").map_err(|e| format!("搜索邮件失败: {}", e))?
    } else {
        search_result.unwrap()
    };

    let mut emails: Vec<EmailData> = Vec::new();
    let uids_vec: Vec<u32> = uids.into_iter().collect();
    let total = uids_vec.len();

    let uids_to_fetch: Vec<u32> = if fetch_oldest {
        // 历史回补：优先抓最早的邮件，便于补齐老历史
        uids_vec.iter().take(limit).copied().collect()
    } else {
        // 增量同步：优先抓最新邮件
        let start = if total > limit { total - limit } else { 0 };
        uids_vec[start..].to_vec()
    };

    for uid in uids_to_fetch {
        match fetch_single_email(&mut session, uid) {
            Ok(email_data) => emails.push(email_data),
            Err(e) => {
                error!("获取邮件 {} 失败: {}", uid, e);
                continue;
            }
        }
    }

    let _ = session.logout();

    info!("✅ IMAP 收取完成，共 {} 封邮件", emails.len());
    Ok(FetchResult {
        success: true,
        message: format!("收取成功，共 {} 封邮件", emails.len()),
        count: emails.len(),
        emails,
    })
}

// ── 邮件解析（未修改）─────────────────────────────────────────

fn parse_email_timestamp_ms(parsed: &mailparse::ParsedMail, fallback_ms: i64) -> i64 {
    let raw_date = parsed
        .headers
        .iter()
        .find(|h| h.get_key_ref().eq_ignore_ascii_case("Date"))
        .map(|h| h.get_value())
        .unwrap_or_default();

    chrono::DateTime::parse_from_rfc2822(raw_date.trim())
        .map(|dt| dt.timestamp_millis())
        .or_else(|_| chrono::DateTime::parse_from_rfc3339(raw_date.trim()).map(|dt| dt.timestamp_millis()))
        .unwrap_or(fallback_ms)
}

fn fetch_single_email<T: Read + Write>(
    session: &mut imap::Session<T>,
    uid: u32,
) -> Result<EmailData, String> {
    let uid_set = format!("{}", uid);
    let messages = session
        .uid_fetch(&uid_set, "RFC822")
        .map_err(|e| format!("获取邮件失败: {}", e))?;

    let message = messages
        .iter()
        .next()
        .ok_or("没有获取到邮件数据")?;

    let body = message.body().ok_or("邮件内容为空")?;

    // 解析邮件
    let parsed = mailparse::parse_mail(body)
        .map_err(|e| format!("解析邮件失败: {}", e))?;

    let subject = parsed
        .headers
        .iter()
        .find(|h| h.get_key_ref() == "Subject")
        .map(|h| h.get_value())
        .unwrap_or_else(|| "(无主题)".to_string());

    let from_addr = parsed
        .headers
        .iter()
        .find(|h| h.get_key_ref() == "From")
        .map(|h| h.get_value())
        .unwrap_or_default();

    let to_addr = parsed
        .headers
        .iter()
        .find(|h| h.get_key_ref() == "To")
        .map(|h| h.get_value())
        .unwrap_or_default();

    let message_id = parsed
        .headers
        .iter()
        .find(|h| h.get_key_ref() == "Message-ID")
        .map(|h| h.get_value())
        .unwrap_or_else(|| format!("<imap-{}@local>", uid));

    let (content_text, content_html, attachments) = extract_content(&parsed);

    let now = Utc::now().timestamp_millis();
    let email_time_ms = parse_email_timestamp_ms(&parsed, now);

    Ok(EmailData {
        message_id,
        subject,
        from_addr,
        to_addr,
        content_text,
        content_html,
        email_date_ms: email_time_ms,
        received_at_ms: email_time_ms,
        attachments,
    })
}

fn extract_content(mail: &mailparse::ParsedMail) -> (String, String, Vec<AttachmentData>) {
    let mut content_text = String::new();
    let mut content_html = String::new();
    let mut attachments = Vec::new();

    if mail.subparts.is_empty() {
        let content_type = mail.ctype.mimetype.as_str();
        let disposition = mail
            .headers
            .iter()
            .find(|h| h.get_key_ref().eq_ignore_ascii_case("Content-Disposition"))
            .map(|h| h.get_value())
            .unwrap_or_default();

        let disposition_lower = disposition.to_lowercase();
        let content_id = mail
            .headers
            .iter()
            .find(|h| h.get_key_ref().eq_ignore_ascii_case("Content-ID"))
            .map(|h| h.get_value())
            .unwrap_or_default();

        // 判断是否是附件：
        // 1. Content-Disposition 明确为 attachment → 是附件
        // 2. 非 text/multipart 但 Content-Disposition 为 inline 或有 Content-ID → 内嵌资源，不是附件
        // 3. 非 text/multipart 且无 inline/Content-ID → 当作附件
        let is_attachment = if disposition_lower.contains("attachment") {
            true
        } else if !content_type.starts_with("text/") && !content_type.starts_with("multipart/") {
            !disposition_lower.contains("inline") && content_id.is_empty()
        } else {
            false
        };

        if is_attachment {
            // 这是附件
            if let Ok(body_raw) = mail.get_body_raw() {
                if !body_raw.is_empty() && body_raw.len() <= 25 * 1024 * 1024 {
                    let filename = mail
                        .ctype
                        .params
                        .get("name")
                        .cloned()
                        .or_else(|| {
                            // 从 Content-Disposition 提取 filename
                            disposition
                                .split(';')
                                .find_map(|part| {
                                    let part = part.trim();
                                    if part.to_lowercase().starts_with("filename=") {
                                        Some(part[9..].trim_matches('"').to_string())
                                    } else {
                                        None
                                    }
                                })
                        })
                        .unwrap_or_else(|| "attachment".to_string());

                    attachments.push(AttachmentData {
                        filename,
                        content_type: content_type.to_string(),
                        size: body_raw.len(),
                        data: body_raw,
                    });
                }
            }
        } else if let Ok(body) = mail.get_body() {
            if content_type.contains("text/plain") {
                content_text = body;
            } else if content_type.contains("text/html") {
                content_html = body;
            }
        }
    } else {
        for part in &mail.subparts {
            let (text, html, atts) = extract_content(part);
            if !text.is_empty() {
                content_text = text;
            }
            if !html.is_empty() {
                content_html = html;
            }
            attachments.extend(atts);
        }
    }

    (content_text, content_html, attachments)
}
