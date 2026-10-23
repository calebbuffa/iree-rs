//! Native asynchronous device invocation with retained result storage.
#![cfg(all(feature = "compiler", feature = "cpu"))]

use iree::{CompileOptions, Compiler, ExecutionModel, Input, Instance, Target, Value};

const ADD: &str = r#"
module {
  func.func @main(%a: tensor<4xf32>, %b: tensor<4xf32>) -> tensor<4xf32> {
    %sum = stablehlo.add %a, %b : tensor<4xf32>
    return %sum : tensor<4xf32>
  }
}
"#;

fn run(target: Target, driver: &str) -> iree::Result<()> {
    let options = CompileOptions::new(target)
        .input(Input::StableHlo)
        .execution_model(ExecutionModel::AsyncExternal);
    let mut compiler = Compiler::new()?;
    let artifact = compiler.compile_with_options(ADD, &options)?;
    let device = Instance::new()?.device(driver)?;
    let mut session = device.load(&artifact)?;
    assert_eq!(
        session
            .function_attribute("module.main", "iree.abi.model")?
            .as_deref(),
        Some("coarse-fences")
    );
    let function = session.function("module.main")?;
    assert_eq!(function.name, "module.main");
    assert!(function.calling_convention.starts_with('0'));
    assert!(
        function
            .attributes
            .iter()
            .any(|attr| attr.key == "iree.abi.model" && attr.value == "coarse-fences")
    );
    let modules = session.modules()?;
    assert!(modules.iter().any(|module| module.name == "hal"));
    assert!(
        modules
            .iter()
            .any(|module| module.exports.contains(&function))
    );
    let inputs = [
        Value::from(device.tensor(&[4], &[1_f32, 2., 3., 4.])?),
        Value::from(device.tensor(&[4], &[10_f32, 20., 30., 40.])?),
    ];
    for _ in 0..20 {
        let outputs = pollster::block_on(session.call_async("module.main", &inputs)?)?;
        let [Value::Tensor(result)] = outputs.as_slice() else {
            panic!("expected tensor output");
        };
        assert_eq!(result.read::<f32>()?, [11., 22., 33., 44.]);
    }
    {
        use std::{
            future::Future,
            pin::Pin,
            task::{Context, Poll, Waker},
        };
        let mut invocation = session.call_async("module.main", &inputs)?;
        pollster::block_on(&mut invocation)?;
        assert!(invocation.completion().is_ready()?);
        let mut context = Context::from_waker(Waker::noop());
        assert!(matches!(
            Pin::new(&mut invocation).poll(&mut context),
            Poll::Ready(Err(error)) if error.code() == iree::StatusCode::INVALID_ARGUMENT
        ));
    }
    let foreign_device = Instance::new()?.device(driver)?;
    let foreign_fence = foreign_device.semaphore(0)?.at(1)?;
    assert!(
        session
            .call_async_after("module.main", &inputs, &foreign_fence)
            .is_err()
    );
    assert!(session.call_async("module.main", &inputs[..1]).is_err());
    session.drain()?;
    for _ in 0..20 {
        drop(session.call_async("module.main", &inputs)?);
    }
    session.drain()?;
    let destination_source = r#"
module {
  func.func @main(%a: tensor<4xf32>, %out: tensor<4xf32> {iree.abi.output = 0 : index}) -> tensor<4xf32> {
    %sum = stablehlo.add %a, %a : tensor<4xf32>
    return %sum : tensor<4xf32>
  }
}
"#;
    let destination_artifact = compiler.compile_with_options(destination_source, &options)?;
    let mut destination_session = device.load(&destination_artifact)?;
    let input = device.tensor(&[4], &[3_f32; 4])?;
    let output = device.tensor(&[4], &[0_f32; 4])?;
    let args = [Value::from(input), Value::from(output.clone())];
    let gate = device.semaphore(0)?;
    let fence = gate.at(1)?;
    let invocation = destination_session.call_async_after("module.main", &args, &fence)?;
    assert!(!invocation.completion().is_ready()?);
    drop(invocation);
    gate.signal(1)?;
    assert_eq!(output.read::<f32>()?, [6.; 4]);
    destination_session.drain()?;
    drop(session.call_async("module.main", &inputs)?);
    drop(inputs);
    drop(device);
    drop(artifact);
    drop(session);
    let artifact = compiler.compile(ADD, target)?;
    let mut session = Instance::new()?.device(driver)?.load(&artifact)?;
    assert!(session.call_async("module.main", &[]).is_err());
    Ok(())
}

#[test]
fn cpu_native_async_invocation() -> iree::Result<()> {
    run(Target::Cpu, "local-task")
}

#[cfg(feature = "vulkan")]
#[test]
#[ignore = "requires compatible Vulkan hardware"]
fn vulkan_native_async_invocation() -> iree::Result<()> {
    run(Target::Vulkan, "vulkan")
}
