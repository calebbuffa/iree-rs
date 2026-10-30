//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir};
use ireec::{Compiler, GlobalOptions};
use ireert::hal::{BufferParams, BufferUsage, ElementType, EncodingType, MemoryAccess, MemoryType};
use ireert::vm::{
    BytecodeModule, Context, DebugSink, HalModuleDevicePolicy, Instance, List, Module, TypeDef,
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

const PROGRAM: &str = include_str!("../../extern/iree/samples/sink_callback/model.mlir");

fn main() -> Result<(), Box<dyn Error>> {
    Compiler::set_global_options(&GlobalOptions::default().with_trace_dispatch_tensors())?;
    let archive = compile_mlir("sink_callback.mlir", PROGRAM, &[])?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let results = Arc::new(Mutex::new(Vec::<(String, usize, f32)>::new()));
    let callback_results = results.clone();
    let sink = DebugSink::new(move |key, views| {
        let key = String::from_utf8_lossy(key).into_owned();
        let mut results = callback_results
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for (index, view) in views.iter().enumerate() {
            // Trace callbacks run after their corresponding dispatch has produced these values.
            // SAFETY: the traced model dispatch tensors are initialized f32 values.
            let values = unsafe { view.read_elements_unchecked::<f32>(ElementType::FLOAT_32)? };
            results.push((key.clone(), index, values.iter().sum()));
        }
        Ok(())
    });
    let hal_module = Module::hal(
        &instance,
        HalModuleDevicePolicy::default(),
        sample_device.device.device_group(),
        Default::default(),
        sink,
    )?;
    // SAFETY: `archive` was just produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let function = context.resolve_function("module.main")?;
    let allocator = sample_device.allocator();
    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };
    let a = vec![0.25f32; 4 * 16];
    let b = vec![1.0f32; 16 * 16];
    let a_view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &[4, 16],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &a,
    )?;
    let b_view = ireert::hal::BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        &[16, 16],
        ElementType::FLOAT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        &b,
    )?;
    let mut inputs = List::new(TypeDef::Any, 2)?;
    inputs.resize(2)?;
    inputs.set_buffer_view(0, &a_view)?;
    inputs.set_buffer_view(1, &b_view)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    let output = outputs
        .get_buffer_view(0)
        .ok_or_else(|| ireert::Error::new(ireert::Code::DataLoss, "model returned no tensor"))?;
    // SAFETY: the compiled model returns an initialized f32 tensor.
    let values = unsafe { output.read_elements_unchecked::<f32>(ElementType::FLOAT_32)? };
    assert!(
        values
            .iter()
            .all(|value| (*value - 1.0 / 16.0).abs() < 1e-6)
    );
    let results = results
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert_eq!(results.len(), 5);
    assert!(results[0].0.starts_with("main_dispatch_0"));
    assert!(results[0].0.ends_with("inputs"));
    assert_eq!(results[0].1, 0);
    assert!((results[0].2 - 16.0).abs() < 1e-4);
    assert!(results[1].0.starts_with("main_dispatch_0"));
    assert!(results[1].0.ends_with("inputs"));
    assert_eq!(results[1].1, 1);
    assert!((results[1].2 - 256.0).abs() < 1e-4);
    assert!(results[2].0.starts_with("main_dispatch_0"));
    assert!(results[2].0.ends_with("outputs"));
    assert!((results[2].2 - 256.0).abs() < 1e-3);
    assert!(results[3].0.starts_with("main_dispatch_1"));
    assert!(results[3].0.ends_with("inputs"));
    assert!((results[3].2 - results[2].2).abs() < 1e-4);
    assert!(results[4].0.starts_with("main_dispatch_1"));
    assert!(results[4].0.ends_with("outputs"));
    assert!((results[4].2 - 4.0).abs() < 1e-4);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
