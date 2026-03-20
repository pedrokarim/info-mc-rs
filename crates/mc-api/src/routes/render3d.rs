use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::IntoResponse;
use serde::Deserialize;

use mc_render3d::{BackEquipment, RenderParams, render_skin_png};
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
    pub time: Option<f32>,
    /// Back equipment: "cape", "elytra", or "none" (default: "cape").
    pub back: Option<String>,
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
            cape: profile.cape.map(|c| CapeResponse {
                url: c.url,
                active: None,
            }),
            optifine_cape: None,
            labymod_cape: None,
            retrieved_at: chrono::Utc::now().to_rfc3339(),
            popularity: None,
        };
        state.player_cache.insert(cache_key, r.clone()).await;
        r
    };

    let skin_url = player
        .skin
        .as_ref()
        .map(|s| s.url.clone())
        .ok_or_else(|| ApiError::NotFound("player has no skin".into()))?;

    let slim = player
        .skin
        .as_ref()
        .map(|s| s.model == "slim")
        .unwrap_or(false);

    let back_equipment = match params.back.as_deref() {
        Some("elytra") => BackEquipment::Elytra,
        Some("none") => BackEquipment::None,
        _ => BackEquipment::Cape, // default
    };

    // Fetch skin texture
    let skin_rgba = fetch_skin(&skin_url).await.map_err(ApiError::from)?;

    // Fetch cape texture if needed
    let cape_rgba = if back_equipment != BackEquipment::None {
        if let Some(cape_url) = player.cape.as_ref().map(|c| &c.url) {
            fetch_skin(cape_url).await.ok()
        } else {
            None
        }
    } else {
        None
    };

    let width = params.width.unwrap_or(240).clamp(8, 512);
    let height = params.height.unwrap_or(360).clamp(8, 512);
    let theta = params.theta.unwrap_or(30.0).to_radians();
    let phi = params.phi.unwrap_or(21.0).to_radians();
    let time = params.time.unwrap_or(90.0);

    let png_bytes = render_skin_png(
        &skin_rgba,
        &RenderParams {
            width,
            height,
            slim,
            theta,
            phi,
            time,
            cape_rgba,
            back_equipment,
        },
    )
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok((
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        png_bytes,
    ))
}
