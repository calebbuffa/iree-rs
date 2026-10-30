//! WGPU render → GPU buffer copy → IREE edge analysis → window presentation.
//!
//! Each frame WGPU renders an animated image and copies it into a shared
//! buffer, an IREE program detects its edges into a second shared buffer, and
//! WGPU composites both to the window. There is no per-frame CPU readback, no
//! lease and no `unsafe`: the context orders every submission. With separate
//! queues the frame never blocks on IREE; WGPU waits for it on the GPU.
//!
//! Lost surfaces are recreated; device loss is fatal.
//!
//! ```text
//! cargo run -p iree-wgpu --features compiler --example render_analysis [-- --frames COUNT]
//! ```

#[path = "support/edge_analysis.rs"]
mod analysis;

use iree_wgpu::{
    Context, ContextOptions, Program, SharedBuffer, Surface, compiler::InputType,
    runtime::hal::ElementType, wgpu,
};
use std::{error::Error, sync::Arc, time::Instant};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const DISPLAY: &str = r#"
@group(0) @binding(0) var image: texture_2d<f32>;
@group(0) @binding(1) var<storage, read> edges: array<f32>;
@vertex fn vertex(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
    return vec4<f32>(positions[i], 0.0, 1.0);
}
@fragment fn fragment(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    // Presentation uses a fixed-size viewport; resizing letterboxes the image.
    let xy = clamp(vec2<i32>(p.xy), vec2<i32>(0), vec2<i32>(255, 191));
    let edge_xy = clamp(xy - vec2<i32>(1), vec2<i32>(0), vec2<i32>(253, 189));
    let edge = edges[u32(edge_xy.y) * 254u + u32(edge_xy.x)];
    let base = textureLoad(image, xy, 0).rgb;
    return vec4<f32>(mix(base, vec3<f32>(1.0, 0.12, 0.02), smoothstep(0.05, 0.2, edge)), 1.0);
}
"#;

#[derive(Debug, PartialEq, Eq)]
enum SurfaceRecovery {
    Skip,
    Reconfigure,
    Recreate,
}

fn surface_recovery(status: wgpu::CurrentSurfaceTexture) -> Result<SurfaceRecovery> {
    match status {
        wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
            Ok(SurfaceRecovery::Skip)
        }
        wgpu::CurrentSurfaceTexture::Outdated => Ok(SurfaceRecovery::Reconfigure),
        wgpu::CurrentSurfaceTexture::Lost => Ok(SurfaceRecovery::Recreate),
        wgpu::CurrentSurfaceTexture::Validation => {
            Err("surface acquisition failed validation".into())
        }
        wgpu::CurrentSurfaceTexture::Success(_) | wgpu::CurrentSurfaceTexture::Suboptimal(_) => {
            Err("surface recovery received a successfully acquired frame".into())
        }
    }
}

struct SurfaceSize {
    width: u32,
    height: u32,
    dirty: bool,
}

impl SurfaceSize {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            dirty: true,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.dirty = true;
    }

    fn drawable(&self) -> bool {
        self.width != 0 && self.height != 0
    }

    fn invalidate(&mut self) {
        self.dirty = true;
    }

    fn configured(&mut self) {
        self.dirty = false;
    }

    fn pending_extent(&self) -> Option<(u32, u32)> {
        (self.drawable() && self.dirty).then_some((self.width, self.height))
    }
}

struct Renderer {
    surface: Surface,
    config: wgpu::SurfaceConfiguration,
    context: Context,
    program: Program,
    input: SharedBuffer,
    output: SharedBuffer,
    image: wgpu::Texture,
    image_view: wgpu::TextureView,
    render: wgpu::RenderPipeline,
    render_group: wgpu::BindGroup,
    display: wgpu::RenderPipeline,
    display_group: wgpu::BindGroup,
    uniform: wgpu::Buffer,
    started: Instant,
    surface_size: SurfaceSize,
}

impl Renderer {
    async fn new(window: Arc<Window>) -> Result<Self> {
        let (context, surface) =
            Context::with_window(window.clone(), ContextOptions::default()).await?;
        println!(
            "Shared Vulkan adapter: {} ({:?})",
            context.adapter_name(),
            context.queue_topology()
        );
        let size = window.inner_size();
        let config = surface
            .default_config(size.width.max(1), size.height.max(1))
            .ok_or("adapter cannot present to this surface")?;
        let program = context.compile(&analysis::source(), InputType::StableHlo)?;
        let input = context.allocate_shared((analysis::HEIGHT * analysis::WIDTH * 16) as usize)?;
        let output =
            context.allocate_shared((analysis::EDGE_HEIGHT * analysis::EDGE_WIDTH * 4) as usize)?;
        let device = context.device();
        let image = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("GPU-resident rendered image"),
            size: extent(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let image_view = image.create_view(&Default::default());
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("animation time"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let render = pipeline(
            device,
            analysis::RENDER,
            wgpu::TextureFormat::Rgba32Float,
            None,
        );
        let render_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("animation binding"),
            layout: &render.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        let display_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image and shared edge buffer"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("presentation layout"),
            bind_group_layouts: &[Some(&display_layout)],
            immediate_size: 0,
        });
        let display = pipeline(device, DISPLAY, config.format, Some(&layout));
        // The shared buffer is bound once; the context orders every use.
        let display_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("present shared IREE edges"),
            layout: &display_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&image_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: output.binding(),
                },
            ],
        });
        Ok(Self {
            surface,
            config,
            context,
            program,
            input,
            output,
            image,
            image_view,
            render,
            render_group,
            display,
            display_group,
            uniform,
            started: Instant::now(),
            surface_size: SurfaceSize::new(size.width, size.height),
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.surface_size.resize(width, height);
    }

    fn configure(&mut self, width: u32, height: u32) -> Result<()> {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.config)?;
        self.surface_size.configured();
        Ok(())
    }

    fn recreate_surface(&mut self, window: Arc<Window>) -> Result<()> {
        let surface = self.context.create_surface(window)?;
        let config = surface
            .default_config(
                self.surface_size.width.max(1),
                self.surface_size.height.max(1),
            )
            .ok_or("recreated surface is incompatible with the shared adapter")?;
        if config.format != self.config.format {
            let device = self.context.device();
            let group_layout = self.display.get_bind_group_layout(0);
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("recreated presentation layout"),
                bind_group_layouts: &[Some(&group_layout)],
                immediate_size: 0,
            });
            self.display = pipeline(device, DISPLAY, config.format, Some(&layout));
        }
        self.surface = surface;
        self.config = config;
        self.surface_size.invalidate();
        Ok(())
    }

    fn draw(&mut self, window: &Arc<Window>) -> Result<bool> {
        if !self.surface_size.drawable() {
            return Ok(false);
        }
        if let Some((width, height)) = self.surface_size.pending_extent() {
            self.configure(width, height)?;
        }
        let frame = match self.surface.current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {
                // Reconfigure only after this acquired frame has been presented.
                self.surface_size.invalidate();
                frame
            }
            status => {
                match surface_recovery(status)? {
                    SurfaceRecovery::Skip => {}
                    SurfaceRecovery::Reconfigure => {
                        let size = window.inner_size();
                        self.resize(size.width, size.height);
                    }
                    SurfaceRecovery::Recreate => {
                        let size = window.inner_size();
                        self.resize(size.width, size.height);
                        self.recreate_surface(window.clone())?;
                    }
                }
                return Ok(false);
            }
        };
        let context = &self.context;
        let device = context.device();
        let mut time = [0u8; 16];
        time[..4].copy_from_slice(&self.started.elapsed().as_secs_f32().to_ne_bytes());
        context.write_buffer(&self.uniform, 0, &time)?;

        // 1. WGPU renders and copies the image into the shared input.
        let mut encoder = device.create_command_encoder(&Default::default());
        render_pass(
            &mut encoder,
            &self.image_view,
            &self.render,
            &self.render_group,
            None,
        );
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.image,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: self.input.wgpu(),
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(analysis::WIDTH * 16),
                    rows_per_image: Some(analysis::HEIGHT),
                },
            },
            extent(),
        );
        context.submit([encoder.finish()])?;

        // 2. IREE analyzes it in place, after the WGPU work above.
        let results = self.program.call_views(
            "module.main",
            &[
                &self.input.view(
                    &[analysis::HEIGHT as _, analysis::WIDTH as _, 4],
                    ElementType::FLOAT_32,
                )?,
                &self.output.view(
                    &[analysis::EDGE_HEIGHT as _, analysis::EDGE_WIDTH as _],
                    ElementType::FLOAT_32,
                )?,
            ],
        )?;
        if results.len() != 1 {
            return Err("edge analysis returned an unexpected result count".into());
        }

        // 3. WGPU composites the edges, after the IREE work above.
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = device.create_command_encoder(&Default::default());
        render_pass(
            &mut encoder,
            &view,
            &self.display,
            &self.display_group,
            Some((
                self.config.width.min(analysis::WIDTH),
                self.config.height.min(analysis::HEIGHT),
            )),
        );
        context.submit([encoder.finish()])?;
        window.pre_present_notify();
        context.present(frame)?;
        Ok(true)
    }
}

fn extent() -> wgpu::Extent3d {
    wgpu::Extent3d {
        width: analysis::WIDTH,
        height: analysis::HEIGHT,
        depth_or_array_layers: 1,
    }
}

fn pipeline(
    device: &wgpu::Device,
    source: &str,
    format: wgpu::TextureFormat,
    layout: Option<&wgpu::PipelineLayout>,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("render/analysis example WGSL"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render/analysis example pipeline"),
        layout,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vertex"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fragment"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}

fn render_pass(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    pipeline: &wgpu::RenderPipeline,
    group: &wgpu::BindGroup,
    viewport: Option<(u32, u32)>,
) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("render or composite"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        })],
        ..Default::default()
    });
    if let Some((width, height)) = viewport {
        pass.set_viewport(0.0, 0.0, width as f32, height as f32, 0.0, 1.0);
    }
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, group, &[]);
    pass.draw(0..3, 0..1);
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    error: Option<Box<dyn Error>>,
    frame_limit: Option<u64>,
    frames: u64,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() || self.error.is_some() {
            return;
        }
        let result = (|| {
            let window = Arc::new(
                event_loop.create_window(
                    Window::default_attributes()
                        .with_title("WGPU render -> IREE edges -> screen")
                        .with_inner_size(winit::dpi::PhysicalSize::new(
                            analysis::WIDTH,
                            analysis::HEIGHT,
                        )),
                )?,
            );
            let renderer = pollster::block_on(Renderer::new(window.clone()))?;
            Ok::<_, Box<dyn Error>>((window, renderer))
        })();
        match result {
            Ok((window, renderer)) => {
                self.renderer = Some(renderer);
                window.request_redraw();
                self.window = Some(window);
            }
            Err(error) => {
                eprintln!("Graphics initialization failed: {error}");
                self.error = Some(error);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(window) = &self.window else { return };
        if window.id() != id {
            return;
        }
        let Some(renderer) = &mut self.renderer else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                renderer.resize(size.width, size.height);
                if renderer.surface_size.drawable() {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => match renderer.draw(window) {
                Ok(presented) => {
                    self.frames += u64::from(presented);
                    if self.frame_limit.is_some_and(|limit| self.frames >= limit) {
                        println!(
                            "Presented {} frames without CPU image readback",
                            self.frames
                        );
                        event_loop.exit();
                    } else if renderer.surface_size.drawable() {
                        window.request_redraw();
                    }
                }
                Err(error) => {
                    eprintln!("Rendering failed: {error}");
                    self.error = Some(error);
                    event_loop.exit();
                }
            },
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let mut arguments = std::env::args().skip(1);
    let frame_limit = match arguments.next().as_deref() {
        None => None,
        Some("--frames") => Some(
            arguments
                .next()
                .ok_or("--frames requires a count")?
                .parse::<u64>()?,
        ),
        Some(_) => return Err("usage: render_analysis [--frames COUNT]".into()),
    };
    if frame_limit == Some(0) || arguments.next().is_some() {
        return Err("usage: render_analysis [--frames POSITIVE_COUNT]".into());
    }
    let mut app = App {
        frame_limit,
        ..Default::default()
    };
    EventLoop::new()?.run_app(&mut app)?;
    if let Some(error) = app.error {
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_acquisition_failures() {
        use wgpu::CurrentSurfaceTexture as Status;
        for (status, expected) in [
            (Status::Timeout, SurfaceRecovery::Skip),
            (Status::Occluded, SurfaceRecovery::Skip),
            (Status::Outdated, SurfaceRecovery::Reconfigure),
            (Status::Lost, SurfaceRecovery::Recreate),
        ] {
            assert_eq!(surface_recovery(status).unwrap(), expected);
        }
        assert!(
            surface_recovery(Status::Validation)
                .unwrap_err()
                .to_string()
                .contains("failed validation")
        );
    }

    #[test]
    fn zero_extent_defers_configuration_until_restored() {
        for (width, height) in [(0, 0), (0, 192), (256, 0)] {
            let mut size = SurfaceSize::new(width, height);
            assert!(!size.drawable());
            assert_eq!(size.pending_extent(), None);
            assert!(size.dirty);
            size.resize(640, 480);
            assert_eq!(size.pending_extent(), Some((640, 480)));
        }
    }

    #[test]
    fn resize_coalesces_and_recovery_is_deferred() {
        let mut size = SurfaceSize::new(256, 192);
        assert_eq!(size.pending_extent(), Some((256, 192)));
        size.configured();
        assert_eq!(size.pending_extent(), None);
        size.resize(800, 600);
        size.resize(1024, 768);
        assert_eq!(size.pending_extent(), Some((1024, 768)));
        size.configured();
        // Suboptimal acquisition schedules configuration after presentation.
        size.invalidate();
        assert_eq!(size.pending_extent(), Some((1024, 768)));
        size.resize(0, 0);
        assert_eq!(size.pending_extent(), None);
        size.resize(320, 240);
        assert_eq!(size.pending_extent(), Some((320, 240)));
    }
}
