//! HAL file CTS ports.

#[path = "support/cts.rs"]
mod cts;
#[path = "support/cts_buffer.rs"]
mod cts_buffer;

use cts::devices;
use cts_buffer::{buffer, read};
use ireert::{
    Code,
    hal::{BufferBinding, FileRange, MemoryAccess, QueueAffinity, QueueSubmission, file::File},
    io::{FileAccess, FileHandle, FileMode, Stream, StreamMode},
};
use std::{
    fs,
    io::Read,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_FILE: AtomicU64 = AtomicU64::new(0);

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn deterministic_bytes(length: usize) -> Vec<u8> {
    (0..length)
        .map(|i| (i.wrapping_mul(37).wrapping_add(11) % 256) as u8)
        .collect()
}

struct TestPath(PathBuf);
impl Drop for TestPath {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn with_provider<R>(
    device: &ireert::hal::GroupedDevice,
    native: bool,
    access: MemoryAccess,
    initial: &[u8],
    operation: impl FnOnce(&ireert::hal::GroupedDevice, &File<'static>) -> TestResult<R>,
) -> TestResult<Option<(R, Vec<u8>)>> {
    if !native {
        let handle = FileHandle::from_bytes(
            initial.to_vec().into_boxed_slice(),
            FileAccess::READ | FileAccess::WRITE,
        )?;
        let file = File::import(device, QueueAffinity::ANY, access, &handle)?;
        let result = operation(device, &file)?;
        drop(file);
        let mut backing = vec![0; initial.len()];
        Stream::open(&handle, StreamMode::READABLE, 0)?.read_exact(&mut backing)?;
        return Ok(Some((result, backing)));
    }

    let path = TestPath(std::env::current_dir()?.join(format!(
        "iree-hal-cts-{}-{}.bin",
        std::process::id(),
        NEXT_FILE.fetch_add(1, Ordering::Relaxed)
    )));
    fs::write(&path.0, initial)?;
    let path_text = path.0.to_str().ok_or("HAL CTS test path is not UTF-8")?;
    let handle = match FileHandle::open(
        path_text,
        FileMode::READ
            | FileMode::WRITE
            | FileMode::RANDOM_ACCESS
            | FileMode::SHARE_READ
            | FileMode::SHARE_WRITE
            | FileMode::ASYNC,
    ) {
        Ok(handle) => handle,
        Err(error) if matches!(error.code(), Code::Unavailable | Code::Unimplemented) => {
            eprintln!("skipping native_file provider: {error}");
            return Ok(None);
        }
        Err(error) => return Err(error.into()),
    };
    let file = match File::import(device, QueueAffinity::ANY, access, &handle) {
        Ok(file) => file,
        Err(error) if matches!(error.code(), Code::Unavailable | Code::Unimplemented) => {
            // Upstream FileTest skips the native provider only for UNAVAILABLE or UNIMPLEMENTED.
            eprintln!("skipping native_file provider: {error}");
            return Ok(None);
        }
        Err(error) => return Err(error.into()),
    };
    let result = operation(device, &file)?;
    drop(file);
    drop(handle);
    let contents = fs::read(&path.0)?;
    Ok(Some((result, contents)))
}

fn read_file(
    device: &ireert::hal::GroupedDevice,
    file: &File<'static>,
    file_offset: u64,
    buffer: &ireert::hal::Buffer,
    buffer_offset: ireert::hal::DeviceSize,
    length: ireert::hal::DeviceSize,
) -> ireert::Result<()> {
    let transfer = device.queue_read(
        &QueueSubmission::new(QueueAffinity::ANY),
        file,
        FileRange::new(file_offset, length),
        BufferBinding::new(buffer, buffer_offset, length),
    )?;
    transfer.wait()
}

fn write_file(
    device: &ireert::hal::GroupedDevice,
    buffer: &ireert::hal::Buffer,
    buffer_offset: ireert::hal::DeviceSize,
    file: &File<'static>,
    file_offset: u64,
    length: ireert::hal::DeviceSize,
) -> ireert::Result<()> {
    let transfer = device.queue_write(
        &QueueSubmission::new(QueueAffinity::ANY),
        BufferBinding::new(buffer, buffer_offset, length),
        file,
        FileRange::new(file_offset, length),
    )?;
    transfer.wait()
}

#[test]
fn read_entire_file() -> TestResult {
    for (_, _, device) in devices()? {
        for native in [false, true] {
            let initial = vec![0xde; 128];
            let Some((actual, contents)) =
                with_provider(&device, native, MemoryAccess::READ, &initial, |d, f| {
                    let b = buffer(d, 128)?;
                    read_file(d, f, 0, &b, 0, 128)?;
                    Ok(read(&b, 0, 128)?)
                })?
            else {
                continue;
            };
            assert_eq!(contents, initial);
            assert_eq!(actual, initial);
        }
    }
    Ok(())
}

#[test]
fn read_subrange_with_offsets() -> TestResult {
    for (_, _, device) in devices()? {
        for native in [false, true] {
            let initial = deterministic_bytes(512);
            let Some((actual, contents)) =
                with_provider(&device, native, MemoryAccess::READ, &initial, |d, f| {
                    let b = buffer(d, 384)?;
                    b.zero(0, 384)?;
                    read_file(d, f, 97, &b, 13, 211)?;
                    Ok(read(&b, 0, 384)?)
                })?
            else {
                continue;
            };
            assert_eq!(contents, initial);
            let mut expected = vec![0; 384];
            expected[13..224].copy_from_slice(&initial[97..308]);
            assert_eq!(actual, expected);
        }
    }
    Ok(())
}

#[test]
fn write_subrange_with_offsets() -> TestResult {
    for (_, _, device) in devices()? {
        for native in [false, true] {
            let initial = deterministic_bytes(512);
            let source = deterministic_bytes(384);
            let Some(((), actual)) = with_provider(
                &device,
                native,
                MemoryAccess::READ | MemoryAccess::WRITE,
                &initial,
                |d, f| {
                    let b = buffer(d, 384)?;
                    b.write(0, &source)?;
                    write_file(d, &b, 23, f, 41, 307)?;
                    Ok(())
                },
            )?
            else {
                continue;
            };
            let mut expected = initial;
            expected[41..348].copy_from_slice(&source[23..330]);
            assert_eq!(actual, expected);
        }
    }
    Ok(())
}

#[test]
fn chained_write_then_read_with_semaphores() -> TestResult {
    for (_, _, device) in devices()? {
        for native in [false, true] {
            let initial = vec![0; 768];
            let source = deterministic_bytes(512);
            let Some((actual, contents)) = with_provider(
                &device,
                native,
                MemoryAccess::READ | MemoryAccess::WRITE,
                &initial,
                |d, f| {
                    let source_buffer = buffer(d, 512)?;
                    source_buffer.write(0, &source)?;
                    let target = buffer(d, 512)?;
                    target.zero(0, 512)?;
                    let write = d.queue_write(
                        &QueueSubmission::new(QueueAffinity::ANY),
                        BufferBinding::new(&source_buffer, 19, 333),
                        f,
                        FileRange::new(123, 333),
                    )?;
                    let dependency = [write.completion_point()];
                    let read_transfer = d.queue_read(
                        &QueueSubmission::new(QueueAffinity::ANY).with_waits(dependency),
                        f,
                        FileRange::new(123, 333),
                        BufferBinding::new(&target, 37, 333),
                    )?;
                    read_transfer.wait()?;
                    write.wait()?;
                    Ok(read(&target, 0, 512)?)
                },
            )?
            else {
                continue;
            };
            let mut expected = vec![0; 512];
            expected[37..370].copy_from_slice(&source[19..352]);
            assert_eq!(actual, expected);
            let mut expected_file = vec![0; 768];
            expected_file[123..456].copy_from_slice(&source[19..352]);
            assert_eq!(contents, expected_file);
        }
    }
    Ok(())
}

#[test]
fn large_range_round_trip() -> TestResult {
    for (_, _, device) in devices()? {
        for native in [false, true] {
            let source = deterministic_bytes(256 * 1024 + 31);
            let initial = vec![0; source.len()];
            let Some((actual, contents)) = with_provider(
                &device,
                native,
                MemoryAccess::READ | MemoryAccess::WRITE,
                &initial,
                |d, f| {
                    let from = buffer(d, source.len())?;
                    from.write(0, &source)?;
                    let to = buffer(d, source.len())?;
                    write_file(d, &from, 0, f, 0, source.len())?;
                    read_file(d, f, 0, &to, 0, source.len())?;
                    Ok(read(&to, 0, source.len())?)
                },
            )?
            else {
                continue;
            };
            assert_eq!(actual, source);
            assert_eq!(contents, source);
        }
    }
    Ok(())
}
