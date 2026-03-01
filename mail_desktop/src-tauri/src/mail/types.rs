use log::info;
use serde::{Deserialize, Serialize};
use std::process::Command;

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

/// 获取邮箱服务商的服务器配置（先查已知列表，再通过 MX 记录自动检测）
pub fn get_server_config(domain: &str) -> Option<ServerConfig> {
    // 1. 先查已知域名
    if let Some(config) = get_known_server_config(domain) {
        return Some(config);
    }

    // 2. 未知域名，通过 MX 记录自动检测
    info!("域名 {} 不在已知列表，尝试 MX 记录自动检测", domain);
    detect_server_config_by_mx(domain)
}

/// 已知邮箱域名的服务器配置
fn get_known_server_config(domain: &str) -> Option<ServerConfig> {
    match domain {
        // 腾讯
        "qq.com" | "foxmail.com" => Some(ServerConfig {
            imap_host: "imap.qq.com",
            imap_port: 993,
            pop3_host: "pop.qq.com",
            pop3_port: 995,
        }),
        // 网易
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
        "yeah.net" => Some(ServerConfig {
            imap_host: "imap.yeah.net",
            imap_port: 993,
            pop3_host: "pop.yeah.net",
            pop3_port: 995,
        }),
        // Google
        "gmail.com" => Some(ServerConfig {
            imap_host: "imap.gmail.com",
            imap_port: 993,
            pop3_host: "pop.gmail.com",
            pop3_port: 995,
        }),
        // 微软
        "outlook.com" | "hotmail.com" | "live.com" | "live.cn" => Some(ServerConfig {
            imap_host: "outlook.office365.com",
            imap_port: 993,
            pop3_host: "outlook.office365.com",
            pop3_port: 995,
        }),
        // 雅虎
        "yahoo.com" | "yahoo.cn" | "yahoo.com.cn" => Some(ServerConfig {
            imap_host: "imap.mail.yahoo.com",
            imap_port: 993,
            pop3_host: "pop.mail.yahoo.com",
            pop3_port: 995,
        }),
        // 新浪
        "sina.com" | "sina.cn" | "vip.sina.com" => Some(ServerConfig {
            imap_host: "imap.sina.com",
            imap_port: 993,
            pop3_host: "pop.sina.com",
            pop3_port: 995,
        }),
        // 搜狐
        "sohu.com" => Some(ServerConfig {
            imap_host: "imap.sohu.com",
            imap_port: 993,
            pop3_host: "pop3.sohu.com",
            pop3_port: 995,
        }),
        // 天翼（中国电信）
        "189.cn" => Some(ServerConfig {
            imap_host: "imap.189.cn",
            imap_port: 993,
            pop3_host: "pop.189.cn",
            pop3_port: 995,
        }),
        // 移动（139邮箱）
        "139.com" => Some(ServerConfig {
            imap_host: "imap.139.com",
            imap_port: 993,
            pop3_host: "pop.139.com",
            pop3_port: 995,
        }),
        // 阿里个人邮箱
        "aliyun.com" => Some(ServerConfig {
            imap_host: "imap.aliyun.com",
            imap_port: 993,
            pop3_host: "pop.aliyun.com",
            pop3_port: 995,
        }),
        // Tom 邮箱
        "tom.com" => Some(ServerConfig {
            imap_host: "imap.tom.com",
            imap_port: 993,
            pop3_host: "pop.tom.com",
            pop3_port: 995,
        }),
        // iCloud
        "icloud.com" | "me.com" | "mac.com" => Some(ServerConfig {
            imap_host: "imap.mail.me.com",
            imap_port: 993,
            pop3_host: "pop.mail.me.com",
            pop3_port: 995,
        }),
        _ => None,
    }
}

/// 通过 DNS MX 记录自动检测邮件服务器配置
fn detect_server_config_by_mx(domain: &str) -> Option<ServerConfig> {
    let mx_hosts = lookup_mx_records(domain);
    if mx_hosts.is_empty() {
        info!("未查询到 {} 的 MX 记录", domain);
        return None;
    }

    info!("查询到 MX 记录: {:?}", mx_hosts);

    for mx in &mx_hosts {
        let mx_lower = mx.to_lowercase();

        // 网易企业邮箱 (MX: qiye163mx01.mxmail.netease.com)
        if mx_lower.contains("netease.com") {
            info!("MX 匹配: 网易企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.qiye.163.com",
                imap_port: 993,
                pop3_host: "pop.qiye.163.com",
                pop3_port: 995,
            });
        }
        // 腾讯企业邮箱 (MX: mxbiz1.qq.com / cloudmx.qq.com)
        if mx_lower.contains("qq.com") || mx_lower.contains("tencent.com") {
            info!("MX 匹配: 腾讯企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.exmail.qq.com",
                imap_port: 993,
                pop3_host: "pop.exmail.qq.com",
                pop3_port: 995,
            });
        }
        // Google Workspace (MX: aspmx.l.google.com)
        if mx_lower.contains("google.com") || mx_lower.contains("googlemail.com") {
            info!("MX 匹配: Google Workspace");
            return Some(ServerConfig {
                imap_host: "imap.gmail.com",
                imap_port: 993,
                pop3_host: "pop.gmail.com",
                pop3_port: 995,
            });
        }
        // Microsoft 365 / Exchange Online (MX: *.mail.protection.outlook.com)
        if mx_lower.contains("outlook") || mx_lower.contains("microsoft.com") || mx_lower.contains("office365") {
            info!("MX 匹配: Microsoft 365");
            return Some(ServerConfig {
                imap_host: "outlook.office365.com",
                imap_port: 993,
                pop3_host: "outlook.office365.com",
                pop3_port: 995,
            });
        }
        // 阿里企业邮箱 / 万网企业邮 (MX: mx1.mxhichina.com / mail.aliyun.com)
        if mx_lower.contains("mxhichina.com") || mx_lower.contains("aliyun.com") || mx_lower.contains("alibaba-inc.com") {
            info!("MX 匹配: 阿里企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.mxhichina.com",
                imap_port: 993,
                pop3_host: "pop.mxhichina.com",
                pop3_port: 995,
            });
        }
        // 263企业邮箱 (MX: mx.263.net)
        if mx_lower.contains("263.net") {
            info!("MX 匹配: 263企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.263.net",
                imap_port: 993,
                pop3_host: "pop.263.net",
                pop3_port: 995,
            });
        }
        // Zoho 企业邮箱 (MX: mx.zoho.com)
        if mx_lower.contains("zoho.com") || mx_lower.contains("zohomail.com") {
            info!("MX 匹配: Zoho 企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.zoho.com",
                imap_port: 993,
                pop3_host: "pop.zoho.com",
                pop3_port: 995,
            });
        }
        // Yandex 企业邮箱 (MX: mx.yandex.ru / mx.yandex.net)
        if mx_lower.contains("yandex.ru") || mx_lower.contains("yandex.net") {
            info!("MX 匹配: Yandex 企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.yandex.com",
                imap_port: 993,
                pop3_host: "pop.yandex.com",
                pop3_port: 995,
            });
        }
        // Fastmail (MX: in1-smtp.messagingengine.com)
        if mx_lower.contains("messagingengine.com") || mx_lower.contains("fastmail") {
            info!("MX 匹配: Fastmail");
            return Some(ServerConfig {
                imap_host: "imap.fastmail.com",
                imap_port: 993,
                pop3_host: "pop.fastmail.com",
                pop3_port: 995,
            });
        }
        // iCloud / Apple (MX: mx01.mail.icloud.com)
        if mx_lower.contains("icloud.com") || mx_lower.contains("apple.com") {
            info!("MX 匹配: iCloud");
            return Some(ServerConfig {
                imap_host: "imap.mail.me.com",
                imap_port: 993,
                pop3_host: "pop.mail.me.com",
                pop3_port: 995,
            });
        }
        // 35互联 / 云邮 (MX: mx1.yunyou.top)
        if mx_lower.contains("yunyou.top") || mx_lower.contains("35.cn") || mx_lower.contains("35.com") {
            info!("MX 匹配: 35互联企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.35.cn",
                imap_port: 993,
                pop3_host: "pop.35.cn",
                pop3_port: 995,
            });
        }
        // GoDaddy / Secureserver (MX: mailstore1.secureserver.net)
        if mx_lower.contains("secureserver.net") {
            info!("MX 匹配: GoDaddy 企业邮箱");
            return Some(ServerConfig {
                imap_host: "imap.secureserver.net",
                imap_port: 993,
                pop3_host: "pop.secureserver.net",
                pop3_port: 995,
            });
        }
        // Amazon WorkMail (MX: inbound-smtp.*.amazonaws.com)
        if mx_lower.contains("amazonaws.com") {
            info!("MX 匹配: Amazon WorkMail");
            return Some(ServerConfig {
                imap_host: "imap.mail.us-east-1.awsapps.com",
                imap_port: 993,
                pop3_host: "pop.mail.us-east-1.awsapps.com",
                pop3_port: 995,
            });
        }
    }

    info!("MX 记录无法匹配已知邮件服务商: {:?}", mx_hosts);
    None
}

/// 查询域名的 MX 记录
fn lookup_mx_records(domain: &str) -> Vec<String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("nslookup")
            .args(["-type=MX", domain])
            .output()
    } else {
        Command::new("dig")
            .args(["MX", domain, "+short"])
            .output()
    };

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse_mx_output(&stdout)
        }
        Err(e) => {
            info!("MX 查询命令执行失败: {}", e);
            Vec::new()
        }
    }
}

/// 解析 MX 查询输出
fn parse_mx_output(output: &str) -> Vec<String> {
    let mut hosts = Vec::new();
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(host) = extract_mx_host(line) {
            hosts.push(host);
        }
    }
    hosts
}

/// 从单行中提取 MX 主机名
fn extract_mx_host(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // dig +short 格式: "5 qiye163mx01.mxmail.netease.com."
    if parts.len() == 2 {
        if parts[0].parse::<u16>().is_ok() {
            return Some(parts[1].trim_end_matches('.').to_string());
        }
    }

    // nslookup 格式: "ysstech.com  mail exchanger = 5 qiye163mx01.mxmail.netease.com."
    if line.contains("mail exchanger") {
        if let Some(last) = parts.last() {
            return Some(last.trim_end_matches('.').to_string());
        }
    }

    None
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
    pub protocol: Option<String>,
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
