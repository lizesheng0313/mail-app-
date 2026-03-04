use crate::mail::types::{AttachmentData, EmailData, FetchResult, LoginResult};
use chrono::Utc;
use log::{error, info};
use std::io::{Read, Write};
use std::net::TcpStream;

/// POP3 登录验证
pub async fn verify_login(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
) -> Result<LoginResult, String> {
    info!("尝试 POP3 登录验证: {} -> {}:{}", email, host, port);

    // POP3 使用阻塞 IO，需要在线程池中执行
    let email = email.to_string();
    let password = password.to_string();
    let host = host.to_string();

    let result = tokio::task::spawn_blocking(move || {
        pop3_login_sync(&email, &password, &host, port)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?;

    result
}

fn pop3_login_sync(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
) -> Result<LoginResult, String> {
    info!("开始连接 POP3 服务器: {}:{}", host, port);
    
    // 使用 native-tls 连接
    let connector = native_tls::TlsConnector::builder()
        .danger_accept_invalid_certs(false)
        .build()
        .map_err(|e| format!("TLS 初始化失败: {}", e))?;
    
    // 直接连接，让系统处理DNS
    let tcp_stream = TcpStream::connect((host, port))
        .map_err(|e| format!("TCP 连接失败 {}:{} - {}", host, port, e))?;
    
    tcp_stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .ok();
    
    let mut tls_stream = connector
        .connect(host, tcp_stream)
        .map_err(|e| format!("TLS 连接失败: {}", e))?;

    // 读取欢迎消息
    let mut response = read_line(&mut tls_stream)?;
    
    if !response.starts_with("+OK") {
        return Ok(LoginResult {
            success: false,
            message: format!("服务器响应错误: {}", response),
            protocol: Some("pop3".to_string()),
            host: Some(host.to_string()),
            port: Some(port),
        });
    }

    // 发送 USER 命令
    tls_stream
        .write_all(format!("USER {}\r\n", email).as_bytes())
        .map_err(|e| format!("发送失败: {}", e))?;
    response = read_line(&mut tls_stream)?;
    
    if !response.starts_with("+OK") {
        return Ok(LoginResult {
            success: false,
            message: "用户名错误".to_string(),
            protocol: Some("pop3".to_string()),
            host: Some(host.to_string()),
            port: Some(port),
        });
    }

    // 发送 PASS 命令
    tls_stream
        .write_all(format!("PASS {}\r\n", password).as_bytes())
        .map_err(|e| format!("发送失败: {}", e))?;
    response = read_line(&mut tls_stream)?;
    
    if !response.starts_with("+OK") {
        return Ok(LoginResult {
            success: false,
            message: "邮箱或授权码错误，请检查后重试".to_string(),
            protocol: Some("pop3".to_string()),
            host: Some(host.to_string()),
            port: Some(port),
        });
    }

    // 发送 QUIT 命令
    let _ = tls_stream.write_all(b"QUIT\r\n");

    info!("✅ POP3 登录验证成功: {}", email);
    Ok(LoginResult {
        success: true,
        message: "登录验证成功".to_string(),
        protocol: Some("pop3".to_string()),
        host: Some(host.to_string()),
        port: Some(port),
    })
}

/// POP3 收取邮件
pub async fn fetch_emails(
    email: &str,
    password: &str,
    host: &str,
    port: u16,
    limit: usize,
) -> Result<FetchResult, String> {
    info!("开始 POP3 收取邮件: {} -> {}:{}", email, host, port);

    let email = email.to_string();
    let password = password.to_string();
    let host = host.to_string();

    let result = tokio::task::spawn_blocking(move || {
        pop3_fetch_sync(&email, &password, &host, port, limit)
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?;

    result
}

fn pop3_fetch_sync(
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
    
    let mut tls_stream = connector
        .connect(host, tcp_stream)
        .map_err(|e| format!("TLS 连接失败: {}", e))?;

    // 读取欢迎消息
    let _ = read_line(&mut tls_stream)?;

    // 登录
    tls_stream.write_all(format!("USER {}\r\n", email).as_bytes()).map_err(|e| format!("发送失败: {}", e))?;
    let _ = read_line(&mut tls_stream)?;

    tls_stream.write_all(format!("PASS {}\r\n", password).as_bytes()).map_err(|e| format!("发送失败: {}", e))?;
    let response = read_line(&mut tls_stream)?;
    
    if !response.starts_with("+OK") {
        error!("POP3 登录失败，服务器响应: {}", response.trim());
        return Err(format!("登录失败: {}", response.trim()));
    }

    // 获取邮件数量
    tls_stream.write_all(b"STAT\r\n").map_err(|e| format!("发送失败: {}", e))?;
    let response = read_line(&mut tls_stream)?;
    
    let parts: Vec<&str> = response.split_whitespace().collect();
    let total: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    let mut emails: Vec<EmailData> = Vec::new();
    let start = if total > limit { total - limit + 1 } else { 1 };

    for i in start..=total {
        match fetch_single_pop3(&mut tls_stream, i) {
            Ok(email_data) => emails.push(email_data),
            Err(e) => {
                error!("获取邮件 {} 失败: {}", i, e);
                continue;
            }
        }
    }

    let _ = tls_stream.write_all(b"QUIT\r\n");

    info!("✅ POP3 收取完成，共 {} 封邮件", emails.len());
    Ok(FetchResult {
        success: true,
        message: format!("收取成功，共 {} 封邮件", emails.len()),
        count: emails.len(),
        emails,
    })
}

/// 从 TLS 流中读取一行
fn read_line<R: Read>(stream: &mut R) -> Result<String, String> {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];
    
    loop {
        match stream.read(&mut byte) {
            Ok(0) => break,
            Ok(_) => {
                buf.push(byte[0]);
                if buf.ends_with(b"\r\n") {
                    break;
                }
            }
            Err(e) => return Err(format!("读取失败: {}", e)),
        }
    }
    
    String::from_utf8(buf).map_err(|e| format!("UTF-8 解码失败: {}", e))
}

/// 读取多行响应直到遇到单独的 '.'
fn read_multiline<R: Read>(stream: &mut R) -> Result<Vec<u8>, String> {
    let mut content = Vec::new();
    
    loop {
        let line = read_line(stream)?;
        
        if line.trim() == "." {
            break;
        }
        
        // 处理字节填充（以 .. 开头的行）
        if line.starts_with("..") {
            content.extend(line[1..].as_bytes());
        } else {
            content.extend(line.as_bytes());
        }
    }
    
    Ok(content)
}

fn fetch_single_pop3<S: Read + Write>(
    stream: &mut S,
    msg_num: usize,
) -> Result<EmailData, String> {
    // 发送 RETR 命令
    stream
        .write_all(format!("RETR {}\r\n", msg_num).as_bytes())
        .map_err(|e| format!("发送失败: {}", e))?;

    let response = read_line(stream)?;
    
    if !response.starts_with("+OK") {
        return Err(format!("获取邮件失败: {}", response));
    }

    // 读取邮件内容直到遇到单独的 '.'
    let mail_content = read_multiline(stream)?;

    // 解析邮件
    let parsed = mailparse::parse_mail(&mail_content)
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
        .unwrap_or_else(|| format!("<pop3-{}@local>", msg_num));

    let (content_text, content_html, attachments) = extract_content(&parsed);

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
