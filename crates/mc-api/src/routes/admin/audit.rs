use std::sync::Arc;

use axum::Json;
use axum::extract::{Extension, Query, State};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

use super::players::PaginatedResponse;

#[derive(Deserialize)]
pub struct AuditQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AuditEntry {
    pub id: i64,
    pub discord_id: String,
    pub action: String,
    pub detail: Option<String>,
    pub created_at: String,
}

/// GET /api/v1/admin/audit (protected)
pub async fn list_audit(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
    Query(query): Query<AuditQuery>,
) -> Result<Json<PaginatedResponse<AuditEntry>>, ApiError> {
    let limit = query.limit.clamp(1, 100);
    let offset = query.offset.max(0);

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admin_audit_log")
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let rows: Vec<AuditEntry> = sqlx::query_as(
        "SELECT id, discord_id, action, detail, created_at
         FROM admin_audit_log ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
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
