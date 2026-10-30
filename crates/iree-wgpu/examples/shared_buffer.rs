//! WGPU compute → IREE destination-passing analysis → host readback.
//!
//! A WGPU compute shader writes a shared buffer, an IREE program reads it in
//! place and writes a second shared buffer, and the host reads the result.
//! No `unsafe`, leases or fences: the context orders every submission.
//!
//! ```text
//! cargo run -p iree-wgpu --features compiler --example shared_buffer
//! ```

use iree_wgpu::{Context, ContextOptions, compiler::InputType, runtime::hal::ElementType, wgpu};

const SOURCE: &str = r#"
module {
  func.func @main(%input: tensor<4xf32>,
      %destination: tensor<4xf32> {iree.abi.output = 0 : index}) -> tensor<4xf32> {
    %two = stablehlo.constant dense<2.0> : tensor<4xf32>
    %result = stablehlo.multiply %input, %two : tensor<4xf32>
    return %result : tensor<4xf32>
  }
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    println!(
        "Shared Vulkan adapter: {} ({:?})",
        context.adapter_name(),
        context.queue_topology()
    );
    let program = context.compile(SOURCE, InputType::StableHlo)?;
    let input = context.allocate_shared(16)?;
    let destination = context.allocate_shared(16)?;

    // WGPU writes the shared input on the GPU.
    let device = context.device();
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("generate shared array on the GPU"),
        source: wgpu::ShaderSource::Wgsl(
            r#"
@group(0) @binding(0) var<storage, read_write> values: array<f32>;
@compute @workgroup_size(4)
fn main(@builtin(local_invocation_index) index: u32) {
    values[index] = f32(index + 1u);
}
"#
            .into(),
        ),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("shared array producer"),
        layout: None,
        module: &shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("shared input"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input.binding(),
        }],
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    context.submit([encoder.finish()])?;

    // IREE observes the WGPU write and writes the destination in place.
    let results = program.call_views(
        "module.main",
        &[
            &input.view(&[4], ElementType::FLOAT_32)?,
            &destination.view(&[4], ElementType::FLOAT_32)?,
        ],
    )?;
    assert_eq!(results.len(), 1);

    // SAFETY: module.main writes every destination element.
    let values = unsafe { context.read::<f32>(&destination)? };
    assert_eq!(values, [2.0, 4.0, 6.0, 8.0]);
    println!("Shared result: {values:?}");
    Ok(())
}
