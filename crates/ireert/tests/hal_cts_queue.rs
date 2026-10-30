//! Rust ports of representative upstream HAL queue CTS cases.

mod support {
    #[path = "cts_exec.rs"]
    pub mod cts_exec;
}

use ireert::hal::{
    AllocaFlags, BufferBinding, BufferParams, BufferUsage, DeallocaFlags, ExecuteFlags,
    HostCallFlags, MemoryAccess, MemoryType, QueueAffinity, QueueSubmission, Semaphore,
    SemaphorePoint, WaitFlags,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use support::cts_exec::{allocate_transfer_buffer, available_devices};

fn signal_point(semaphore: &Semaphore, value: u64) -> SemaphorePoint<'_> {
    SemaphorePoint { semaphore, value }
}

macro_rules! on_each_device {
    ($name:ident, |$driver:pat_param, $device:ident: $device_type:ty| $body:block) => {
        #[test]
        fn $name() -> ireert::Result<()> {
            let devices = available_devices()?;
            assert!(!devices.is_empty(), "no usable HAL backend available");
            for (driver, device) in devices {
                let run = |$driver: &str, $device: $device_type| -> ireert::Result<()> {
                    $body
                    Ok(())
                };
                run(&driver, &device)?;
            }
            Ok(())
        }
    };
}

fn await_signal(device: &ireert::hal::GroupedDevice, signal: &Semaphore) -> ireert::Result<()> {
    device.wait_semaphores(
        ireert::hal::WaitMode::All,
        &[signal_point(signal, 1)],
        None,
        WaitFlags::YIELD,
    )
}

on_each_device!(
    fill_entire_buffer_1_byte,
    |_, device: &ireert::hal::GroupedDevice| {
        let buffer = allocate_transfer_buffer(device, 64)?;
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_fill(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            BufferBinding::new(&buffer, 0, 64),
            &[0xAB],
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0; 64];
        // SAFETY: allocation initializes the full buffer; the preceding fill preserves that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert!(actual.iter().all(|byte| *byte == 0xAB));
    }
);

on_each_device!(
    fill_entire_buffer_2_byte,
    |_, device: &ireert::hal::GroupedDevice| {
        let buffer = allocate_transfer_buffer(device, 64)?;
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_fill(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            BufferBinding::new(&buffer, 0, 64),
            &[0xEF, 0xBE],
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0; 64];
        // SAFETY: allocation initializes the full buffer; the preceding fill preserves that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert!(actual.chunks_exact(2).all(|word| word == [0xEF, 0xBE]));
    }
);

on_each_device!(
    fill_entire_buffer_4_byte,
    |_, device: &ireert::hal::GroupedDevice| {
        let buffer = allocate_transfer_buffer(device, 64)?;
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_fill(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            BufferBinding::new(&buffer, 0, 64),
            &[0xFE, 0xCA, 0xAD, 0xDE],
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0; 64];
        // SAFETY: allocation initializes the full buffer; the preceding fill preserves that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert!(
            actual
                .chunks_exact(4)
                .all(|word| word == [0xFE, 0xCA, 0xAD, 0xDE])
        );
    }
);

on_each_device!(fill_subrange, |_, device: &ireert::hal::GroupedDevice| {
    let buffer = allocate_transfer_buffer(device, 256)?;
    let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_fill(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
        BufferBinding::new(&buffer, 64, 128),
        &[0x0D, 0xF0, 0xFE, 0xCA],
    )?;
    await_signal(device, &signal)?;
    let mut actual = vec![0xFF; 256];
    // SAFETY: allocation initializes the full buffer; the preceding fill preserves that.
    unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
    assert!(actual[..64].iter().all(|byte| *byte == 0));
    assert!(actual[192..].iter().all(|byte| *byte == 0));
    assert!(
        actual[64..192]
            .chunks_exact(4)
            .all(|word| word == [0x0D, 0xF0, 0xFE, 0xCA])
    );
});

on_each_device!(
    update_entire_buffer,
    |_, device: &ireert::hal::GroupedDevice| {
        let buffer = allocate_transfer_buffer(device, 16)?;
        let source: Vec<u8> = (0..16).collect();
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_update(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            &source,
            0,
            BufferBinding::new(&buffer, 0, 16),
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0; 16];
        // SAFETY: allocation initializes the full buffer; the preceding update preserves that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert_eq!(actual, source);
    }
);

on_each_device!(
    update_with_source_offset,
    |_, device: &ireert::hal::GroupedDevice| {
        let buffer = allocate_transfer_buffer(device, 8)?;
        let source = [10, 11, 12, 13, 14, 15, 16, 17];
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_update(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            &source,
            2,
            BufferBinding::new(&buffer, 1, 4),
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0xFF; 8];
        // SAFETY: allocation initializes the full buffer; the preceding update preserves that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert_eq!(&actual[1..5], &[12, 13, 14, 15]);
        assert_eq!(actual[0], 0);
        assert!(actual[5..].iter().all(|byte| *byte == 0));
    }
);

on_each_device!(
    copy_entire_buffer,
    |_, device: &ireert::hal::GroupedDevice| {
        let source = allocate_transfer_buffer(device, 16)?;
        let target = allocate_transfer_buffer(device, 16)?;
        device.transfer_h2d(&(0..16).collect::<Vec<u8>>(), &source, 0, None)?;
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_copy(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            BufferBinding::new(&source, 0, 16),
            BufferBinding::new(&target, 0, 16),
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0; 16];
        // SAFETY: both buffers are initialized before the queued copy.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }?;
        assert_eq!(actual, (0..16).collect::<Vec<u8>>());
    }
);

on_each_device!(
    copy_with_offsets,
    |_, device: &ireert::hal::GroupedDevice| {
        let source = allocate_transfer_buffer(device, 16)?;
        let target = allocate_transfer_buffer(device, 16)?;
        device.transfer_h2d(&(0..8).collect::<Vec<u8>>(), &source, 3, None)?;
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_copy(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            BufferBinding::new(&source, 3, 8),
            BufferBinding::new(&target, 4, 8),
        )?;
        await_signal(device, &signal)?;
        let mut actual = vec![0xFF; 16];
        // SAFETY: both buffers are initialized before the queued copy.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }?;
        assert!(actual[..4].iter().all(|byte| *byte == 0));
        assert_eq!(&actual[4..12], &(0..8).collect::<Vec<u8>>());
        assert!(actual[12..].iter().all(|byte| *byte == 0));
    }
);

on_each_device!(basic_alloca, |_, device: &ireert::hal::GroupedDevice| {
    let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    let buffer = device.queue_alloca(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
        None,
        BufferParams {
            usage: BufferUsage::TRANSFER,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::OPTIMAL,
            ..BufferParams::default()
        },
        64,
        AllocaFlags::default(),
    )?;
    await_signal(device, &signal)?;
    assert_eq!(buffer.byte_length(), 64);
    let release = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_dealloca(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&release, 1)]),
        &buffer,
        DeallocaFlags::default(),
    )?;
    await_signal(device, &release)?;
});

on_each_device!(
    submit_with_no_command_buffers,
    |driver, device: &ireert::hal::GroupedDevice| {
        // Upstream tags this suite async_queue, excluding local-sync.
        if driver == "local-sync" {
            return Ok(());
        }
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_barrier(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            ExecuteFlags::default(),
        )?;
        await_signal(device, &signal)?;
    }
);

on_each_device!(async_callback, |_, device: &ireert::hal::GroupedDevice| {
    let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    let called = Arc::new(AtomicBool::new(false));
    let called_in_callback = called.clone();
    device.queue_host_call(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
        HostCallFlags::default(),
        move |_| {
            called_in_callback.store(true, Ordering::SeqCst);
            Ok(())
        },
    )?;
    await_signal(device, &signal)?;
    assert!(called.load(Ordering::SeqCst));
});

on_each_device!(
    host_call_waits_for_semaphore,
    |driver, device: &ireert::hal::GroupedDevice| {
        if driver == "local-sync" {
            return Ok(());
        }
        let wait = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        let called = Arc::new(AtomicBool::new(false));
        let called_in_callback = called.clone();
        device.queue_host_call(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([signal_point(&wait, 1)])
                .with_signals([signal_point(&signal, 1)]),
            HostCallFlags::default(),
            move |_| {
                called_in_callback.store(true, Ordering::SeqCst);
                Ok(())
            },
        )?;
        assert!(!called.load(Ordering::SeqCst));
        wait.signal(1)?;
        await_signal(device, &signal)?;
        assert!(called.load(Ordering::SeqCst));
    }
);

on_each_device!(
    host_call_error_fails_signal,
    |_, device: &ireert::hal::GroupedDevice| {
        let signal = Semaphore::new(device, QueueAffinity::ANY, 0)?;
        device.queue_host_call(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            HostCallFlags::default(),
            |_| {
                Err(ireert::Error::new(
                    ireert::Code::PermissionDenied,
                    "test error from queued host callback",
                ))
            },
        )?;
        let error = device
            .wait_semaphores(
                ireert::hal::WaitMode::All,
                &[signal_point(&signal, 1)],
                None,
                WaitFlags::YIELD,
            )
            .expect_err("callback failure should fail its signal semaphore");
        assert_eq!(error.code(), ireert::Code::PermissionDenied);
    }
);

#[test]
fn deferred_wait_before_signal() -> ireert::Result<()> {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (driver, device) in devices {
        // Upstream tags semaphore-submission and deferred queue cases async_queue; local-sync
        // blocks inside the submission call and cannot represent wait-before-signal ordering.
        if driver == "local-sync" {
            continue;
        }
        let wait = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        let signal = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_barrier(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([signal_point(&wait, 1)])
                .with_signals([signal_point(&signal, 1)]),
            ExecuteFlags::default(),
        )?;
        wait.signal(1)?;
        await_signal(&device, &signal)?;
    }
    Ok(())
}

#[test]
fn fill_large_buffer() -> ireert::Result<()> {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let size = 64 * 1024;
        let buffer = allocate_transfer_buffer(&device, size)?;
        let signal = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_fill(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
            BufferBinding::new(&buffer, 0, size),
            &[0xA1, 0xB2, 0xC3, 0xD4],
        )?;
        await_signal(&device, &signal)?;
        let mut actual = vec![0; size];
        // SAFETY: allocation initializes the full buffer; the preceding fill preserves that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert!(
            actual
                .chunks_exact(4)
                .all(|word| word == [0xA1, 0xB2, 0xC3, 0xD4])
        );
    }
    Ok(())
}

#[test]
fn fill_size_alignment_and_pattern_classes() -> ireert::Result<()> {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let buffer = allocate_transfer_buffer(&device, 128)?;
        let mut expected = vec![0; 128];
        for (offset, length, pattern) in [
            (0usize, 64usize, &[0x11][..]),
            (2, 30, &[0x22, 0x33][..]),
            (4, 60, &[0x44, 0x55, 0x66, 0x77][..]),
        ] {
            let signal = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
            device.queue_fill(
                &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 1)]),
                BufferBinding::new(&buffer, offset, length),
                pattern,
            )?;
            await_signal(&device, &signal)?;
            for (index, byte) in expected[offset..offset + length].iter_mut().enumerate() {
                *byte = pattern[index % pattern.len()];
            }
        }
        let mut actual = vec![0; 128];
        // SAFETY: allocation initializes the full buffer; the fills preserve that.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }?;
        assert_eq!(actual, expected);
    }
    Ok(())
}

#[test]
fn chained_fill_copy_fill() -> ireert::Result<()> {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let source = allocate_transfer_buffer(&device, 64)?;
        let target = allocate_transfer_buffer(&device, 64)?;
        let fill = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        let copy = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        let finish = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_fill(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&fill, 1)]),
            BufferBinding::new(&source, 0, 64),
            &[0x5A],
        )?;
        device.queue_copy(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([signal_point(&fill, 1)])
                .with_signals([signal_point(&copy, 1)]),
            BufferBinding::new(&source, 0, 64),
            BufferBinding::new(&target, 0, 64),
        )?;
        device.queue_fill(
            &QueueSubmission::new(QueueAffinity::ANY)
                .with_waits([signal_point(&copy, 1)])
                .with_signals([signal_point(&finish, 1)]),
            BufferBinding::new(&target, 0, 64),
            &[0xA5],
        )?;
        await_signal(&device, &finish)?;
        let mut actual = vec![0; 64];
        // SAFETY: allocation initializes the full buffer; the queued commands preserve that.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }?;
        assert!(actual.iter().all(|byte| *byte == 0xA5));
    }
    Ok(())
}

#[test]
fn barrier_signals() -> ireert::Result<()> {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let signal = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_barrier(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&signal, 3)]),
            ExecuteFlags::default(),
        )?;
        signal.wait(3, None, WaitFlags::YIELD)?;
        assert_eq!(signal.query()?, 3);
    }
    Ok(())
}

#[test]
fn no_wait_semaphores() -> ireert::Result<()> {
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let called = Arc::new(AtomicBool::new(false));
        let called_in_callback = called.clone();
        let completion = Semaphore::new(&device, QueueAffinity::ANY, 0)?;
        device.queue_host_call(
            &QueueSubmission::new(QueueAffinity::ANY).with_signals([signal_point(&completion, 1)]),
            HostCallFlags::default(),
            move |_| {
                called_in_callback.store(true, Ordering::SeqCst);
                Ok(())
            },
        )?;
        await_signal(&device, &completion)?;
        assert!(called.load(Ordering::SeqCst));
    }
    Ok(())
}

#[test]
fn safe_queue_execute_rejects_borrowed_binding_lifetimes() -> ireert::Result<()> {
    use ireert::hal::{CommandBuffer, CommandBufferMode, CommandCategory};
    let devices = available_devices()?;
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let buffer = allocate_transfer_buffer(&device, 16)?;
        let command_buffer = CommandBuffer::create(
            &device,
            CommandBufferMode::ONE_SHOT,
            CommandCategory::TRANSFER,
            QueueAffinity::ANY,
            1,
        )?;
        let result = device.queue_execute(
            &QueueSubmission::new(QueueAffinity::ANY),
            &command_buffer,
            &[ireert::hal::BufferBinding {
                buffer: &buffer,
                offset: 0,
                length: 16,
            }],
            ireert::hal::ExecuteFlags::BORROW_BINDING_TABLE_LIFETIME,
        );
        assert!(matches!(result, Err(error) if error.code() == ireert::Code::InvalidArgument));
    }
    Ok(())
}
