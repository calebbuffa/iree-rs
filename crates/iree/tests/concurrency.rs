//! Shared native devices with independent per-thread Rust/session ownership.
#![cfg(all(feature = "cpu", feature = "task"))]

use iree::{Instance, SharedDevice};

#[test]
fn one_device_services_independent_worker_threads() -> iree::Result<()> {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Instance>();
    assert_send_sync::<SharedDevice>();
    let instance = Instance::new()?;
    let device = instance.shared_device("local-task")?;
    drop(instance);
    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let device = device.clone();
            std::thread::spawn(move || -> iree::Result<()> {
                let local = device.local();
                for _ in 0..20 {
                    let tensor = local.tensor(&[4], &[worker; 4])?;
                    assert_eq!(tensor.read::<i32>()?, [worker; 4]);
                }
                Ok(())
            })
        })
        .collect();
    drop(device);
    for worker in workers {
        worker.join().unwrap()?;
    }
    Ok(())
}

#[cfg(feature = "compiler")]
#[test]
fn shared_hardware_isolated_vm_contexts() -> iree::Result<()> {
    use iree::{Compiler, Target};
    let artifact = Compiler::new()?.compile(
        r#"
module {
  func.func @main(%a: tensor<4xf32>) -> tensor<4xf32> {
    %sum = stablehlo.add %a, %a : tensor<4xf32>
    return %sum : tensor<4xf32>
  }
}
"#,
        Target::Cpu,
    )?;
    let device = Instance::new()?.shared_device("local-task")?;
    let workers: Vec<_> = (1..5)
        .map(|worker| {
            let device = device.clone();
            let artifact = artifact.clone();
            std::thread::spawn(move || -> iree::Result<()> {
                let local = device.local();
                let input = local.tensor(&[4], &[worker as f32; 4])?;
                let mut session = local.load(&artifact)?;
                drop(device);
                drop(artifact);
                for _ in 0..20 {
                    let outputs = session.call("module.main", std::slice::from_ref(&input))?;
                    assert_eq!(outputs[0].read::<f32>()?, [2.0 * worker as f32; 4]);
                }
                Ok(())
            })
        })
        .collect();
    drop(device);
    drop(artifact);
    for worker in workers {
        worker.join().unwrap()?;
    }
    Ok(())
}
