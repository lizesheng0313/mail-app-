use serde::{Deserialize, Serialize};

/// 邮箱登录配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxConfig {
    pub email: String,
    pub password: String,
    pub protocol: String, // "imap" 或 "pop3"
    pub host: Option<String>,
    pub port: Option<u16>,
}

/// 常见邮箱服务商配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub imap_host: &'static str,
    pub imap_port: u16,
    pub pop3_host: &'static str,
    pub pop3_port: u16,
}

/// 获取邮箱服务商的服务器配置
pub fn get_server_config(domain: &str) -> Option<ServerConfig> {
    match domain {
        "qq.com" => Some(ServerConfig {
            imap_host: "imap.qq.com",
            imap_port: 993,
            pop3_host: "pop.qq.com",
            pop3_port: 995,
        }),
        "163.com" => Some(ServerConfig {
            imap_host: "imap.163.com",
            imap_port: 993,
            pop3_host: "pop.163.com",
            pop3_port: 995,
        }),
        "126.com" => Some(ServerConfig {
            imap_host: "imap.126.com",
            imap_port: 993,
            pop3_host: "pop.126.com",
            pop3_port: 995,
        }),
        "gmail.com" => Some(ServerConfig {
            imap_host: "imap.gmail.com",
            imap_port: 993,
            pop3_host: "pop.gmail.com",
            pop3_port: 995,
        }),
        "outlook.com" | "hotmail.com" => Some(ServerConfig {
            imap_host: "outlook.office365.com",
            imap_port: 993,
            pop3_host: "outlook.office365.com",
            pop3_port: 995,
        }),
        "yahoo.com" => Some(ServerConfig {
            imap_host: "imap.mail.yahoo.com",
            imap_port: 993,
            pop3_host: "pop.mail.yahoo.com",
            pop3_port: 995,
        }),
        "sina.com" | "sina.cn" => Some(ServerConfig {
            imap_host: "imap.sina.com",
            imap_port: 993,
            pop3_host: "pop.sina.com",
            pop3_port: 995,
        }),
        "sohu.com" => Some(ServerConfig {
            imap_host: "imap.sohu.com",
            imap_port: 993,
            pop3_host: "pop3.sohu.com",
            pop3_port: 995,
        }),
        "189.cn" => Some(ServerConfig {
            imap_host: "imap.189.cn",
            imap_port: 993,
            pop3_host: "pop.189.cn",
            pop3_port: 995,
        }),
        _ => None,
    }
}

/// 邮件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailData {
    pub message_id: String,
    pub subject: String,
    pub from_addr: String,
    pub to_addr: String,
    pub content_text: String,
    pub content_html: String,
    pub email_date_ms: i64,
    pub received_at_ms: i64,
}

/// 登录验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub success: bool,
    pub message: String,
    pub host: Option<String>,
    pub port: Option<u16>,
}

/// 收取邮件结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchResult {
    pub success: bool,
    pub message: String,
    pub emails: Vec<EmailData>,
    pub count: usize,
}
