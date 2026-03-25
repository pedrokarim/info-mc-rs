use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use mc_skin::RenderOptions;

use crate::error::ApiError;
use crate::state::SharedState;

#[derive(Deserialize)]
pub struct RenderQuery {
    /// "face", "head", or "full" (default: "full")
    #[serde(default = "default_render_type")]
    pub r#type: String,
    /// Pixel size (8-512, default: 128)
    #[serde(default = "default_size")]
    pub size: u32,
    /// Include overlay layer (default: true)
    #[serde(default = "default_overlay")]
    pub overlay: bool,
}

fn default_render_type() -> String {
    "full".to_string()
}
fn default_size() -> u32 {
    128
}
fn default_overlay() -> bool {
    true
}

pub async fn render_skin(
    State(state): State<SharedState>,
    Path(identifier): Path<String>,
    Query(query): Query<RenderQuery>,
) -> Result<Response, ApiError> {
    let size = query.size.clamp(8, 512);

    // Get player profile to find skin URL
    let profile = state
        .mojang
        .get_player(&identifier)
        .await
        .map_err(|e| match e {
            mc_mojang::MojangError::PlayerNotFound(_) => {
                ApiError::InvalidAddress(format!("player not found: {identifier}"))
            }
            mc_mojang::MojangError::InvalidUsername(msg) => ApiError::InvalidAddress(msg),
            other => ApiError::Internal(other.to_string()),
        })?;

    let skin_url = profile
        .skin
        .as_ref()
        .map(|s| s.url.as_str())
        .ok_or_else(|| ApiError::Internal("player has no skin".into()))?;

    // Fetch skin image
    let skin_img = mc_skin::fetch_skin(&state.http, skin_url)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to fetch skin: {e}")))?;

    let opts = RenderOptions {
        size,
        overlay: query.overlay,
    };

    // Render based on type
    let rendered = match query.r#type.as_str() {
        "face" => mc_skin::render_face(&skin_img, &opts),
        "head" => mc_skin::render_head(&skin_img, &opts),
        _ => mc_skin::render_full_body(&skin_img, &opts),
    }
    .map_err(|e| ApiError::Internal(format!("render failed: {e}")))?;

    // Encode as PNG
    let mut png_bytes = std::io::Cursor::new(Vec::new());
    rendered
        .write_to(&mut png_bytes, image::ImageFormat::Png)
        .map_err(|e| ApiError::Internal(format!("PNG encode failed: {e}")))?;

    Ok((
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        png_bytes.into_inner(),
    )
        .into_response())
}
