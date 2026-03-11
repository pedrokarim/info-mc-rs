use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::net::UdpSocket;
use tokio::time::timeout;

use crate::error::{McProtocolError, Result};

pub const RAKNET_MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

const UNCONNECTED_PING: u8 = 0x01;
const UNCONNECTED_PONG: u8 = 0x1C;

pub struct BedrockConfig {
    pub timeout: Duration,
}

impl Default for BedrockConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BedrockResponse {
    pub edition: String,
    pub motd_line1: String,
    pub protocol_version: u32,
    pub version: String,
    pub players_online: u32,
    pub players_max: u32,
    pub server_uid: String,
    pub motd_line2: Option<String>,
    pub gamemode: Option<String>,
    pub gamemode_numeric: Option<u32>,
    pub port_v4: Option<u16>,
    pub port_v6: Option<u16>,
    pub latency_ms: u64,
}

/// Ping a Bedrock Edition server using the RakNet Unconnected Ping protocol.
pub async fn ping_bedrock(
    address: &str,
    port: u16,
    config: &BedrockConfig,
) -> Result<BedrockResponse> {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .map_err(McProtocolError::Io)?;

    socket
        .connect((address, port))
        .await
        .map_err(|e| McProtocolError::ConnectionRefused(format!("{address}:{port} - {e}")))?;

    // Build Unconnected Ping packet
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;
    let client_guid: i64 = rand::random();

    let mut packet = Vec::with_capacity(1 + 8 + 16 + 8);
    packet.push(UNCONNECTED_PING);
    packet.extend_from_slice(&timestamp.to_be_bytes());
    packet.extend_from_slice(&RAKNET_MAGIC);
    packet.extend_from_slice(&client_guid.to_be_bytes());

    let ping_time = Instant::now();
    socket.send(&packet).await.map_err(McProtocolError::Io)?;

    // Receive Unconnected Pong
    let mut buf = [0u8; 2048];
    let len = timeout(config.timeout, socket.recv(&mut buf))
        .await
        .map_err(|_| McProtocolError::Timeout(config.timeout))?
        .map_err(McProtocolError::Io)?;

    let latency_ms = ping_time.elapsed().as_millis() as u64;

    parse_pong(&buf[..len], latency_ms)
}

fn parse_pong(data: &[u8], latency_ms: u64) -> Result<BedrockResponse> {
    // Pong format:
    //   packet_id(1) + time(8) + server_guid(8) + magic(16) + string_length(2) + string
    let min_len = 1 + 8 + 8 + 16 + 2;
    if data.len() < min_len {
        return Err(McProtocolError::InvalidResponse(format!(
            "pong too short: {} bytes",
            data.len()
        )));
    }

    if data[0] != UNCONNECTED_PONG {
        return Err(McProtocolError::InvalidResponse(format!(
            "expected pong (0x1C), got {:#x}",
            data[0]
        )));
    }

    let str_len_offset = 1 + 8 + 8 + 16; // 33
    let str_len = u16::from_be_bytes([data[str_len_offset], data[str_len_offset + 1]]) as usize;

    let str_start = str_len_offset + 2;
    let str_end = (str_start + str_len).min(data.len());

    let server_id = std::str::from_utf8(&data[str_start..str_end])
        .map_err(|e| McProtocolError::InvalidResponse(format!("invalid UTF-8: {e}")))?;

    parse_server_id(server_id, latency_ms)
}

fn parse_server_id(server_id: &str, latency_ms: u64) -> Result<BedrockResponse> {
    let fields: Vec<&str> = server_id.split(';').collect();

    if fields.len() < 6 {
        return Err(McProtocolError::InvalidResponse(format!(
            "server ID has too few fields ({}): {server_id}",
            fields.len()
        )));
    }

    Ok(BedrockResponse {
        edition: fields[0].to_string(),
        motd_line1: fields[1].to_string(),
        protocol_version: fields[2].parse().unwrap_or(0),
        version: fields[3].to_string(),
        players_online: fields[4].parse().unwrap_or(0),
        players_max: fields[5].parse().unwrap_or(0),
        server_uid: fields.get(6).unwrap_or(&"").to_string(),
        motd_line2: fields.get(7).map(|s| s.to_string()),
        gamemode: fields.get(8).map(|s| s.to_string()),
        gamemode_numeric: fields.get(9).and_then(|s| s.parse().ok()),
        port_v4: fields.get(10).and_then(|s| s.parse().ok()),
        port_v6: fields.get(11).and_then(|s| s.parse().ok()),
        latency_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_server_id_full() {
        let id = "MCPE;Dedicated Server;486;1.18.0;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133";
        let resp = parse_server_id(id, 42).unwrap();

        assert_eq!(resp.edition, "MCPE");
        assert_eq!(resp.motd_line1, "Dedicated Server");
        assert_eq!(resp.protocol_version, 486);
        assert_eq!(resp.version, "1.18.0");
        assert_eq!(resp.players_online, 0);
        assert_eq!(resp.players_max, 10);
        assert_eq!(resp.server_uid, "13253860892328930865");
        assert_eq!(resp.motd_line2.as_deref(), Some("Bedrock level"));
        assert_eq!(resp.gamemode.as_deref(), Some("Survival"));
        assert_eq!(resp.gamemode_numeric, Some(1));
        assert_eq!(resp.port_v4, Some(19132));
        assert_eq!(resp.port_v6, Some(19133));
        assert_eq!(resp.latency_ms, 42);
    }

    #[test]
    fn test_parse_server_id_minimal() {
        let id = "MCPE;My Server;486;1.18.0;5;20";
        let resp = parse_server_id(id, 10).unwrap();

        assert_eq!(resp.edition, "MCPE");
        assert_eq!(resp.motd_line1, "My Server");
        assert_eq!(resp.players_online, 5);
        assert_eq!(resp.players_max, 20);
        assert!(resp.motd_line2.is_none());
        assert!(resp.gamemode.is_none());
    }

    #[test]
    fn test_parse_server_id_too_short() {
        let id = "MCPE;Server;486";
        let result = parse_server_id(id, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_raknet_magic() {
        assert_eq!(RAKNET_MAGIC.len(), 16);
        assert_eq!(RAKNET_MAGIC[0], 0x00);
        assert_eq!(RAKNET_MAGIC[15], 0x78);
    }
}
