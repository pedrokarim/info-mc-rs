mod fetch;
mod render2d;

pub use fetch::fetch_skin;
pub use render2d::{render_face, render_head, render_full_body, RenderOptions};

#[derive(Debug, thiserror::Error)]
pub enum SkinError {
    #[error("failed to fetch skin: {0}")]
    Fetch(String),

    #[error("invalid skin image: {0}")]
    InvalidImage(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("image error: {0}")]
    Image(#[from] image::ImageError),
}

pub type Result<T> = std::result::Result<T, SkinError>;
