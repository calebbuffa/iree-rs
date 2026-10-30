//! Regression coverage for isolated VMVX loader type-registry interactions.
#![cfg(feature = "cpu")]

#[cfg(feature = "local-task")]
use ireert::task::{Executor, Topology};
use ireert::{Code, hal::backends::local_loader::ExecutableLoader, vm::Instance};
#[cfg(feature = "local-task")]
use ireert::{
    asynchronous::ProactorPool,
    hal::{Allocator, DeviceCreateParams},
};

#[test]
fn all_available_loaders_block_shared_instance_until_drop() {
    // SAFETY: this test process has not created a VM instance or VM references.
    let loaders = unsafe { ExecutableLoader::all_available(None) }.unwrap();
    let error = match Instance::new() {
        Ok(_) => {
            panic!("shared instance creation must be rejected while the isolated loader lives")
        }
        Err(error) => error,
    };
    assert_eq!(error.code(), Code::FailedPrecondition);

    #[cfg(feature = "local-task")]
    {
        let pool = ProactorPool::shared().unwrap();
        let params = DeviceCreateParams::new(&pool);
        let allocator = Allocator::heap("isolated-vmvx-test").unwrap();
        let topology = Topology::from_group_count(1);
        let executor = Executor::new(&topology, Default::default()).unwrap();
        let error = match ireert::hal::backends::local_task::create_device(
            "local-task",
            Default::default(),
            &[executor],
            &loaders,
            Some(&allocator),
            &params,
        ) {
            Ok(_) => panic!("safe local-task construction must reject isolated VMVX loaders"),
            Err(error) => error,
        };
        assert_eq!(error.code(), Code::FailedPrecondition);
    }

    drop(loaders);
    assert!(Instance::new().is_ok());
}
