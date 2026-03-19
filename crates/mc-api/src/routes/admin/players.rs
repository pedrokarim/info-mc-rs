use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AdminPlayerQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_sort")]
    pub sort: String,
    pub search: Option<String>,
}

fn default_limit() -> i64 {
    20
}

fn default_sort() -> String {
    "last_seen_at".to_string()
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AdminPlayerEntry {
    pub uuid: String,
    pub username: String,
    pub skin_url: Option<String>,
    pub skin_model: Option<String>,
    pub status: String,
    pub views: i64,
    pub likes: i64,
    pub first_seen_at: String,
    pub last_seen_at: String,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// GET /api/v1/admin/players (protected)
pub async fn list_players(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
    Query(query): Query<AdminPlayerQuery>,
) -> Result<Json<PaginatedResponse<AdminPlayerEntry>>, ApiError> {
    let limit = query.limit.clamp(1, 100);
    let offset = query.offset.max(0);
    let has_search = query.search.as_ref().is_some_and(|s| !s.is_empty());
    let search_pattern = query
        .search
        .as_ref()
        .map(|s| format!("%{s}%"))
        .unwrap_or_default();

    // Get total count
    let total: i64 = if has_search {
        sqlx::query_scalar("SELECT COUNT(*) FROM players WHERE username LIKE ?")
            .bind(&search_pattern)
            .fetch_one(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
    } else {
        sqlx::query_scalar("SELECT COUNT(*) FROM players")
            .fetch_one(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
    };

    // Fetch rows with dynamic sort (branches to avoid SQL injection)
    let rows: Vec<AdminPlayerEntry> = match (query.sort.as_str(), has_search) {
        ("views", true) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players WHERE username LIKE ? ORDER BY views DESC LIMIT ? OFFSET ?",
            )
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("views", false) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players ORDER BY views DESC LIMIT ? OFFSET ?",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("likes", true) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players WHERE username LIKE ? ORDER BY likes DESC LIMIT ? OFFSET ?",
            )
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("likes", false) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players ORDER BY likes DESC LIMIT ? OFFSET ?",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("first_seen_at", true) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players WHERE username LIKE ? ORDER BY first_seen_at DESC LIMIT ? OFFSET ?",
            )
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("first_seen_at", false) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players ORDER BY first_seen_at DESC LIMIT ? OFFSET ?",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        // Default: last_seen_at
        (_, true) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players WHERE username LIKE ? ORDER BY last_seen_at DESC LIMIT ? OFFSET ?",
            )
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        (_, false) => {
            sqlx::query_as(
                "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
                 FROM players ORDER BY last_seen_at DESC LIMIT ? OFFSET ?",
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
    }
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(PaginatedResponse {
        data: rows,
        total,
        limit,
        offset,
    }))
}

// -- Moderation --

#[derive(Deserialize)]
pub struct ModeratePlayerBody {
    /// "active", "banned", "flagged"
    pub status: Option<String>,
    /// Reset views to 0
    #[serde(default)]
    pub reset_views: bool,
    /// Reset likes to 0 (also purges from likes table)
    #[serde(default)]
    pub reset_likes: bool,
}

/// PATCH /api/v1/admin/players/{uuid} (protected)
pub async fn moderate_player(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(uuid): Path<String>,
    Json(body): Json<ModeratePlayerBody>,
) -> Result<impl IntoResponse, ApiError> {
    // Check player exists
    let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM players WHERE uuid = ?")
        .bind(&uuid)
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if exists == 0 {
        return Err(ApiError::NotFound(format!("player {uuid} not found")));
    }

    let mut actions: Vec<String> = Vec::new();

    // Update status
    if let Some(ref status) = body.status {
        match status.as_str() {
            "active" | "banned" | "flagged" => {
                sqlx::query("UPDATE players SET status = ? WHERE uuid = ?")
                    .bind(status)
                    .bind(&uuid)
                    .execute(&state.db)
                    .await
                    .map_err(|e| ApiError::InternalError(e.to_string()))?;
                actions.push(format!("status={status}"));
            }
            _ => return Err(ApiError::InvalidAddress("invalid status value".into())),
        }
    }

    // Reset views
    if body.reset_views {
        sqlx::query("UPDATE players SET views = 0 WHERE uuid = ?")
            .bind(&uuid)
            .execute(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;
        actions.push("reset_views".into());
    }

    // Reset likes
    if body.reset_likes {
        sqlx::query("UPDATE players SET likes = 0 WHERE uuid = ?")
            .bind(&uuid)
            .execute(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;
        sqlx::query("DELETE FROM likes WHERE entity_type = 'player' AND entity_id = ?")
            .bind(&uuid)
            .execute(&state.db)
            .await
            .ok();
        actions.push("reset_likes".into());
    }

    // Audit log
    if !actions.is_empty() {
        let detail = format!("player={uuid} actions={}", actions.join(","));
        sqlx::query(
            "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'moderate_player', ?)",
        )
        .bind(&claims.sub)
        .bind(&detail)
        .execute(&state.db)
        .await
        .ok();
    }

    Ok(StatusCode::OK)
}

/// DELETE /api/v1/admin/players/{uuid} (protected)
pub async fn delete_player(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let result = sqlx::query("DELETE FROM players WHERE uuid = ?")
        .bind(&uuid)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("player {uuid} not found")));
    }

    // Also clean up likes
    sqlx::query("DELETE FROM likes WHERE entity_type = 'player' AND entity_id = ?")
        .bind(&uuid)
        .execute(&state.db)
        .await
        .ok();

    // Audit log
    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'delete_player', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("player={uuid}"))
    .execute(&state.db)
    .await
    .ok();

    Ok(StatusCode::NO_CONTENT)
}
