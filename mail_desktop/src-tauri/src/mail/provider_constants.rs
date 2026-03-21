#[derive(Debug, Clone, Copy)]
pub struct MailServerProfile {
    pub imap_host: &'static str,
    pub imap_port: u16,
    pub pop3_host: &'static str,
    pub pop3_port: u16,
    pub smtp_candidates: &'static [(&'static str, u16)],
}

#[derive(Debug, Clone, Copy)]
pub struct KnownMailProvider {
    pub domains: &'static [&'static str],
    pub profile: MailServerProfile,
}

#[derive(Debug, Clone, Copy)]
pub struct HostedMailProvider {
    pub name: &'static str,
    pub mx_patterns: &'static [&'static str],
    pub profile: MailServerProfile,
}

const SMTP_QQ: &[(&str, u16)] = &[("smtp.qq.com", 465), ("smtp.qq.com", 587)];
const SMTP_163: &[(&str, u16)] = &[("smtp.163.com", 465), ("smtp.163.com", 587)];
const SMTP_126: &[(&str, u16)] = &[("smtp.126.com", 465), ("smtp.126.com", 587)];
const SMTP_YEAH: &[(&str, u16)] = &[("smtp.yeah.net", 465), ("smtp.yeah.net", 587)];
const SMTP_163_NET: &[(&str, u16)] = &[("smtp.163.net", 465), ("smtp.163.net", 587)];
const SMTP_GMAIL: &[(&str, u16)] = &[("smtp.gmail.com", 587), ("smtp.gmail.com", 465)];
const SMTP_OFFICE365: &[(&str, u16)] = &[("smtp.office365.com", 587)];
const SMTP_YAHOO: &[(&str, u16)] = &[("smtp.mail.yahoo.com", 465), ("smtp.mail.yahoo.com", 587)];
const SMTP_SINA: &[(&str, u16)] = &[("smtp.sina.com", 465), ("smtp.sina.com", 587)];
const SMTP_SOHU: &[(&str, u16)] = &[("smtp.sohu.com", 465), ("smtp.sohu.com", 587)];
const SMTP_189: &[(&str, u16)] = &[("smtp.189.cn", 465), ("smtp.189.cn", 587)];
const SMTP_139: &[(&str, u16)] = &[("smtp.139.com", 465), ("smtp.139.com", 587)];
const SMTP_WO: &[(&str, u16)] = &[("smtp.wo.cn", 465), ("smtp.wo.cn", 587)];
const SMTP_ALIYUN: &[(&str, u16)] = &[("smtp.aliyun.com", 465), ("smtp.aliyun.com", 587)];
const SMTP_21CN: &[(&str, u16)] = &[("smtp.21cn.com", 465), ("smtp.21cn.com", 587)];
const SMTP_TOM: &[(&str, u16)] = &[("smtp.tom.com", 465), ("smtp.tom.com", 587)];
const SMTP_ICLOUD: &[(&str, u16)] = &[("smtp.mail.me.com", 587), ("smtp.mail.me.com", 465)];
const SMTP_ZOHO_COM: &[(&str, u16)] = &[("smtp.zoho.com", 465), ("smtp.zoho.com", 587)];
const SMTP_ZOHO_EU: &[(&str, u16)] = &[("smtp.zoho.eu", 465), ("smtp.zoho.eu", 587)];
const SMTP_ZOHO_IN: &[(&str, u16)] = &[("smtp.zoho.in", 465), ("smtp.zoho.in", 587)];
const SMTP_ZOHO_AU: &[(&str, u16)] = &[("smtp.zoho.com.au", 465), ("smtp.zoho.com.au", 587)];
const SMTP_ZOHO_JP: &[(&str, u16)] = &[("smtp.zoho.jp", 465), ("smtp.zoho.jp", 587)];
const SMTP_ZOHO_SA: &[(&str, u16)] = &[("smtp.zoho.sa", 465), ("smtp.zoho.sa", 587)];
const SMTP_AOL: &[(&str, u16)] = &[("smtp.aol.com", 465), ("smtp.aol.com", 587)];
const SMTP_FASTMAIL: &[(&str, u16)] = &[("smtp.fastmail.com", 465), ("smtp.fastmail.com", 587)];
const SMTP_GMX: &[(&str, u16)] = &[("mail.gmx.com", 587), ("mail.gmx.com", 465)];
const SMTP_WEB_DE: &[(&str, u16)] = &[("smtp.web.de", 587), ("smtp.web.de", 465)];
const SMTP_MAIL_COM: &[(&str, u16)] = &[("smtp.mail.com", 465), ("smtp.mail.com", 587)];
const SMTP_MAIL_RU: &[(&str, u16)] = &[("smtp.mail.ru", 465), ("smtp.mail.ru", 587)];
const SMTP_NETEASE_ENT: &[(&str, u16)] = &[("smtp.qiye.163.com", 994), ("smtp.qiye.163.com", 465)];
const SMTP_EXMAIL_QQ: &[(&str, u16)] = &[("smtp.exmail.qq.com", 465), ("smtp.exmail.qq.com", 587)];
const SMTP_ALIYUN_ENT: &[(&str, u16)] = &[("smtp.qiye.aliyun.com", 465), ("smtp.qiye.aliyun.com", 587)];
const SMTP_RACKSPACE: &[(&str, u16)] = &[("secure.emailsrvr.com", 465), ("secure.emailsrvr.com", 587)];
const SMTP_MIGADU: &[(&str, u16)] = &[("smtp.migadu.com", 465), ("smtp.migadu.com", 587)];
const SMTP_TITAN: &[(&str, u16)] = &[("smtp.titan.email", 465), ("smtp.titan.email", 587)];
const SMTP_PRIVATEEMAIL: &[(&str, u16)] = &[("mail.privateemail.com", 465), ("mail.privateemail.com", 587)];

pub const KNOWN_MAIL_PROVIDERS: &[KnownMailProvider] = &[
    KnownMailProvider {
        domains: &["qq.com", "foxmail.com"],
        profile: MailServerProfile {
            imap_host: "imap.qq.com",
            imap_port: 993,
            pop3_host: "pop.qq.com",
            pop3_port: 995,
            smtp_candidates: SMTP_QQ,
        },
    },
    KnownMailProvider {
        domains: &["163.com", "vip.163.com"],
        profile: MailServerProfile {
            imap_host: "imap.163.com",
            imap_port: 993,
            pop3_host: "pop.163.com",
            pop3_port: 995,
            smtp_candidates: SMTP_163,
        },
    },
    KnownMailProvider {
        domains: &["126.com", "vip.126.com"],
        profile: MailServerProfile {
            imap_host: "imap.126.com",
            imap_port: 993,
            pop3_host: "pop.126.com",
            pop3_port: 995,
            smtp_candidates: SMTP_126,
        },
    },
    KnownMailProvider {
        domains: &["yeah.net"],
        profile: MailServerProfile {
            imap_host: "imap.yeah.net",
            imap_port: 993,
            pop3_host: "pop.yeah.net",
            pop3_port: 995,
            smtp_candidates: SMTP_YEAH,
        },
    },
    KnownMailProvider {
        domains: &["163.net"],
        profile: MailServerProfile {
            imap_host: "imap.163.net",
            imap_port: 993,
            pop3_host: "pop.163.net",
            pop3_port: 995,
            smtp_candidates: SMTP_163_NET,
        },
    },
    KnownMailProvider {
        domains: &["gmail.com", "googlemail.com"],
        profile: MailServerProfile {
            imap_host: "imap.gmail.com",
            imap_port: 993,
            pop3_host: "pop.gmail.com",
            pop3_port: 995,
            smtp_candidates: SMTP_GMAIL,
        },
    },
    KnownMailProvider {
        domains: &["outlook.com", "hotmail.com", "live.com", "live.cn", "msn.com"],
        profile: MailServerProfile {
            imap_host: "outlook.office365.com",
            imap_port: 993,
            pop3_host: "outlook.office365.com",
            pop3_port: 995,
            smtp_candidates: SMTP_OFFICE365,
        },
    },
    KnownMailProvider {
        domains: &["yahoo.com", "yahoo.cn", "yahoo.com.cn", "ymail.com"],
        profile: MailServerProfile {
            imap_host: "imap.mail.yahoo.com",
            imap_port: 993,
            pop3_host: "pop.mail.yahoo.com",
            pop3_port: 995,
            smtp_candidates: SMTP_YAHOO,
        },
    },
    KnownMailProvider {
        domains: &["sina.com", "sina.cn", "vip.sina.com"],
        profile: MailServerProfile {
            imap_host: "imap.sina.com",
            imap_port: 993,
            pop3_host: "pop.sina.com",
            pop3_port: 995,
            smtp_candidates: SMTP_SINA,
        },
    },
    KnownMailProvider {
        domains: &["sohu.com"],
        profile: MailServerProfile {
            imap_host: "imap.sohu.com",
            imap_port: 993,
            pop3_host: "pop3.sohu.com",
            pop3_port: 995,
            smtp_candidates: SMTP_SOHU,
        },
    },
    KnownMailProvider {
        domains: &["189.cn"],
        profile: MailServerProfile {
            imap_host: "imap.189.cn",
            imap_port: 993,
            pop3_host: "pop.189.cn",
            pop3_port: 995,
            smtp_candidates: SMTP_189,
        },
    },
    KnownMailProvider {
        domains: &["139.com"],
        profile: MailServerProfile {
            imap_host: "imap.139.com",
            imap_port: 993,
            pop3_host: "pop.139.com",
            pop3_port: 995,
            smtp_candidates: SMTP_139,
        },
    },
    KnownMailProvider {
        domains: &["wo.cn"],
        profile: MailServerProfile {
            imap_host: "imap.wo.cn",
            imap_port: 993,
            pop3_host: "pop.wo.cn",
            pop3_port: 995,
            smtp_candidates: SMTP_WO,
        },
    },
    KnownMailProvider {
        domains: &["aliyun.com"],
        profile: MailServerProfile {
            imap_host: "imap.aliyun.com",
            imap_port: 993,
            pop3_host: "pop.aliyun.com",
            pop3_port: 995,
            smtp_candidates: SMTP_ALIYUN,
        },
    },
    KnownMailProvider {
        domains: &["21cn.com"],
        profile: MailServerProfile {
            imap_host: "imap.21cn.com",
            imap_port: 993,
            pop3_host: "pop.21cn.com",
            pop3_port: 995,
            smtp_candidates: SMTP_21CN,
        },
    },
    KnownMailProvider {
        domains: &["tom.com"],
        profile: MailServerProfile {
            imap_host: "imap.tom.com",
            imap_port: 993,
            pop3_host: "pop.tom.com",
            pop3_port: 995,
            smtp_candidates: SMTP_TOM,
        },
    },
    KnownMailProvider {
        domains: &["icloud.com", "me.com", "mac.com"],
        profile: MailServerProfile {
            imap_host: "imap.mail.me.com",
            imap_port: 993,
            pop3_host: "pop.mail.me.com",
            pop3_port: 995,
            smtp_candidates: SMTP_ICLOUD,
        },
    },
    KnownMailProvider {
        domains: &["zoho.com", "zohomail.com"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.com",
            imap_port: 993,
            pop3_host: "pop.zoho.com",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_COM,
        },
    },
    KnownMailProvider {
        domains: &["zoho.eu"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.eu",
            imap_port: 993,
            pop3_host: "pop.zoho.eu",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_EU,
        },
    },
    KnownMailProvider {
        domains: &["zoho.in"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.in",
            imap_port: 993,
            pop3_host: "pop.zoho.in",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_IN,
        },
    },
    KnownMailProvider {
        domains: &["zoho.com.au"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.com.au",
            imap_port: 993,
            pop3_host: "pop.zoho.com.au",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_AU,
        },
    },
    KnownMailProvider {
        domains: &["zoho.jp"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.jp",
            imap_port: 993,
            pop3_host: "pop.zoho.jp",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_JP,
        },
    },
    KnownMailProvider {
        domains: &["zoho.sa"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.sa",
            imap_port: 993,
            pop3_host: "pop.zoho.sa",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_SA,
        },
    },
    KnownMailProvider {
        domains: &["aol.com"],
        profile: MailServerProfile {
            imap_host: "imap.aol.com",
            imap_port: 993,
            pop3_host: "pop.aol.com",
            pop3_port: 995,
            smtp_candidates: SMTP_AOL,
        },
    },
    KnownMailProvider {
        domains: &["fastmail.com", "fastmail.fm"],
        profile: MailServerProfile {
            imap_host: "imap.fastmail.com",
            imap_port: 993,
            pop3_host: "pop.fastmail.com",
            pop3_port: 995,
            smtp_candidates: SMTP_FASTMAIL,
        },
    },
    KnownMailProvider {
        domains: &["gmx.com", "gmx.net", "gmx.de", "gmx.at", "gmx.ch"],
        profile: MailServerProfile {
            imap_host: "imap.gmx.com",
            imap_port: 993,
            pop3_host: "pop.gmx.com",
            pop3_port: 995,
            smtp_candidates: SMTP_GMX,
        },
    },
    KnownMailProvider {
        domains: &["web.de"],
        profile: MailServerProfile {
            imap_host: "imap.web.de",
            imap_port: 993,
            pop3_host: "pop3.web.de",
            pop3_port: 995,
            smtp_candidates: SMTP_WEB_DE,
        },
    },
    KnownMailProvider {
        domains: &["mail.com"],
        profile: MailServerProfile {
            imap_host: "imap.mail.com",
            imap_port: 993,
            pop3_host: "pop.mail.com",
            pop3_port: 995,
            smtp_candidates: SMTP_MAIL_COM,
        },
    },
    KnownMailProvider {
        domains: &["mail.ru", "bk.ru", "inbox.ru", "list.ru"],
        profile: MailServerProfile {
            imap_host: "imap.mail.ru",
            imap_port: 993,
            pop3_host: "pop.mail.ru",
            pop3_port: 995,
            smtp_candidates: SMTP_MAIL_RU,
        },
    },
];

pub const HOSTED_MAIL_PROVIDERS: &[HostedMailProvider] = &[
    HostedMailProvider {
        name: "Zoho AU",
        mx_patterns: &["zoho.com.au"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.com.au",
            imap_port: 993,
            pop3_host: "pop.zoho.com.au",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_AU,
        },
    },
    HostedMailProvider {
        name: "Zoho EU",
        mx_patterns: &["zoho.eu"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.eu",
            imap_port: 993,
            pop3_host: "pop.zoho.eu",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_EU,
        },
    },
    HostedMailProvider {
        name: "Zoho IN",
        mx_patterns: &["zoho.in"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.in",
            imap_port: 993,
            pop3_host: "pop.zoho.in",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_IN,
        },
    },
    HostedMailProvider {
        name: "Zoho JP",
        mx_patterns: &["zoho.jp"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.jp",
            imap_port: 993,
            pop3_host: "pop.zoho.jp",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_JP,
        },
    },
    HostedMailProvider {
        name: "Zoho SA",
        mx_patterns: &["zoho.sa"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.sa",
            imap_port: 993,
            pop3_host: "pop.zoho.sa",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_SA,
        },
    },
    HostedMailProvider {
        name: "Zoho",
        mx_patterns: &["zoho.com", "zohomail.com"],
        profile: MailServerProfile {
            imap_host: "imap.zoho.com",
            imap_port: 993,
            pop3_host: "pop.zoho.com",
            pop3_port: 995,
            smtp_candidates: SMTP_ZOHO_COM,
        },
    },
    HostedMailProvider {
        name: "网易企业邮箱",
        mx_patterns: &["netease.com", "ym.163.com", "qiye.163.com", "mxmail.netease.com"],
        profile: MailServerProfile {
            imap_host: "imap.qiye.163.com",
            imap_port: 993,
            pop3_host: "pop.qiye.163.com",
            pop3_port: 995,
            smtp_candidates: SMTP_NETEASE_ENT,
        },
    },
    HostedMailProvider {
        name: "腾讯企业邮箱",
        mx_patterns: &["exmail.qq.com", "mxbiz.qq.com"],
        profile: MailServerProfile {
            imap_host: "imap.exmail.qq.com",
            imap_port: 993,
            pop3_host: "pop.exmail.qq.com",
            pop3_port: 995,
            smtp_candidates: SMTP_EXMAIL_QQ,
        },
    },
    HostedMailProvider {
        name: "阿里云企业邮箱",
        mx_patterns: &["qiye.aliyun.com", "mxn.qiye.aliyun"],
        profile: MailServerProfile {
            imap_host: "imap.qiye.aliyun.com",
            imap_port: 993,
            pop3_host: "pop3.qiye.aliyun.com",
            pop3_port: 995,
            smtp_candidates: SMTP_ALIYUN_ENT,
        },
    },
    HostedMailProvider {
        name: "Google Workspace",
        mx_patterns: &["aspmx.l.google.com", "google.com", "googlemail.com"],
        profile: MailServerProfile {
            imap_host: "imap.gmail.com",
            imap_port: 993,
            pop3_host: "pop.gmail.com",
            pop3_port: 995,
            smtp_candidates: SMTP_GMAIL,
        },
    },
    HostedMailProvider {
        name: "Microsoft 365",
        mx_patterns: &["protection.outlook.com", "mail.protection.outlook.com", "outlook.com"],
        profile: MailServerProfile {
            imap_host: "outlook.office365.com",
            imap_port: 993,
            pop3_host: "outlook.office365.com",
            pop3_port: 995,
            smtp_candidates: SMTP_OFFICE365,
        },
    },
    HostedMailProvider {
        name: "Fastmail",
        mx_patterns: &["messagingengine.com"],
        profile: MailServerProfile {
            imap_host: "imap.fastmail.com",
            imap_port: 993,
            pop3_host: "pop.fastmail.com",
            pop3_port: 995,
            smtp_candidates: SMTP_FASTMAIL,
        },
    },
    HostedMailProvider {
        name: "Rackspace Email",
        mx_patterns: &["emailsrvr.com"],
        profile: MailServerProfile {
            imap_host: "secure.emailsrvr.com",
            imap_port: 993,
            pop3_host: "secure.emailsrvr.com",
            pop3_port: 995,
            smtp_candidates: SMTP_RACKSPACE,
        },
    },
    HostedMailProvider {
        name: "Migadu",
        mx_patterns: &["migadu.com"],
        profile: MailServerProfile {
            imap_host: "imap.migadu.com",
            imap_port: 993,
            pop3_host: "pop.migadu.com",
            pop3_port: 995,
            smtp_candidates: SMTP_MIGADU,
        },
    },
    HostedMailProvider {
        name: "Titan",
        mx_patterns: &["titan.email"],
        profile: MailServerProfile {
            imap_host: "imap.titan.email",
            imap_port: 993,
            pop3_host: "pop.titan.email",
            pop3_port: 995,
            smtp_candidates: SMTP_TITAN,
        },
    },
    HostedMailProvider {
        name: "Private Email",
        mx_patterns: &["privateemail.com"],
        profile: MailServerProfile {
            imap_host: "mail.privateemail.com",
            imap_port: 993,
            pop3_host: "mail.privateemail.com",
            pop3_port: 995,
            smtp_candidates: SMTP_PRIVATEEMAIL,
        },
    },
];

pub fn find_known_provider(domain: &str) -> Option<MailServerProfile> {
    let normalized = domain.trim().to_ascii_lowercase();
    KNOWN_MAIL_PROVIDERS
        .iter()
        .find(|provider| provider.domains.iter().any(|item| *item == normalized))
        .map(|provider| provider.profile)
}

pub fn find_hosted_provider_by_mx(mx_host: &str) -> Option<&'static HostedMailProvider> {
    let normalized = mx_host.trim().to_ascii_lowercase();
    HOSTED_MAIL_PROVIDERS
        .iter()
        .find(|provider| provider.mx_patterns.iter().any(|pattern| normalized.contains(pattern)))
}
