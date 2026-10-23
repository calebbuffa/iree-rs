//! Programmatic worker topology and configured native CPU devices.
#![cfg(feature = "task")]

use iree::task::{CoreDistribution, CorePreference, ExecutorOptions, Topology};

#[test]
fn programmatic_executor_options_and_topology() -> iree::Result<()> {
    assert!(Topology::workers(0).is_err());
    assert!(Topology::workers(257).is_err());
    assert!(Topology::logical_cpus(&[]).is_err());
    assert!(Topology::logical_cpus(&[0, 0]).is_err());
    assert!(!Topology::numa_nodes().is_empty());
    let topology = Topology::workers(2)?;
    assert!(
        ExecutorOptions::default()
            .worker_base_index(u32::MAX as usize)
            .build(&topology)
            .is_err()
    );
    assert!(
        ExecutorOptions::default()
            .stack_size(usize::MAX)
            .build(&topology)
            .is_err()
    );
    assert!(
        ExecutorOptions::default()
            .local_memory_size(usize::MAX)
            .build(&topology)
            .is_err()
    );
    assert!(
        ExecutorOptions::default()
            .local_memory_size((isize::MAX as usize) / 2 + 1)
            .build(&topology)
            .is_err()
    );
    let executor = ExecutorOptions::default()
        .local_memory_size(64 * 1024)
        .build(&topology)?;
    assert_eq!(executor.worker_count(), 2);
    let clone = executor.clone();
    drop(executor);
    clone.trim();
    let topology =
        Topology::physical_cores(None, CorePreference::Any, CoreDistribution::Compact, 2)?;
    assert!((1..=2).contains(&topology.worker_count()));
    Ok(())
}

#[cfg(all(feature = "compiler", feature = "local-task"))]
#[test]
fn configured_worker_pool_runs_compiled_code() -> iree::Result<()> {
    use iree::{Compiler, Instance, Target, task::TaskDeviceOptions};
    let executor = ExecutorOptions::default().build(&Topology::workers(2)?)?;
    let device = TaskDeviceOptions::default()
        .build(&Instance::new()?, &[executor.clone(), executor.clone()])?;
    drop(executor);
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
    let input = device.tensor(&[4], &[2_f32; 4])?;
    let mut session = device.load(&artifact)?;
    assert_eq!(
        session.call("module.main", &[input])?[0].read::<f32>()?,
        [4.; 4]
    );
    Ok(())
}
