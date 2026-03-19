use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

use super::players::PaginatedResponse;

#[derive(Serialize, sqlx::FromRow)]
pub struct AlertEntry {
    pub id: i64,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub resolved: i64,
    pub resolved_by: Option<String>,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

#[derive(Deserialize)]
pub struct AlertQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    /// "all", "active", "resolved" (default: "active")
    #[serde(default = "default_filter")]
    pub filter: String,
}

fn default_limit() -> i64 {
    50
}

fn default_filter() -> String {
    "active".to_string()
}

/// GET /api/v1/admin/alerts (protected)
pub async fn list_alerts(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
    Query(query): Query<AlertQuery>,
) -> Result<Json<PaginatedResponse<AlertEntry>>, ApiError> {
    let limit = query.limit.clamp(1, 100);
    let offset = query.offset.max(0);

    let (count_query, data_query) = match query.filter.as_str() {
        "resolved" => (
            "SELECT COUNT(*) FROM admin_alerts WHERE resolved = 1",
            "SELECT id, alert_type, severity, message, entity_type, entity_id, resolved, resolved_by, created_at, resolved_at FROM admin_alerts WHERE resolved = 1 ORDER BY created_at DESC LIMIT ? OFFSET ?",
        ),
        "all" => (
            "SELECT COUNT(*) FROM admin_alerts",
            "SELECT id, alert_type, severity, message, entity_type, entity_id, resolved, resolved_by, created_at, resolved_at FROM admin_alerts ORDER BY created_at DESC LIMIT ? OFFSET ?",
        ),
        _ => (
            "SELECT COUNT(*) FROM admin_alerts WHERE resolved = 0",
            "SELECT id, alert_type, severity, message, entity_type, entity_id, resolved, resolved_by, created_at, resolved_at FROM admin_alerts WHERE resolved = 0 ORDER BY created_at DESC LIMIT ? OFFSET ?",
        ),
    };

    let total: i64 = sqlx::query_scalar(count_query)
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let rows: Vec<AlertEntry> = sqlx::query_as(data_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total,
        limit,
        offset,
    }))
}

/// PATCH /api/v1/admin/alerts/{id} — resolve an alert (protected)
pub async fn resolve_alert(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let result = sqlx::query(
        "UPDATE admin_alerts SET resolved = 1, resolved_by = ?, resolved_at = datetime('now') WHERE id = ? AND resolved = 0",
    )
    .bind(&claims.sub)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!(
            "alert {id} not found or already resolved"
        )));
    }

    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'resolve_alert', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("alert_id={id}"))
    .execute(&state.db)
    .await
    .ok();

    Ok(StatusCode::OK)
}

/// Check if a like spike should trigger an alert.
/// Called from the likes module after a successful like.
pub async fn check_like_spike(db: &sqlx::SqlitePool, entity_type: &str, entity_id: &str) {
    // Read threshold from config
    let threshold: i64 = sqlx::query_scalar::<_, String>(
        "SELECT value FROM admin_config WHERE key = 'like_alert_threshold'",
    )
    .fetch_optional(db)
    .await
    .ok()
    .flatten()
    .and_then(|v| v.parse().ok())
    .unwrap_or(50);

    // Count likes in the last 10 minutes for this entity
    let recent_likes = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM likes WHERE entity_type = ? AND entity_id = ? AND created_at > datetime('now', '-10 minutes')",
    )
    .bind(entity_type)
    .bind(entity_id)
    .fetch_one(db)
    .await
    .unwrap_or(0);

    if i64::from(recent_likes) >= threshold {
        // Check if we already have an active alert for this entity
        let existing = sqlx::query_scalar::<_, i32>(
            "SELECT COUNT(*) FROM admin_alerts WHERE alert_type = 'like_spike' AND entity_type = ? AND entity_id = ? AND resolved = 0",
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_one(db)
        .await
        .unwrap_or(1); // Default to 1 to avoid duplicate

        if existing == 0 {
            sqlx::query(
                "INSERT INTO admin_alerts (alert_type, severity, message, entity_type, entity_id) VALUES ('like_spike', 'warning', ?, ?, ?)",
            )
            .bind(format!(
                "{recent_likes} likes in 10 minutes on {entity_type} {entity_id}"
            ))
            .bind(entity_type)
            .bind(entity_id)
            .execute(db)
            .await
            .ok();
        }
    }
}
