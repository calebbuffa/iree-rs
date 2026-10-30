//! Parallel model-comparison port of IREE's asynchronous sink callback sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir_for_backend};
use ireec::{Compiler, GlobalOptions};
use ireert::{
    hal::{BufferParams, BufferUsage, ElementType, EncodingType, MemoryAccess, MemoryType},
    io::{FileAccess, FileHandle, ParameterIndex},
    vm::{BytecodeModule, Context, DebugSink, Instance, List, Module, TypeDef},
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread::{self, ScopedJoinHandle},
};

const MODEL_A: &str = include_str!("../../extern/iree/samples/sink_callback/model_a.mlir");
const MODEL_B: &str = include_str!("../../extern/iree/samples/sink_callback/model_b.mlir");
const DRIVER: &str = include_str!("../../extern/iree/samples/sink_callback/driver_for_async.mlir");

#[derive(Clone, Debug)]
struct TensorStat {
    key: String,
    index: usize,
    mean: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    Compiler::set_global_options(&GlobalOptions::default().with_trace_dispatch_tensors())?;
    let flags = ["--iree-llvmcpu-target-cpu=host"];
    let target = iree_samples::vm_support::LocalBackend::LlvmCpu;
    let model_a = compile_mlir_for_backend("model_a.mlir", MODEL_A, target.clone(), &flags)?;
    let model_b = compile_mlir_for_backend("model_b.mlir", MODEL_B, target.clone(), &flags)?;
    let driver = compile_mlir_for_backend("driver_for_async.mlir", DRIVER, target, &flags)?;
    let weight_values = (0..18 * 18)
        .map(|index| u16::try_from(index).map(|value| f32::from(value) / (18.0 * 18.0)))
        .collect::<Result<Vec<_>, _>>()?;
    let weights = weight_values
        .into_iter()
        .flat_map(f32::to_ne_bytes)
        .collect::<Vec<_>>();
    let (stats_a, stats_b) = thread::scope(|scope| -> Result<_, Box<dyn Error>> {
        let driver_a = driver.clone();
        let weights_a = weights.clone();
        let worker_a = scope.spawn(move || run_variant(model_a, driver_a, weights_a));
        let worker_b = scope.spawn(move || run_variant(model_b, driver, weights));
        let stats_a = join_variant(worker_a);
        let stats_b = join_variant(worker_b);
        Ok((stats_a?, stats_b?))
    })?;
    assert_eq!(stats_a.len(), stats_b.len());
    let mut divergence_count = 0;
    for (a, b) in stats_a.iter().zip(&stats_b) {
        assert_eq!((a.key.as_str(), a.index), (b.key.as_str(), b.index));
        let difference = (a.mean - b.mean).abs();
        if a.key.contains("generic") && a.key.ends_with("outputs") && a.index == 0 {
            assert!((difference - (std::f32::consts::PI - std::f32::consts::E)).abs() < 1e-3);
            divergence_count += 1;
        } else {
            assert!(
                difference < 1e-6,
                "unexpected divergence at {}[{}]: {difference}",
                a.key,
                a.index
            );
        }
    }
    assert_eq!(divergence_count, 1);
    Ok(())
}

fn join_variant(
    handle: ScopedJoinHandle<'_, Result<Vec<TensorStat>, Box<dyn Error + Send + Sync>>>,
) -> Result<Vec<TensorStat>, Box<dyn Error>> {
    match handle.join() {
        Ok(result) => result.map_err(|error| std::io::Error::other(error.to_string()).into()),
        Err(_) => Err(std::io::Error::other("sink callback worker panicked").into()),
    }
}

fn run_variant(
    model: Vec<u8>,
    driver: Vec<u8>,
    weights: Vec<u8>,
) -> Result<Vec<TensorStat>, Box<dyn Error + Send + Sync>> {
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()
        .map_err(|error| std::io::Error::other(error.to_string()))?;
    let stats = Arc::new(Mutex::new(Vec::<TensorStat>::new()));
    let callback_stats = stats.clone();
    let sink = DebugSink::new(move |key, views| {
        let key = String::from_utf8_lossy(key).into_owned();
        let mut stats = callback_stats
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for (index, view) in views.iter().enumerate() {
            // SAFETY: the upstream traced model dispatch tensors are initialized f32 values.
            let values = unsafe { view.read_elements_unchecked::<f32>(ElementType::FLOAT_32)? };
            let count = u16::try_from(values.len()).map_err(|_| {
                ireert::Error::new(ireert::Code::OutOfRange, "trace tensor is too large")
            })?;
            let mean = values.iter().copied().sum::<f32>() / f32::from(count);
            stats.push(TensorStat {
                key: key.clone(),
                index,
                mean,
            });
        }
        Ok(())
    });
    let hal_module = Module::hal(
        &instance,
        Default::default(),
        sample_device.device.device_group(),
        Default::default(),
        sink,
    )?;
    let weight_length = u64::try_from(weights.len())?;
    let file = FileHandle::from_bytes(weights.into_boxed_slice(), FileAccess::READ)?;
    let mut index = ParameterIndex::new()?;
    index.add_file("my_weights", &[], &file, 0, weight_length)?;
    let provider = index.provider("my_scope", 1)?;
    let io_module = Module::io_parameters(&instance, &[provider])?;
    // SAFETY: both archives were compiled from the checked-in upstream MLIR above.
    let model = unsafe { BytecodeModule::new(&instance, Default::default(), model)? }.into_module();
    // SAFETY: both archives were compiled from the checked-in upstream MLIR above.
    let driver =
        unsafe { BytecodeModule::new(&instance, Default::default(), driver)? }.into_module();
    let context = Context::with_modules(
        &instance,
        Default::default(),
        &[hal_module, io_module, model, driver],
    )?;
    let function = context.resolve_function("driver.main")?;
    let input = vec![1.0f32; 4 * 18];
    let input_view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &sample_device.allocator(),
        &[4, 18],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        BufferParams {
            usage: BufferUsage::DEFAULT,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::DEVICE_LOCAL,
            ..Default::default()
        },
        &input,
    )?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.resize(1)?;
    inputs.set_buffer_view(0, &input_view)?;
    let mut outputs = List::new(TypeDef::Any, 0)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    Ok(stats
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
