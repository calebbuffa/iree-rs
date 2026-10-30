//! HAL core CTS ports.

#[path = "support/cts.rs"]
mod cts;

use cts::devices;
use ireert::{
    Code,
    asynchronous::ProactorPool,
    hal::{
        CommandBuffer, CommandBufferMode, CommandCategory, DeviceCreateParams, Driver,
        DriverRegistry, Event, ExecuteFlags, ExecutionStage, QueueAffinity, QueueSubmission,
        Semaphore, SemaphorePoint, WaitFlags, WaitMode,
    },
};
use std::{thread, time::Duration};

#[test]
fn imports_cannot_escalate_immutable_host_file_access() -> ireert::Result<()> {
    use ireert::{
        hal::{
            MemoryAccess,
            file::{File, FileCache},
        },
        io::FileHandle,
    };
    let handle = FileHandle::from_owner(b"immutable backing")?;
    for (_, _, device) in devices()? {
        let error = File::import(&device, QueueAffinity::ANY, MemoryAccess::WRITE, &handle)
            .err()
            .unwrap();
        assert_eq!(error.code(), Code::PermissionDenied);
        let cache = FileCache::new()?;
        let error = cache
            .lookup(&device, QueueAffinity::ANY, MemoryAccess::WRITE, &handle)
            .err()
            .unwrap();
        assert_eq!(error.code(), Code::PermissionDenied);
        let file = File::import(&device, QueueAffinity::ANY, MemoryAccess::READ, &handle)?;
        assert!(file.allowed_access().contains(MemoryAccess::READ));
        assert!(!file.allowed_access().contains(MemoryAccess::WRITE));
    }
    Ok(())
}

fn try_create_available_driver(
    registry: &DriverRegistry,
    name: &str,
) -> ireert::Result<Option<Driver>> {
    match cts::create_test_driver(registry, name) {
        Ok(driver) => Ok(Some(driver)),
        Err(error)
            if matches!(
                error.code(),
                Code::Unavailable | Code::Unimplemented | Code::NotFound
            ) =>
        {
            eprintln!("skipping {name}: driver unavailable: {error}");
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

fn try_create_event(backend: &str, device: &ireert::hal::Device) -> ireert::Result<Option<Event>> {
    match Event::create(device, QueueAffinity::ANY) {
        Ok(event) => Ok(Some(event)),
        Err(error) if error.code() == Code::Unimplemented => {
            eprintln!("skipping {backend}: HAL events are unsupported");
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

#[test]
fn query_and_create_available_devices_by_id() -> ireert::Result<()> {
    let _guard = cts::lock_test_devices();
    let registry = cts::registry()?;
    let pool = ProactorPool::shared()?;
    for info in registry.enumerate()? {
        if !cts::selected_driver(&info.name) {
            continue;
        }
        let Some(driver) = try_create_available_driver(&registry, &info.name)? else {
            continue;
        };
        let params = DeviceCreateParams::new(&pool);
        for device_info in driver.devices()? {
            let device = driver.create_device_by_id(device_info.id, &[], &params)?;
            assert!(!device.id().is_empty());
        }
    }
    Ok(())
}

#[test]
fn query_and_create_available_devices_by_ordinal() -> ireert::Result<()> {
    let _guard = cts::lock_test_devices();
    let registry = cts::registry()?;
    let pool = ProactorPool::shared()?;
    for info in registry.enumerate()? {
        if !cts::selected_driver(&info.name) {
            continue;
        }
        let Some(driver) = try_create_available_driver(&registry, &info.name)? else {
            continue;
        };
        let params = DeviceCreateParams::new(&pool);
        for (ordinal, _) in driver.devices()?.iter().enumerate() {
            let device = driver.create_device_by_ordinal(ordinal, &[], &params)?;
            assert!(!device.id().is_empty());
        }
    }
    Ok(())
}

#[test]
fn query_and_create_available_devices_by_path() -> ireert::Result<()> {
    let _guard = cts::lock_test_devices();
    let registry = cts::registry()?;
    let pool = ProactorPool::shared()?;
    for info in registry.enumerate()? {
        if !cts::selected_driver(&info.name) {
            continue;
        }
        let Some(driver) = try_create_available_driver(&registry, &info.name)? else {
            continue;
        };
        let devices = driver.devices()?;
        if devices.is_empty() {
            // Upstream explicitly skips this case when no devices are enumerated.
            eprintln!("skipping {}: no available devices", info.name);
            continue;
        }
        let params = DeviceCreateParams::new(&pool);
        let mut tested_empty_path = false;
        for device_info in &devices {
            tested_empty_path |= device_info.path.is_empty();
            match driver.create_device_by_path(&info.name, &device_info.path, &[], &params) {
                Ok(device) => assert!(!device.id().is_empty()),
                // Upstream permits NOT_FOUND only for non-empty driver-specific paths.
                Err(error) if !device_info.path.is_empty() && error.code() == Code::NotFound => {}
                Err(error) => return Err(error),
            }
        }
        if !tested_empty_path {
            driver.create_device_by_path(&info.name, "", &[], &params)?;
        }
    }
    Ok(())
}

#[test]
fn event_create() {
    for (backend, _, device) in devices().unwrap() {
        drop(try_create_event(&backend, &device).unwrap());
    }
}

#[test]
fn grouped_device_retains_topology_after_group_owner_drops() {
    for (_, _, device) in devices().unwrap() {
        let group = device.device_group().clone();
        let retained = group.device(0).expect("group contains its device");
        drop(group);
        drop(device);

        assert!(retained.topology_info().is_some());
        retained.queue_flush(QueueAffinity::ANY).unwrap();
    }
}

fn submit_and_wait(device: &ireert::hal::GroupedDevice, cb: &CommandBuffer) -> ireert::Result<()> {
    let done = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_execute(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
            semaphore: &done,
            value: 1,
        }]),
        cb,
        &[],
        ExecuteFlags::default(),
    )?;
    device.wait_semaphores(
        WaitMode::All,
        &[SemaphorePoint {
            semaphore: &done,
            value: 1,
        }],
        None,
        WaitFlags::default(),
    )
}

#[test]
fn event_signal_and_reset() -> ireert::Result<()> {
    for (backend, _, device) in devices().unwrap() {
        let Some(event) = try_create_event(&backend, &device).unwrap() else {
            continue;
        };
        let mut cb = CommandBuffer::create(
            &device,
            CommandBufferMode::ONE_SHOT,
            CommandCategory::DISPATCH,
            QueueAffinity::ANY,
            0,
        )
        .unwrap();
        cb.begin().unwrap();
        cb.signal_event(&event, ExecutionStage::COMMAND_PROCESS)
            .unwrap();
        cb.reset_event(&event, ExecutionStage::COMMAND_RETIRE)
            .unwrap();
        cb.end().unwrap();
        submit_and_wait(&device, &cb)?;
    }
    Ok(())
}

#[test]
fn event_submit_with_chained_command_buffers() -> ireert::Result<()> {
    for (backend, _, device) in devices().unwrap() {
        let Some(event) = try_create_event(&backend, &device).unwrap() else {
            continue;
        };
        let mut first = CommandBuffer::create(
            &device,
            CommandBufferMode::ONE_SHOT,
            CommandCategory::DISPATCH,
            QueueAffinity::ANY,
            0,
        )
        .unwrap();
        first.begin().unwrap();
        first
            .signal_event(&event, ExecutionStage::COMMAND_RETIRE)
            .unwrap();
        first.end().unwrap();
        let mut second = CommandBuffer::create(
            &device,
            CommandBufferMode::ONE_SHOT,
            CommandCategory::DISPATCH,
            QueueAffinity::ANY,
            0,
        )
        .unwrap();
        second.begin().unwrap();
        second
            .wait_events(
                &[&event],
                ExecutionStage::COMMAND_RETIRE,
                ExecutionStage::COMMAND_ISSUE,
                &[],
                &[],
            )
            .unwrap();
        second.end().unwrap();
        submit_and_wait(&device, &first)?;
        submit_and_wait(&device, &second)?;
    }
    Ok(())
}

fn semaphore(device: &ireert::hal::Device) -> ireert::Result<Semaphore> {
    Semaphore::new(device, QueueAffinity::ANY, 0)
}

#[test]
fn semaphore_no_op() {
    for (_, _, d) in devices().unwrap() {
        let s = Semaphore::new(&d, QueueAffinity::ANY, 123).unwrap();
        assert_eq!(s.query().unwrap(), 123);
    }
}
#[test]
fn semaphore_normal_signaling() {
    for (_, _, d) in devices().unwrap() {
        let s = Semaphore::new(&d, QueueAffinity::ANY, 2).unwrap();
        assert_eq!(s.query().unwrap(), 2);
        s.signal(3).unwrap();
        assert_eq!(s.query().unwrap(), 3);
        s.signal(40).unwrap();
        assert_eq!(s.query().unwrap(), 40);
    }
}
#[test]
fn semaphore_failure() {
    for (_, _, d) in devices().unwrap() {
        let s = Semaphore::new(&d, QueueAffinity::ANY, 2).unwrap();
        s.signal(3).unwrap();
        assert_eq!(s.query().unwrap(), 3);
        s.fail(Code::Unknown);
        assert_eq!(s.query().unwrap_err().code(), Code::Unknown);
    }
}
#[test]
fn semaphore_empty_wait() {
    for (_, _, d) in devices().unwrap() {
        for mode in [WaitMode::Any, WaitMode::All] {
            d.wait_semaphores(mode, &[], None, WaitFlags::default())
                .unwrap();
        }
    }
}
#[test]
fn semaphore_wait_already_signaled() {
    for (_, _, d) in devices().unwrap() {
        let s = Semaphore::new(&d, QueueAffinity::ANY, 2).unwrap();
        for v in [1, 2] {
            s.wait(v, None, WaitFlags::default()).unwrap();
        }
    }
}
#[test]
fn semaphore_wait_unsignaled() {
    for (_, _, d) in devices().unwrap() {
        let s = Semaphore::new(&d, QueueAffinity::ANY, 2).unwrap();
        assert_eq!(
            s.wait(3, Some(Duration::ZERO), WaitFlags::default())
                .unwrap_err()
                .code(),
            Code::DeadlineExceeded
        );
    }
}
#[test]
fn semaphore_wait_all_but_not_all_signaled() {
    for (_, _, d) in devices().unwrap() {
        let a = Semaphore::new(&d, QueueAffinity::ANY, 0).unwrap();
        let b = Semaphore::new(&d, QueueAffinity::ANY, 1).unwrap();
        let points = [
            SemaphorePoint {
                semaphore: &a,
                value: 1,
            },
            SemaphorePoint {
                semaphore: &b,
                value: 1,
            },
        ];
        assert_eq!(
            d.wait_semaphores(
                WaitMode::All,
                &points,
                Some(Duration::ZERO),
                WaitFlags::default()
            )
            .unwrap_err()
            .code(),
            Code::DeadlineExceeded
        );
    }
}
#[test]
fn semaphore_wait_all_and_all_signaled() {
    for (_, _, d) in devices().unwrap() {
        let a = Semaphore::new(&d, QueueAffinity::ANY, 1).unwrap();
        let b = Semaphore::new(&d, QueueAffinity::ANY, 1).unwrap();
        let points = [
            SemaphorePoint {
                semaphore: &a,
                value: 1,
            },
            SemaphorePoint {
                semaphore: &b,
                value: 1,
            },
        ];
        d.wait_semaphores(WaitMode::All, &points, None, WaitFlags::default())
            .unwrap();
    }
}
#[test]
fn semaphore_wait_any_already_signaled() {
    for (_, _, d) in devices().unwrap() {
        let a = Semaphore::new(&d, QueueAffinity::ANY, 0).unwrap();
        let b = Semaphore::new(&d, QueueAffinity::ANY, 1).unwrap();
        let points = [
            SemaphorePoint {
                semaphore: &a,
                value: 1,
            },
            SemaphorePoint {
                semaphore: &b,
                value: 1,
            },
        ];
        d.wait_semaphores(WaitMode::Any, &points, None, WaitFlags::default())
            .unwrap();
    }
}
#[test]
fn semaphore_fail_then_wait() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let s = semaphore(&d)?;
        s.fail(Code::Cancelled);
        assert_eq!(
            s.wait(1, None, WaitFlags::default()).unwrap_err().code(),
            Code::Cancelled
        );
        let (status, value) = s.query_with_value();
        assert_eq!(status.unwrap_err().code(), Code::Cancelled);
        assert!(value >= Semaphore::FAILURE_VALUE);
    }
    Ok(())
}
#[test]
fn semaphore_failure_preserves_status_code() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let s = semaphore(&d)?;
        s.fail(Code::DataLoss);
        let (status, value) = s.query_with_value();
        assert_eq!(status.unwrap_err().code(), Code::DataLoss);
        assert!(value >= Semaphore::FAILURE_VALUE);
    }
    Ok(())
}
#[test]
fn semaphore_double_failure_preserves_first() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let s = semaphore(&d)?;
        s.fail(Code::DataLoss);
        s.fail(Code::Cancelled);
        let (status, value) = s.query_with_value();
        assert_eq!(status.unwrap_err().code(), Code::DataLoss);
        assert!(value >= Semaphore::FAILURE_VALUE);
    }
    Ok(())
}

#[test]
fn semaphore_thread_wait_later_signaled_beyond() {
    for (_, _, d) in devices().unwrap() {
        let s = Semaphore::new(&d, QueueAffinity::ANY, 2).unwrap();
        let signal = s.clone();
        let thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            signal.signal(10).unwrap();
        });
        s.wait(3, None, WaitFlags::default()).unwrap();
        thread.join().unwrap();
    }
}
#[test]
fn semaphore_thread_wait_any_later_signaled() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let a = semaphore(&d)?;
        let b = semaphore(&d)?;
        let signal = b.clone();
        let thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            signal.signal(1).unwrap();
        });
        let points = [
            SemaphorePoint {
                semaphore: &a,
                value: 1,
            },
            SemaphorePoint {
                semaphore: &b,
                value: 1,
            },
        ];
        d.wait_semaphores(WaitMode::Any, &points, None, WaitFlags::default())
            .unwrap();
        thread.join().unwrap();
    }
    Ok(())
}
#[test]
fn semaphore_thread_ping_pong() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let a2b = semaphore(&d)?;
        let b2a = semaphore(&d)?;
        let a = a2b.clone();
        let b = b2a.clone();
        let thread = thread::spawn(move || {
            a.wait(0, None, WaitFlags::default()).unwrap();
            b.signal(1).unwrap();
            a.wait(4, None, WaitFlags::default()).unwrap();
        });
        b2a.wait(1, None, WaitFlags::default()).unwrap();
        a2b.signal(4).unwrap();
        thread.join().unwrap();
    }
    Ok(())
}
#[test]
fn semaphore_thread_wait_on_the_same_value_multiple_times() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let s = semaphore(&d)?;
        let signal = s.clone();
        let thread = thread::spawn(move || signal.signal(1).unwrap());
        s.wait(1, None, WaitFlags::default()).unwrap();
        assert_eq!(s.query().unwrap(), 1);
        s.wait(1, None, WaitFlags::default()).unwrap();
        assert_eq!(s.query().unwrap(), 1);
        thread.join().unwrap();
    }
    Ok(())
}
#[test]
fn semaphore_thread_wait_for_finite_time() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        for timeout in [Some(Duration::ZERO), Some(Duration::from_nanos(1))] {
            let s = semaphore(&d)?;
            assert_eq!(
                s.wait(1, timeout, WaitFlags::default()).unwrap_err().code(),
                Code::DeadlineExceeded
            );
            assert_eq!(s.query().unwrap(), 0);
            let signal = s.clone();
            let thread = thread::spawn(move || signal.signal(1).unwrap());
            let deadline = std::time::Instant::now() + Duration::from_secs(5);
            loop {
                match s.wait(1, Some(Duration::from_millis(1)), WaitFlags::default()) {
                    Ok(()) => break,
                    Err(e)
                        if e.code() == Code::DeadlineExceeded
                            && std::time::Instant::now() < deadline => {}
                    Err(e) => return Err(e),
                }
            }
            assert_eq!(s.query().unwrap(), 1);
            thread.join().unwrap();
        }
    }
    Ok(())
}
#[test]
fn semaphore_thread_simultaneous_multi_wait_all() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let a = semaphore(&d)?;
        let b = semaphore(&d)?;
        thread::scope(|scope| {
            for _ in 0..2 {
                let aa = a.clone();
                let bb = b.clone();
                let device = &d;
                scope.spawn(move || {
                    let points = [
                        SemaphorePoint {
                            semaphore: &aa,
                            value: 1,
                        },
                        SemaphorePoint {
                            semaphore: &bb,
                            value: 1,
                        },
                    ];
                    device
                        .wait_semaphores(WaitMode::All, &points, None, WaitFlags::default())
                        .unwrap();
                });
            }
            let aa = a.clone();
            let bb = b.clone();
            scope.spawn(move || {
                aa.signal(1).unwrap();
                bb.signal(1).unwrap();
            });
        });
        assert_eq!(a.query().unwrap(), 1);
        assert_eq!(b.query().unwrap(), 1);
    }
    Ok(())
}
#[test]
fn semaphore_thread_wait_then_fail() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let s = semaphore(&d)?;
        let fail = s.clone();
        let thread = thread::spawn(move || fail.fail(Code::Cancelled));
        assert_eq!(
            s.wait(1, None, WaitFlags::default()).unwrap_err().code(),
            Code::Cancelled
        );
        let (status, value) = s.query_with_value();
        assert_eq!(status.unwrap_err().code(), Code::Cancelled);
        assert!(value >= Semaphore::FAILURE_VALUE);
        thread.join().unwrap();
    }
    Ok(())
}
#[test]
fn semaphore_thread_multi_wait_then_fail() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let a = semaphore(&d)?;
        let b = semaphore(&d)?;
        let fail = a.clone();
        let thread = thread::spawn(move || fail.fail(Code::Cancelled));
        let points = [
            SemaphorePoint {
                semaphore: &a,
                value: 1,
            },
            SemaphorePoint {
                semaphore: &b,
                value: 1,
            },
        ];
        assert_eq!(
            d.wait_semaphores(WaitMode::All, &points, None, WaitFlags::default())
                .unwrap_err()
                .code(),
            Code::Cancelled
        );
        let (status, value) = a.query_with_value();
        assert_eq!(status.unwrap_err().code(), Code::Cancelled);
        assert!(value >= Semaphore::FAILURE_VALUE);
        assert_eq!(b.query().unwrap(), 0);
        thread.join().unwrap();
    }
    Ok(())
}
#[test]
fn semaphore_thread_device_multi_wait_then_fail() -> ireert::Result<()> {
    for (_, _, d) in devices().unwrap() {
        let a = semaphore(&d)?;
        let b = semaphore(&d)?;
        let fail = a.clone();
        let thread = thread::spawn(move || fail.fail(Code::Cancelled));
        let points = [
            SemaphorePoint {
                semaphore: &a,
                value: 1,
            },
            SemaphorePoint {
                semaphore: &b,
                value: 1,
            },
        ];
        assert_eq!(
            d.wait_semaphores(WaitMode::Any, &points, None, WaitFlags::default())
                .unwrap_err()
                .code(),
            Code::Cancelled
        );
        let (status, value) = a.query_with_value();
        assert_eq!(status.unwrap_err().code(), Code::Cancelled);
        assert!(value >= Semaphore::FAILURE_VALUE);
        assert_eq!(b.query().unwrap(), 0);
        thread.join().unwrap();
    }
    Ok(())
}

#[test]
fn executable_cache_create() {
    for (name, _, device) in devices().unwrap() {
        drop(ireert::hal::ExecutableCache::create(&device, &name).unwrap());
    }
}

#[test]
fn executable_cache_cant_prepare_unknown_format() {
    for (_, _, device) in devices().unwrap() {
        let cache = ireert::hal::ExecutableCache::create(&device, "default").unwrap();
        assert!(!cache.can_prepare_format(ireert::hal::ExecutableCachingMode::default(), "FOO?"));
    }
}
