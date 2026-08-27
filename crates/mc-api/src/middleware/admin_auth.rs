use std::net::SocketAddr;

use axum::extract::{ConnectInfo, State};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::ascencia::{AdminUserInfo, cookie_value, load_session};
use crate::state::SharedState;

#[derive(Debug, Clone)]
pub struct AdminClaims {
    pub sub: String,
    pub session_token_hash: String,
    pub user: AdminUserInfo,
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
            "message": "Session Ascencia ID absente ou invalide"
        });
        (axum::http::StatusCode::UNAUTHORIZED, axum::Json(body)).into_response()
    };

    let forbidden = |message: &str| {
        let body = serde_json::json!({
            "error": "forbidden",
            "message": message
        });
        (axum::http::StatusCode::FORBIDDEN, axum::Json(body)).into_response()
    };

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
        let allowed: Vec<&str> = whitelist.split(',').map(str::trim).collect();
        if !allowed.iter().any(|ip| *ip == client_ip) {
            return forbidden("Adresse IP absente de la liste d’administration");
        }
    }

    if request.method() != axum::http::Method::GET {
        let valid_origin = request
            .headers()
            .get(axum::http::header::ORIGIN)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|origin| origin == state.ascencia.allowed_origin);
        if !valid_origin {
            return forbidden("Origine de la requête refusée");
        }
    }

    let Some(raw_token) = cookie_value(request.headers()) else {
        return unauthorized();
    };
    let session = match load_session(&state, raw_token).await {
        Ok(session) => session,
        Err(_) => return unauthorized(),
    };

    let claims = AdminClaims {
        sub: session.user.account_id.clone(),
        session_token_hash: session.token_hash,
        user: session.user,
    };
    request.extensions_mut().insert(claims);
    next.run(request).await
}
