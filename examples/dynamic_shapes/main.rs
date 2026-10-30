//! Rust port of the corresponding IREE upstream sample.

use iree_samples::vm_support::{SampleDevice, compile_mlir_for_backend};
use ireert::{
    hal::{
        BufferParams, BufferUsage, BufferView, ElementType, EncodingType, MemoryAccess, MemoryType,
    },
    vm::{BytecodeModule, Context, Instance, List, Module, TypeDef},
};
use std::error::Error;

// The upstream notebooks generate this module; the three exports mirror their documented signatures.
const PROGRAM: &str = r#"
func.func @reduce_sum_1d(%values: tensor<?xi32>) -> tensor<i32> {
  %zero = arith.constant 0 : i32
  %c0 = arith.constant 0 : index
  %c1 = arith.constant 1 : index
  %length = tensor.dim %values, %c0 : tensor<?xi32>
  %sum = scf.for %i = %c0 to %length step %c1 iter_args(%acc = %zero) -> (i32) {
    %value = tensor.extract %values[%i] : tensor<?xi32>
    %next = arith.addi %acc, %value : i32
    scf.yield %next : i32
  }
  %result = tensor.from_elements %sum : tensor<i32>
  return %result : tensor<i32>
}
func.func @reduce_sum_2d(%values: tensor<?x3xi32>) -> tensor<3xi32> {
  %zero = arith.constant 0 : i32
  %c0 = arith.constant 0 : index
  %c1 = arith.constant 1 : index
  %c3 = arith.constant 3 : index
  %rows = tensor.dim %values, %c0 : tensor<?x3xi32>
  %empty = tensor.empty() : tensor<3xi32>
  %result = scf.for %column = %c0 to %c3 step %c1 iter_args(%out = %empty) -> (tensor<3xi32>) {
    %sum = scf.for %row = %c0 to %rows step %c1 iter_args(%acc = %zero) -> (i32) {
      %value = tensor.extract %values[%row, %column] : tensor<?x3xi32>
      %next = arith.addi %acc, %value : i32
      scf.yield %next : i32
    }
    %updated = tensor.insert %sum into %out[%column] : tensor<3xi32>
    scf.yield %updated : tensor<3xi32>
  }
  return %result : tensor<3xi32>
}
func.func @add_one(%values: tensor<?xi32>) -> tensor<?xi32> {
  %one = arith.constant 1 : i32
  %c0 = arith.constant 0 : index
  %c1 = arith.constant 1 : index
  %length = tensor.dim %values, %c0 : tensor<?xi32>
  %empty = tensor.empty(%length) : tensor<?xi32>
  %result = scf.for %i = %c0 to %length step %c1 iter_args(%out = %empty) -> (tensor<?xi32>) {
    %value = tensor.extract %values[%i] : tensor<?xi32>
    %added = arith.addi %value, %one : i32
    %updated = tensor.insert %added into %out[%i] : tensor<?xi32>
    scf.yield %updated : tensor<?xi32>
  }
  return %result : tensor<?xi32>
}
"#;

fn main() -> Result<(), Box<dyn Error>> {
    let archive = compile_mlir_for_backend(
        "dynamic_shapes.mlir",
        PROGRAM,
        iree_samples::vm_support::LocalBackend::LlvmCpu,
        &[],
    )?;
    let instance = Instance::new()?;
    let sample_device = SampleDevice::from_environment()?;
    let hal_module = sample_device.hal_module(&instance)?;
    // SAFETY: `archive` was just produced by the compiler in this process.
    let bytecode = unsafe { BytecodeModule::new(&instance, Default::default(), archive)? };
    let module: Module = bytecode.into_module();
    let context = Context::with_modules(&instance, Default::default(), &[hal_module, module])?;
    let params = BufferParams {
        usage: BufferUsage::DEFAULT,
        access: MemoryAccess::ALL,
        memory_type: MemoryType::DEVICE_LOCAL,
        ..Default::default()
    };

    let sum_1d = invoke(
        &context,
        "module.reduce_sum_1d",
        &[1, 10, 100],
        &[3],
        &sample_device,
        params,
    )?;
    assert_eq!(sum_1d.shape()?, Vec::<ireert::hal::Dim>::new());
    // SAFETY: the reduction result is a scalar i32 tensor.
    let sum_1d_value = unsafe { sum_1d.read_elements_unchecked::<i32>(ElementType::INT_32)? };
    assert_eq!(sum_1d_value, [111]);

    for (input, expected) in [
        (vec![1, 2, 3, 10, 20, 30], vec![11, 22, 33]),
        (
            vec![1, 2, 3, 10, 20, 30, 100, 200, 300],
            vec![111, 222, 333],
        ),
    ] {
        let output = invoke(
            &context,
            "module.reduce_sum_2d",
            &input,
            &[ireert::hal::Dim::try_from(input.len() / 3)?, 3],
            &sample_device,
            params,
        )?;
        assert_eq!(output.shape()?, [3]);
        // SAFETY: the reduction result is an initialized 3-element i32 tensor.
        let actual = unsafe { output.read_elements_unchecked::<i32>(ElementType::INT_32)? };
        assert_eq!(actual, expected);
    }

    let input = [1, 10, 100];
    let output = invoke(
        &context,
        "module.add_one",
        &input,
        &[3],
        &sample_device,
        params,
    )?;
    // SAFETY: the compiled `add_one` export returns an initialized i32 tensor.
    let actual = unsafe { output.read_elements_unchecked::<i32>(ElementType::INT_32)? };
    assert_eq!(actual, [2, 11, 101]);
    Ok(())
}

fn invoke(
    context: &Context,
    function_name: &str,
    values: &[i32],
    shape: &[ireert::hal::Dim],
    sample_device: &SampleDevice,
    params: BufferParams,
) -> Result<BufferView, Box<dyn Error>> {
    let allocator = sample_device.allocator();
    let view = BufferView::allocate_buffer_copy_typed(
        &sample_device.device,
        &allocator,
        shape,
        ElementType::INT_32,
        EncodingType::DENSE_ROW_MAJOR,
        params,
        values,
    )?;
    let mut inputs = List::new(TypeDef::Any, 1)?;
    inputs.resize(1)?;
    inputs.set_buffer_view(0, &view)?;
    let mut outputs = List::new(TypeDef::Any, 1)?;
    let function = context.resolve_function(function_name)?;
    context.invoke(&function, Default::default(), &inputs, &mut outputs)?;
    outputs
        .get_buffer_view(0)
        .ok_or_else(|| ireert::Error::new(ireert::Code::DataLoss, "missing tensor result").into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
