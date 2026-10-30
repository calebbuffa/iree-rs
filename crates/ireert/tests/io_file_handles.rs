//! File-content integration cases mapped onto the available file-handle APIs.

use ireert::io::{FileAccess, FileHandle, FileMapping, FileMode, MappingFlags, Stream, StreamMode};
use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

struct TestFile(PathBuf);
impl TestFile {
    fn new() -> std::io::Result<Self> {
        let path = std::env::current_dir()?.join(format!(
            "ireert-io-contents-{}-{}.bin",
            std::process::id(),
            FILE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        Ok(Self(path))
    }
    fn path(&self) -> std::io::Result<&str> {
        self.0.to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "test path is not UTF-8")
        })
    }
}
impl Drop for TestFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
static FILE_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[test]
fn stream_permissions_cannot_exceed_handle_permissions() -> ireert::Result<()> {
    for access in [
        FileAccess::READ,
        FileAccess::WRITE,
        FileAccess::READ | FileAccess::WRITE,
    ] {
        let handle = FileHandle::from_bytes(vec![0; 8].into_boxed_slice(), access)?;
        for mode in [
            StreamMode::READABLE,
            StreamMode::WRITABLE,
            StreamMode::READABLE | StreamMode::WRITABLE,
        ] {
            let allowed = (!mode.contains(StreamMode::READABLE)
                || access.contains(FileAccess::READ))
                && (!mode.contains(StreamMode::WRITABLE) || access.contains(FileAccess::WRITE));
            let result = Stream::open(&handle, mode, 0);
            assert_eq!(result.is_ok(), allowed);
            if let Err(error) = result {
                assert_eq!(error.code(), ireert::Code::PermissionDenied);
            }
        }
    }
    let immutable = FileHandle::from_owner(b"immutable")?;
    assert!(matches!(
        Stream::open(&immutable, StreamMode::WRITABLE, 0),
        Err(error) if error.code() == ireert::Code::PermissionDenied
    ));
    assert!(matches!(
        ireert::hal::profiling::file_sink(&immutable),
        Err(error) if error.code() == ireert::Code::PermissionDenied
    ));
    Ok(())
}

fn write_contents(path: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let handle = FileHandle::create(
        path,
        FileMode::READ | FileMode::WRITE | FileMode::OVERWRITE | FileMode::SHARE_READ,
        data.len() as u64,
    )?;
    let mode = StreamMode::from_bits_retain(
        StreamMode::READABLE.bits() | StreamMode::WRITABLE.bits() | StreamMode::SEEKABLE.bits(),
    );
    let mut stream = Stream::open(&handle, mode, 0)?;
    stream.write_all(data)?;
    handle.flush()?;
    Ok(())
}

#[test]
fn write_read_and_preload_contents() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    let data = b"file contents read/write preload";
    write_contents(file.path()?, data)?;
    let handle = FileHandle::open(file.path()?, FileMode::READ | FileMode::SHARE_READ)?;
    let mode =
        StreamMode::from_bits_retain(StreamMode::READABLE.bits() | StreamMode::SEEKABLE.bits());
    let mut stream = Stream::open(&handle, mode, 0)?;
    let mut actual = vec![0; data.len()];
    stream.read_exact(&mut actual)?;
    assert_eq!(actual, data);

    let preloaded = FileHandle::preload(file.path()?, FileMode::READ)?;
    assert!(preloaded.access().contains(FileAccess::READ));
    // SAFETY: the preloaded read-only allocation has no mutable aliases.
    let mapping = unsafe { FileMapping::map(&preloaded, 0, None, MappingFlags::default()) }?;
    assert_eq!(
        mapping
            .as_bytes()
            .ok_or("preloaded mapping is not readable")?,
        data
    );
    Ok(())
}

#[test]
fn concurrent_read_handles_observe_same_contents() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    let data = b"concurrent read opens";
    write_contents(file.path()?, data)?;
    let first = FileHandle::open(file.path()?, FileMode::READ | FileMode::SHARE_READ)?;
    let second = FileHandle::open(file.path()?, FileMode::READ | FileMode::SHARE_READ)?;
    let mode =
        StreamMode::from_bits_retain(StreamMode::READABLE.bits() | StreamMode::SEEKABLE.bits());
    let mut first_stream = Stream::open(&first, mode, 0)?;
    let mut second_stream = Stream::open(&second, mode, 0)?;
    let mut first_contents = vec![0; data.len()];
    let mut second_contents = vec![0; data.len()];
    first_stream.read_exact(&mut first_contents)?;
    second_stream.seek(SeekFrom::Start(0))?;
    second_stream.read_exact(&mut second_contents)?;
    assert_eq!(first_contents, data);
    assert_eq!(second_contents, data);
    Ok(())
}

#[test]
fn write_only_mapping_cannot_be_read_through_safe_api() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    let data = b"write-only mapping";
    write_contents(file.path()?, data)?;
    let handle = FileHandle::open(
        file.path()?,
        FileMode::READ | FileMode::WRITE | FileMode::SHARE_READ | FileMode::SHARE_WRITE,
    )?;
    // SAFETY: no other handle or process mutates this file while the mapping is live.
    let mut mapping = unsafe {
        FileMapping::map_with_access(
            &handle,
            FileAccess::WRITE,
            0,
            Some(data.len()),
            MappingFlags::default(),
        )
    }?;
    assert!(mapping.as_bytes().is_none());
    mapping
        .as_mut_slice()
        .ok_or("write-only mapping is not writable")?[0] = b'W';
    drop(mapping);

    let mode = StreamMode::from_bits_retain(StreamMode::READABLE.bits());
    let mut stream = Stream::open(&handle, mode, 0)?;
    let mut actual = vec![0; data.len()];
    stream.read_exact(&mut actual)?;
    assert_eq!(actual[0], b'W');
    Ok(())
}
