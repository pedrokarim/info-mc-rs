use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::error::{McProtocolError, Result};
use crate::types::{build_packet, read_packet, write_string, write_varint};

/// Configuration for Server List Ping.
pub struct SlpConfig {
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
    /// MC protocol version to announce. Default: 769 (1.21.4)
    pub protocol_version: i32,
}

impl Default for SlpConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(10),
            protocol_version: 769,
        }
    }
}

/// Full response from a Java Edition Server List Ping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlpResponse {
    pub version: Version,
    pub players: Players,
    pub description: MotdComponent,
    #[serde(default)]
    pub favicon: Option<String>,
    #[serde(default, rename = "enforcesSecureChat")]
    pub enforces_secure_chat: Option<bool>,
    /// Latency in milliseconds (measured, not from JSON).
    #[serde(skip)]
    pub latency_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    #[serde(default)]
    pub sample: Vec<PlayerSample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSample {
    pub name: String,
    pub id: String,
}

/// MOTD can be a plain string or a structured Chat Component JSON object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MotdComponent {
    Plain(String),
    Component(serde_json::Value),
}

impl MotdComponent {
    /// Extract plain text, stripping all formatting codes.
    pub fn to_plain_text(&self) -> String {
        match self {
            MotdComponent::Plain(s) => strip_formatting(s),
            MotdComponent::Component(v) => {
                let mut out = String::new();
                extract_text(v, &mut out);
                out
            }
        }
    }
}

fn strip_formatting(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '§' || c == '&' {
            chars.next(); // skip the formatting code
        } else {
            result.push(c);
        }
    }
    result
}

fn extract_text(value: &serde_json::Value, out: &mut String) {
    match value {
        serde_json::Value::String(s) => out.push_str(&strip_formatting(s)),
        serde_json::Value::Object(obj) => {
            if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
                out.push_str(&strip_formatting(text));
            }
            if let Some(extra) = obj.get("extra").and_then(|v| v.as_array()) {
                for item in extra {
                    extract_text(item, out);
                }
            }
        }
        _ => {}
    }
}

/// Ping a Java Edition Minecraft server using the Server List Ping protocol.
///
/// `connect_address` is the IP/hostname to connect to (TCP).
/// `server_hostname` is sent in the handshake (used by proxies like BungeeCord to route).
/// `port` is the server port (default 25565).
pub async fn ping_java(
    connect_address: &str,
    port: u16,
    server_hostname: &str,
    config: &SlpConfig,
) -> Result<SlpResponse> {
    // 1. TCP connect with timeout
    let stream = timeout(
        config.connect_timeout,
        TcpStream::connect((connect_address, port)),
    )
    .await
    .map_err(|_| McProtocolError::Timeout(config.connect_timeout))?
    .map_err(|e| {
        if e.kind() == std::io::ErrorKind::ConnectionRefused {
            McProtocolError::ConnectionRefused(format!("{connect_address}:{port}"))
        } else {
            McProtocolError::Io(e)
        }
    })?;

    let (reader, mut writer) = tokio::io::split(stream);
    let mut reader = BufReader::new(reader);

    // 2. Send Handshake packet (0x00)
    // server_hostname is used here (not connect_address) so proxies can route correctly
    let mut handshake_payload = Vec::new();
    handshake_payload.extend_from_slice(&write_varint(config.protocol_version));
    handshake_payload.extend_from_slice(&write_string(server_hostname));
    handshake_payload.extend_from_slice(&port.to_be_bytes());
    handshake_payload.extend_from_slice(&write_varint(1)); // next_state = Status

    let handshake_packet = build_packet(0x00, &handshake_payload);
    tokio::io::AsyncWriteExt::write_all(&mut writer, &handshake_packet).await?;

    // 3. Send Status Request (0x00, empty)
    let status_request = build_packet(0x00, &[]);
    tokio::io::AsyncWriteExt::write_all(&mut writer, &status_request).await?;

    // 4. Read Status Response
    let (packet_id, payload) = timeout(config.read_timeout, read_packet(&mut reader))
        .await
        .map_err(|_| McProtocolError::Timeout(config.read_timeout))??;

    if packet_id != 0x00 {
        return Err(McProtocolError::UnexpectedPacketId {
            expected: 0x00,
            got: packet_id,
        });
    }

    // Parse the JSON string from the payload
    let json_str = {
        let mut cursor = std::io::Cursor::new(&payload);
        crate::types::read_string_sync(&mut cursor)?
    };

    let mut response: SlpResponse = serde_json::from_str(&json_str)?;

    // 5. Send Ping (0x01) with timestamp
    let ping_time = Instant::now();
    let ping_payload = 1i64.to_be_bytes(); // arbitrary nonce
    let ping_packet = build_packet(0x01, &ping_payload);
    tokio::io::AsyncWriteExt::write_all(&mut writer, &ping_packet).await?;

    // 6. Read Pong (0x01)
    let (pong_id, _pong_payload) = timeout(config.read_timeout, read_packet(&mut reader))
        .await
        .map_err(|_| McProtocolError::Timeout(config.read_timeout))??;

    if pong_id != 0x01 {
        return Err(McProtocolError::UnexpectedPacketId {
            expected: 0x01,
            got: pong_id,
        });
    }

    // 7. Calculate latency
    response.latency_ms = ping_time.elapsed().as_millis() as u64;

    Ok(response)
}
