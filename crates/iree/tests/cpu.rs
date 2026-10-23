//! Compiler and CPU runtime integration regressions.
#![cfg(all(feature = "compiler", feature = "cpu"))]

use iree::{Compiler, Instance, Target};

const ADD: &str = r#"
module {
  func.func @main(%a: tensor<4xf32>, %b: tensor<4xf32>) -> tensor<4xf32> {
    %sum = stablehlo.add %a, %b : tensor<4xf32>
    return %sum : tensor<4xf32>
  }
}
"#;

#[test]
fn stablehlo_executes_on_cpu_and_owns_its_artifact() -> iree::Result<()> {
    let artifact = Compiler::new()?.compile(ADD, Target::Cpu)?;
    assert!(!artifact.as_bytes().is_empty());
    let instance = Instance::new()?;
    let device = instance.device("local-task")?;
    let a = device.tensor(&[4], &[1.0f32, 2.0, 3.0, 4.0])?;
    let b = device.tensor(&[4], &[10.0f32, 20.0, 30.0, 40.0])?;
    let mut session = device.load(&artifact)?;
    drop(artifact);
    drop(instance);
    let outputs = session.call("module.main", &[a.clone(), b.clone()])?;
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].shape(), vec![4]);
    assert_eq!(outputs[0].read::<f32>()?, vec![11.0, 22.0, 33.0, 44.0]);
    assert!(outputs[0].read::<i32>().is_err());
    let again = session.call("module.main", &[a, b])?;
    assert_eq!(again[0].read::<f32>()?, vec![11.0, 22.0, 33.0, 44.0]);
    Ok(())
}

#[test]
fn invalid_source_reports_diagnostics_and_compiler_recovers() -> iree::Result<()> {
    let mut compiler = Compiler::new()?;
    let error = match compiler.compile("module { broken", Target::Cpu) {
        Ok(_) => panic!("invalid MLIR must fail"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("parsing"));
    assert!(!compiler.compile(ADD, Target::Cpu)?.as_bytes().is_empty());
    assert!(compiler.compile("nul\0source", Target::Cpu).is_err());
    assert!(
        compiler
            .compile_with_flags(ADD, &["--iree-not-a-real-option"])
            .is_err()
    );
    assert!(!compiler.compile(ADD, Target::Cpu)?.as_bytes().is_empty());
    Ok(())
}

#[cfg(feature = "vulkan")]
#[test]
fn compiler_targets_do_not_leak_between_calls() -> iree::Result<()> {
    let mut compiler = Compiler::new()?;
    let _vulkan = compiler.compile(ADD, Target::Vulkan)?;
    let cpu = compiler.compile(ADD, Target::Cpu)?;
    let device = Instance::new()?.device("local-sync")?;
    let mut session = device.load(&cpu)?;
    let inputs = [
        device.tensor(&[4], &[1.0f32; 4])?,
        device.tensor(&[4], &[2.0f32; 4])?,
    ];
    assert_eq!(
        session.call("module.main", &inputs)?[0].read::<f32>()?,
        [3.0; 4]
    );
    Ok(())
}

#[test]
fn rejects_bad_shape_missing_function_and_wrong_device() -> iree::Result<()> {
    let instance = Instance::new()?;
    let device = instance.device("local-sync")?;
    let artifact = Compiler::new()?.compile(ADD, Target::Cpu)?;
    let mut session = device.load(&artifact)?;
    assert!(session.call("module.missing", &[]).is_err());
    assert!(session.call("module.main", &[]).is_err());
    let wrong_shape = device.tensor(&[2], &[1.0f32, 2.0])?;
    assert!(
        session
            .call("module.main", &[wrong_shape.clone(), wrong_shape])
            .is_err()
    );
    let other = instance.device("local-sync")?;
    let foreign = other.tensor(&[4], &[1.0f32; 4])?;
    assert!(
        session
            .call("module.main", &[foreign.clone(), foreign])
            .is_err()
    );
    Ok(())
}

#[test]
fn matrix_multiplication_and_multiple_results() -> iree::Result<()> {
    let source = r#"
module {
  func.func @main(%a: tensor<2x3xf32>, %b: tensor<3x2xf32>)
      -> (tensor<2x2xf32>, tensor<2x3xf32>) {
    %out = stablehlo.dot_general %a, %b,
      contracting_dims = [1] x [0],
      precision = [DEFAULT, DEFAULT]
      : (tensor<2x3xf32>, tensor<3x2xf32>) -> tensor<2x2xf32>
    return %out, %a : tensor<2x2xf32>, tensor<2x3xf32>
  }
}"#;
    let artifact = Compiler::new()?.compile(source, Target::Cpu)?;
    let device = Instance::new()?.device("local-task")?;
    let mut session = device.load(&artifact)?;
    let a = device.tensor(&[2, 3], &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0])?;
    let b = device.tensor(&[3, 2], &[7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0])?;
    let result = session.call("module.main", &[a, b])?;
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].read::<f32>()?, vec![58.0, 64.0, 139.0, 154.0]);
    assert_eq!(result[1].read::<f32>()?, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    Ok(())
}

#[test]
fn dynamic_shapes_reuse_one_compiled_module() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
module {
  func.func @main(%a: tensor<?xf32>, %b: tensor<?xf32>) -> tensor<?xf32> {
    %sum = stablehlo.add %a, %b : tensor<?xf32>
    return %sum : tensor<?xf32>
  }
}"#;
    let mut compiler = Compiler::new()?;
    let artifact = compiler.compile(source, Target::Cpu)?;
    let device = Instance::new()?.device("local-task")?;
    let mut session = device.load(&artifact)?;
    for length in [1, 4, 17, 0] {
        let a = (0..length).map(|i| i as f32).collect::<Vec<_>>();
        let b = vec![0.5f32; length];
        let inputs = [device.tensor(&[length], &a)?, device.tensor(&[length], &b)?];
        let outputs = session.call("module.main", &inputs)?;
        assert_eq!(outputs[0].shape(), [length]);
        assert_eq!(
            outputs[0].read::<f32>()?,
            a.into_iter().map(|value| value + 0.5).collect::<Vec<_>>()
        );
    }
    Ok(())
}

#[test]
fn multiple_modules_share_one_device_but_have_isolated_state() -> iree::Result<()> {
    let artifact = Compiler::new()?.compile(ADD, Target::Cpu)?;
    let device = Instance::new()?.device("local-task")?;
    let inputs = [
        device.tensor(&[4], &[1.0f32; 4])?,
        device.tensor(&[4], &[4.0f32; 4])?,
    ];
    let mut first = device.load(&artifact)?;
    let mut second = device.load(&artifact)?;
    for _ in 0..3 {
        assert_eq!(
            first.call("module.main", &inputs)?[0].read::<f32>()?,
            [5.0; 4]
        );
        assert_eq!(
            second.call("module.main", &inputs)?[0].read::<f32>()?,
            [5.0; 4]
        );
    }
    drop(first);
    assert_eq!(
        second.call("module.main", &inputs)?[0].read::<f32>()?,
        [5.0; 4]
    );
    Ok(())
}

#[test]
fn compiler_handles_on_separate_threads_make_progress() {
    std::thread::scope(|scope| {
        let handles = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let artifact = Compiler::new()?.compile(ADD, Target::Cpu)?;
                    assert!(!artifact.as_bytes().is_empty());
                    iree::Result::Ok(())
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            handle.join().unwrap().unwrap();
        }
    });
}
