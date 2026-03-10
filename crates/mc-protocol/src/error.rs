use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum McProtocolError {
    #[error("connection refused: {0}")]
    ConnectionRefused(String),

    #[error("operation timed out after {0:?}")]
    Timeout(Duration),

    #[error("DNS resolution failed for {0}")]
    DnsFailure(String),

    #[error("invalid response: {0}")]
    InvalidResponse(String),

    #[error("VarInt exceeds maximum length (5 bytes)")]
    VarIntTooLong,

    #[error("VarLong exceeds maximum length (10 bytes)")]
    VarLongTooLong,

    #[error("string exceeds maximum length: {len} > {max}")]
    StringTooLong { len: usize, max: usize },

    #[error("unexpected packet ID: expected {expected:#x}, got {got:#x}")]
    UnexpectedPacketId { expected: i32, got: i32 },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, McProtocolError>;
