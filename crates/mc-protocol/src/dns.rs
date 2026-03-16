use std::net::IpAddr;

use hickory_resolver::TokioResolver;

use crate::error::{McProtocolError, Result};

const DEFAULT_MC_PORT: u16 = 25565;

/// Returns true if the IP address is private, loopback, or otherwise not routable
/// on the public internet. Used to prevent SSRF attacks.
fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_unspecified()
                || v4.is_broadcast()
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unspecified()
                || (v6.segments()[0] & 0xfe00) == 0xfc00 // unique local (fc00::/7)
                || (v6.segments()[0] & 0xffc0) == 0xfe80 // link-local (fe80::/10)
        }
    }
}

fn reject_private_ip(ip: &str) -> Result<()> {
    if let Ok(parsed) = ip.parse::<IpAddr>() {
        if is_private_ip(&parsed) {
            return Err(McProtocolError::DnsFailure(
                "address resolves to a private/internal IP".to_string(),
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct ResolvedAddress {
    pub hostname: String,
    pub resolved_host: String,
    pub port: u16,
    pub ip: String,
    pub srv_found: bool,
}

/// Resolve a Minecraft server address, checking DNS SRV records.
///
/// Supports formats: `hostname`, `hostname:port`, `1.2.3.4`, `1.2.3.4:25565`
/// If a port is explicitly given, SRV lookup is skipped.
/// If the input is an IP literal, DNS is skipped entirely.
pub async fn resolve_address(address: &str) -> Result<ResolvedAddress> {
    let (hostname, explicit_port) = parse_address(address);
    let port = explicit_port.unwrap_or(DEFAULT_MC_PORT);

    // If it's an IP literal, skip DNS but reject private IPs
    if hostname.parse::<IpAddr>().is_ok() {
        reject_private_ip(hostname)?;
        return Ok(ResolvedAddress {
            hostname: hostname.to_string(),
            resolved_host: hostname.to_string(),
            port,
            ip: hostname.to_string(),
            srv_found: false,
        });
    }

    let resolver = TokioResolver::builder_tokio()
        .map_err(|e| McProtocolError::DnsFailure(format!("resolver init: {e}")))?
        .build();

    // Try SRV lookup if no explicit port was given
    let (target, final_port, srv_found) = if explicit_port.is_none() {
        match resolve_srv(&resolver, hostname).await {
            Some((host, srv_port)) => (host, srv_port, true),
            None => (hostname.to_string(), port, false),
        }
    } else {
        (hostname.to_string(), port, false)
    };

    // Resolve the target hostname to an IP
    let ip = resolve_ip(&resolver, &target)
        .await
        .map_err(|_| McProtocolError::DnsFailure(format!("failed to resolve {target}")))?;

    // Reject private/internal IPs after DNS resolution (prevent DNS rebinding)
    reject_private_ip(&ip)?;

    Ok(ResolvedAddress {
        hostname: hostname.to_string(),
        resolved_host: target,
        port: final_port,
        ip,
        srv_found,
    })
}

fn parse_address(address: &str) -> (&str, Option<u16>) {
    // Handle [ipv6]:port format
    if let Some(bracket_end) = address.find(']') {
        let host = &address[..=bracket_end];
        if address.len() > bracket_end + 2 && address.as_bytes()[bracket_end + 1] == b':' {
            let port = address[bracket_end + 2..].parse().ok();
            return (host, port);
        }
        return (host, None);
    }

    // Handle host:port format (only if there's exactly one colon, not IPv6)
    if let Some(colon_pos) = address.rfind(':') {
        if address.matches(':').count() == 1 {
            let host = &address[..colon_pos];
            let port = address[colon_pos + 1..].parse().ok();
            return (host, port);
        }
    }

    (address, None)
}

async fn resolve_srv(resolver: &TokioResolver, hostname: &str) -> Option<(String, u16)> {
    let srv_name = format!("_minecraft._tcp.{hostname}.");
    let lookup = resolver.srv_lookup(&srv_name).await.ok()?;
    let record = lookup.iter().next()?;

    let target = record.target().to_string();
    let target = target.trim_end_matches('.').to_string();
    let port = record.port();

    Some((target, port))
}

async fn resolve_ip(resolver: &TokioResolver, hostname: &str) -> std::result::Result<String, ()> {
    // Try A record first, then AAAA
    if let Ok(lookup) = resolver.lookup_ip(hostname).await {
        if let Some(ip) = lookup.iter().next() {
            return Ok(ip.to_string());
        }
    }
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_address() {
        assert_eq!(
            parse_address("play.hypixel.net"),
            ("play.hypixel.net", None)
        );
        assert_eq!(
            parse_address("play.hypixel.net:25565"),
            ("play.hypixel.net", Some(25565))
        );
        assert_eq!(parse_address("1.2.3.4"), ("1.2.3.4", None));
        assert_eq!(parse_address("1.2.3.4:12345"), ("1.2.3.4", Some(12345)));
        assert_eq!(parse_address("[::1]"), ("[::1]", None));
        assert_eq!(parse_address("[::1]:25565"), ("[::1]", Some(25565)));
    }

    #[test]
    fn test_ip_literal_detection() {
        assert!("1.2.3.4".parse::<std::net::IpAddr>().is_ok());
        assert!("::1".parse::<std::net::IpAddr>().is_ok());
        assert!("play.hypixel.net".parse::<std::net::IpAddr>().is_err());
    }

    #[test]
    fn test_is_private_ip() {
        let private_ips = [
            "127.0.0.1", "10.0.0.1", "172.16.0.1", "192.168.1.1",
            "169.254.1.1", "0.0.0.0", "::1",
        ];
        for ip in private_ips {
            let parsed: IpAddr = ip.parse().unwrap();
            assert!(is_private_ip(&parsed), "{ip} should be private");
        }

        let public_ips = ["8.8.8.8", "1.1.1.1", "142.250.80.46"];
        for ip in public_ips {
            let parsed: IpAddr = ip.parse().unwrap();
            assert!(!is_private_ip(&parsed), "{ip} should be public");
        }
    }

    #[test]
    fn test_reject_private_ip() {
        assert!(reject_private_ip("127.0.0.1").is_err());
        assert!(reject_private_ip("192.168.1.1").is_err());
        assert!(reject_private_ip("8.8.8.8").is_ok());
    }
}
