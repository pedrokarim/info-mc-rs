use std::sync::atomic::Ordering;

use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::state::SharedState;

/// Middleware that returns 503 on all public routes when maintenance mode is active.
/// Admin routes (/api/v1/admin/*) are NOT affected.
pub async fn maintenance_middleware(
    State(state): State<SharedState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if state.maintenance_mode.load(Ordering::Relaxed) {
        // Let admin routes through
        let path = request.uri().path();
        if path.starts_with("/api/v1/admin") || path == "/health" {
            return next.run(request).await;
        }

        let body = serde_json::json!({
            "error": "maintenance",
            "message": "Service temporarily unavailable for maintenance"
        });
        return (StatusCode::SERVICE_UNAVAILABLE, axum::Json(body)).into_response();
    }

    next.run(request).await
}
