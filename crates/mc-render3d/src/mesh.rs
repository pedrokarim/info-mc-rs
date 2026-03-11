//! Minecraft skin mesh builder.
//!
//! Generates vertex/index buffers for each body part with correct UV mapping.
//! UV coordinates are derived from pixel positions in the 64×64 (or 64×32) skin texture.

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3};

/// One vertex: position, UV, normal.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    fn new(pos: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self {
            pos: pos.to_array(),
            uv: uv.to_array(),
            normal: normal.to_array(),
        }
    }
}

/// A mesh part: vertices + indices + transform.
pub struct Part {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub transform: Mat4,
}

/// UV rectangle in texture pixels → [0..1] range.
/// `flip_x`/`flip_y` mirrors the face.
#[derive(Clone, Copy)]
struct UvRect {
    u0: f32,
    v0: f32,
    u1: f32,
    v1: f32,
}

impl UvRect {
    fn from_px(x: f32, y: f32, w: f32, h: f32, tw: f32, th: f32) -> Self {
        let flip_x = w < 0.0;
        let flip_y = h < 0.0;
        let (ax, bx) = if flip_x {
            ((x + w.abs()) / tw, x / tw)
        } else {
            (x / tw, (x + w) / tw)
        };
        let (ay, by) = if flip_y {
            ((y + h.abs()) / th, y / th)
        } else {
            (y / th, (y + h) / th)
        };
        Self { u0: ax, v0: ay, u1: bx, v1: by }
    }
}

/// Build a box mesh for one body part.
/// `faces` = [right, left, top, bottom, front, back] UV rects in pixel coords.
/// `w, h, d` = dimensions in MC pixels (= world units here).
/// The box is centered at origin before the transform is applied.
pub fn build_box(
    w: f32, h: f32, d: f32,
    faces: &[[f32; 4]; 6], // [x, y, w, h]
    tw: f32, th: f32,
) -> (Vec<Vertex>, Vec<u16>) {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;

    // face order: +x, -x, +y, -y, +z, -z
    let normals = [
        Vec3::X, Vec3::NEG_X,
        Vec3::Y, Vec3::NEG_Y,
        Vec3::Z, Vec3::NEG_Z,
    ];

    // Corner positions for each face (CCW winding when viewed from outside)
    let face_corners: [[Vec3; 4]; 6] = [
        // +x (right)
        [Vec3::new(hw,-hh,-hd), Vec3::new(hw,-hh, hd), Vec3::new(hw, hh, hd), Vec3::new(hw, hh,-hd)],
        // -x (left)
        [Vec3::new(-hw,-hh, hd), Vec3::new(-hw,-hh,-hd), Vec3::new(-hw, hh,-hd), Vec3::new(-hw, hh, hd)],
        // +y (top)
        [Vec3::new(-hw, hh, hd), Vec3::new( hw, hh, hd), Vec3::new( hw, hh,-hd), Vec3::new(-hw, hh,-hd)],
        // -y (bottom)
        [Vec3::new(-hw,-hh,-hd), Vec3::new( hw,-hh,-hd), Vec3::new( hw,-hh, hd), Vec3::new(-hw,-hh, hd)],
        // +z (front)
        [Vec3::new(-hw,-hh, hd), Vec3::new( hw,-hh, hd), Vec3::new( hw, hh, hd), Vec3::new(-hw, hh, hd)],
        // -z (back)
        [Vec3::new( hw,-hh,-hd), Vec3::new(-hw,-hh,-hd), Vec3::new(-hw, hh,-hd), Vec3::new( hw, hh,-hd)],
    ];

    // UV corners: bl, br, tr, tl (matching face_corners order)
    let mut vertices = Vec::with_capacity(24);
    let mut indices: Vec<u16> = Vec::with_capacity(36);

    for (fi, corners) in face_corners.iter().enumerate() {
        let uv_r = UvRect::from_px(
            faces[fi][0], faces[fi][1],
            faces[fi][2], faces[fi][3],
            tw, th,
        );
        let uvs = [
            Vec2::new(uv_r.u0, uv_r.v1), // bl
            Vec2::new(uv_r.u1, uv_r.v1), // br
            Vec2::new(uv_r.u1, uv_r.v0), // tr
            Vec2::new(uv_r.u0, uv_r.v0), // tl
        ];
        let base = vertices.len() as u16;
        for (ci, &c) in corners.iter().enumerate() {
            vertices.push(Vertex::new(c, uvs[ci], normals[fi]));
        }
        // Two triangles: 0,1,2 and 0,2,3
        indices.extend_from_slice(&[base, base+1, base+2, base, base+2, base+3]);
    }

    (vertices, indices)
}

/// Build the full Minecraft character mesh (base + overlay layers).
/// Returns a list of `Part` with their world transforms.
/// Face order: +x, -x, +y, -y, +z, -z  (right, left, top, bottom, front, back)
/// Side faces of arms (+x/-x) always use depth=4, not aw.
pub fn build_character(slim: bool) -> Vec<Part> {
    const TW: f32 = 64.0;
    const TH: f32 = 64.0;

    let aw: f32 = if slim { 3.0 } else { 4.0 }; // arm width
    let arm_off: f32 = if slim { 5.5 } else { 6.0 };

    // ── Base layer UV tables ──────────────────────────────────────────
    let head_uv: [[f32; 4]; 6] = [
        [16.0,8.0,8.0,8.0],  [0.0,8.0,8.0,8.0],
        [8.0,0.0,8.0,8.0],   [16.0,0.0,8.0,8.0],
        [8.0,8.0,8.0,8.0],   [24.0,8.0,8.0,8.0],
    ];
    let body_uv: [[f32; 4]; 6] = [
        [28.0,20.0,4.0,12.0], [16.0,20.0,4.0,12.0],
        [20.0,16.0,8.0,4.0],  [28.0,16.0,8.0,4.0],
        [20.0,20.0,8.0,12.0], [32.0,20.0,8.0,12.0],
    ];
    let rarm_uv: [[f32; 4]; 6] = [
        [44.0+aw,20.0,4.0,12.0], [40.0,20.0,4.0,12.0],
        [44.0,16.0,aw,4.0],       [44.0+aw,16.0,aw,4.0],
        [44.0,20.0,aw,12.0],      [44.0+aw+4.0,20.0,aw,12.0],
    ];
    let larm_uv: [[f32; 4]; 6] = [
        [36.0+aw,52.0,4.0,12.0], [32.0,52.0,4.0,12.0],
        [36.0,48.0,aw,4.0],       [36.0+aw,48.0,aw,4.0],
        [36.0,52.0,aw,12.0],      [36.0+aw+4.0,52.0,aw,12.0],
    ];
    let rleg_uv: [[f32; 4]; 6] = [
        [8.0,20.0,4.0,12.0],  [0.0,20.0,4.0,12.0],
        [4.0,16.0,4.0,4.0],   [8.0,16.0,4.0,4.0],
        [4.0,20.0,4.0,12.0],  [12.0,20.0,4.0,12.0],
    ];
    let lleg_uv: [[f32; 4]; 6] = [
        [24.0,52.0,4.0,12.0], [16.0,52.0,4.0,12.0],
        [20.0,48.0,4.0,4.0],  [24.0,48.0,4.0,4.0],
        [20.0,52.0,4.0,12.0], [28.0,52.0,4.0,12.0],
    ];

    // ── Overlay layer UV tables ───────────────────────────────────────
    let hat_uv: [[f32; 4]; 6] = [
        [48.0,8.0,8.0,8.0],  [32.0,8.0,8.0,8.0],
        [40.0,0.0,8.0,8.0],  [48.0,0.0,8.0,8.0],
        [40.0,8.0,8.0,8.0],  [56.0,8.0,8.0,8.0],
    ];
    let jacket_uv: [[f32; 4]; 6] = [
        [28.0,36.0,4.0,12.0], [16.0,36.0,4.0,12.0],
        [20.0,32.0,8.0,4.0],  [28.0,32.0,8.0,4.0],
        [20.0,36.0,8.0,12.0], [32.0,36.0,8.0,12.0],
    ];
    let rsleeve_uv: [[f32; 4]; 6] = [
        [44.0+aw,36.0,4.0,12.0], [40.0,36.0,4.0,12.0],
        [44.0,32.0,aw,4.0],       [44.0+aw,32.0,aw,4.0],
        [44.0,36.0,aw,12.0],      [44.0+aw+4.0,36.0,aw,12.0],
    ];
    let lsleeve_uv: [[f32; 4]; 6] = [
        [52.0+aw,52.0,4.0,12.0], [48.0,52.0,4.0,12.0],
        [52.0,48.0,aw,4.0],       [52.0+aw,48.0,aw,4.0],
        [52.0,52.0,aw,12.0],      [52.0+aw+4.0,52.0,aw,12.0],
    ];
    let rpant_uv: [[f32; 4]; 6] = [
        [8.0,36.0,4.0,12.0],  [0.0,36.0,4.0,12.0],
        [4.0,32.0,4.0,4.0],   [8.0,32.0,4.0,4.0],
        [4.0,36.0,4.0,12.0],  [12.0,36.0,4.0,12.0],
    ];
    let lpant_uv: [[f32; 4]; 6] = [
        [8.0,52.0,4.0,12.0],  [0.0,52.0,4.0,12.0],
        [4.0,48.0,4.0,4.0],   [8.0,48.0,4.0,4.0],
        [4.0,52.0,4.0,12.0],  [12.0,52.0,4.0,12.0],
    ];

    // Overlay boxes are 0.5 larger than base (0.25 per face, matches MC client)
    const OV: f32 = 0.5;

    let mut parts = Vec::new();

    // ── Head + Hat ───────────────────────────────────────────────────
    let head_t = Mat4::from_translation(Vec3::new(0.0, 12.0, 0.0));
    let (v, i) = build_box(8.0, 8.0, 8.0, &head_uv, TW, TH);
    parts.push(Part { vertices: v, indices: i, transform: head_t });
    let (v, i) = build_box(8.0+OV, 8.0+OV, 8.0+OV, &hat_uv, TW, TH);
    parts.push(Part { vertices: v, indices: i, transform: head_t });

    // ── Body + Jacket ────────────────────────────────────────────────
    let (v, i) = build_box(8.0, 12.0, 4.0, &body_uv, TW, TH);
    parts.push(Part { vertices: v, indices: i, transform: Mat4::IDENTITY });
    let (v, i) = build_box(8.0+OV, 12.0+OV, 4.0+OV, &jacket_uv, TW, TH);
    parts.push(Part { vertices: v, indices: i, transform: Mat4::IDENTITY });

    // ── Right arm + Sleeve ───────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(-arm_off, 6.0, 0.0))
              * Mat4::from_translation(Vec3::new(0.0, -6.0, 0.0));
        let (v, i) = build_box(aw, 12.0, 4.0, &rarm_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
        let (v, i) = build_box(aw+OV, 12.0+OV, 4.0+OV, &rsleeve_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
    }

    // ── Left arm + Sleeve ────────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(arm_off, 6.0, 0.0))
              * Mat4::from_translation(Vec3::new(0.0, -6.0, 0.0));
        let (v, i) = build_box(aw, 12.0, 4.0, &larm_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
        let (v, i) = build_box(aw+OV, 12.0+OV, 4.0+OV, &lsleeve_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
    }

    // ── Right leg + Pants ────────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(-2.0, -6.0, 0.0))
              * Mat4::from_translation(Vec3::new(0.0, -6.0, 0.0));
        let (v, i) = build_box(4.0, 12.0, 4.0, &rleg_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
        let (v, i) = build_box(4.0+OV, 12.0+OV, 4.0+OV, &rpant_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
    }

    // ── Left leg + Pants ─────────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(2.0, -6.0, 0.0))
              * Mat4::from_translation(Vec3::new(0.0, -6.0, 0.0));
        let (v, i) = build_box(4.0, 12.0, 4.0, &lleg_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
        let (v, i) = build_box(4.0+OV, 12.0+OV, 4.0+OV, &lpant_uv, TW, TH);
        parts.push(Part { vertices: v, indices: i, transform: t });
    }

    parts
}
