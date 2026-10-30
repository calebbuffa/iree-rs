//! Ports of `file_contents_test.cc` against the RAII FileContents wrapper.

use ireert::io::{FileAccess, FileContents};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

struct TestFile(PathBuf);
impl TestFile {
    fn new() -> std::io::Result<Self> {
        Ok(Self(std::env::current_dir()?.join(format!(
            "ireert-file-contents-{}-{}.bin",
            std::process::id(),
            FILE_ID.fetch_add(1, Ordering::Relaxed)
        ))))
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
static FILE_ID: AtomicUsize = AtomicUsize::new(0);

fn fixture(name: &[u8], padded_size: usize) -> Vec<u8> {
    let mut bytes = format!("Test with name {}\n", String::from_utf8_lossy(name)).into_bytes();
    bytes.resize(bytes.len().max(padded_size), 0);
    bytes
}

#[test]
fn read_write_contents_preload() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    assert!(!file.0.exists());
    let expected = fixture(b"ReadWriteContents", 32);
    FileContents::write(file.path()?, &expected)?;
    let contents = FileContents::read(file.path()?)?;
    assert_eq!(contents.len(), expected.len());
    assert_eq!(
        contents.as_bytes().ok_or("contents are not readable")?,
        expected
    );
    Ok(())
}

#[test]
fn read_write_contents_mmap() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    assert!(!file.0.exists());
    let expected = fixture(b"ReadWriteContents", 4096);
    FileContents::write(file.path()?, &expected)?;
    // SAFETY: this test does not mutate the file or create another mapping with write access.
    let contents = unsafe { FileContents::map(file.path()?, FileAccess::READ) }?;
    assert_eq!(contents.len(), expected.len());
    assert_eq!(
        contents
            .as_bytes()
            .ok_or("mapped contents are not readable")?,
        expected
    );
    Ok(())
}

#[test]
fn concurrent_read_opens() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    assert!(!file.0.exists());
    let expected = fixture(b"ConcurrentReadOpens", 4096);
    FileContents::write(file.path()?, &expected)?;
    // SAFETY: both mappings are read-only and the file remains unchanged.
    let first = unsafe { FileContents::map(file.path()?, FileAccess::READ) }?;
    // SAFETY: same unchanged read-only backing file as the first mapping.
    let second = unsafe { FileContents::map(file.path()?, FileAccess::READ) }?;
    assert_eq!(first.len(), second.len());
    assert_eq!(
        first.as_bytes().ok_or("first mapping is not readable")?,
        second.as_bytes().ok_or("second mapping is not readable")?
    );
    Ok(())
}

#[test]
fn owned_contents_are_mutably_borrowable() -> Result<(), Box<dyn std::error::Error>> {
    let file = TestFile::new()?;
    let expected = fixture(b"mutable", 16);
    FileContents::write(file.path()?, &expected)?;
    let mut contents = FileContents::read(file.path()?)?;
    contents
        .as_mut_bytes()
        .ok_or("owned contents are not writable")?[0] = b'X';
    assert_eq!(
        contents
            .as_bytes()
            .ok_or("owned contents are not readable")?[0],
        b'X'
    );
    Ok(())
}
