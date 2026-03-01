use crate::mail;
use crate::mail::types::{get_server_config, EmailData, FetchResult, LoginResult};
use log::{error, info};
use serde::{Deserialize, Serialize};

/// 添加外部邮箱请求
#[derive(Debug, Deserialize)]
pub struct AddExternalMailboxRequest {
    pub email: String,
    pub password: String,
    pub protocol: String, // "imap" 或 "pop3"
    pub host: Option<String>,
    pub port: Option<u16>,
}

/// 收取邮件请求
#[derive(Debug, Deserialize)]
pub struct FetchEmailsRequest {
    pub mailbox_id: i64,
    pub email: String,
    pub password: String,
    pub protocol: String,
    pub host: String,
    pub port: u16,
    pub token: String, // 用于同步到远程服务器
}

/// 同步邮件请求（发送到远程服务器）
#[derive(Debug, Serialize)]
pub struct SyncEmailsRequest {
    pub mailbox_id: i64,
    pub emails: Vec<EmailData>,
}

/// API 响应
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

/// 添加外部邮箱（验证登录）
/// 前端调用：invoke('add_external_mailbox', { email, password, protocol, host?, port? })
/// protocol: "imap" | "pop3" | "auto"（自动检测，先试IMAP再试POP3）
#[tauri::command]
pub async fn add_external_mailbox(
    email: String,
    password: String,
    protocol: String,
    host: Option<String>,
    port: Option<u16>,
) -> Result<LoginResult, String> {
    info!("收到添加外部邮箱请求: {} ({})", email, protocol);

    // 获取本地出口 IP（用于日志记录）
    match get_local_ip().await {
        Ok(ip) => info!("🌐 使用本地 IP 登录邮箱: {}", ip),
        Err(e) => info!("⚠️  无法获取本地 IP: {}", e),
    }

    // 获取邮箱域名
    let domain = email
        .split('@')
        .nth(1)
        .ok_or("无效的邮箱地址")?;

    let proto = protocol.to_lowercase();

    // 用户指定了自定义服务器
    if let (Some(h), Some(p)) = (host.clone(), port) {
        let result = if proto == "imap" {
            mail::imap::verify_login(&email, &password, &h, p).await?
        } else {
            mail::pop3::verify_login(&email, &password, &h, p).await?
        };
        return Ok(result);
    }

    // 自动检测服务器配置
    let config = get_server_config(domain)
        .ok_or(format!("无法识别邮箱 {} 的服务器配置，请手动填写服务器地址", domain))?;

    if proto == "auto" {
        // 自动模式：先试 IMAP，失败再试 POP3
        info!("自动检测协议：先尝试 IMAP {}:{}", config.imap_host, config.imap_port);
        let imap_result = mail::imap::verify_login(&email, &password, config.imap_host, config.imap_port).await?;
        if imap_result.success {
            return Ok(imap_result);
        }

        info!("IMAP 登录失败({}), 尝试 POP3 {}:{}", imap_result.message, config.pop3_host, config.pop3_port);
        let pop3_result = mail::pop3::verify_login(&email, &password, config.pop3_host, config.pop3_port).await?;
        if pop3_result.success {
            return Ok(pop3_result);
        }

        // 两个都失败
        Ok(LoginResult {
            success: false,
            message: format!("IMAP 和 POP3 均登录失败。IMAP: {}; POP3: {}", imap_result.message, pop3_result.message),
            protocol: None,
            host: None,
            port: None,
        })
    } else if proto == "imap" {
        Ok(mail::imap::verify_login(&email, &password, config.imap_host, config.imap_port).await?)
    } else {
        Ok(mail::pop3::verify_login(&email, &password, config.pop3_host, config.pop3_port).await?)
    }
}

/// 获取本地出口 IP（用于验证）
async fn get_local_ip() -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.ipify.org?format=text")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("获取 IP 失败: {}", e))?;
    
    let ip = response
        .text()
        .await
        .map_err(|e| format!("解析 IP 失败: {}", e))?;
    
    Ok(ip)
}

/// 收取邮件
/// 前端调用：invoke('fetch_emails', { mailboxId, email, password, protocol, host?, port?, token, serverUrl })
#[tauri::command]
pub async fn fetch_emails(
    mailbox_id: i64,
    email: String,
    password: String,
    protocol: String,
    host: Option<String>,
    port: Option<u16>,
    token: String,
    server_url: String,
) -> Result<FetchResult, String> {
    // 自动检测服务器配置（当 host/port 缺失时）
    let (final_host, final_port, final_protocol) = if let (Some(h), Some(p)) = (host.as_deref().filter(|s| !s.is_empty()), port) {
        (h.to_string(), p, protocol.clone())
    } else {
        let domain = email.split('@').nth(1).ok_or("无效的邮箱地址")?;
        let config = get_server_config(domain)
            .ok_or(format!("无法识别邮箱 {} 的服务器配置", domain))?;

        if protocol.to_lowercase() == "imap" || protocol.to_lowercase() == "auto" {
            (config.imap_host.to_string(), config.imap_port, "imap".to_string())
        } else {
            (config.pop3_host.to_string(), config.pop3_port, "pop3".to_string())
        }
    };

    info!("收到收取邮件请求: {} ({}) -> {}:{}", email, final_protocol, final_host, final_port);
    info!("密码长度: {}, token长度: {}, serverUrl: {}", password.len(), token.len(), server_url);

    // 本地收取邮件
    let result = if final_protocol.to_lowercase() == "imap" {
        match mail::imap::fetch_emails(&email, &password, &final_host, final_port, 50).await {
            Ok(r) => r,
            Err(e) => {
                error!("IMAP 收取失败: {}", e);
                return Err(e);
            }
        }
    } else {
        match mail::pop3::fetch_emails(&email, &password, &final_host, final_port, 50).await {
            Ok(r) => r,
            Err(e) => {
                error!("POP3 收取失败: {}", e);
                return Err(e);
            }
        }
    };

    if result.success && !result.emails.is_empty() {
        // 同步到远程服务器
        if let Err(e) = sync_emails_to_server(&server_url, &token, mailbox_id, &result.emails).await {
            error!("同步邮件到服务器失败: {}", e);
            // 即使同步失败也返回收取结果，让用户知道邮件已收取
        }
    }

    Ok(result)
}

/// 同步邮件到远程服务器
async fn sync_emails_to_server(
    server_url: &str,
    token: &str,
    mailbox_id: i64,
    emails: &[EmailData],
) -> Result<(), String> {
    info!("同步 {} 封邮件到服务器", emails.len());

    let client = reqwest::Client::new();
    let url = format!("{}/unified-emails/external-emails/sync", server_url);

    let request_body = SyncEmailsRequest {
        mailbox_id,
        emails: emails.to_vec(),
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if response.status().is_success() {
        info!("✅ 邮件同步成功");
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("服务器返回错误 {}: {}", status, body))
    }
}

/// 测试连接（用于调试）
#[tauri::command]
pub async fn test_connection() -> Result<String, String> {
    Ok("Tauri 后端连接正常".to_string())
}

/// 检查是否在 Tauri 环境
#[tauri::command]
pub fn is_tauri() -> bool {
    true
}
