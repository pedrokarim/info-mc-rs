use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("wgpu adapter not found")]
    AdapterNotFound,
    #[error("wgpu device error: {0}")]
    DeviceError(#[from] wgpu::RequestDeviceError),
    #[error("skin fetch failed: {0}")]
    FetchError(#[from] reqwest::Error),
    #[error("image decode error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("buffer map error: {0}")]
    BufferMap(#[from] wgpu::BufferAsyncError),
    #[error("skin texture must be 64×32 or 64×64, got {0}×{1}")]
    InvalidSkinSize(u32, u32),
}
