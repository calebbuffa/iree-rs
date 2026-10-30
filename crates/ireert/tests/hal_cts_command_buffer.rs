//! Rust ports of representative upstream HAL command-buffer CTS cases.

mod support {
    #[path = "cts_compile.rs"]
    pub mod cts_compile;
    #[path = "cts_exec.rs"]
    pub mod cts_exec;
}

use ireert::hal::{
    CommandBuffer, CommandBufferMode, CommandCategory, QueueAffinity, QueueSubmission,
    SemaphorePoint,
};
use support::cts_exec::{allocate_transfer_buffer, available_devices};

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

fn make_command_buffer(device: &ireert::hal::Device) -> ireert::Result<CommandBuffer> {
    CommandBuffer::create(
        device,
        CommandBufferMode::ONE_SHOT,
        CommandCategory::ANY,
        QueueAffinity::ANY,
        0,
    )
}

fn submit_and_wait(
    device: &ireert::hal::GroupedDevice,
    command_buffer: &CommandBuffer,
) -> ireert::Result<()> {
    let completion = ireert::hal::Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_execute(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
            semaphore: &completion,
            value: 1,
        }]),
        command_buffer,
        &[],
        ireert::hal::ExecuteFlags::default(),
    )?;
    completion.wait(1, None, ireert::hal::WaitFlags::YIELD)
}

on_each_device!(create, |_, device: &ireert::hal::GroupedDevice| {
    let command_buffer = make_command_buffer(device)?;
    assert!(
        command_buffer
            .allowed_categories()
            .contains(CommandCategory::DISPATCH)
    );
});

on_each_device!(begin_end, |_, device: &ireert::hal::GroupedDevice| {
    let mut command_buffer = make_command_buffer(device)?;
    command_buffer.begin().expect("begin recording");
    command_buffer.end().expect("end recording");
});

on_each_device!(submit_empty, |_, device: &ireert::hal::GroupedDevice| {
    let mut command_buffer = make_command_buffer(device)?;
    command_buffer.begin().expect("begin recording");
    command_buffer.end().expect("end recording");
    submit_and_wait(device, &command_buffer)?;
});

on_each_device!(fill_buffer, |_, device: &ireert::hal::GroupedDevice| {
    let target = allocate_transfer_buffer(device, 32).expect("allocate target");
    let mut command_buffer = make_command_buffer(device)?;
    command_buffer.begin().expect("begin recording");
    command_buffer
        .fill_buffer(&target, 4, 20, &[0xA5, 0x5A, 0xC3, 0x3C])
        .expect("record fill");
    command_buffer.end().expect("end recording");
    submit_and_wait(device, &command_buffer)?;
    let mut actual = vec![0xFF; 32];
    // SAFETY: allocation initializes the full target before the fill.
    unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
    let mut expected = vec![0; 32];
    for (index, byte) in expected[4..24].iter_mut().enumerate() {
        *byte = [0xA5, 0x5A, 0xC3, 0x3C][index % 4];
    }
    assert_eq!(actual, expected);
});

on_each_device!(with_offsets, |_, device: &ireert::hal::GroupedDevice| {
    let target = allocate_transfer_buffer(device, 16).expect("allocate target");
    let mut command_buffer = make_command_buffer(device)?;
    let source: Vec<u8> = (1..=16).collect();
    command_buffer.begin().expect("begin recording");
    command_buffer
        .update_buffer(&source, 4, &target, 4, 8)
        .expect("record update");
    command_buffer.end().expect("end recording");
    submit_and_wait(device, &command_buffer)?;
    let mut actual = vec![0xFF; 16];
    // SAFETY: allocation initializes the full target before the update.
    unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
    assert_eq!(&actual[4..12], &source[4..12]);
    assert!(actual[..4].iter().all(|byte| *byte == 0));
    assert!(actual[12..].iter().all(|byte| *byte == 0));
});

on_each_device!(copy_buffer, |_, device: &ireert::hal::GroupedDevice| {
    let source = allocate_transfer_buffer(device, 16).expect("allocate source");
    let target = allocate_transfer_buffer(device, 16).expect("allocate target");
    device
        .transfer_h2d(&[0, 1, 2, 3, 4, 5, 6, 7], &source, 3, None)
        .expect("initialize source");
    let mut command_buffer = make_command_buffer(device)?;
    command_buffer.begin().expect("begin recording");
    command_buffer
        .copy_buffer(&source, 3, &target, 4, 8)
        .expect("record copy");
    command_buffer.end().expect("end recording");
    submit_and_wait(device, &command_buffer)?;
    let mut actual = vec![0xFF; 16];
    // SAFETY: both buffers are initialized before the recorded copy.
    unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read target");
    assert_eq!(&actual[4..12], &[0, 1, 2, 3, 4, 5, 6, 7]);
});

#[test]
fn compile_upstream_dispatch_testdata_for_each_backend() {
    for (driver, device) in available_devices().expect("enumerate usable backends") {
        let artifact = support::cts_compile::compile_cts_executable(
            &device,
            &driver,
            "command_buffer_dispatch_test.mlir",
        )
        .expect("compile upstream dispatch testdata");
        if let Some((_, bytes)) = artifact {
            assert!(!bytes.is_empty());
        } else {
            eprintln!("skipping {driver}: no supported executable format");
        }
    }
}

fn assert_recorded_fill(
    device: &ireert::hal::GroupedDevice,
    buffer_size: usize,
    offset: usize,
    length: usize,
    pattern: &[u8],
) -> ireert::Result<()> {
    let target = allocate_transfer_buffer(device, buffer_size)?;
    let mut command_buffer = make_command_buffer(device)?;
    command_buffer.begin()?;
    command_buffer.fill_buffer(&target, offset, length, pattern)?;
    command_buffer.end()?;
    submit_and_wait(device, &command_buffer)?;
    let mut actual = vec![0xFF; buffer_size];
    // SAFETY: allocation initializes the full target before the fill.
    unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }?;
    let mut expected = vec![0; buffer_size];
    for (index, byte) in expected[offset..offset + length].iter_mut().enumerate() {
        *byte = pattern[index % pattern.len()];
    }
    assert_eq!(actual, expected);
    Ok(())
}

macro_rules! recorded_fill_case {
    ($name:ident, $size:expr, $offset:expr, $length:expr, $pattern:expr) => {
        on_each_device!($name, |_, device: &ireert::hal::GroupedDevice| {
            assert_recorded_fill(device, $size, $offset, $length, &$pattern)?;
        });
    };
}

recorded_fill_case!(pattern1_size1_offset0_length1, 1, 0, 1, [0x01]);
recorded_fill_case!(pattern1_size5_offset0_length5, 5, 0, 5, [0x01]);
recorded_fill_case!(pattern1_size16_offset0_length1, 16, 0, 1, [0x01]);
recorded_fill_case!(pattern1_size16_offset0_length3, 16, 0, 3, [0x01]);
recorded_fill_case!(pattern1_size16_offset0_length8, 16, 0, 8, [0x01]);
recorded_fill_case!(pattern1_size16_offset2_length8, 16, 2, 8, [0x01]);
recorded_fill_case!(pattern2_size2_offset0_length2, 2, 0, 2, [0x02, 0x03]);
recorded_fill_case!(pattern2_size16_offset0_length8, 16, 0, 8, [0x02, 0x03]);
recorded_fill_case!(pattern2_size16_offset0_length10, 16, 0, 10, [0x02, 0x03]);
recorded_fill_case!(pattern2_size16_offset2_length8, 16, 2, 8, [0x02, 0x03]);
recorded_fill_case!(
    pattern4_size4_offset0_length4,
    4,
    0,
    4,
    [0x04, 0x05, 0x06, 0x07]
);
recorded_fill_case!(
    pattern4_size4_offset16_length4,
    20,
    16,
    4,
    [0x04, 0x05, 0x06, 0x07]
);
recorded_fill_case!(
    pattern4_size16_offset0_length8,
    16,
    0,
    8,
    [0x04, 0x05, 0x06, 0x07]
);
recorded_fill_case!(
    pattern4_size16_offset8_length8,
    16,
    8,
    8,
    [0x04, 0x05, 0x06, 0x07]
);

#[test]
fn fill_size_alignment_and_pattern_classes() -> ireert::Result<()> {
    let devices = available_devices().expect("enumerate usable backends");
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        for (size, offset, length, pattern) in [
            (64, 0, 64, &[0xA1][..]),
            (64, 4, 28, &[0xA2, 0xA3][..]),
            (64, 8, 32, &[0xA4, 0xA5, 0xA6, 0xA7][..]),
        ] {
            assert_recorded_fill(&device, size, offset, length, pattern)?;
        }
    }
    Ok(())
}

#[test]
fn copy_whole_buffer() -> ireert::Result<()> {
    let devices = available_devices().expect("enumerate usable backends");
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        let source = allocate_transfer_buffer(&device, 64).expect("allocate source");
        let target = allocate_transfer_buffer(&device, 64).expect("allocate target");
        let expected: Vec<u8> = (0..64).map(|i| (i * 13 + 7) as u8).collect();
        device
            .transfer_h2d(&expected, &source, 0, None)
            .expect("initialize source");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .copy_buffer(&source, 0, &target, 0, 64)
            .expect("record copy");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 64];
        // SAFETY: both buffers are initialized before the recorded copy.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read target");
        assert_eq!(actual, expected);
    }
    Ok(())
}

#[test]
fn copy_size_and_alignment_classes() -> ireert::Result<()> {
    let devices = available_devices().expect("enumerate usable backends");
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (_, device) in devices {
        for (size, source_offset, target_offset, length) in
            [(32, 0, 0, 1), (64, 3, 7, 17), (128, 16, 32, 64)]
        {
            let source = allocate_transfer_buffer(&device, size).expect("allocate source");
            let target = allocate_transfer_buffer(&device, size).expect("allocate target");
            let expected: Vec<u8> = (0..length).map(|i| (i * 17 + 3) as u8).collect();
            device
                .transfer_h2d(&expected, &source, source_offset, None)
                .expect("initialize source");
            let mut command_buffer = make_command_buffer(&device)?;
            command_buffer.begin().expect("begin recording");
            command_buffer
                .copy_buffer(&source, source_offset, &target, target_offset, length)
                .expect("record copy");
            command_buffer.end().expect("end recording");
            submit_and_wait(&device, &command_buffer)?;
            let mut actual = vec![0; size];
            // SAFETY: allocation initializes the full target before the copy.
            unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read target");
            assert_eq!(
                &actual[target_offset..target_offset + length],
                expected.as_slice()
            );
        }
    }
    Ok(())
}

#[test]
fn whole_buffer() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let source: Vec<u8> = (0..16).map(|i| (i + 1) as u8).collect();
        let target = allocate_transfer_buffer(&device, 16).expect("allocate target");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .update_buffer(&source, 0, &target, 0, 16)
            .expect("record update");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 16];
        // SAFETY: allocation initializes the full target before the update.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
        assert_eq!(actual, source);
    }
    Ok(())
}

#[test]
fn larger_payload() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let source: Vec<u8> = (0..256).map(|i| (i ^ 0xA5) as u8).collect();
        let target = allocate_transfer_buffer(&device, 256).expect("allocate target");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .update_buffer(&source, 0, &target, 0, 256)
            .expect("record update");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 256];
        // SAFETY: allocation initializes the full target before the update.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
        assert_eq!(actual, source);
    }
    Ok(())
}

#[test]
fn subspan() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let target = allocate_transfer_buffer(&device, 16).expect("allocate target");
        let subspan = target.subspan(4, 8).expect("create target subspan");
        let source: Vec<u8> = (1..=16).collect();
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .update_buffer(&source, 4, &subspan, 4, 4)
            .expect("record update");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0xFF; 16];
        // SAFETY: allocation initializes the full target before the update.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
        assert_eq!(actual, [0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0]);
        let mut subspan_data = vec![0xFF; 8];
        // SAFETY: the source subspan contains bytes from the initialized target.
        unsafe { device.transfer_d2h(&subspan, 0, &mut subspan_data, None) }.expect("read subspan");
        assert_eq!(subspan_data, [0, 0, 0, 0, 5, 6, 7, 8]);
    }
    Ok(())
}

fn allocate_queue_buffer(
    device: &ireert::hal::GroupedDevice,
    size: usize,
    access: ireert::hal::MemoryAccess,
) -> ireert::Result<ireert::hal::Buffer> {
    use ireert::hal::{AllocaFlags, BufferParams, BufferUsage, MemoryType, Semaphore};
    let committed = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    let buffer = device.queue_alloca(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
            semaphore: &committed,
            value: 1,
        }]),
        None,
        BufferParams {
            usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE,
            access,
            memory_type: MemoryType::OPTIMAL_FOR_DEVICE,
            ..BufferParams::default()
        },
        size,
        AllocaFlags::default(),
    )?;
    committed.wait(1, None, ireert::hal::WaitFlags::YIELD)?;
    Ok(buffer)
}

fn deallocate_queue_buffer(
    device: &ireert::hal::GroupedDevice,
    buffer: &ireert::hal::Buffer,
) -> ireert::Result<()> {
    use ireert::hal::DeallocaFlags;
    let released = ireert::hal::Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_dealloca(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
            semaphore: &released,
            value: 1,
        }]),
        buffer,
        DeallocaFlags::default(),
    )?;
    released.wait(1, None, ireert::hal::WaitFlags::YIELD)
}

#[test]
fn fill_transient_buffer() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let buffer = allocate_queue_buffer(&device, 64, ireert::hal::MemoryAccess::ALL)?;
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer(&buffer, 0, 64, &[0xA1, 0xB2, 0xC3, 0xD4])
            .expect("record fill");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 64];
        // SAFETY: the command buffer fills the entire transfer range.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }.expect("read buffer");
        assert!(
            actual
                .chunks_exact(4)
                .all(|word| word == [0xA1, 0xB2, 0xC3, 0xD4])
        );
        deallocate_queue_buffer(&device, &buffer)?;
    }
    Ok(())
}

#[test]
fn fill_transient_buffer_subrange() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let buffer = allocate_queue_buffer(&device, 64, ireert::hal::MemoryAccess::ALL)?;
        device
            .transfer_h2d(&[0; 64], &buffer, 0, None)
            .expect("initialize buffer");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer(&buffer, 16, 32, &[0x5A])
            .expect("record fill");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0xFF; 64];
        // SAFETY: transfer_h2d initializes the range before the fill.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }.expect("read buffer");
        assert!(actual[..16].iter().all(|byte| *byte == 0));
        assert!(actual[16..48].iter().all(|byte| *byte == 0x5A));
        assert!(actual[48..].iter().all(|byte| *byte == 0));
        deallocate_queue_buffer(&device, &buffer)?;
    }
    Ok(())
}

#[test]
fn fill_transient_buffer_1_byte() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let buffer = allocate_queue_buffer(&device, 16, ireert::hal::MemoryAccess::ALL)?;
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer(&buffer, 0, 16, &[0xAB])
            .expect("record fill");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 16];
        // SAFETY: the command buffer fills the entire transfer range.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, None) }.expect("read buffer");
        assert!(actual.iter().all(|byte| *byte == 0xAB));
        deallocate_queue_buffer(&device, &buffer)?;
    }
    Ok(())
}

#[test]
fn copy_to_transient_buffer() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let source = allocate_transfer_buffer(&device, 64).expect("allocate source");
        let target = allocate_queue_buffer(&device, 64, ireert::hal::MemoryAccess::ALL)?;
        let expected: Vec<u8> = (0..64).map(|i| (i * 3 + 1) as u8).collect();
        device
            .transfer_h2d(&expected, &source, 0, None)
            .expect("initialize source");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .copy_buffer(&source, 0, &target, 0, 64)
            .expect("record copy");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 64];
        // SAFETY: the command buffer copies every initialized source byte.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read target");
        assert_eq!(actual, expected);
        deallocate_queue_buffer(&device, &target)?;
    }
    Ok(())
}

#[test]
fn copy_from_transient_buffer() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let source = allocate_queue_buffer(&device, 64, ireert::hal::MemoryAccess::ALL)?;
        let target = allocate_transfer_buffer(&device, 64).expect("allocate target");
        let expected = [0xDE, 0xAD, 0xBE, 0xEF];
        let mut fill = make_command_buffer(&device)?;
        fill.begin().expect("begin recording");
        fill.fill_buffer(&source, 0, 64, &expected)
            .expect("record fill");
        fill.end().expect("end recording");
        submit_and_wait(&device, &fill)?;
        let mut copy = make_command_buffer(&device)?;
        copy.begin().expect("begin recording");
        copy.copy_buffer(&source, 0, &target, 0, 64)
            .expect("record copy");
        copy.end().expect("end recording");
        submit_and_wait(&device, &copy)?;
        let mut actual = vec![0; 64];
        // SAFETY: the command buffer copies every byte from the fully filled source.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read target");
        assert!(actual.chunks_exact(4).all(|word| word == expected));
        deallocate_queue_buffer(&device, &source)?;
    }
    Ok(())
}

#[test]
fn fill_then_copy_in_single_command_buffer() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let transient = allocate_queue_buffer(&device, 64, ireert::hal::MemoryAccess::ALL)?;
        let output = allocate_transfer_buffer(&device, 64).expect("allocate output");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer(&transient, 0, 64, &[0x3C])
            .expect("record fill");
        command_buffer
            .copy_buffer(&transient, 0, &output, 0, 64)
            .expect("record copy");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 64];
        // SAFETY: the command buffer fills the entire transient before copying it.
        unsafe { device.transfer_d2h(&output, 0, &mut actual, None) }.expect("read output");
        assert!(actual.iter().all(|byte| *byte == 0x3C));
        deallocate_queue_buffer(&device, &transient)?;
    }
    Ok(())
}

#[test]
fn update_transient_buffer() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let target = allocate_queue_buffer(&device, 16, ireert::hal::MemoryAccess::ALL)?;
        let source: Vec<u8> = (0..16).map(|i| (i * 5 + 2) as u8).collect();
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .update_buffer(&source, 0, &target, 0, 16)
            .expect("record update");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        let mut actual = vec![0; 16];
        // SAFETY: the command buffer updates the entire target range.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
        assert_eq!(actual, source);
        deallocate_queue_buffer(&device, &target)?;
    }
    Ok(())
}

#[test]
fn fill_transient_with_zero_access_flags() -> ireert::Result<()> {
    for (_, device) in available_devices().expect("enumerate usable backends") {
        let target = allocate_queue_buffer(&device, 16, ireert::hal::MemoryAccess::default())?;
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer(&target, 0, 16, &[0x12, 0x34, 0x56, 0x78])
            .expect("record fill");
        command_buffer.end().expect("end recording");
        submit_and_wait(&device, &command_buffer)?;
        deallocate_queue_buffer(&device, &target)?;
    }
    Ok(())
}

#[test]
fn static_ref_recorded_before_alloca_commit_executes() -> ireert::Result<()> {
    use ireert::hal::{
        AllocaFlags, BufferParams, BufferUsage, MemoryAccess, MemoryType, Semaphore,
    };
    let devices = available_devices().expect("enumerate usable backends");
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (driver, device) in devices {
        // Upstream tags this suite async_queue, excluding local-sync.
        if driver == "local-sync" {
            continue;
        }
        let allocation_done =
            Semaphore::new(&device, QueueAffinity::ANY, 0).expect("create semaphore");
        let target = device
            .queue_alloca(
                &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
                    semaphore: &allocation_done,
                    value: 1,
                }]),
                None,
                BufferParams {
                    usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE,
                    access: MemoryAccess::ALL,
                    memory_type: MemoryType::OPTIMAL_FOR_DEVICE,
                    ..BufferParams::default()
                },
                16,
                AllocaFlags::default(),
            )
            .expect("queue allocate");
        let mut command_buffer = make_command_buffer(&device)?;
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer(&target, 0, 16, &[0xA5])
            .expect("record direct reference");
        command_buffer.end().expect("end recording");
        let complete = Semaphore::new(&device, QueueAffinity::ANY, 0).expect("create semaphore");
        device
            .queue_execute(
                &QueueSubmission::new(QueueAffinity::ANY)
                    .with_waits([SemaphorePoint {
                        semaphore: &allocation_done,
                        value: 1,
                    }])
                    .with_signals([SemaphorePoint {
                        semaphore: &complete,
                        value: 1,
                    }]),
                &command_buffer,
                &[],
                ireert::hal::ExecuteFlags::default(),
            )
            .expect("submit command buffer");
        complete
            .wait(1, None, ireert::hal::WaitFlags::YIELD)
            .expect("wait for execution");
        let mut actual = vec![0; 16];
        // SAFETY: the command buffer fills the entire target range.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
        assert!(actual.iter().all(|byte| *byte == 0xA5));
        deallocate_queue_buffer(&device, &target)?;
    }
    Ok(())
}

#[test]
fn indirect_ref_resolved_after_alloca_commit() -> ireert::Result<()> {
    use ireert::hal::{
        AllocaFlags, BufferBinding, BufferParams, BufferUsage, CommandCategory, MemoryAccess,
        MemoryType, Semaphore,
    };
    let devices = available_devices().expect("enumerate usable backends");
    assert!(!devices.is_empty(), "no usable HAL backend available");
    for (driver, device) in devices {
        // Upstream tags this suite async_queue, excluding local-sync.
        if driver == "local-sync" {
            continue;
        }
        let allocation_done =
            Semaphore::new(&device, QueueAffinity::ANY, 0).expect("create semaphore");
        let target = device
            .queue_alloca(
                &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
                    semaphore: &allocation_done,
                    value: 1,
                }]),
                None,
                BufferParams {
                    usage: BufferUsage::TRANSFER | BufferUsage::DISPATCH_STORAGE,
                    access: MemoryAccess::ALL,
                    memory_type: MemoryType::OPTIMAL_FOR_DEVICE,
                    ..BufferParams::default()
                },
                16,
                AllocaFlags::default(),
            )
            .expect("queue allocate");
        let mut command_buffer = CommandBuffer::create(
            &device,
            CommandBufferMode::ONE_SHOT,
            CommandCategory::TRANSFER,
            QueueAffinity::ANY,
            1,
        )
        .expect("create command buffer");
        command_buffer.begin().expect("begin recording");
        command_buffer
            .fill_buffer_ref(ireert::hal::BufferRef::indirect(0, 0, 16), &[0x5A])
            .expect("record indirect reference");
        command_buffer.end().expect("end recording");
        let complete = Semaphore::new(&device, QueueAffinity::ANY, 0).expect("create semaphore");
        device
            .queue_execute(
                &QueueSubmission::new(QueueAffinity::ANY)
                    .with_waits([SemaphorePoint {
                        semaphore: &allocation_done,
                        value: 1,
                    }])
                    .with_signals([SemaphorePoint {
                        semaphore: &complete,
                        value: 1,
                    }]),
                &command_buffer,
                &[BufferBinding {
                    buffer: &target,
                    offset: 0,
                    length: 16,
                }],
                ireert::hal::ExecuteFlags::default(),
            )
            .expect("submit command buffer");
        complete
            .wait(1, None, ireert::hal::WaitFlags::YIELD)
            .expect("wait for execution");
        let mut actual = vec![0; 16];
        // SAFETY: the command buffer fills the entire target range.
        unsafe { device.transfer_d2h(&target, 0, &mut actual, None) }.expect("read buffer");
        assert!(actual.iter().all(|byte| *byte == 0x5A));
        deallocate_queue_buffer(&device, &target)?;
    }
    Ok(())
}
