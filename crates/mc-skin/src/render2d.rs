use image::{RgbaImage, imageops};

use crate::{Result, SkinError};

pub struct RenderOptions {
    pub size: u32,
    pub overlay: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            size: 128,
            overlay: true,
        }
    }
}

/// Render the front face of the head (8x8 pixels from the skin, scaled up).
pub fn render_face(skin: &RgbaImage, opts: &RenderOptions) -> Result<RgbaImage> {
    validate_skin(skin)?;

    // Head front face: x=8, y=8, 8x8
    let mut face = crop_region(skin, 8, 8, 8, 8);

    // Overlay (hat layer) head front: x=40, y=8, 8x8
    if opts.overlay {
        let overlay = crop_region(skin, 40, 8, 8, 8);
        imageops::overlay(&mut face, &overlay, 0, 0);
    }

    Ok(scale_nearest(&face, opts.size, opts.size))
}

/// Render the head with all 6 visible faces in isometric-ish front view.
/// Returns a simple front view of the head for now.
pub fn render_head(skin: &RgbaImage, opts: &RenderOptions) -> Result<RgbaImage> {
    // For now, just render the face. Isometric rendering can be added later.
    render_face(skin, opts)
}

/// Render a full front view of the player body (flattened, not isometric).
/// Layout: head(8x8) on top of body(8x12), arms(4x12) on sides, legs(4x12) below.
/// Total base: 16x32 pixels.
pub fn render_full_body(skin: &RgbaImage, opts: &RenderOptions) -> Result<RgbaImage> {
    validate_skin(skin)?;
    let is_64x64 = skin.height() == 64;

    // Base canvas: 16 wide, 32 tall
    let mut canvas = RgbaImage::new(16, 32);

    // --- Base layers ---

    // Head front: 8x8 at skin(8,8), place at canvas(4,0)
    let head = crop_region(skin, 8, 8, 8, 8);
    imageops::overlay(&mut canvas, &head, 4, 0);

    // Body front: 8x12 at skin(20,20), place at canvas(4,8)
    let body = crop_region(skin, 20, 20, 8, 12);
    imageops::overlay(&mut canvas, &body, 4, 8);

    // Right arm front: 4x12 at skin(44,20), place at canvas(0,8)
    let right_arm = crop_region(skin, 44, 20, 4, 12);
    imageops::overlay(&mut canvas, &right_arm, 0, 8);

    // Left arm front: depends on skin format
    if is_64x64 {
        // 64x64 skin: left arm at skin(36,52)
        let left_arm = crop_region(skin, 36, 52, 4, 12);
        imageops::overlay(&mut canvas, &left_arm, 12, 8);
    } else {
        // 64x32 legacy skin: mirror right arm
        let mut left_arm = right_arm.clone();
        imageops::flip_horizontal_in_place(&mut left_arm);
        imageops::overlay(&mut canvas, &left_arm, 12, 8);
    }

    // Right leg front: 4x12 at skin(4,20), place at canvas(4,20)
    let right_leg = crop_region(skin, 4, 20, 4, 12);
    imageops::overlay(&mut canvas, &right_leg, 4, 20);

    // Left leg front: depends on skin format
    if is_64x64 {
        // 64x64 skin: left leg at skin(20,52)
        let left_leg = crop_region(skin, 20, 52, 4, 12);
        imageops::overlay(&mut canvas, &left_leg, 8, 20);
    } else {
        // Legacy: mirror right leg
        let mut left_leg = right_leg.clone();
        imageops::flip_horizontal_in_place(&mut left_leg);
        imageops::overlay(&mut canvas, &left_leg, 8, 20);
    }

    // --- Overlay layers (64x64 skins only) ---
    if opts.overlay && is_64x64 {
        // Head overlay: skin(40,8)
        let head_ol = crop_region(skin, 40, 8, 8, 8);
        imageops::overlay(&mut canvas, &head_ol, 4, 0);

        // Body overlay: skin(20,36)
        let body_ol = crop_region(skin, 20, 36, 8, 12);
        imageops::overlay(&mut canvas, &body_ol, 4, 8);

        // Right arm overlay: skin(44,36)
        let ra_ol = crop_region(skin, 44, 36, 4, 12);
        imageops::overlay(&mut canvas, &ra_ol, 0, 8);

        // Left arm overlay: skin(52,52)
        let la_ol = crop_region(skin, 52, 52, 4, 12);
        imageops::overlay(&mut canvas, &la_ol, 12, 8);

        // Right leg overlay: skin(4,36)
        let rl_ol = crop_region(skin, 4, 36, 4, 12);
        imageops::overlay(&mut canvas, &rl_ol, 4, 20);

        // Left leg overlay: skin(4,52)
        let ll_ol = crop_region(skin, 4, 52, 4, 12);
        imageops::overlay(&mut canvas, &ll_ol, 8, 20);
    }

    // Scale to target size, maintaining 1:2 aspect ratio
    let width = opts.size;
    let height = opts.size * 2;
    Ok(scale_nearest(&canvas, width, height))
}

fn validate_skin(skin: &RgbaImage) -> Result<()> {
    let (w, h) = skin.dimensions();
    if w != 64 || (h != 64 && h != 32) {
        return Err(SkinError::InvalidImage(format!(
            "expected 64x64 or 64x32, got {w}x{h}"
        )));
    }
    Ok(())
}

fn crop_region(img: &RgbaImage, x: u32, y: u32, w: u32, h: u32) -> RgbaImage {
    imageops::crop_imm(img, x, y, w, h).to_image()
}

fn scale_nearest(img: &RgbaImage, width: u32, height: u32) -> RgbaImage {
    imageops::resize(img, width, height, imageops::FilterType::Nearest)
}
