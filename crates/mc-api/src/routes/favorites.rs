use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

/// Validate a Minecraft UUID format (32 hex chars, with or without dashes).
fn is_valid_uuid(s: &str) -> bool {
    let stripped: String = s.chars().filter(|c| *c != '-').collect();
    stripped.len() == 32 && stripped.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validate a Minecraft username (1-16 chars, alphanumeric + underscore).
fn is_valid_username(s: &str) -> bool {
    !s.is_empty() && s.len() <= 16 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

#[derive(Serialize, sqlx::FromRow)]
struct FavoriteRow {
    uuid: String,
    username: String,
    favorited_at: String,
}

#[derive(Serialize)]
struct FavoriteStatus {
    favorited: bool,
}

#[derive(Deserialize)]
pub struct AddFavoriteBody {
    pub username: String,
}

/// GET /api/v1/favorites — list all favorites
pub async fn list_favorites(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, StatusCode> {
    let rows: Vec<FavoriteRow> = sqlx::query_as(
        "SELECT uuid, username, favorited_at FROM favorites ORDER BY favorited_at DESC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(rows))
}

/// GET /api/v1/favorites/:uuid — check if UUID is favorited
pub async fn is_favorite(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    if !is_valid_uuid(&uuid) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM favorites WHERE uuid = ?")
        .bind(&uuid)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(FavoriteStatus {
        favorited: exists > 0,
    }))
}

/// POST /api/v1/favorites/:uuid — add to favorites
pub async fn add_favorite(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
    Json(body): Json<AddFavoriteBody>,
) -> Result<impl IntoResponse, StatusCode> {
    if !is_valid_uuid(&uuid) || !is_valid_username(&body.username) {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query(
        "INSERT OR REPLACE INTO favorites (uuid, username, favorited_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(&uuid)
    .bind(&body.username)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

/// DELETE /api/v1/favorites/:uuid — remove from favorites
pub async fn remove_favorite(
    State(state): State<Arc<AppState>>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    if !is_valid_uuid(&uuid) {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query("DELETE FROM favorites WHERE uuid = ?")
        .bind(&uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
