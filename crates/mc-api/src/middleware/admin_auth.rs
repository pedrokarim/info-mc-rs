use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::state::SharedState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminClaims {
    pub sub: String,
    pub jti: String,
    pub username: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn admin_auth_middleware(
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let unauthorized = || {
        let body = serde_json::json!({
            "error": "unauthorized",
            "message": "Missing or invalid authentication token"
        });
        (axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
    };

    let forbidden = |msg: &str| {
        let body = serde_json::json!({
            "error": "forbidden",
            "message": msg
        });
        (axum::http::StatusCode::FORBIDDEN, axum::Json(body)).into_response()
    };

    // IP whitelist check
    let whitelist = sqlx::query_scalar::<_, String>(
        "SELECT value FROM admin_config WHERE key = 'admin_ip_whitelist'",
    )
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten()
    .unwrap_or_default();

    if !whitelist.is_empty() {
        let client_ip = addr.ip().to_string();
        let allowed: Vec<&str> = whitelist.split(',').map(|s| s.trim()).collect();
        if !allowed.iter().any(|ip| *ip == client_ip) {
            return forbidden("IP address not in admin whitelist");
        }
    }

    // Extract Bearer token from Authorization header
    let token = match request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
    {
        Some(t) => t.to_string(),
        None => return unauthorized(),
    };

    // Decode & verify JWT
    let claims = match jsonwebtoken::decode::<AdminClaims>(
        &token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => data.claims,
        Err(_) => return unauthorized(),
    };

    // Verify session still exists in DB (allows revocation via logout)
    let session_exists = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM admin_sessions WHERE id = ? AND expires_at > datetime('now')",
    )
    .bind(&claims.jti)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    if session_exists == 0 {
        return unauthorized();
    }

    // Inject claims into request extensions for handlers
    request.extensions_mut().insert(claims);
    next.run(request).await
}
