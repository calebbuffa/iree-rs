//! Rust port of IREE's MNIST vision-inference sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir_for_backend};
use ireert::{
    hal::{
        BufferParams, BufferUsage, BufferView, ElementType, EncodingType, MemoryAccess, MemoryType,
    },
    vm::{BytecodeModule, Context, Instance, List, Module, TypeDef},
};
use std::error::Error;

const MODEL: &str = include_str!("../../extern/iree/samples/models/mnist.mlir");
const IMAGE: &[u8] = include_bytes!("../../extern/iree/samples/vision_inference/mnist_test.png");

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir_for_backend(
        "mnist.mlir",
        MODEL,
        iree_samples::vm_support::LocalBackend::LlvmCpu,
        &["--iree-input-type=stablehlo"],
    )?;
    let pixels = preprocess_image(IMAGE)?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    // SAFETY: this archive was produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let function = context.resolve_function("module.predict")?;

    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };
    let allocator = sample_device.allocator();
    let input = BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &[1, 28, 28, 1],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &pixels,
    )?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.resize(1)?;
    inputs.set_buffer_view(0, &input)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let output = outputs
        .get_buffer_view(0)
        .ok_or("MNIST model returned no prediction tensor")?;
    let output_buffer = output.buffer();
    let mut scores = [0.0f32; 10];
    // SAFETY: module.predict initializes every output score before returning.
    unsafe {
        sample_device
            .device
            .transfer_d2h_typed(&output_buffer, 0, &mut scores, None)?
    };

    assert!(scores.iter().all(|score| score.is_finite()));
    assert!(scores.iter().all(|score| (0.0..=1.0).contains(score)));
    let confidence_sum: f32 = scores.iter().sum();
    assert!((confidence_sum - 1.0).abs() < 1e-4);
    println!("MNIST prediction: {}", argmax(&scores));
    Ok(())
}

fn preprocess_image(bytes: &[u8]) -> Result<Vec<f32>, Box<dyn Error>> {
    let image = image::load_from_memory(bytes)?
        .resize_exact(28, 28, image::imageops::FilterType::CatmullRom)
        .to_luma8();
    Ok(image
        .into_raw()
        .into_iter()
        .map(|pixel| f32::from(pixel) / 255.0)
        .collect())
}

fn argmax(values: &[f32]) -> usize {
    values
        .iter()
        .enumerate()
        .max_by(|(_, left), (_, right)| left.total_cmp(right))
        .map_or(0, |(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::{IMAGE, argmax, preprocess_image};

    #[test]
    fn image_preprocessing_produces_normalized_mnist_tensor()
    -> Result<(), Box<dyn std::error::Error>> {
        let pixels = preprocess_image(IMAGE)?;
        assert_eq!(pixels.len(), 28 * 28);
        assert!(pixels.iter().all(|pixel| (0.0..=1.0).contains(pixel)));
        assert!(pixels.iter().any(|pixel| *pixel > 0.0));
        Ok(())
    }

    #[test]
    fn argmax_returns_the_highest_confidence_index() {
        assert_eq!(argmax(&[0.1, 0.7, 0.2]), 1);
    }

    #[test]
    fn runs_upstream_mnist_inference() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
