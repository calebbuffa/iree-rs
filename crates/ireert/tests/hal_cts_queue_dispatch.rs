//! Rust port of the upstream direct queue-dispatch CTS case.

mod support {
    #[path = "cts_compile.rs"]
    pub mod cts_compile;
    #[path = "cts_exec.rs"]
    pub mod cts_exec;
}

use ireert::hal::{
    BufferBinding, BufferParams, BufferRef, BufferUsage, DispatchConfig, DispatchFlags,
    ExecutableCache, ExecutableCachingMode, ExecutableFunction, ExecutableParams, MemoryAccess,
    MemoryType, QueueAffinity, QueueDispatch, QueueSubmission, Semaphore, SemaphorePoint,
    WaitFlags,
};
use std::error::Error as StdError;
use support::cts_exec::{allocate_transfer_buffer, available_devices};

type TestResult<T = ()> = std::result::Result<T, Box<dyn StdError>>;

#[test]
fn dispatch_with_constants_and_bindings() -> TestResult {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");

    for (driver, device) in devices {
        let Some((format, bytes)) = support::cts_compile::compile_cts_executable(
            &device,
            &driver,
            "command_buffer_dispatch_constants_bindings_test.mlir",
        )?
        else {
            eprintln!("skipping {driver}: no supported executable format");
            continue;
        };
        let cache = ExecutableCache::create(&device, "queue-dispatch-cts")?;
        let params = ExecutableParams::new(
            QueueAffinity::ANY,
            ExecutableCachingMode::ALIAS_PROVIDED_DATA,
            &format,
            &bytes,
            &[],
        );
        // SAFETY: this test compiles the executable from the vendored upstream CTS source.
        let executable = unsafe { cache.prepare(&params) }?;

        let buffer_params = BufferParams {
            usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
            ..BufferParams::default()
        };
        let input = device.allocator().allocate_buffer(buffer_params, 16)?;
        let output = device.allocator().allocate_buffer(buffer_params, 16)?;
        let input_data = [1_u32, 2, 3, 4]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        device.transfer_h2d(&input_data, &input, 0, None)?;
        device.transfer_h2d(&[0; 16], &output, 0, None)?;

        let constants = [3_u32, 10]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        let bindings = [
            BufferRef::new(&input, 0, 16),
            BufferRef::new(&output, 0, 16),
        ];
        let readback = allocate_transfer_buffer(&device, 16)?;
        for (lifetime_mode, flags) in [
            ("retained", DispatchFlags::default()),
            ("borrowed", DispatchFlags::BORROW_RESOURCE_LIFETIMES),
        ] {
            device.transfer_h2d(&[0; 16], &output, 0, None)?;
            let dispatch = QueueDispatch::new(
                &executable,
                ExecutableFunction::from_index(0),
                DispatchConfig {
                    workgroup_count: [1, 1, 1],
                    ..DispatchConfig::default()
                },
            )
            .with_constants(&constants)
            .with_bindings(&bindings)
            .with_flags(flags);
            let completion = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
            let submission =
                &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
                    semaphore: &completion,
                    value: 1,
                }]);
            if flags.contains(DispatchFlags::BORROW_RESOURCE_LIFETIMES) {
                // SAFETY: executable, buffers, dispatch data, and completion signal stay live
                // until the completion semaphore is observed below.
                unsafe { device.queue_dispatch_unchecked(submission, &dispatch) }?;
            } else {
                device.queue_dispatch(submission, &dispatch)?;
            }
            completion.wait(1, None, WaitFlags::YIELD)?;

            let actual = read_dispatch_output(&device, &output, &readback, &completion)?;
            assert_eq!(
                actual,
                [13, 16, 19, 22],
                "backend {driver}, {lifetime_mode}"
            );
        }

        let sentinel = [99_u32; 4]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        device.transfer_h2d(&sentinel, &output, 0, None)?;
        let gate = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        let completion = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        let noop = QueueDispatch::new(
            &executable,
            ExecutableFunction::from_index(0),
            DispatchConfig {
                workgroup_count: [0; 3],
                ..DispatchConfig::default()
            },
        )
        .with_constants(&constants)
        .with_bindings(&bindings);
        device.queue_dispatch(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([SemaphorePoint {
                    semaphore: &gate,
                    value: 1,
                }])
                .with_signals([SemaphorePoint {
                    semaphore: &completion,
                    value: 1,
                }]),
            &noop,
        )?;
        assert_eq!(completion.query()?, 0, "backend {driver}");
        gate.signal(1)?;
        completion.wait(1, None, WaitFlags::YIELD)?;
        let actual = read_dispatch_output(&device, &output, &readback, &completion)?;
        assert_eq!(
            actual, [99; 4],
            "no-op dispatch modified output on {driver}"
        );
    }

    Ok(())
}

#[test]
fn dispatch_waits_for_deferred_semaphore_before_execution() -> TestResult {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");

    for (driver, device) in devices {
        let Some((format, bytes)) = support::cts_compile::compile_cts_executable(
            &device,
            &driver,
            "command_buffer_dispatch_constants_bindings_test.mlir",
        )?
        else {
            eprintln!("skipping {driver}: no supported executable format");
            continue;
        };
        let cache = ExecutableCache::create(&device, "queue-dispatch-cts")?;
        let params = ExecutableParams::new(
            QueueAffinity::ANY,
            ExecutableCachingMode::ALIAS_PROVIDED_DATA,
            &format,
            &bytes,
            &[],
        );
        // SAFETY: this test compiles the executable from the vendored upstream CTS source.
        let executable = unsafe { cache.prepare(&params) }?;

        let buffer_params = BufferParams {
            usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
            ..BufferParams::default()
        };
        let input = device.allocator().allocate_buffer(buffer_params, 16)?;
        let output = device.allocator().allocate_buffer(buffer_params, 16)?;
        let input_data = [1_u32, 2, 3, 4]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        let sentinel = [99_u32; 4]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        device.transfer_h2d(&input_data, &input, 0, None)?;
        device.transfer_h2d(&sentinel, &output, 0, None)?;

        let constants = [3_u32, 10]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        let bindings = [
            BufferRef::new(&input, 0, 16),
            BufferRef::new(&output, 0, 16),
        ];
        let dispatch = QueueDispatch::new(
            &executable,
            ExecutableFunction::from_index(0),
            DispatchConfig {
                workgroup_count: [1, 1, 1],
                ..DispatchConfig::default()
            },
        )
        .with_constants(&constants)
        .with_bindings(&bindings);
        let gate = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        let completion = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_dispatch(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([SemaphorePoint {
                    semaphore: &gate,
                    value: 1,
                }])
                .with_signals([SemaphorePoint {
                    semaphore: &completion,
                    value: 1,
                }]),
            &dispatch,
        )?;

        assert_eq!(completion.query()?, 0, "backend {driver}");
        let readback = allocate_transfer_buffer(&device, 16)?;
        let pending_copy = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_copy(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([SemaphorePoint {
                    semaphore: &completion,
                    value: 1,
                }])
                .with_signals([SemaphorePoint {
                    semaphore: &pending_copy,
                    value: 1,
                }]),
            BufferBinding::new(&output, 0, 16),
            BufferBinding::new(&readback, 0, 16),
        )?;
        assert_eq!(pending_copy.query()?, 0, "backend {driver}");

        gate.signal(1)?;
        completion.wait(1, None, WaitFlags::YIELD)?;
        pending_copy.wait(1, None, WaitFlags::YIELD)?;
        let mut output_data = [0_u8; 16];
        // SAFETY: dispatch completion and the dependent readback copy precede this transfer.
        unsafe { device.transfer_d2h(&readback, 0, &mut output_data, None) }?;
        let actual: Vec<u32> = output_data
            .chunks_exact(std::mem::size_of::<u32>())
            .map(|bytes| {
                let mut word = [0; std::mem::size_of::<u32>()];
                word.copy_from_slice(bytes);
                u32::from_ne_bytes(word)
            })
            .collect();
        assert_eq!(actual, [13, 16, 19, 22], "backend {driver}");
    }

    Ok(())
}

#[test]
fn indirect_dispatch_uses_buffer_workgroup_counts() -> TestResult {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");

    const WORKGROUP_COUNT: usize = 4;
    const OUTPUT_ELEMENT_COUNT: usize = 32;
    const SENTINEL: u32 = 0xCDCD_CDCD;
    const OUTPUT_BYTES: usize = OUTPUT_ELEMENT_COUNT * std::mem::size_of::<u32>();
    const PARAMETER_BYTES: usize = 3 * std::mem::size_of::<u32>();

    for (driver, device) in devices {
        let Some((format, bytes)) = support::cts_compile::compile_cts_executable(
            &device,
            &driver,
            "command_buffer_dispatch_multi_workgroup_test.mlir",
        )?
        else {
            eprintln!("skipping {driver}: no supported executable format");
            continue;
        };
        let cache = ExecutableCache::create(&device, "queue-indirect-dispatch-cts")?;
        let params = ExecutableParams::new(
            QueueAffinity::ANY,
            ExecutableCachingMode::ALIAS_PROVIDED_DATA,
            &format,
            &bytes,
            &[],
        );
        // SAFETY: this test compiles the executable from the vendored upstream CTS source.
        let executable = unsafe { cache.prepare(&params) }?;

        let output = device.allocator().allocate_buffer(
            BufferParams {
                usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE,
                access: MemoryAccess::ALL,
                memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
                ..BufferParams::default()
            },
            OUTPUT_BYTES,
        )?;
        let workgroup_counts = device.allocator().allocate_buffer(
            BufferParams {
                usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_INDIRECT_PARAMETERS,
                access: MemoryAccess::ALL,
                memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
                ..BufferParams::default()
            },
            PARAMETER_BYTES,
        )?;
        let readback = allocate_transfer_buffer(&device, OUTPUT_BYTES)?;

        let counts = [WORKGROUP_COUNT as u32, 1, 1]
            .into_iter()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        let updated = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_update(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
                semaphore: &updated,
                value: 1,
            }]),
            &counts,
            0,
            BufferBinding::new(&workgroup_counts, 0, PARAMETER_BYTES),
        )?;

        let output_binding = [BufferRef::new(&output, 0, OUTPUT_BYTES)];
        for (mode, flags) in [
            ("static", DispatchFlags::STATIC_INDIRECT_PARAMETERS),
            ("dynamic", DispatchFlags::DYNAMIC_INDIRECT_PARAMETERS),
        ] {
            let sentinel = [SENTINEL; OUTPUT_ELEMENT_COUNT]
                .into_iter()
                .flat_map(u32::to_ne_bytes)
                .collect::<Vec<_>>();
            device.transfer_h2d(&sentinel, &output, 0, None)?;

            let completion = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
            let dispatch = QueueDispatch::new(
                &executable,
                ExecutableFunction::from_index(0),
                DispatchConfig {
                    workgroup_count: [OUTPUT_ELEMENT_COUNT as u32, 1, 1],
                    ..DispatchConfig::default()
                },
            )
            .with_workgroup_count_buffer(BufferRef::new(&workgroup_counts, 0, PARAMETER_BYTES))
            .with_bindings(&output_binding)
            .with_flags(flags);
            device.queue_dispatch(
                &QueueSubmission::new(QueueAffinity::ANY)
                    .with_waits([SemaphorePoint {
                        semaphore: &updated,
                        value: 1,
                    }])
                    .with_signals([SemaphorePoint {
                        semaphore: &completion,
                        value: 1,
                    }]),
                &dispatch,
            )?;
            completion.wait(1, None, WaitFlags::YIELD)?;

            let copied = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
            device.queue_copy(
                &QueueSubmission::new(QueueAffinity::ANY)
                    .with_waits([SemaphorePoint {
                        semaphore: &completion,
                        value: 1,
                    }])
                    .with_signals([SemaphorePoint {
                        semaphore: &copied,
                        value: 1,
                    }]),
                BufferBinding::new(&output, 0, OUTPUT_BYTES),
                BufferBinding::new(&readback, 0, OUTPUT_BYTES),
            )?;
            copied.wait(1, None, WaitFlags::YIELD)?;
            let mut output_data = vec![0; OUTPUT_BYTES];
            // SAFETY: dispatch completion and the dependent readback copy precede this transfer.
            unsafe { device.transfer_d2h(&readback, 0, &mut output_data, None) }?;
            let actual: Vec<u32> = output_data
                .chunks_exact(std::mem::size_of::<u32>())
                .map(|bytes| {
                    let mut word = [0; std::mem::size_of::<u32>()];
                    word.copy_from_slice(bytes);
                    u32::from_ne_bytes(word)
                })
                .collect();
            let mut expected = [SENTINEL; OUTPUT_ELEMENT_COUNT];
            for (index, value) in expected[..WORKGROUP_COUNT].iter_mut().enumerate() {
                *value = index as u32;
            }
            assert_eq!(
                actual, expected,
                "backend {driver}, {mode} indirect parameters"
            );
        }
    }

    Ok(())
}

fn read_dispatch_output(
    device: &ireert::hal::GroupedDevice,
    output: &ireert::hal::Buffer,
    readback: &ireert::hal::Buffer,
    completion: &Semaphore,
) -> ireert::Result<Vec<u32>> {
    let copied = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_copy(
        &QueueSubmission::new(QueueAffinity::ANY)
            .with_waits([SemaphorePoint {
                semaphore: completion,
                value: 1,
            }])
            .with_signals([SemaphorePoint {
                semaphore: &copied,
                value: 1,
            }]),
        BufferBinding::new(output, 0, 16),
        BufferBinding::new(readback, 0, 16),
    )?;
    copied.wait(1, None, WaitFlags::YIELD)?;

    let mut output_data = [0_u8; 16];
    // SAFETY: dispatch and its readback copy both precede this transfer.
    unsafe { device.transfer_d2h(readback, 0, &mut output_data, None) }?;
    Ok(output_data
        .chunks_exact(std::mem::size_of::<u32>())
        .map(|bytes| {
            let mut word = [0; std::mem::size_of::<u32>()];
            word.copy_from_slice(bytes);
            u32::from_ne_bytes(word)
        })
        .collect())
}
