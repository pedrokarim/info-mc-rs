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

#[derive(Deserialize)]
pub struct AdminServerQuery {
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
pub struct AdminServerEntry {
    pub address: String,
    pub hostname: String,
    pub ip: String,
    pub port: i64,
    pub edition: String,
    pub version_name: Option<String>,
    pub motd_clean: Option<String>,
    pub max_players: Option<i64>,
    pub status: String,
    pub views: i64,
    pub likes: i64,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub last_online_at: Option<String>,
}

/// GET /api/v1/admin/servers (protected)
pub async fn list_servers(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
    Query(query): Query<AdminServerQuery>,
) -> Result<Json<PaginatedResponse<AdminServerEntry>>, ApiError> {
    let limit = query.limit.clamp(1, 100);
    let offset = query.offset.max(0);
    let has_search = query.search.as_ref().is_some_and(|s| !s.is_empty());
    let search_pattern = query
        .search
        .as_ref()
        .map(|s| format!("%{s}%"))
        .unwrap_or_default();

    let total: i64 = if has_search {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM servers WHERE address LIKE ? OR hostname LIKE ?",
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    } else {
        sqlx::query_scalar("SELECT COUNT(*) FROM servers")
            .fetch_one(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
    };

    let base_cols = "address, hostname, ip, port, edition, version_name, motd_clean, max_players, status, views, likes, first_seen_at, last_seen_at, last_online_at";

    let rows: Vec<AdminServerEntry> = match (query.sort.as_str(), has_search) {
        ("views", true) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers WHERE address LIKE ? OR hostname LIKE ? ORDER BY views DESC LIMIT ? OFFSET ?"
            ))
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("views", false) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers ORDER BY views DESC LIMIT ? OFFSET ?"
            ))
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("likes", true) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers WHERE address LIKE ? OR hostname LIKE ? ORDER BY likes DESC LIMIT ? OFFSET ?"
            ))
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("likes", false) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers ORDER BY likes DESC LIMIT ? OFFSET ?"
            ))
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("first_seen_at", true) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers WHERE address LIKE ? OR hostname LIKE ? ORDER BY first_seen_at DESC LIMIT ? OFFSET ?"
            ))
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        ("first_seen_at", false) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers ORDER BY first_seen_at DESC LIMIT ? OFFSET ?"
            ))
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        // Default: last_seen_at
        (_, true) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers WHERE address LIKE ? OR hostname LIKE ? ORDER BY last_seen_at DESC LIMIT ? OFFSET ?"
            ))
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(limit)
            .bind(offset)
            .fetch_all(&state.db)
            .await
        }
        (_, false) => {
            sqlx::query_as(&format!(
                "SELECT {base_cols} FROM servers ORDER BY last_seen_at DESC LIMIT ? OFFSET ?"
            ))
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
pub struct ModerateServerBody {
    /// "active", "banned", "flagged", "verified", "pinned"
    pub status: Option<String>,
    #[serde(default)]
    pub reset_views: bool,
    #[serde(default)]
    pub reset_likes: bool,
}

/// PATCH /api/v1/admin/servers/{address} (protected)
pub async fn moderate_server(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(address): Path<String>,
    Json(body): Json<ModerateServerBody>,
) -> Result<impl IntoResponse, ApiError> {
    let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM servers WHERE address = ?")
        .bind(&address)
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if exists == 0 {
        return Err(ApiError::NotFound(format!("server {address} not found")));
    }

    let mut actions: Vec<String> = Vec::new();

    if let Some(ref status) = body.status {
        match status.as_str() {
            "active" | "banned" | "flagged" | "verified" | "pinned" => {
                sqlx::query("UPDATE servers SET status = ? WHERE address = ?")
                    .bind(status)
                    .bind(&address)
                    .execute(&state.db)
                    .await
                    .map_err(|e| ApiError::InternalError(e.to_string()))?;
                actions.push(format!("status={status}"));
            }
            _ => return Err(ApiError::InvalidAddress("invalid status value".into())),
        }
    }

    if body.reset_views {
        sqlx::query("UPDATE servers SET views = 0 WHERE address = ?")
            .bind(&address)
            .execute(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;
        actions.push("reset_views".into());
    }

    if body.reset_likes {
        sqlx::query("UPDATE servers SET likes = 0 WHERE address = ?")
            .bind(&address)
            .execute(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;
        sqlx::query("DELETE FROM likes WHERE entity_type = 'server' AND entity_id = ?")
            .bind(&address)
            .execute(&state.db)
            .await
            .ok();
        actions.push("reset_likes".into());
    }

    if !actions.is_empty() {
        let detail = format!("server={address} actions={}", actions.join(","));
        sqlx::query(
            "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'moderate_server', ?)",
        )
        .bind(&claims.sub)
        .bind(&detail)
        .execute(&state.db)
        .await
        .ok();
    }

    Ok(StatusCode::OK)
}

/// DELETE /api/v1/admin/servers/{address} (protected)
pub async fn delete_server(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let result = sqlx::query("DELETE FROM servers WHERE address = ?")
        .bind(&address)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("server {address} not found")));
    }

    sqlx::query("DELETE FROM likes WHERE entity_type = 'server' AND entity_id = ?")
        .bind(&address)
        .execute(&state.db)
        .await
        .ok();

    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'delete_server', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("server={address}"))
    .execute(&state.db)
    .await
    .ok();

    Ok(StatusCode::NO_CONTENT)
}
