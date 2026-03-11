use std::time::Duration;

use serde::Serialize;
use tokio::net::UdpSocket;
use tokio::time::timeout;

use crate::error::{McProtocolError, Result};

const MAGIC: [u8; 2] = [0xFE, 0xFD];
const TYPE_HANDSHAKE: u8 = 0x09;
const TYPE_STAT: u8 = 0x00;

pub struct QueryConfig {
    pub timeout: Duration,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BasicStat {
    pub motd: String,
    pub gametype: String,
    pub map: String,
    pub num_players: u32,
    pub max_players: u32,
    pub host_port: u16,
    pub host_ip: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FullStat {
    pub motd: String,
    pub gametype: String,
    pub game_id: String,
    pub version: String,
    pub plugins: PluginInfo,
    pub map: String,
    pub num_players: u32,
    pub max_players: u32,
    pub host_port: u16,
    pub host_ip: String,
    pub players: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginInfo {
    pub software: String,
    pub version: String,
    pub plugins: Vec<String>,
}

/// Query a server for basic stats (requires `enable-query=true` on the server).
pub async fn query_basic(address: &str, port: u16, config: &QueryConfig) -> Result<BasicStat> {
    let socket = bind_socket().await?;
    socket
        .connect((address, port))
        .await
        .map_err(|e| McProtocolError::Io(e))?;

    let session_id = generate_session_id();
    let challenge = handshake(&socket, session_id, config.timeout).await?;

    // Send basic stat request
    let mut request = Vec::with_capacity(11);
    request.extend_from_slice(&MAGIC);
    request.push(TYPE_STAT);
    request.extend_from_slice(&session_id.to_be_bytes());
    request.extend_from_slice(&challenge.to_be_bytes());

    socket
        .send(&request)
        .await
        .map_err(|e| McProtocolError::Io(e))?;

    let mut buf = [0u8; 4096];
    let len = timeout(config.timeout, socket.recv(&mut buf))
        .await
        .map_err(|_| McProtocolError::Timeout(config.timeout))?
        .map_err(|e| McProtocolError::Io(e))?;

    parse_basic_stat(&buf[..len])
}

/// Query a server for full stats (requires `enable-query=true` on the server).
pub async fn query_full(address: &str, port: u16, config: &QueryConfig) -> Result<FullStat> {
    let socket = bind_socket().await?;
    socket
        .connect((address, port))
        .await
        .map_err(|e| McProtocolError::Io(e))?;

    let session_id = generate_session_id();
    let challenge = handshake(&socket, session_id, config.timeout).await?;

    // Send full stat request (with 4 bytes padding)
    let mut request = Vec::with_capacity(15);
    request.extend_from_slice(&MAGIC);
    request.push(TYPE_STAT);
    request.extend_from_slice(&session_id.to_be_bytes());
    request.extend_from_slice(&challenge.to_be_bytes());
    request.extend_from_slice(&[0x00; 4]); // padding for full stat

    socket.send(&request).await.map_err(McProtocolError::Io)?;

    let mut buf = [0u8; 8192];
    let len = timeout(config.timeout, socket.recv(&mut buf))
        .await
        .map_err(|_| McProtocolError::Timeout(config.timeout))?
        .map_err(McProtocolError::Io)?;

    parse_full_stat(&buf[..len])
}

async fn bind_socket() -> Result<UdpSocket> {
    UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(McProtocolError::Io)
}

fn generate_session_id() -> i32 {
    rand::random::<i32>() & 0x0F0F0F0F
}

async fn handshake(socket: &UdpSocket, session_id: i32, dur: Duration) -> Result<i32> {
    let mut request = Vec::with_capacity(7);
    request.extend_from_slice(&MAGIC);
    request.push(TYPE_HANDSHAKE);
    request.extend_from_slice(&session_id.to_be_bytes());

    socket.send(&request).await.map_err(McProtocolError::Io)?;

    let mut buf = [0u8; 128];
    let len = timeout(dur, socket.recv(&mut buf))
        .await
        .map_err(|_| McProtocolError::Timeout(dur))?
        .map_err(McProtocolError::Io)?;

    // Response: type(1) + session_id(4) + challenge_token(null-terminated string)
    if len < 5 {
        return Err(McProtocolError::InvalidResponse(
            "handshake response too short".into(),
        ));
    }

    let token_bytes = &buf[5..len];
    let token_str = token_bytes.split(|&b| b == 0).next().unwrap_or(token_bytes);

    let token_str = std::str::from_utf8(token_str)
        .map_err(|e| McProtocolError::InvalidResponse(format!("invalid challenge token: {e}")))?;

    token_str
        .trim()
        .parse::<i32>()
        .map_err(|e| McProtocolError::InvalidResponse(format!("invalid challenge number: {e}")))
}

fn parse_basic_stat(data: &[u8]) -> Result<BasicStat> {
    // Skip type(1) + session_id(4) = 5 bytes
    if data.len() < 5 {
        return Err(McProtocolError::InvalidResponse(
            "basic stat too short".into(),
        ));
    }

    let payload = &data[5..];
    let strings = read_null_terminated_strings(payload);

    if strings.len() < 5 {
        return Err(McProtocolError::InvalidResponse(format!(
            "expected 5+ fields, got {}",
            strings.len()
        )));
    }

    // After the 5 strings: host_port(u16 LE) + host_ip(null-terminated)
    let mut offset = 0;
    for s in &strings[..5] {
        offset += s.len() + 1; // +1 for null terminator
    }

    let remaining = &payload[offset..];
    let host_port = if remaining.len() >= 2 {
        u16::from_le_bytes([remaining[0], remaining[1]])
    } else {
        25565
    };

    let host_ip = if remaining.len() > 2 {
        let ip_bytes = &remaining[2..];
        ip_bytes
            .split(|&b| b == 0)
            .next()
            .and_then(|b| std::str::from_utf8(b).ok())
            .unwrap_or("")
            .to_string()
    } else {
        String::new()
    };

    Ok(BasicStat {
        motd: strings[0].clone(),
        gametype: strings[1].clone(),
        map: strings[2].clone(),
        num_players: strings[3].parse().unwrap_or(0),
        max_players: strings[4].parse().unwrap_or(0),
        host_port,
        host_ip,
    })
}

fn parse_full_stat(data: &[u8]) -> Result<FullStat> {
    // Skip type(1) + session_id(4) + padding(11) = 16 bytes
    if data.len() < 16 {
        return Err(McProtocolError::InvalidResponse(
            "full stat too short".into(),
        ));
    }

    let payload = &data[16..];

    // Parse K/V section: key\0value\0...terminated by \0\0
    let mut kv = std::collections::HashMap::new();
    let mut pos = 0;

    loop {
        if pos >= payload.len() {
            break;
        }

        let key = read_cstring(payload, pos);
        pos += key.len() + 1;

        if key.is_empty() {
            break;
        }

        let value = if pos < payload.len() {
            let v = read_cstring(payload, pos);
            pos += v.len() + 1;
            v
        } else {
            String::new()
        };

        kv.insert(key, value);
    }

    // Skip padding before players section (10 bytes: \x01player_\0\0)
    // Find the player section by looking for "player_\0\0" pattern
    pos += 10.min(payload.len().saturating_sub(pos));

    // Parse player names
    let mut players = Vec::new();
    while pos < payload.len() {
        let name = read_cstring(payload, pos);
        pos += name.len() + 1;
        if name.is_empty() {
            break;
        }
        players.push(name);
    }

    let plugins_raw = kv.get("plugins").cloned().unwrap_or_default();
    let plugins = parse_plugins(&plugins_raw);

    Ok(FullStat {
        motd: kv.get("hostname").cloned().unwrap_or_default(),
        gametype: kv.get("gametype").cloned().unwrap_or_default(),
        game_id: kv.get("game_id").cloned().unwrap_or_default(),
        version: kv.get("version").cloned().unwrap_or_default(),
        plugins,
        map: kv.get("map").cloned().unwrap_or_default(),
        num_players: kv
            .get("numplayers")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        max_players: kv
            .get("maxplayers")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        host_port: kv
            .get("hostport")
            .and_then(|s| s.parse().ok())
            .unwrap_or(25565),
        host_ip: kv.get("hostip").cloned().unwrap_or_default(),
        players,
    })
}

fn parse_plugins(raw: &str) -> PluginInfo {
    // Format: "Paper on 1.21.4: EssentialsX 2.20; WorldEdit 7.3"
    if raw.is_empty() {
        return PluginInfo {
            software: String::new(),
            version: String::new(),
            plugins: Vec::new(),
        };
    }

    let (software_part, plugins_part) = raw.split_once(": ").unwrap_or((raw, ""));

    let (software, version) = software_part
        .split_once(" on ")
        .unwrap_or((software_part, ""));

    let plugins: Vec<String> = if plugins_part.is_empty() {
        Vec::new()
    } else {
        plugins_part
            .split("; ")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    PluginInfo {
        software: software.trim().to_string(),
        version: version.trim().to_string(),
        plugins,
    }
}

fn read_null_terminated_strings(data: &[u8]) -> Vec<String> {
    let mut strings = Vec::new();
    let mut start = 0;

    for (i, &byte) in data.iter().enumerate() {
        if byte == 0 {
            if let Ok(s) = std::str::from_utf8(&data[start..i]) {
                strings.push(s.to_string());
            }
            start = i + 1;
        }
    }

    strings
}

fn read_cstring(data: &[u8], offset: usize) -> String {
    if offset >= data.len() {
        return String::new();
    }
    let end = data[offset..]
        .iter()
        .position(|&b| b == 0)
        .map(|p| offset + p)
        .unwrap_or(data.len());

    std::str::from_utf8(&data[offset..end])
        .unwrap_or("")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plugins() {
        let info = parse_plugins("Paper on 1.21.4: EssentialsX 2.20; WorldEdit 7.3");
        assert_eq!(info.software, "Paper");
        assert_eq!(info.version, "1.21.4");
        assert_eq!(info.plugins, vec!["EssentialsX 2.20", "WorldEdit 7.3"]);

        let vanilla = parse_plugins("Vanilla");
        assert_eq!(vanilla.software, "Vanilla");
        assert_eq!(vanilla.version, "");
        assert!(vanilla.plugins.is_empty());

        let empty = parse_plugins("");
        assert_eq!(empty.software, "");
        assert!(empty.plugins.is_empty());
    }

    #[test]
    fn test_read_cstring() {
        let data = b"hello\0world\0";
        assert_eq!(read_cstring(data, 0), "hello");
        assert_eq!(read_cstring(data, 6), "world");
        assert_eq!(read_cstring(data, 12), "");
    }

    #[test]
    fn test_generate_session_id() {
        let id = generate_session_id();
        assert_eq!(id & !0x0F0F0F0F, 0, "session ID must be masked");
    }
}
