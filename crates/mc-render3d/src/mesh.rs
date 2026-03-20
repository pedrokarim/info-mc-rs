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
    /// true for overlay layers (hat, jacket, sleeves, pants) — uses texture alpha.
    /// false for base layers — rendered fully opaque.
    pub is_overlay: bool,
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
        Self {
            u0: ax,
            v0: ay,
            u1: bx,
            v1: by,
        }
    }
}

/// Build a box mesh for one body part.
/// `faces` = [right, left, top, bottom, front, back] UV rects in pixel coords.
/// `w, h, d` = dimensions in MC pixels (= world units here).
/// The box is centered at origin before the transform is applied.
pub fn build_box(
    w: f32,
    h: f32,
    d: f32,
    faces: &[[f32; 4]; 6], // [x, y, w, h]
    tw: f32,
    th: f32,
) -> (Vec<Vertex>, Vec<u16>) {
    let hw = w / 2.0;
    let hh = h / 2.0;
    let hd = d / 2.0;

    // face order: +x, -x, +y, -y, +z, -z
    let normals = [
        Vec3::X,
        Vec3::NEG_X,
        Vec3::Y,
        Vec3::NEG_Y,
        Vec3::Z,
        Vec3::NEG_Z,
    ];

    // Corner positions for each face — all CCW winding when viewed from outside.
    // Order within each face: bl, br, tr, tl (as seen from outside the cube).
    let face_corners: [[Vec3; 4]; 6] = [
        // +x (character left) — viewed from +x: y up, z left
        [
            Vec3::new(hw, -hh, hd),
            Vec3::new(hw, -hh, -hd),
            Vec3::new(hw, hh, -hd),
            Vec3::new(hw, hh, hd),
        ],
        // -x (character right) — viewed from -x: y up, z right
        [
            Vec3::new(-hw, -hh, -hd),
            Vec3::new(-hw, -hh, hd),
            Vec3::new(-hw, hh, hd),
            Vec3::new(-hw, hh, -hd),
        ],
        // +y (top) — viewed from +y: x right, z forward (=bottom of screen)
        [
            Vec3::new(-hw, hh, hd),
            Vec3::new(hw, hh, hd),
            Vec3::new(hw, hh, -hd),
            Vec3::new(-hw, hh, -hd),
        ],
        // -y (bottom)
        [
            Vec3::new(-hw, -hh, -hd),
            Vec3::new(hw, -hh, -hd),
            Vec3::new(hw, -hh, hd),
            Vec3::new(-hw, -hh, hd),
        ],
        // +z (front) — viewed from +z: x right, y up
        [
            Vec3::new(-hw, -hh, hd),
            Vec3::new(hw, -hh, hd),
            Vec3::new(hw, hh, hd),
            Vec3::new(-hw, hh, hd),
        ],
        // -z (back) — viewed from -z: x left, y up
        [
            Vec3::new(hw, -hh, -hd),
            Vec3::new(-hw, -hh, -hd),
            Vec3::new(-hw, hh, -hd),
            Vec3::new(hw, hh, -hd),
        ],
    ];

    // UV corners: bl, br, tr, tl — uniform mapping for all faces (no special swap).
    let mut vertices = Vec::with_capacity(24);
    let mut indices: Vec<u16> = Vec::with_capacity(36);

    for (fi, corners) in face_corners.iter().enumerate() {
        let uv_r = UvRect::from_px(
            faces[fi][0],
            faces[fi][1],
            faces[fi][2],
            faces[fi][3],
            tw,
            th,
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
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    (vertices, indices)
}

/// Build the full Minecraft character mesh (base + overlay layers).
/// Returns a list of `Part` with their world transforms.
/// Face order: +x, -x, +y, -y, +z, -z  (right, left, top, bottom, front, back)
/// Side faces of arms (+x/-x) always use depth=4, not aw.
/// `has_overlay` should be true only if the skin image has any pixel with alpha < 255
/// (otherwise the overlay UV regions contain opaque garbage and must be skipped).
pub fn build_character(slim: bool, has_overlay: bool, time: f32) -> Vec<Part> {
    const TW: f32 = 64.0;
    const TH: f32 = 64.0;

    let aw: f32 = if slim { 3.0 } else { 4.0 }; // arm width
    let arm_off: f32 = if slim { 5.5 } else { 6.0 };

    // ── Base layer UV tables ──────────────────────────────────────────
    let head_uv: [[f32; 4]; 6] = [
        [16.0, 8.0, 8.0, 8.0],
        [0.0, 8.0, 8.0, 8.0],
        [8.0, 0.0, 8.0, 8.0],
        [16.0, 7.0, 8.0, -8.0],
        [8.0, 8.0, 8.0, 8.0],
        [24.0, 8.0, 8.0, 8.0],
    ];
    let body_uv: [[f32; 4]; 6] = [
        [28.0, 20.0, 4.0, 12.0],
        [16.0, 20.0, 4.0, 12.0],
        [20.0, 16.0, 8.0, 4.0],
        [28.0, 19.0, 8.0, -4.0],
        [20.0, 20.0, 8.0, 12.0],
        [32.0, 20.0, 8.0, 12.0],
    ];
    let rarm_uv: [[f32; 4]; 6] = [
        [44.0 + aw, 20.0, 4.0, 12.0],
        [40.0, 20.0, 4.0, 12.0],
        [44.0, 16.0, aw, 4.0],
        [44.0 + aw, 19.0, aw, -4.0],
        [44.0, 20.0, aw, 12.0],
        [44.0 + aw + 4.0, 20.0, aw, 12.0],
    ];
    let larm_uv: [[f32; 4]; 6] = [
        [36.0 + aw, 52.0, 4.0, 12.0],
        [32.0, 52.0, 4.0, 12.0],
        [36.0, 48.0, aw, 4.0],
        [36.0 + aw, 51.0, aw, -4.0],
        [36.0, 52.0, aw, 12.0],
        [36.0 + aw + 4.0, 52.0, aw, 12.0],
    ];
    let rleg_uv: [[f32; 4]; 6] = [
        [8.0, 20.0, 4.0, 12.0],
        [0.0, 20.0, 4.0, 12.0],
        [4.0, 16.0, 4.0, 4.0],
        [8.0, 19.0, 4.0, -4.0],
        [4.0, 20.0, 4.0, 12.0],
        [12.0, 20.0, 4.0, 12.0],
    ];
    let lleg_uv: [[f32; 4]; 6] = [
        [24.0, 52.0, 4.0, 12.0],
        [16.0, 52.0, 4.0, 12.0],
        [20.0, 48.0, 4.0, 4.0],
        [24.0, 51.0, 4.0, -4.0],
        [20.0, 52.0, 4.0, 12.0],
        [28.0, 52.0, 4.0, 12.0],
    ];

    // ── Overlay layer UV tables ───────────────────────────────────────
    let hat_uv: [[f32; 4]; 6] = [
        [48.0, 8.0, 8.0, 8.0],
        [32.0, 8.0, 8.0, 8.0],
        [40.0, 0.0, 8.0, 8.0],
        [48.0, 7.0, 8.0, -8.0],
        [40.0, 8.0, 8.0, 8.0],
        [56.0, 8.0, 8.0, 8.0],
    ];
    let jacket_uv: [[f32; 4]; 6] = [
        [28.0, 36.0, 4.0, 12.0],
        [16.0, 36.0, 4.0, 12.0],
        [20.0, 32.0, 8.0, 4.0],
        [28.0, 35.0, 8.0, -4.0],
        [20.0, 36.0, 8.0, 12.0],
        [32.0, 36.0, 8.0, 12.0],
    ];
    let rsleeve_uv: [[f32; 4]; 6] = [
        [44.0 + aw, 36.0, 4.0, 12.0],
        [40.0, 36.0, 4.0, 12.0],
        [44.0, 32.0, aw, 4.0],
        [44.0 + aw, 35.0, aw, -4.0],
        [44.0, 36.0, aw, 12.0],
        [44.0 + aw + 4.0, 36.0, aw, 12.0],
    ];
    let lsleeve_uv: [[f32; 4]; 6] = [
        [52.0 + aw, 52.0, 4.0, 12.0],
        [48.0, 52.0, 4.0, 12.0],
        [52.0, 48.0, aw, 4.0],
        [52.0 + aw, 51.0, aw, -4.0],
        [52.0, 52.0, aw, 12.0],
        [52.0 + aw + 4.0, 52.0, aw, 12.0],
    ];
    let rpant_uv: [[f32; 4]; 6] = [
        [8.0, 36.0, 4.0, 12.0],
        [0.0, 36.0, 4.0, 12.0],
        [4.0, 32.0, 4.0, 4.0],
        [8.0, 35.0, 4.0, -4.0],
        [4.0, 36.0, 4.0, 12.0],
        [12.0, 36.0, 4.0, 12.0],
    ];
    let lpant_uv: [[f32; 4]; 6] = [
        [8.0, 52.0, 4.0, 12.0],
        [0.0, 52.0, 4.0, 12.0],
        [4.0, 48.0, 4.0, 4.0],
        [8.0, 51.0, 4.0, -4.0],
        [4.0, 52.0, 4.0, 12.0],
        [12.0, 52.0, 4.0, 12.0],
    ];

    // Overlay boxes are 0.5 larger than base (0.25 per face, matches MC client)
    const OV: f32 = 0.5;

    // Walking pose: time in degrees, default 90 = max swing
    let angle = time.to_radians().sin();

    let mut parts = Vec::new();

    // ── Head + Hat ───────────────────────────────────────────────────
    let head_t = Mat4::from_translation(Vec3::new(0.0, 12.0, 0.0));
    let (v, i) = build_box(8.0, 8.0, 8.0, &head_uv, TW, TH);
    parts.push(Part {
        vertices: v,
        indices: i,
        transform: head_t,
        is_overlay: false,
    });
    if has_overlay {
        let (v, i) = build_box(9.0, 9.0, 9.0, &hat_uv, TW, TH);
        parts.push(Part {
            vertices: v,
            indices: i,
            transform: head_t,
            is_overlay: true,
        });
    }

    // ── Body + Jacket ────────────────────────────────────────────────
    let body_t = Mat4::from_translation(Vec3::new(0.0, 2.0, 0.0));
    let (v, i) = build_box(8.0, 12.0, 4.0, &body_uv, TW, TH);
    parts.push(Part {
        vertices: v,
        indices: i,
        transform: body_t,
        is_overlay: false,
    });
    if has_overlay {
        let (v, i) = build_box(8.0 + OV, 12.0 + OV, 4.0 + OV, &jacket_uv, TW, TH);
        parts.push(Part {
            vertices: v,
            indices: i,
            transform: body_t,
            is_overlay: true,
        });
    }

    // ── Right arm + Sleeve ───────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(-arm_off, 6.0, 0.0))
            * Mat4::from_rotation_x((-18f32).to_radians() * angle)
            * Mat4::from_translation(Vec3::new(0.0, -4.0, 0.0));
        let (v, i) = build_box(aw, 12.0, 4.0, &rarm_uv, TW, TH);
        parts.push(Part {
            vertices: v,
            indices: i,
            transform: t,
            is_overlay: false,
        });
        if has_overlay {
            let (v, i) = build_box(aw + OV, 12.0 + OV, 4.0 + OV, &rsleeve_uv, TW, TH);
            parts.push(Part {
                vertices: v,
                indices: i,
                transform: t,
                is_overlay: true,
            });
        }
    }

    // ── Left arm + Sleeve ────────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(arm_off, 6.0, 0.0))
            * Mat4::from_rotation_x((18f32).to_radians() * angle)
            * Mat4::from_translation(Vec3::new(0.0, -4.0, 0.0));
        let (v, i) = build_box(aw, 12.0, 4.0, &larm_uv, TW, TH);
        parts.push(Part {
            vertices: v,
            indices: i,
            transform: t,
            is_overlay: false,
        });
        if has_overlay {
            let (v, i) = build_box(aw + OV, 12.0 + OV, 4.0 + OV, &lsleeve_uv, TW, TH);
            parts.push(Part {
                vertices: v,
                indices: i,
                transform: t,
                is_overlay: true,
            });
        }
    }

    // ── Right leg + Pants ────────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(-2.0, -4.0, 0.0))
            * Mat4::from_rotation_x((20f32).to_radians() * angle)
            * Mat4::from_translation(Vec3::new(0.0, -6.0, 0.0));
        let (v, i) = build_box(4.0, 12.0, 4.0, &rleg_uv, TW, TH);
        parts.push(Part {
            vertices: v,
            indices: i,
            transform: t,
            is_overlay: false,
        });
        if has_overlay {
            let (v, i) = build_box(4.0 + OV, 12.0 + OV, 4.0 + OV, &rpant_uv, TW, TH);
            parts.push(Part {
                vertices: v,
                indices: i,
                transform: t,
                is_overlay: true,
            });
        }
    }

    // ── Left leg + Pants ─────────────────────────────────────────────
    {
        let t = Mat4::from_translation(Vec3::new(2.0, -4.0, 0.0))
            * Mat4::from_rotation_x((-20f32).to_radians() * angle)
            * Mat4::from_translation(Vec3::new(0.0, -6.0, 0.0));
        let (v, i) = build_box(4.0, 12.0, 4.0, &lleg_uv, TW, TH);
        parts.push(Part {
            vertices: v,
            indices: i,
            transform: t,
            is_overlay: false,
        });
        if has_overlay {
            let (v, i) = build_box(4.0 + OV, 12.0 + OV, 4.0 + OV, &lpant_uv, TW, TH);
            parts.push(Part {
                vertices: v,
                indices: i,
                transform: t,
                is_overlay: true,
            });
        }
    }

    parts
}

/// Back equipment type for cape/elytra rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackEquipment {
    Cape,
    Elytra,
    None,
}

/// Build a cape mesh using the cape texture.
/// Returns a single Part positioned at the back of the body with a slight tilt.
pub fn build_cape(cape_tw: f32, cape_th: f32) -> Vec<Part> {
    // Cape scale detection (same logic as client)
    let cs = cape_scale(cape_th);

    // Cape UVs in cape texture pixel coords — same as Three.js client
    let cape_uv: [[f32; 4]; 6] = [
        [11.0 * cs, cs, cs, 16.0 * cs],        // +x right
        [0.0, cs, cs, 16.0 * cs],              // -x left
        [cs, 0.0, 10.0 * cs, cs],              // +y top
        [11.0 * cs, cs, 10.0 * cs, -cs],       // -y bottom (flip)
        [cs, cs, 10.0 * cs, 16.0 * cs],        // +z front (outer)
        [12.0 * cs, cs, 10.0 * cs, 16.0 * cs], // -z back (inner)
    ];

    let (v, i) = build_box(10.0, 16.0, 1.0, &cape_uv, cape_tw, cape_th);

    // Transform: translate mesh so pivot is at top edge, then position at back of body
    // Mesh offset: (0, -8, 0.5) then group at (0, 8, -2) rotated Y=π and tilt X=18°
    let t = Mat4::from_translation(Vec3::new(0.0, 8.0, -2.0))
        * Mat4::from_rotation_y(std::f32::consts::PI)
        * Mat4::from_rotation_x(18f32.to_radians())
        * Mat4::from_translation(Vec3::new(0.0, -8.0, 0.5));

    vec![Part {
        vertices: v,
        indices: i,
        transform: t,
        is_overlay: false,
    }]
}

/// Build elytra wings using the cape texture.
/// Returns 2 Parts (left wing + right wing) in idle pose.
pub fn build_elytra(cape_tw: f32, cape_th: f32) -> Vec<Part> {
    let cs = cape_scale(cape_th);

    // Elytra UVs from skinview3d: setCapeUVs(22, 0, 10, 20, 2)
    let elytra_uv: [[f32; 4]; 6] = [
        [(22.0 + 10.0) * cs, 2.0 * cs, 2.0 * cs, 20.0 * cs], // +x right side
        [22.0 * cs, 2.0 * cs, 2.0 * cs, 20.0 * cs],          // -x left side
        [(22.0 + 2.0) * cs, 0.0, 10.0 * cs, 2.0 * cs],       // +y top
        [(22.0 + 2.0 + 10.0) * cs, 0.0, 10.0 * cs, 2.0 * cs], // -y bottom
        [(22.0 + 2.0) * cs, 2.0 * cs, 10.0 * cs, 20.0 * cs], // +z front (outer)
        [
            (22.0 + 2.0 + 10.0 + 2.0) * cs,
            2.0 * cs,
            10.0 * cs,
            20.0 * cs,
        ], // -z back (inner)
    ];

    let (v_left, i_left) = build_box(12.0, 22.0, 4.0, &elytra_uv, cape_tw, cape_th);
    let (v_right_raw, i_right) = build_box(12.0, 22.0, 4.0, &elytra_uv, cape_tw, cape_th);

    // Mirror right wing on X axis (negate X positions and X normals)
    let v_right: Vec<Vertex> = v_right_raw
        .into_iter()
        .map(|mut v| {
            v.pos[0] = -v.pos[0];
            v.normal[0] = -v.normal[0];
            v
        })
        .collect();

    // Left wing: parent at (0,8,0), wing pivot at (5,0,0) rot (0.2618, 0.01, 0.1), mesh at (-5,-10,0)
    let left_t = Mat4::from_translation(Vec3::new(0.0, 8.0, 0.0))
        * Mat4::from_translation(Vec3::new(5.0, 0.0, 0.0))
        * Mat4::from_euler(glam::EulerRot::XYZ, 0.2618, 0.01, 0.1)
        * Mat4::from_translation(Vec3::new(-5.0, -10.0, 0.0));

    // Right wing: parent at (0,8,0), wing pivot at (-5,0,0) rot (0.2618, -0.01, -0.1), mesh at (5,-10,0)
    let right_t = Mat4::from_translation(Vec3::new(0.0, 8.0, 0.0))
        * Mat4::from_translation(Vec3::new(-5.0, 0.0, 0.0))
        * Mat4::from_euler(glam::EulerRot::XYZ, 0.2618, -0.01, -0.1)
        * Mat4::from_translation(Vec3::new(5.0, -10.0, 0.0));

    vec![
        Part {
            vertices: v_left,
            indices: i_left,
            transform: left_t,
            is_overlay: false,
        },
        Part {
            vertices: v_right,
            indices: i_right,
            transform: right_t,
            is_overlay: false,
        },
    ]
}

/// Detect cape texture scale from height.
fn cape_scale(h: f32) -> f32 {
    let h = h as u32;
    if h.is_multiple_of(22) {
        return (h / 22) as f32;
    }
    if h.is_multiple_of(17) {
        return (h / 17) as f32;
    }
    if h >= 32 && (h & (h - 1)) == 0 {
        return (h / 32) as f32;
    }
    (h / 22).max(1) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    #[test]
    fn test_build_box_vertex_count() {
        let faces = [[0.0, 0.0, 1.0, 1.0]; 6];
        let (verts, indices) = build_box(1.0, 1.0, 1.0, &faces, 64.0, 64.0);
        assert_eq!(verts.len(), 24); // 4 per face × 6 faces
        assert_eq!(indices.len(), 36); // 6 per face × 6 faces
    }

    #[test]
    fn test_build_box_normals() {
        let faces = [[0.0, 0.0, 1.0, 1.0]; 6];
        let (verts, _) = build_box(1.0, 1.0, 1.0, &faces, 64.0, 64.0);

        let expected_normals = [
            Vec3::X,
            Vec3::NEG_X,
            Vec3::Y,
            Vec3::NEG_Y,
            Vec3::Z,
            Vec3::NEG_Z,
        ];

        for (fi, expected) in expected_normals.iter().enumerate() {
            for vi in 0..4 {
                let v = &verts[fi * 4 + vi];
                let n = Vec3::from_array(v.normal);
                assert_eq!(n, *expected, "face {fi} vertex {vi} normal mismatch");
            }
        }
    }

    #[test]
    fn test_build_box_uvs_in_range() {
        let faces = [
            [8.0, 0.0, 8.0, 8.0],
            [0.0, 8.0, 8.0, 8.0],
            [16.0, 8.0, 8.0, 8.0],
            [24.0, 8.0, 8.0, 8.0],
            [8.0, 8.0, 8.0, 8.0],
            [32.0, 8.0, 8.0, 8.0],
        ];
        let (verts, _) = build_box(8.0, 8.0, 8.0, &faces, 64.0, 64.0);
        for (i, v) in verts.iter().enumerate() {
            assert!(
                v.uv[0] >= 0.0 && v.uv[0] <= 1.0,
                "vertex {i} u={} out of range",
                v.uv[0]
            );
            assert!(
                v.uv[1] >= 0.0 && v.uv[1] <= 1.0,
                "vertex {i} v={} out of range",
                v.uv[1]
            );
        }
    }

    #[test]
    fn test_build_character_part_count_no_overlay() {
        let parts = build_character(false, false, 90.0);
        assert_eq!(parts.len(), 6);
    }

    #[test]
    fn test_build_character_part_count_with_overlay() {
        let parts = build_character(false, true, 90.0);
        assert_eq!(parts.len(), 12);
    }

    #[test]
    fn test_build_character_slim_part_count() {
        let parts = build_character(true, true, 90.0);
        assert_eq!(parts.len(), 12);
    }

    #[test]
    fn test_build_character_no_overlay_flags() {
        let parts = build_character(false, false, 90.0);
        for (i, part) in parts.iter().enumerate() {
            assert!(!part.is_overlay, "part {i} should not be overlay");
        }
    }

    #[test]
    fn test_build_character_walking_pose_time_zero() {
        let parts_zero = build_character(false, false, 0.0);
        // With time=0, sin(0)=0 so all limbs have no rotation applied.
        // Arms (indices 2, 3) and legs (indices 4, 5) should have transforms
        // equivalent to pure translation (no rotation component from the swing).
        // Compare right arm and left arm: they should be mirror images in X only.
        let rarm = &parts_zero[2];
        let larm = &parts_zero[3];
        // At time=0 the rotation matrices are identity, so the arm transforms
        // differ only in X translation sign.
        let r_col3 = rarm.transform.col(3);
        let l_col3 = larm.transform.col(3);
        assert!(
            (r_col3.y - l_col3.y).abs() < 1e-5,
            "arms should have same Y at time=0"
        );
        assert!(
            (r_col3.z - l_col3.z).abs() < 1e-5,
            "arms should have same Z at time=0"
        );
    }

    #[test]
    fn test_build_character_walking_pose_time_90() {
        let parts_zero = build_character(false, false, 0.0);
        let parts_90 = build_character(false, false, 90.0);

        // Right arm transform should differ between time=0 and time=90
        let t0 = parts_zero[2].transform;
        let t90 = parts_90[2].transform;
        assert_ne!(
            t0, t90,
            "right arm transform should change with walking time"
        );

        // Right leg transform should also differ
        let t0_leg = parts_zero[4].transform;
        let t90_leg = parts_90[4].transform;
        assert_ne!(
            t0_leg, t90_leg,
            "right leg transform should change with walking time"
        );
    }
}
