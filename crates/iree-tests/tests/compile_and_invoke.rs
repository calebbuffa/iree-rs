//! Compile and invoke a module through the independent compiler and runtime APIs.

use ireec::{Compiler, Device, Options, Output, Pipeline, Target};
use ireert::vm::{
    BytecodeModule, BytecodeModuleFlags, Context, ContextFlags, Instance, InvocationFlags, List,
    TypeDef, Value,
};

#[test]
fn compiled_module_reflects_and_invokes() {
    // Scalar-only, so the module imports no HAL and needs no device.
    let source = br#"
      func.func @add(%a: i32, %b: i32) -> i32 {
        %0 = arith.addi %a, %b : i32
        return %0 : i32
      }
    "#;
    let compiler = Compiler::new().unwrap();
    let mut session = compiler.session().unwrap();
    session
        .configure(
            &Options::default()
                .with_device(Device::Alias("local".into()))
                .with_target(Target::Vmvx),
        )
        .unwrap();
    let mut invocation = session.invocation().unwrap();
    invocation
        .parse_source(session.source_buffer("input.mlir", source).unwrap())
        .unwrap();
    invocation.pipeline(Pipeline::Std).unwrap();
    let mut output = Output::memory(&compiler).unwrap();
    invocation.output_vm_bytecode(&mut output).unwrap();
    let artifact = output.bytes().unwrap().to_vec();

    let instance = Instance::new().unwrap();
    // SAFETY: the archive was just compiled by this test.
    let bytecode = unsafe {
        BytecodeModule::new(&instance, BytecodeModuleFlags::default(), artifact).unwrap()
    };
    let context = Context::with_modules(
        &instance,
        ContextFlags::default(),
        &[bytecode.module().clone()],
    )
    .unwrap();
    let function = context.resolve_function("module.add").unwrap();
    assert_eq!(function.name(), "add");
    assert_eq!(function.argument_result_counts().unwrap(), (2, 1));

    let mut inputs = List::new(TypeDef::Any, 2).unwrap();
    inputs.push(Value::I32(40)).unwrap();
    inputs.push(Value::I32(2)).unwrap();
    let mut outputs = List::new(TypeDef::Any, 1).unwrap();
    context
        .invoke(&function, InvocationFlags::default(), &inputs, &mut outputs)
        .unwrap();
    assert_eq!(outputs.get_value(0).unwrap(), Value::I32(42));
}
