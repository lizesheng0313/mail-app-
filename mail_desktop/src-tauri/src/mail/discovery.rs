use std::collections::HashSet;
use std::net::{TcpStream, ToSocketAddrs};
use std::process::Command;
use std::time::Duration;

#[derive(Debug)]
struct SrvRecord {
    priority: u16,
    weight: u16,
    port: u16,
    host: String,
}

pub fn extract_root_domain(host: &str) -> String {
    let parts: Vec<&str> = host.split('.').filter(|item| !item.is_empty()).collect();
    if parts.len() < 2 {
        return host.to_string();
    }

    let compound_suffixes = [
        ("co", "uk"),
        ("org", "uk"),
        ("ac", "uk"),
        ("gov", "uk"),
        ("com", "au"),
        ("net", "au"),
        ("org", "au"),
        ("com", "br"),
        ("com", "cn"),
        ("net", "cn"),
        ("org", "cn"),
        ("co", "jp"),
        ("com", "hk"),
        ("com", "sg"),
        ("co", "kr"),
    ];

    if parts.len() >= 3 {
        let last = parts[parts.len() - 1];
        let second = parts[parts.len() - 2];
        let third = parts[parts.len() - 3];
        if compound_suffixes
            .iter()
            .any(|(left, right)| second == *left && last == *right)
        {
            return format!("{}.{}.{}", third, second, last);
        }
    }

    format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1])
}

pub fn dedupe_candidates(candidates: Vec<(String, u16)>) -> Vec<(String, u16)> {
    let mut seen: HashSet<(String, u16)> = HashSet::new();
    let mut unique = Vec::new();
    for (host, port) in candidates {
        let key = (host.to_ascii_lowercase(), port);
        if seen.insert(key) {
            unique.push((host, port));
        }
    }
    unique
}

pub fn probe_tcp_sync(host: &str, port: u16) -> bool {
    let addr = match (host, port).to_socket_addrs() {
        Ok(mut addresses) => match addresses.next() {
            Some(address) => address,
            None => return false,
        },
        Err(_) => return false,
    };
    TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok()
}

pub fn query_primary_mx(domain: &str) -> Option<String> {
    let domain_owned = domain.to_string();
    let output = Command::new("nslookup")
        .args(["-type=mx", &domain_owned])
        .output()
        .ok();

    let mut best_host = String::new();
    let mut best_priority = i32::MAX;

    if let Some(nslookup) = output.as_ref() {
        let stdout = String::from_utf8_lossy(&nslookup.stdout);
        for line in stdout.lines() {
            let lower = line.to_ascii_lowercase();
            if !lower.contains("mail exchanger") {
                continue;
            }
            let parts: Vec<&str> = line.split("mail exchanger =").collect();
            if parts.len() < 2 {
                continue;
            }
            let priority = parts[0]
                .split("preference =")
                .nth(1)
                .and_then(|item| item.trim().trim_matches(',').parse::<i32>().ok())
                .unwrap_or(100);
            let host = parts[1].trim().trim_end_matches('.').to_string();
            if !host.is_empty() && priority < best_priority {
                best_priority = priority;
                best_host = host;
            }
        }
    }

    if !best_host.is_empty() {
        return Some(best_host);
    }

    let dig = Command::new("dig")
        .args(["+short", "MX", &domain_owned])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&dig.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let priority = parts[0].parse::<i32>().unwrap_or(100);
        let host = parts[1].trim_end_matches('.').to_string();
        if !host.is_empty() && priority < best_priority {
            best_priority = priority;
            best_host = host;
        }
    }

    if best_host.is_empty() {
        None
    } else {
        Some(best_host)
    }
}

pub fn query_srv_records(service: &str, domain: &str) -> Vec<(String, u16)> {
    let lookup_name = format!("_{}._tcp.{}", service, domain);
    let mut records = parse_srv_nslookup(&lookup_name);
    if records.is_empty() {
        records = parse_srv_dig(&lookup_name);
    }
    records.sort_by(|left, right| {
        left.priority
            .cmp(&right.priority)
            .then(right.weight.cmp(&left.weight))
    });
    dedupe_candidates(
        records
            .into_iter()
            .map(|record| (record.host, record.port))
            .collect(),
    )
}

fn parse_srv_nslookup(lookup_name: &str) -> Vec<SrvRecord> {
    let output = match Command::new("nslookup")
        .args(["-type=srv", lookup_name])
        .output()
    {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut records = Vec::new();
    let mut current_priority: Option<u16> = None;
    let mut current_weight: Option<u16> = None;
    let mut current_port: Option<u16> = None;

    for line in stdout.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();

        if let Some(values) = trimmed.split("service =").nth(1) {
            let parts: Vec<&str> = values.split_whitespace().collect();
            if parts.len() >= 4 {
                let priority = parts[0].parse::<u16>().unwrap_or(100);
                let weight = parts[1].parse::<u16>().unwrap_or(0);
                let port = parts[2].parse::<u16>().unwrap_or(0);
                let host = parts[3].trim_end_matches('.').to_string();
                if port > 0 && !host.is_empty() {
                    records.push(SrvRecord {
                        priority,
                        weight,
                        port,
                        host,
                    });
                }
            }
            continue;
        }

        if let Some(value) = lower.split("priority =").nth(1) {
            current_priority = value.trim().trim_matches(',').parse::<u16>().ok();
            continue;
        }
        if let Some(value) = lower.split("weight =").nth(1) {
            current_weight = value.trim().trim_matches(',').parse::<u16>().ok();
            continue;
        }
        if let Some(value) = lower.split("port =").nth(1) {
            current_port = value.trim().trim_matches(',').parse::<u16>().ok();
            continue;
        }
        if let Some(value) = lower.split("svr hostname =").nth(1) {
            let host = value.trim().trim_end_matches('.').to_string();
            if let Some(port) = current_port {
                records.push(SrvRecord {
                    priority: current_priority.unwrap_or(100),
                    weight: current_weight.unwrap_or(0),
                    port,
                    host,
                });
            }
            current_priority = None;
            current_weight = None;
            current_port = None;
        }
    }

    records
}

fn parse_srv_dig(lookup_name: &str) -> Vec<SrvRecord> {
    let output = match Command::new("dig")
        .args(["+short", "SRV", lookup_name])
        .output()
    {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut records = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        let priority = parts[0].parse::<u16>().unwrap_or(100);
        let weight = parts[1].parse::<u16>().unwrap_or(0);
        let port = parts[2].parse::<u16>().unwrap_or(0);
        let host = parts[3].trim_end_matches('.').to_string();
        if port > 0 && !host.is_empty() {
            records.push(SrvRecord {
                priority,
                weight,
                port,
                host,
            });
        }
    }
    records
}
