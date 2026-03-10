use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum MojangError {
    #[error("player not found: {0}")]
    PlayerNotFound(String),

    #[error("invalid username: {0}")]
    InvalidUsername(String),

    #[error("rate limited by Mojang API")]
    RateLimited,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("invalid profile data: {0}")]
    InvalidProfile(String),
}

pub type Result<T> = std::result::Result<T, MojangError>;

/// UUID formatting helper: insert hyphens into a 32-char hex string.
pub fn format_uuid(hex: &str) -> std::result::Result<String, fmt::Error> {
    if hex.len() != 32 {
        return Err(fmt::Error);
    }
    Ok(format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_uuid() {
        let hex = "ec70bcaf702f4bb8b48d276fa52a780c";
        let formatted = format_uuid(hex).unwrap();
        assert_eq!(formatted, "ec70bcaf-702f-4bb8-b48d-276fa52a780c");
    }

    #[test]
    fn test_format_uuid_invalid() {
        assert!(format_uuid("too_short").is_err());
    }
}
