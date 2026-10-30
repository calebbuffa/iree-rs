//! Public task-topology and executor tests corresponding to `topology_test.cc`.
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::task::{CoreDistribution, CorePerformance, Executor, ExecutorOptions, Topology};

#[test]
fn group_count_and_logical_cpu_topologies() {
    let empty = Topology::from_group_count(0);
    assert!(empty.group_count() <= 1);
    let one = Topology::from_group_count(1);
    assert_eq!(one.group_count(), 1);
    let four = Topology::from_group_count(4);
    assert_eq!(four.group_count(), 4);
    let explicit = Topology::from_logical_cpus(&[0, 1]).unwrap();
    assert_eq!(explicit.group_count(), 2);
    assert!(Topology::from_logical_cpus(&[u32::MAX]).is_err());
}

#[test]
fn physical_core_topology_and_executor_construct() {
    let topology =
        Topology::from_physical_cores(None, CorePerformance::Any, CoreDistribution::Compact, 1)
            .unwrap();
    assert!(topology.group_count() >= 1);
    let executor = Executor::new(&topology, ExecutorOptions::default()).unwrap();
    assert!(!executor.as_raw().is_null());
}
