use axum::extract::{Path, State};
use axum::Json;
use serde::Serialize;

use crate::error::ApiError;
use crate::state::SharedState;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerResponse {
    pub uuid: String,
    pub username: String,
    pub skin: Option<SkinResponse>,
    pub cape: Option<CapeResponse>,
    pub retrieved_at: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkinResponse {
    pub url: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapeResponse {
    pub url: String,
}

pub async fn get_player(
    State(state): State<SharedState>,
    Path(identifier): Path<String>,
) -> Result<Json<PlayerResponse>, ApiError> {
    let identifier = identifier.trim().to_string();
    if identifier.is_empty() {
        return Err(ApiError::InvalidAddress("empty player identifier".into()));
    }

    // Check cache
    let cache_key = identifier.to_lowercase();
    if let Some(cached) = state.player_cache.get(&cache_key).await {
        return Ok(Json((*cached).clone()));
    }

    let profile = state
        .mojang
        .get_player(&identifier)
        .await
        .map_err(|e| match e {
            mc_mojang::MojangError::PlayerNotFound(_) => {
                ApiError::InvalidAddress(format!("player not found: {identifier}"))
            }
            mc_mojang::MojangError::InvalidUsername(msg) => ApiError::InvalidAddress(msg),
            mc_mojang::MojangError::RateLimited => ApiError::RateLimited,
            other => ApiError::Internal(other.to_string()),
        })?;

    let now = chrono::Utc::now().to_rfc3339();

    let response = PlayerResponse {
        uuid: profile.uuid,
        username: profile.username,
        skin: profile.skin.map(|s| SkinResponse {
            url: s.url,
            model: format!("{:?}", s.model).to_lowercase(),
        }),
        cape: profile.cape.map(|c| CapeResponse { url: c.url }),
        retrieved_at: now,
    };

    state
        .player_cache
        .insert(cache_key, response.clone())
        .await;

    Ok(Json(response))
}
