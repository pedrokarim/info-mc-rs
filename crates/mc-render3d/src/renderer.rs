//! Offscreen wgpu renderer for Minecraft skin 3D preview.
//! Renders the character to an RGBA texture then exports PNG bytes.

use std::borrow::Cow;

use bytemuck::cast_slice;
use glam::{Mat4, Vec3};
use image::{ImageFormat, RgbaImage};
use wgpu::util::DeviceExt;

use crate::error::RenderError;
use crate::mesh::{build_character, Vertex};

/// Render parameters.
pub struct RenderParams {
    pub width: u32,
    pub height: u32,
    pub slim: bool,
    /// Horizontal rotation in radians (Y-axis).
    pub theta: f32,
    /// Vertical tilt in radians (X-axis).
    pub phi: f32,
}

impl Default for RenderParams {
    fn default() -> Self {
        Self {
            width: 240,
            height: 360,
            slim: false,
            theta: std::f32::consts::FRAC_PI_6, // 30°
            phi: 0.366,                          // ~21°
        }
    }
}

/// Render the skin texture to PNG bytes.
pub async fn render_skin_png(
    skin_rgba: &RgbaImage,
    params: &RenderParams,
) -> Result<Vec<u8>, RenderError> {
    // ── wgpu device ─────────────────────────────────────────────────
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .ok_or(RenderError::AdapterNotFound)?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await?;

    let w = params.width;
    let h = params.height;

    // ── Color + depth targets ────────────────────────────────────────
    let color_fmt = wgpu::TextureFormat::Rgba8UnormSrgb;
    let depth_fmt = wgpu::TextureFormat::Depth32Float;

    let color_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("color"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: color_fmt,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let color_view = color_tex.create_view(&Default::default());

    let depth_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: depth_fmt,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_view = depth_tex.create_view(&Default::default());

    // ── Skin texture ─────────────────────────────────────────────────
    let skin_tex = device.create_texture_with_data(
        &queue,
        &wgpu::TextureDescriptor {
            label: Some("skin"),
            size: wgpu::Extent3d {
                width: skin_rgba.width(),
                height: skin_rgba.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        skin_rgba.as_raw(),
    );
    let skin_view = skin_tex.create_view(&Default::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // ── Shader (WGSL) ────────────────────────────────────────────────
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("skin_shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SHADER_SRC)),
    });

    // ── Bind group layout ────────────────────────────────────────────
    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            // uniform: matrices
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // texture
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // sampler
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    // ── Pipeline ─────────────────────────────────────────────────────
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("skin_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_fmt,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: depth_fmt,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        multiview: None,
        cache: None,
    });

    // ── Matrices ─────────────────────────────────────────────────────
    let aspect = w as f32 / h as f32;
    let proj = Mat4::perspective_rh(38f32.to_radians(), aspect, 0.1, 1000.0);
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 4.0, 60.0),
        Vec3::new(0.0, 4.0, 0.0),
        Vec3::Y,
    );
    // Character rotation
    let rot = Mat4::from_euler(glam::EulerRot::XYZ, params.phi, params.theta, 0.0);
    let model_base = Mat4::from_translation(Vec3::new(0.0, 4.0, 0.0)) * rot;

    // ── Mesh ─────────────────────────────────────────────────────────
    // Check if skin has any transparency (if not, overlay regions contain garbage)
    let has_overlay = skin_rgba.pixels().any(|p| p.0[3] < 255);
    let parts = build_character(params.slim, has_overlay);

    // ── Render pass ──────────────────────────────────────────────────
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("skin_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &color_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);

        for part in &parts {
            let model = model_base * part.transform;
            let mvp = proj * view * model;

            // Uniform buffer: mvp (16 f32) + model (16 f32) + is_overlay (1 u32 as f32, padded to vec4)
            let mut uniform_data = [0f32; 36];
            uniform_data[..16].copy_from_slice(&mvp.to_cols_array());
            uniform_data[16..32].copy_from_slice(&model.to_cols_array());
            uniform_data[32] = if part.is_overlay { 1.0 } else { 0.0 };

            let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: cast_slice(&uniform_data),
                usage: wgpu::BufferUsages::UNIFORM,
            });

            let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bgl,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: uniform_buf.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&skin_view) },
                    wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
                ],
            });

            let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: cast_slice(&part.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let ibuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: cast_slice(&part.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            pass.set_bind_group(0, &bg, &[]);
            pass.set_vertex_buffer(0, vbuf.slice(..));
            pass.set_index_buffer(ibuf.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..part.indices.len() as u32, 0, 0..1);
        }
    }

    // ── Readback ─────────────────────────────────────────────────────
    let bytes_per_row = align_to(w * 4, 256);
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: (bytes_per_row * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        color_tex.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    queue.submit([encoder.finish()]);

    let slice = readback.slice(..);
    let (tx, rx) = tokio::sync::oneshot::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { let _ = tx.send(r); });
    device.poll(wgpu::Maintain::Wait);
    rx.await.unwrap()?;

    let data = slice.get_mapped_range();
    // Strip wgpu row padding (bytes_per_row may be > w*4)
    let mut pixels: Vec<u8> = Vec::with_capacity((w * h * 4) as usize);
    for row in 0..h {
        let start = (row * bytes_per_row) as usize;
        let end = start + (w * 4) as usize;
        pixels.extend_from_slice(&data[start..end]);
    }
    drop(data);
    readback.unmap();

    // ── Encode PNG ───────────────────────────────────────────────────
    let img = RgbaImage::from_raw(w, h, pixels).expect("valid dimensions");
    let mut png_bytes = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png_bytes), ImageFormat::Png)?;

    Ok(png_bytes)
}

fn align_to(val: u32, align: u32) -> u32 {
    (val + align - 1) & !(align - 1)
}

// ── WGSL Shader ───────────────────────────────────────────────────────
const SHADER_SRC: &str = r#"
struct Uniforms {
    mvp:        mat4x4<f32>,
    model:      mat4x4<f32>,
    is_overlay: f32,
    _pad0:      f32,
    _pad1:      f32,
    _pad2:      f32,
}
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var t_skin: texture_2d<f32>;
@group(0) @binding(2) var s_skin: sampler;

struct VIn {
    @location(0) pos:    vec3<f32>,
    @location(1) uv:     vec2<f32>,
    @location(2) normal: vec3<f32>,
}
struct VOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv:         vec2<f32>,
    @location(1) world_n:    vec3<f32>,
}

@vertex
fn vs_main(v: VIn) -> VOut {
    var out: VOut;
    out.clip   = u.mvp * vec4<f32>(v.pos, 1.0);
    out.uv     = v.uv;
    out.world_n = normalize((u.model * vec4<f32>(v.normal, 0.0)).xyz);
    return out;
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4<f32> {
    let col = textureSample(t_skin, s_skin, in.uv);

    // Overlay parts: discard fully transparent pixels
    // Base parts: render opaque (skin base regions may have garbage alpha)
    if u.is_overlay > 0.5 {
        if col.a < 0.1 { discard; }
    }

    let alpha = select(1.0, col.a, u.is_overlay > 0.5);

    // Simple lambertian shading
    let ambient   = vec3<f32>(0.7, 0.7, 0.7);
    let light_dir = normalize(vec3<f32>(0.678, 0.284, 0.678));
    let diff      = max(dot(in.world_n, light_dir), 0.0) * 0.3;
    let lighting  = ambient + vec3<f32>(diff, diff, diff);

    return vec4<f32>(col.rgb * lighting, alpha);
}
"#;
