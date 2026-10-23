//! Ordered transfer commands, checked slices, and initialized buffer reads.
#![cfg(feature = "cpu")]

use iree::{Instance, Result};

fn run(driver: &str) -> Result<()> {
    let device = Instance::new()?.device(driver)?;
    let scalar = device
        .tensor(&[], &[42_i32])?
        .as_buffer()?
        .as_tensor::<i32>(&[])?;
    assert_eq!(scalar.read::<i32>()?, [42]);
    let empty = device.buffer(&[])?.as_tensor::<u8>(&[0])?;
    assert!(empty.read::<u8>()?.is_empty());
    let a = device.buffer(&[0; 64])?;
    let b = device.buffer(&[0; 64])?;
    let mut commands = device.transfer_commands()?;
    commands.fill(&a, &[1, 2, 3, 4])?;
    commands.copy(&a, &b)?;
    commands.update(&b.slice(4..8)?, &[9; 4])?;
    pollster::block_on(commands.submit()?)?;
    let bytes = b.read()?;
    assert_eq!(&bytes[..4], &[1, 2, 3, 4]);
    assert_eq!(&bytes[4..8], &[9; 4]);
    assert_eq!(&bytes[8..12], &[1, 2, 3, 4]);
    let mut first = device.transfer_commands()?;
    first.fill(&a, &[7])?;
    drop(first.submit()?);
    let mut second = device.transfer_commands()?;
    second.copy(&a, &b)?;
    drop(second.submit()?);
    assert_eq!(b.read()?, [7; 64]);
    b.slice(16..20)?.write(0, &[8; 4])?;
    assert_eq!(b.slice(16..20)?.read()?, [8; 4]);
    assert!(b.slice(60..65).is_err());
    assert!(b.write(63, &[0; 2]).is_err());
    let mut bad = device.transfer_commands()?;
    assert!(bad.copy(&a.slice(0..8)?, &a.slice(4..12)?).is_err());
    assert!(bad.fill(&a.slice(1..5)?, &[0; 4]).is_err());
    assert!(bad.copy(&a.slice(0..4)?, &b).is_err());
    let other = Instance::new()?.device(driver)?.buffer(&[0; 64])?;
    assert!(bad.copy(&a, &other).is_err());
    let tensor = a.as_tensor::<u8>(&[8, 8])?;
    let alias = tensor.as_buffer()?;
    assert!(bad.copy(&a.slice(0..8)?, &alias.slice(4..12)?).is_err());
    assert!(a.as_tensor::<f32>(&[15]).is_err());
    assert!(a.as_tensor::<f32>(&[usize::MAX, 2]).is_err());
    assert!(a.slice(1..5)?.as_tensor::<i32>(&[1]).is_err());
    let sub_tensor = a.slice(8..24)?.as_tensor::<u32>(&[4])?;
    let sub_bytes = sub_tensor.as_buffer()?;
    assert_eq!(sub_bytes.len(), 16);
    sub_bytes.write(0, &[3; 4])?;
    assert_eq!(tensor.read::<u8>()?[8..12], [3; 4]);
    let mut transfers = device.transfer_commands()?;
    transfers.copy(&a.slice(0..4)?, &sub_bytes.slice(4..8)?)?;
    drop(transfers.submit()?);
    assert_eq!(sub_tensor.read::<u32>()?[1], u32::from_ne_bytes([7; 4]));
    assert_eq!(alias.read()?[12..16], [7; 4]);
    drop(tensor);
    drop(sub_tensor);
    drop(a);
    drop(device);
    assert_eq!(sub_bytes.read()?[..4], [3; 4]);
    Ok(())
}

#[test]
fn native_cpu_transfer_commands() -> Result<()> {
    run("local-task")
}

#[cfg(feature = "vulkan")]
#[test]
#[ignore = "requires compatible Vulkan hardware"]
fn native_vulkan_transfer_commands() -> Result<()> {
    run("vulkan")
}

#[cfg(feature = "compiler")]
fn run_compute(target: iree::Target, driver: &str) -> Result<()> {
    use iree::{CompileOptions, Compiler, ExecutionModel, Input, Value};
    let artifact = Compiler::new()?.compile_with_options(
        r#"
module {
  func.func @main(%a: tensor<4xf32>, %out: tensor<4xf32> {iree.abi.output = 0 : index}) -> tensor<4xf32> {
    %sum = stablehlo.add %a, %a : tensor<4xf32>
    return %sum : tensor<4xf32>
  }
}
"#,
        &CompileOptions::new(target)
            .input(Input::StableHlo)
            .execution_model(ExecutionModel::AsyncExternal),
    )?;
    let device = Instance::new()?.device(driver)?;
    let source = device.buffer(&[0; 16])?;
    let destination = device.buffer(&[0; 16])?;
    let observed = device.buffer(&[0; 16])?;
    let args = [
        Value::from(source.as_tensor::<f32>(&[4])?),
        Value::from(destination.as_tensor::<f32>(&[4])?),
    ];
    let mut session = device.load(&artifact)?;
    let mut upload = device.transfer_commands()?;
    upload.fill(&source, &2_f32.to_le_bytes())?;
    drop(upload.submit()?);
    let gate = device.semaphore(0)?;
    drop(session.call_async_after("module.main", &args, &gate.at(1)?)?);
    let mut download = device.transfer_commands()?;
    download.copy(&destination, &observed)?;
    let submission = download.submit()?;
    assert!(!submission.completion().is_ready()?);
    gate.signal(1)?;
    drop(submission);
    assert_eq!(observed.as_tensor::<f32>(&[4])?.read::<f32>()?, [4.; 4]);
    assert_eq!(
        session
            .modules()?
            .iter()
            .filter(|module| module.name == "module")
            .count(),
        1
    );
    session.drain()?;
    Ok(())
}

#[cfg(feature = "compiler")]
#[test]
fn native_cpu_transfer_compute_dependencies() -> Result<()> {
    run_compute(iree::Target::Cpu, "local-task")
}

#[cfg(all(feature = "compiler", feature = "vulkan"))]
#[test]
#[ignore = "requires compatible Vulkan hardware"]
fn native_vulkan_transfer_compute_dependencies() -> Result<()> {
    run_compute(iree::Target::Vulkan, "vulkan")
}
