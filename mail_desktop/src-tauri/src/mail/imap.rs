use crate::mail::types::{EmailData, FetchResult, LoginResult};
use chrono::Utc;
use log::{error, info};
use std::io::{Read, Write};
use std::net::TcpStream;

/// IMAP 登录验证（使用同步方式，在线程池中执行）
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

    let result = tokio::task::spawn_blocking(move || {
        imap_login_sync(&email, &password, &host, port)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?;

    result
}

fn imap_login_sync(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
) -> Result<LoginResult, String> {
    info!("开始连接 IMAP 服务器: {}:{}", host, port);
    
    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(false)
        .build()
        .map_err(|e| format!("TLS 初始化失败: {}", e))?;
    
    let tcp_stream = TcpStream::connect((host, port))
        .map_err(|e| format!("TCP 连接失败 {}:{} - {}", host, port, e))?;
    
    tcp_stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .ok();
    
    let tls_stream = connector
        .connect(host, tcp_stream)
        .map_err(|e| format!("TLS 连接失败: {}", e))?;

    let client = imap::Client::new(tls_stream);
    
    let mut session = match client.login(email, password) {
        Ok(session) => session,
        Err((e, _)) => {
            error!("IMAP 登录失败: {}", e);
            let msg = if e.to_string().contains("authentication") || e.to_string().contains("AUTH") {
                "邮箱或授权码错误，请检查后重试".to_string()
            } else {
                format!("登录失败: {}", e)
            };
            return Ok(LoginResult {
                success: false,
                message: msg,
                protocol: Some("imap".to_string()),
                host: Some(host.to_string()),
                port: Some(port),
            });
        }
    };

    // 登录成功，退出
    let _ = session.logout();

    info!("✅ IMAP 登录验证成功: {}", email);
    Ok(LoginResult {
        success: true,
        message: "登录验证成功".to_string(),
        protocol: Some("imap".to_string()),
        host: Some(host.to_string()),
        port: Some(port),
    })
}

/// IMAP 收取邮件
pub async fn fetch_emails(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
    limit: usize,
) -> Result<FetchResult, String> {
    info!("开始 IMAP 收取邮件: {} -> {}:{}", email, host, port);

    let email = email.to_string();
    let password = password.to_string();
    let host = host.to_string();

    let result = tokio::task::spawn_blocking(move || {
        imap_fetch_sync(&email, &password, &host, port, limit)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?;

    result
}

fn imap_fetch_sync(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
    limit: usize,
) -> Result<FetchResult, String> {
    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(false)
        .build()
        .map_err(|e| format!("TLS 初始化失败: {}", e))?;
    
    let tcp_stream = TcpStream::connect((host, port))
        .map_err(|e| format!("TCP 连接失败 {}:{} - {}", host, port, e))?;
    
    tcp_stream
        .set_read_timeout(Some(std::time::Duration::from_secs(15)))
        .ok();
    
    let tls_stream = connector
        .connect(host, tcp_stream)
        .map_err(|e| format!("TLS 连接失败: {}", e))?;

    let client = imap::Client::new(tls_stream);
    
    let mut session = client
        .login(email, password)
        .map_err(|(e, _)| format!("登录失败: {}", e))?;

    // 选择收件箱
    session
        .select("INBOX")
        .map_err(|e| format!("选择收件箱失败: {}", e))?;

    // 搜索所有邮件
    let uids = session
        .uid_search("ALL")
        .map_err(|e| format!("搜索邮件失败: {}", e))?;

    let mut emails: Vec<EmailData> = Vec::new();
    let uids_vec: Vec<u32> = uids.into_iter().collect();
    let total = uids_vec.len();
    
    // 只取最新的 limit 封
    let start = if total > limit { total - limit } else { 0 };
    let uids_to_fetch = &uids_vec[start..];

    for uid in uids_to_fetch {
        match fetch_single_email(&mut session, *uid) {
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

    let (content_text, content_html) = extract_content(&parsed);

    let now = Utc::now().timestamp_millis();

    Ok(EmailData {
        message_id,
        subject,
        from_addr,
        to_addr,
        content_text,
        content_html,
        email_date_ms: now,
        received_at_ms: now,
    })
}

fn extract_content(mail: &mailparse::ParsedMail) -> (String, String) {
    let mut content_text = String::new();
    let mut content_html = String::new();

    if mail.subparts.is_empty() {
        let content_type = mail.ctype.mimetype.as_str();
        if let Ok(body) = mail.get_body() {
            if content_type.contains("text/plain") {
                content_text = body;
            } else if content_type.contains("text/html") {
                content_html = body;
            }
        }
    } else {
        for part in &mail.subparts {
            let (text, html) = extract_content(part);
            if !text.is_empty() {
                content_text = text;
            }
            if !html.is_empty() {
                content_html = html;
            }
        }
    }

    (content_text, content_html)
}
