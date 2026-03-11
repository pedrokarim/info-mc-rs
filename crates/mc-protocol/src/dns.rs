use hickory_resolver::TokioResolver;

use crate::error::{McProtocolError, Result};

const DEFAULT_MC_PORT: u16 = 25565;

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

    // If it's an IP literal, skip DNS entirely
    if hostname.parse::<std::net::IpAddr>().is_ok() {
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
}
