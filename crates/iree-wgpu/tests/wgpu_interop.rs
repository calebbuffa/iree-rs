//! Explicit Vulkan hardware qualification for WGPU interoperability.
//!
//! Every test uses only the safe API: no leases, fences or `unsafe`.
use iree_wgpu::{Context, ContextOptions, SharedBuffer, runtime::hal::ElementType};

#[cfg(feature = "compiler")]
#[path = "../examples/support/edge_analysis.rs"]
mod edge_analysis;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn isolated() -> TestResult<bool> {
    let thread = std::thread::current();
    let name = thread
        .name()
        .ok_or_else(|| std::io::Error::other("hardware test thread has no name"))?;
    const CHILD: &str = "IREE_WGPU_TEST_CHILD";
    if std::env::var(CHILD).as_deref() == Ok(name) {
        return Ok(false);
    }
    let output = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", name, "--ignored", "--nocapture"])
        .env(CHILD, name)
        .output()?;
    if !output.status.success() || !String::from_utf8_lossy(&output.stdout).contains("1 passed") {
        return Err(std::io::Error::other(format!(
            "{name}: {}\n{}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
        .into());
    }
    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(true)
}

/// Map a staging buffer written by work already submitted through `context`.
#[cfg(feature = "compiler")]
fn map_read(
    context: &Context,
    staging: &wgpu::Buffer,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let (sender, receiver) = std::sync::mpsc::channel();
    staging
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
    context.device().poll(wgpu::PollType::wait_indefinitely())?;
    receiver.recv()??;
    let bytes = staging.slice(..).get_mapped_range()?.to_vec();
    staging.unmap();
    Ok(bytes)
}

#[cfg(feature = "compiler")]
fn f32s(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|bytes| f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        .collect()
}

fn f32_bytes(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}

/// Copy `source` into `target` with a WGPU submission.
fn wgpu_copy(context: &Context, source: &SharedBuffer, target: &SharedBuffer) -> TestResult {
    let mut encoder = context.device().create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source.wgpu(), 0, target.wgpu(), 0, source.len() as u64);
    context.submit([encoder.finish()])?;
    Ok(())
}

#[cfg(feature = "compiler")]
fn multiply_source(destination_passing: bool) -> &'static str {
    if destination_passing {
        r#"
module {
  func.func @main(%input: tensor<256xf32>,
      %destination: tensor<256xf32> {iree.abi.output = 0 : index}) -> tensor<256xf32> {
    %two = stablehlo.constant dense<2.0> : tensor<256xf32>
    %output = stablehlo.multiply %input, %two : tensor<256xf32>
    return %output : tensor<256xf32>
  }
}
"#
    } else {
        r#"
module {
  func.func @main(%input: tensor<256xf32>) -> tensor<256xf32> {
    %two = stablehlo.constant dense<2.0> : tensor<256xf32>
    %output = stablehlo.multiply %input, %two : tensor<256xf32>
    return %output : tensor<256xf32>
  }
}
"#
    }
}

/// This is intentionally an explicit hardware qualification, not a silently
/// skipped test on machines without Vulkan. Run with `--ignored --nocapture`.
#[test]
#[cfg(feature = "compiler")]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn render_iree_render_without_intermediate_readback() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    render_iree_render(false)
}

#[test]
#[cfg(feature = "compiler")]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn render_iree_render_with_destination_passing() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    render_iree_render(true)
}

#[cfg(feature = "compiler")]
fn render_iree_render(destination_passing: bool) -> TestResult {
    use iree_wgpu::compiler::InputType;

    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    println!(
        "Shared Vulkan adapter: {} ({:?})",
        context.adapter_name(),
        context.queue_topology()
    );
    let program = context.compile(multiply_source(destination_passing), InputType::StableHlo)?;
    let input = context.allocate_shared(1024)?;
    let destination = context.allocate_shared(1024)?;

    let device = context.device();
    let extent = wgpu::Extent3d {
        width: 64,
        height: 1,
        depth_or_array_layers: 1,
    };
    let texture_descriptor = wgpu::TextureDescriptor {
        label: Some("offscreen render"),
        size: extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    };
    let rendered = device.create_texture(&texture_descriptor);
    let rendered_view = rendered.create_view(&Default::default());
    let display = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("IREE results rendered by WGPU"),
        ..texture_descriptor
    });
    let display_view = display.create_view(&Default::default());
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("display IREE storage buffer"),
        source: wgpu::ShaderSource::Wgsl(
            r#"
@group(0) @binding(0) var<storage, read> pixels: array<vec4<f32>>;
@vertex fn vertex(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
  let positions = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
  return vec4<f32>(positions[i], 0.0, 1.0);
}
@fragment fn fragment(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
  return pixels[u32(p.x)];
}
"#
            .into(),
        ),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("consume IREE output"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vertex"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fragment"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba32Float,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    });
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("final rendered-image readback"),
        size: 1024,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    for color in [
        wgpu::Color {
            r: 0.25,
            g: 0.5,
            b: 0.75,
            a: 1.0,
        },
        wgpu::Color {
            r: 0.125,
            g: 0.25,
            b: 0.375,
            a: 0.5,
        },
    ] {
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render source image"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &rendered_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &rendered,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: input.wgpu(),
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(1024),
                    rows_per_image: Some(1),
                },
            },
            extent,
        );
        context.submit([encoder.finish()])?;

        // No host waits: IREE consumes the allocation WGPU's copy wrote, and
        // the next WGPU submission consumes IREE's output.
        let input_view = input.view(&[256], ElementType::FLOAT_32)?;
        let copied = if destination_passing {
            let destination_view = destination.view(&[256], ElementType::FLOAT_32)?;
            let results = program.call_views("module.main", &[&input_view, &destination_view])?;
            assert_eq!(results.len(), 1);
            None
        } else {
            let results = program.call_views("module.main", &[&input_view])?;
            assert_eq!(results.len(), 1);
            Some(context.copy_to_shared(&results[0])?)
        };
        let output = copied.as_ref().unwrap_or(&destination);

        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared IREE output"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: output.binding(),
            }],
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render analyzed image"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &display_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.draw(0..3, 0..1);
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &display,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &staging,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(1024),
                    rows_per_image: Some(1),
                },
            },
            extent,
        );
        context.submit([encoder.finish()])?;
        let expected = [
            color.r as f32 * 2.0,
            color.g as f32 * 2.0,
            color.b as f32 * 2.0,
            color.a as f32 * 2.0,
        ];
        for pixel in f32s(&map_read(&context, &staging)?).chunks_exact(4) {
            assert_eq!(pixel, expected);
        }
        println!("render -> shared IREE input -> multiply -> shared output -> render: {color:?}");
    }
    Ok(())
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn cross_api_ordering_without_host_waits() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    println!("queue topology: {:?}", context.queue_topology());

    // Allocation validation and zero fill.
    assert!(context.allocate_shared(0).is_err());
    assert!(context.allocate_shared(3).is_err());
    assert!(context.shared_buffer(&[0u8; 3]).is_err());
    let zeroed = context.allocate_shared(16)?;
    // SAFETY: allocate_shared fills the complete buffer with zero.
    assert_eq!(unsafe { context.read::<f32>(&zeroed)? }, [0.0; 4]);

    // WGPU write -> IREE copy -> WGPU copy -> read, with no waits between.
    let first = context.shared_buffer(&[1.0f32, 2.0, 3.0, 4.0])?;
    context.write_buffer(first.wgpu(), 0, &f32_bytes(&[5.0, 6.0, 7.0, 8.0]))?;
    let copied = context.copy_to_shared(&first.view(&[4], ElementType::FLOAT_32)?)?;
    let last = context.allocate_shared(16)?;
    wgpu_copy(&context, &copied, &last)?;
    // A later WGPU write must not leak into the IREE copy issued before it.
    context.write_buffer(first.wgpu(), 0, &f32_bytes(&[0.0; 4]))?;
    // SAFETY: both buffers were initialized by uploads, writes, or copies.
    let last_values = unsafe { context.read::<f32>(&last)? };
    assert_eq!(last_values, [5.0, 6.0, 7.0, 8.0]);
    // SAFETY: the full buffer was written with zeros above.
    assert_eq!(unsafe { context.read::<f32>(&first)? }, [0.0; 4]);

    // Repeated round trips keep both timelines monotonic.
    let mut value = 1.0f32;
    for _ in 0..64 {
        context.write_buffer(first.wgpu(), 0, &f32_bytes(&[value; 4]))?;
        let copy = context.copy_to_shared(&first.view(&[4], ElementType::FLOAT_32)?)?;
        wgpu_copy(&context, &copy, &last)?;
        value += 1.0;
    }
    // SAFETY: each round writes every source element before copying it.
    let last_values = unsafe { context.read::<f32>(&last)? };
    assert_eq!(last_values, [value - 1.0; 4]);

    // Typed accessors reject mismatches instead of reinterpreting memory.
    let view = context.upload(&[1u32, 2, 3, 4], &[2, 2])?;
    assert!(context.upload(&[1u32, 2, 3], &[2, 2]).is_err());
    // SAFETY: upload initializes every element in the view.
    assert!(unsafe { context.read_view::<f32>(&view) }.is_err());
    // SAFETY: upload initializes every element in the view.
    assert_eq!(unsafe { context.read_view::<u32>(&view)? }, [1, 2, 3, 4]);

    // Buffers cannot be used with a different context.
    let foreign = pollster::block_on(Context::new(ContextOptions::default()))?;
    // SAFETY: the full buffer was initialized above; this context rejects it
    // before reading because the buffer belongs to a different context.
    assert!(unsafe { foreign.read::<f32>(&first) }.is_err());
    context.finish()?;
    Ok(())
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn concurrent_submissions_are_serialized() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    std::thread::scope(|scope| {
        let workers = (0..4u32)
            .map(|worker| {
                let context = context.clone();
                scope.spawn(move || -> Result<(), String> {
                    let run = || -> iree_wgpu::runtime::Result<()> {
                        let source = context.allocate_shared(64)?;
                        for round in 0..32u32 {
                            let value = (worker * 1000 + round) as f32;
                            context.write_buffer(source.wgpu(), 0, &f32_bytes(&[value; 16]))?;
                            let copy = context
                                .copy_to_shared(&source.view(&[16], ElementType::FLOAT_32)?)?;
                            // SAFETY: write_buffer initializes the source before the copy.
                            assert_eq!(unsafe { context.read::<f32>(&copy)? }, [value; 16]);
                        }
                        Ok(())
                    };
                    run().map_err(|error| error.to_string())
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker
                .join()
                .map_err(|_| "submission worker panicked".to_owned())??;
        }
        Ok::<_, String>(())
    })?;
    Ok(())
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn shared_handles_outlive_the_context() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    let shared = context.shared_buffer(&[1.0f32, 2.0, 3.0, 4.0])?;
    let view = shared.view(&[4], ElementType::FLOAT_32)?;
    let uploaded = context.upload(&[1.0f32; 4], &[4])?;
    let buffer = shared.wgpu().clone();
    let device = context.device().clone();
    let copy = context.copy_to_shared(&uploaded)?;
    // The context drains both APIs; the native device stays alive for handles.
    drop(context);

    // WGPU and IREE handles remain valid to use and release in any order.
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });
    drop(shared);
    drop(view);
    drop(group);
    drop(copy);
    drop(buffer);
    drop(uploaded);
    drop(device);
    Ok(())
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn contexts_are_torn_down_repeatedly() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    // Each context's VkDevice is destroyed after its last user, so this does
    // not accumulate devices; handles outliving the context delay teardown.
    for round in 0..8u32 {
        let context = pollster::block_on(Context::new(ContextOptions::default()))?;
        let shared = context.shared_buffer(&[round as f32; 4])?;
        let copy = context.copy_to_shared(&shared.view(&[4], ElementType::FLOAT_32)?)?;
        // SAFETY: shared_buffer initializes every element before the copy.
        let copied = unsafe { context.read::<f32>(&copy)? };
        assert_eq!(copied, [round as f32; 4]);
        drop(context);
        drop(shared);
        drop(copy);
    }
    Ok(())
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn reentrant_calls_fail_instead_of_deadlocking() -> TestResult {
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    };
    if isolated()? {
        return Ok(());
    }
    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    let source = context.shared_buffer(&[1.0f32; 4])?;
    let staging = context.device().create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = context.device().create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(source.wgpu(), 0, &staging, 0, 16);
    context.submit([encoder.finish()])?;

    let submitting = Arc::new(AtomicBool::new(false));
    let outcome = Arc::new(Mutex::new(None));
    {
        let context = context.clone();
        let submitting = submitting.clone();
        let outcome = outcome.clone();
        staging
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let during_submit = submitting.load(Ordering::SeqCst);
                let call = context.write_buffer(source.wgpu(), 0, &[0; 4]);
                if let Ok(mut outcome) = outcome.lock() {
                    *outcome = Some((during_submit, result.map(|()| call.map_err(|e| e.code()))));
                }
            });
    }
    // The copy completes on the GPU, then WGPU may run the map callback inside
    // the next submission, while the context is locked.
    context.finish()?;
    submitting.store(true, Ordering::SeqCst);
    context.submit(std::iter::empty::<wgpu::CommandBuffer>())?;
    submitting.store(false, Ordering::SeqCst);
    context.device().poll(wgpu::PollType::wait_indefinitely())?;
    let (during_submit, call) = outcome
        .lock()
        .map_err(|_| std::io::Error::other("map callback result lock was poisoned"))?
        .take()
        .ok_or_else(|| std::io::Error::other("map callback did not run"))?;
    let call = call?;
    if during_submit {
        assert_eq!(
            call,
            Err(iree_wgpu::runtime::Code::FailedPrecondition),
            "a re-entrant call is refused"
        );
    } else {
        assert_eq!(
            call,
            Ok(()),
            "a callback outside a submission may call the context"
        );
    }
    // The context remains usable either way.
    context.finish()?;
    Ok(())
}

#[test]
#[cfg(feature = "compiler")]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn rendered_edge_analysis_matches_cpu_oracle() -> TestResult {
    if isolated()? {
        return Ok(());
    }
    use edge_analysis::{EDGE_HEIGHT, EDGE_WIDTH, HEIGHT, WIDTH};
    use iree_wgpu::compiler::InputType;

    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    let program = context.compile(&edge_analysis::source(), InputType::StableHlo)?;
    let input = context.allocate_shared((HEIGHT * WIDTH * 16) as usize)?;
    let output = context.allocate_shared((EDGE_HEIGHT * EDGE_WIDTH * 4) as usize)?;
    let device = context.device();
    let extent = wgpu::Extent3d {
        width: WIDTH,
        height: HEIGHT,
        depth_or_array_layers: 1,
    };
    let image = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("edge example qualification"),
        size: extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("example render shader"),
        source: wgpu::ShaderSource::Wgsl(edge_analysis::RENDER.into()),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("example render oracle"),
        layout: None,
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
                format: wgpu::TextureFormat::Rgba32Float,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    });
    let uniform = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("static frame"),
        size: 16,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    context.write_buffer(&uniform, 0, &[0; 16])?;
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("static frame binding"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform.as_entire_binding(),
        }],
    });
    let view = image.create_view(&Default::default());
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
    }
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &image,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: input.wgpu(),
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(WIDTH * 16),
                rows_per_image: Some(HEIGHT),
            },
        },
        extent,
    );
    context.submit([encoder.finish()])?;

    let input_view = input.view(&[HEIGHT as _, WIDTH as _, 4], ElementType::FLOAT_32)?;
    let output_view = output.view(&[EDGE_HEIGHT as _, EDGE_WIDTH as _], ElementType::FLOAT_32)?;
    let results = program.call_views("module.main", &[&input_view, &output_view])?;
    assert_eq!(results.len(), 1);
    // Readbacks here are qualification oracles, not part of the example.
    // SAFETY: upload initializes `input`, and the program writes every output element.
    let pixels = unsafe { context.read::<f32>(&input)? };
    // SAFETY: the edge program writes every output element.
    let edges = unsafe { context.read::<f32>(&output)? };

    let gray = |y: u32, x: u32| pixels[((y * WIDTH + x) * 4) as usize];
    let mut nonzero = 0;
    for y in 0..EDGE_HEIGHT {
        for x in 0..EDGE_WIDTH {
            let gx = gray(y + 1, x + 2) - gray(y + 1, x);
            let gy = gray(y + 2, x + 1) - gray(y, x + 1);
            let expected = (gx * gx + gy * gy).sqrt();
            let actual = edges[(y * EDGE_WIDTH + x) as usize];
            assert!(
                (actual - expected).abs() < 1e-5,
                "edge ({x}, {y}): {actual} != {expected}"
            );
            nonzero += usize::from(actual > 0.05);
        }
    }
    assert!(nonzero > 100, "render must produce a nontrivial edge map");
    Ok(())
}
