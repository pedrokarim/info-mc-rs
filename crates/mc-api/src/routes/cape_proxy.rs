use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};

use crate::state::SharedState;

/// Proxy cape images from third-party servers that don't send CORS headers.
/// GET /api/v1/cape/optifine/{username}
/// GET /api/v1/cape/labymod/{uuid}
pub async fn proxy_cape(
    State(state): State<SharedState>,
    Path((source, identifier)): Path<(String, String)>,
) -> Response {
    let url = match source.as_str() {
        "optifine" => format!("https://optifine.net/capes/{identifier}.png"),
        "labymod" => format!("https://dl.labymod.net/capes/{identifier}"),
        _ => return StatusCode::BAD_REQUEST.into_response(),
    };

    let resp = match state.http.get(&url).send().await {
        Ok(r) => r,
        Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
    };

    if !resp.status().is_success() {
        return StatusCode::NOT_FOUND.into_response();
    }

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
