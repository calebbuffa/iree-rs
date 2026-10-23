//! Explicit Vulkan hardware qualification for WGPU interoperability.
#![cfg(feature = "compiler")]

use iree::{Compiler, Instance, Target};

/// This is intentionally an explicit hardware qualification, not a silently
/// skipped test on machines without Vulkan. Run with `--ignored --nocapture`.
#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn render_iree_render_without_intermediate_readback() -> Result<(), Box<dyn std::error::Error>> {
    render_iree_render(false)
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn render_iree_render_with_destination_passing() -> Result<(), Box<dyn std::error::Error>> {
    render_iree_render(true)
}

fn render_iree_render(destination_passing: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
module {
  func.func @main(%input: tensor<256xf32>) -> tensor<256xf32> {
    %two = stablehlo.constant dense<2.0> : tensor<256xf32>
    %output = stablehlo.multiply %input, %two : tensor<256xf32>
    return %output : tensor<256xf32>
  }
}
"#;
    let destination_source = r#"
module {
  func.func @main(%input: tensor<256xf32>,
      %destination: tensor<256xf32> {iree.abi.output = 0 : index}) -> tensor<256xf32> {
    %two = stablehlo.constant dense<2.0> : tensor<256xf32>
    %output = stablehlo.multiply %input, %two : tensor<256xf32>
    return %output : tensor<256xf32>
  }
}
"#;
    let source = if destination_passing {
        destination_source
    } else {
        source
    };
    let artifact = Compiler::new()?.compile(source, Target::Vulkan)?;
    let instance = Instance::new()?;
    let context = pollster::block_on(iree_gfx::Context::new(&instance))?;
    println!("Shared Vulkan adapter: {}", context.adapter_name());
    let mut session = context.iree_device().load(&artifact)?;
    let input = context.tensor(&[256], &[0.0f32; 256])?;
    let destination = context.tensor(&[256], &[0.0f32; 256])?;
    // Independent VM contexts must share the Vulkan device's single topology.
    let mut second_session = context.iree_device().load(&artifact)?;

    // SAFETY: this test is single-threaded; IREE calls and WGPU submissions are
    // strictly separated by the bridge's blocking memory-barrier handoff.
    let (device, queue) = unsafe { context.graphics() };
    unsafe { context.handoff()? };
    let shared_input = unsafe { context.export_tensor(&input)? };
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
                buffer: &shared_input.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: shared_input.offset,
                    bytes_per_row: Some(1024),
                    rows_per_image: Some(1),
                },
            },
            extent,
        );
        queue.submit([encoder.finish()]);
        unsafe { context.handoff()? };

        // No tensor upload/readback here: IREE consumes the exact allocation
        // that WGPU's texture copy wrote.
        let inputs = if destination_passing {
            vec![input.clone(), destination.clone()]
        } else {
            vec![input.clone()]
        };
        let outputs = session.call("module.main", &inputs)?;
        let output = if destination_passing {
            outputs[0].clone()
        } else {
            context.copy_to_shared(&outputs[0])?
        };
        unsafe { context.handoff()? };
        let shared_output = unsafe { context.export_tensor(&output)? };
        if destination_passing {
            let shared_destination = unsafe { context.export_tensor(&destination)? };
            let raw = {
                let hal =
                    unsafe { shared_output.buffer.as_hal::<wgpu::hal::api::Vulkan>() }.unwrap();
                unsafe { hal.raw_handle() }
            };
            let destination_raw = {
                let hal = unsafe { shared_destination.buffer.as_hal::<wgpu::hal::api::Vulkan>() }
                    .unwrap();
                unsafe { hal.raw_handle() }
            };
            assert_eq!(raw, destination_raw);
        }
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared IREE output"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(shared_output.binding()),
            }],
        });
        // The exported WGPU lease must keep native storage alive independently.
        drop(outputs);
        drop(output);
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
        queue.submit([encoder.finish()]);
        unsafe { context.handoff()? };
        let (sender, receiver) = std::sync::mpsc::channel();
        staging
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                sender.send(result).unwrap();
            });
        device.poll(wgpu::PollType::wait_indefinitely())?;
        receiver.recv()??;
        {
            let mapped = staging.slice(..).get_mapped_range()?;
            let expected = [
                color.r as f32 * 2.0,
                color.g as f32 * 2.0,
                color.b as f32 * 2.0,
                color.a as f32 * 2.0,
            ];
            for pixel in mapped.chunks_exact(16) {
                for (component, expected) in pixel.chunks_exact(4).zip(expected) {
                    let actual = f32::from_ne_bytes(component.try_into()?);
                    assert_eq!(actual, expected);
                }
            }
        }
        staging.unmap();
        std::mem::swap(&mut session, &mut second_session);
        println!("render -> shared IREE input -> multiply -> shared output -> render: {color:?}");
    }
    Ok(())
}

#[test]
#[ignore = "requires a Vulkan 1.3 graphics/compute device"]
fn exported_buffer_outlives_the_iree_context() -> Result<(), Box<dyn std::error::Error>> {
    let instance = Instance::new()?;
    let context = pollster::block_on(iree_gfx::Context::new(&instance))?;
    assert!(context.tensor::<f32>(&[0], &[]).is_err());
    assert!(context.tensor(&[3], &[0u8; 3]).is_err());
    assert!(context.tensor(&[2], &[0.0f32]).is_err());
    let tensor = context.tensor(&[4], &[1.0f32, 2.0, 3.0, 4.0])?;
    let (device, queue) = unsafe { context.graphics() };
    let (device, queue) = (device.clone(), queue.clone());
    unsafe { context.handoff()? };
    let shared = unsafe { context.export_tensor(&tensor)? };
    drop(tensor);
    drop(context);
    drop(instance);

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("export lifetime readback"),
        size: 16,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(&shared.buffer, 0, &staging, 0, 16);
    queue.submit([encoder.finish()]);
    let (sender, receiver) = std::sync::mpsc::channel();
    staging
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).unwrap();
        });
    device.poll(wgpu::PollType::wait_indefinitely())?;
    receiver.recv()??;
    {
        let mapped = staging.slice(..).get_mapped_range()?;
        let values = mapped
            .chunks_exact(4)
            .map(|bytes| f32::from_ne_bytes(bytes.try_into().unwrap()))
            .collect::<Vec<_>>();
        assert_eq!(values, [1.0, 2.0, 3.0, 4.0]);
    }
    staging.unmap();
    Ok(())
}
