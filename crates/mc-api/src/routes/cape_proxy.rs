use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};

use crate::state::SharedState;

fn is_valid_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.')
}

/// Proxy cape images from third-party servers that don't send CORS headers.
/// GET /api/v1/cape/optifine/{username}
/// GET /api/v1/cape/labymod/{uuid}
pub async fn proxy_cape(
    State(state): State<SharedState>,
    Path((source, identifier)): Path<(String, String)>,
) -> Response {
    if !is_valid_identifier(&identifier) {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let urls: Vec<String> = match source.as_str() {
        "optifine" => vec![
            format!("https://optifine.net/capes/{identifier}.png"),
            format!("https://optifine.net/capes/inactive/{identifier}.png"),
        ],
        "labymod" => vec![format!("https://dl.labymod.net/capes/{identifier}")],
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    // Try each URL until one succeeds
    let mut resp = None;
    for url in &urls {
        if let Ok(r) = state.http.get(url).send().await
            && r.status().is_success()
        {
            resp = Some(r);
            break;
        }
    }
    let resp = match resp {
        Some(r) => r,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let content_type = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .cloned()
        .unwrap_or_else(|| HeaderValue::from_static("image/png"));

    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=3600")
        .body(Body::from(bytes))
        .unwrap()
}
