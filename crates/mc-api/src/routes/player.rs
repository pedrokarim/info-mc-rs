use axum::Json;
use axum::extract::{Path, State};
use serde::Serialize;

use crate::error::ApiError;
use crate::state::SharedState;

#[derive(Debug, Clone, Serialize)]
pub struct PlayerResponse {
    pub uuid: String,
    pub username: String,
    pub skin: Option<SkinResponse>,
    pub cape: Option<CapeResponse>,
    pub optifine_cape: Option<CapeResponse>,
    pub labymod_cape: Option<CapeResponse>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
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

    // Check third-party capes in parallel (best-effort, 3s timeout on client)
    let uuid_clean = profile.uuid.replace('-', "");
    let (optifine_cape, labymod_cape) = tokio::join!(
        check_optifine_cape(&state.http, &profile.username),
        check_labymod_cape(&state.http, &uuid_clean),
    );

    // Use proxy URLs so the frontend doesn't hit CORS issues
    let optifine_cape = optifine_cape.map(|c| CapeResponse {
        url: format!("/api/v1/cape/optifine/{}", profile.username),
        active: c.active,
    });
    let labymod_cape = labymod_cape.map(|_| CapeResponse {
        url: format!("/api/v1/cape/labymod/{uuid_clean}"),
        active: None,
    });

    let now = chrono::Utc::now().to_rfc3339();

    let response = PlayerResponse {
        uuid: profile.uuid,
        username: profile.username,
        skin: profile.skin.map(|s| SkinResponse {
            url: s.url,
            model: format!("{:?}", s.model).to_lowercase(),
        }),
        cape: profile.cape.map(|c| CapeResponse { url: c.url, active: None }),
        optifine_cape,
        labymod_cape,
        retrieved_at: now,
    };

    state.player_cache.insert(cache_key, response.clone()).await;

    Ok(Json(response))
}

/// Check both active and inactive Optifine capes.
async fn check_optifine_cape(http: &reqwest::Client, username: &str) -> Option<CapeResponse> {
    // Try active cape first
    let active_url = format!("https://optifine.net/capes/{username}.png");
    if let Ok(resp) = http.head(&active_url).send().await {
        if resp.status().is_success() {
            return Some(CapeResponse {
                url: active_url,
                active: Some(true),
            });
        }
    }
    // Fallback to inactive cape
    let inactive_url = format!("https://optifine.net/capes/inactive/{username}.png");
    let resp = http.head(&inactive_url).send().await.ok()?;
    if resp.status().is_success() {
        Some(CapeResponse {
            url: inactive_url,
            active: Some(false),
        })
    } else {
        None
    }
}

/// HEAD `https://dl.labymod.net/capes/{uuid_no_dashes}`
async fn check_labymod_cape(http: &reqwest::Client, uuid: &str) -> Option<CapeResponse> {
    let url = format!("https://dl.labymod.net/capes/{uuid}");
    let resp = http.head(&url).send().await.ok()?;
    if resp.status().is_success() {
        Some(CapeResponse { url, active: None })
    } else {
        None
    }
}
