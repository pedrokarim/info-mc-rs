use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("invalid address: {0}")]
    InvalidAddress(String),

    #[error("DNS resolution failed: {0}")]
    DnsFailure(String),

    #[error("server offline: {0}")]
    ServerOffline(String),

    #[error("request timed out")]
    Timeout,

    #[error("rate limit exceeded")]
    RateLimited,

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_code) = match &self {
            ApiError::InvalidAddress(_) => (StatusCode::BAD_REQUEST, "invalid_address"),
            ApiError::DnsFailure(_) => (StatusCode::UNPROCESSABLE_ENTITY, "dns_resolution_failed"),
            ApiError::ServerOffline(_) => (StatusCode::OK, "server_offline"), // 200 with online: false
            ApiError::Timeout => (StatusCode::GATEWAY_TIMEOUT, "timeout"),
            ApiError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "rate_limit_exceeded"),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        };

        let body = ErrorResponse {
            error: error_code.to_string(),
            message: self.to_string(),
        };

        (status, axum::Json(body)).into_response()
    }
}

impl From<mc_protocol::McProtocolError> for ApiError {
    fn from(err: mc_protocol::McProtocolError) -> Self {
        match err {
            mc_protocol::McProtocolError::ConnectionRefused(addr) => ApiError::ServerOffline(addr),
            mc_protocol::McProtocolError::Timeout(_) => ApiError::Timeout,
            mc_protocol::McProtocolError::DnsFailure(msg) => ApiError::DnsFailure(msg),
            mc_protocol::McProtocolError::InvalidResponse(msg) => {
                ApiError::Internal(format!("invalid response: {msg}"))
            }
            other => ApiError::Internal(other.to_string()),
        }
    }
}
