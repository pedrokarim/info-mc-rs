use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;

use mc_render3d::{render_skin_png, RenderParams};
use mc_skin::fetch_skin;

use crate::error::ApiError;
use crate::routes::player::{CapeResponse, PlayerResponse, SkinResponse};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct Render3dParams {
    pub theta: Option<f32>,
    pub phi: Option<f32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub async fn render_skin_3d(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    Query(params): Query<Render3dParams>,
) -> Result<impl IntoResponse, ApiError> {
    let identifier = identifier.trim().to_string();
    let cache_key = identifier.to_lowercase();

    let player: PlayerResponse = if let Some(cached) = state.player_cache.get(&cache_key).await {
        (*cached).clone()
    } else {
        let profile = state
            .mojang
            .get_player(&identifier)
            .await
            .map_err(|e| match e {
                mc_mojang::MojangError::PlayerNotFound(_) => {
                    ApiError::NotFound(format!("player not found: {identifier}"))
                }
                other => ApiError::Internal(other.to_string()),
            })?;

        let r = PlayerResponse {
            uuid: profile.uuid,
            username: profile.username.clone(),
            skin: profile.skin.map(|s| SkinResponse {
                url: s.url,
                model: format!("{:?}", s.model).to_lowercase(),
            }),
            cape: profile.cape.map(|c| CapeResponse { url: c.url }),
            optifine_cape: None,
            labymod_cape: None,
            retrieved_at: chrono::Utc::now().to_rfc3339(),
        };
        state.player_cache.insert(cache_key, r.clone()).await;
        r
    };

    let skin_url = player
        .skin
        .as_ref()
        .map(|s| s.url.clone())
        .ok_or_else(|| ApiError::NotFound("player has no skin".into()))?;

    let slim = player.skin.as_ref().map(|s| s.model == "slim").unwrap_or(false);

    let skin_rgba = fetch_skin(&skin_url).await.map_err(ApiError::from)?;

    let width = params.width.unwrap_or(240).clamp(8, 512);
    let height = params.height.unwrap_or(360).clamp(8, 512);
    let theta = params.theta.unwrap_or(30.0).to_radians();
    let phi = params.phi.unwrap_or(21.0).to_radians();

    let png_bytes = render_skin_png(&skin_rgba, &RenderParams { width, height, slim, theta, phi })
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(([( header::CONTENT_TYPE, "image/png")], png_bytes))
}
