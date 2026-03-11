use image::RgbaImage;

use crate::{Result, SkinError};

/// Fetch a skin PNG from a URL and return it as an RGBA image.
pub async fn fetch_skin(url: &str) -> Result<RgbaImage> {
    let client = reqwest::Client::new();
    let bytes = client
        .get(url)
        .send()
        .await?
        .error_for_status()
        .map_err(|e| SkinError::Fetch(e.to_string()))?
        .bytes()
        .await?;

    let img = image::load_from_memory(&bytes)?;
    Ok(img.to_rgba8())
}
