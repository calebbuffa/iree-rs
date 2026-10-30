//! Upstream memory-stream contract tests (`memory_stream_test.cc`).
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::io::{Stream, StreamMode};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const READ: StreamMode =
    StreamMode::from_bits_retain(StreamMode::READABLE.bits() | StreamMode::SEEKABLE.bits());
const RW: StreamMode = StreamMode::from_bits_retain(
    StreamMode::READABLE.bits() | StreamMode::WRITABLE.bits() | StreamMode::SEEKABLE.bits(),
);
const RWM: StreamMode = StreamMode::from_bits_retain(RW.bits() | StreamMode::MAPPABLE.bits());

#[test]
fn wrap_and_empty() {
    let mut bytes = [0, 1, 2, 3, 4];
    let stream = Stream::memory(&mut bytes, READ).unwrap();
    assert!(stream.mode().contains(StreamMode::READABLE));
    assert_eq!(stream.offset(), 0);
    assert_eq!(stream.length(), 5);
    assert!(!stream.is_eos());
    drop(stream);

    let mut empty = [];
    let stream = Stream::memory(&mut empty, READ).unwrap();
    assert_eq!(stream.offset(), 0);
    assert_eq!(stream.length(), 0);
    assert!(stream.is_eos());
}

#[test]
fn owner_release_callback_runs_once() {
    struct Owner(Arc<AtomicUsize>, [u8; 1]);
    impl AsRef<[u8]> for Owner {
        fn as_ref(&self) -> &[u8] {
            &self.1
        }
    }
    impl Drop for Owner {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
    let drops = Arc::new(AtomicUsize::new(0));
    let stream = Stream::from_owner(Owner(drops.clone(), [0]), READ).unwrap();
    let clone_of_owner_probe = drops.clone();
    assert_eq!(clone_of_owner_probe.load(Ordering::SeqCst), 0);
    drop(stream);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn seek_modes_and_bounds() {
    let mut bytes = [0, 1, 2, 3, 4];
    let mut stream = Stream::memory(&mut bytes, READ).unwrap();
    assert_eq!(stream.seek(SeekFrom::Start(0)).unwrap(), 0);
    assert_eq!(stream.seek(SeekFrom::End(0)).unwrap(), 5);
    assert!(stream.is_eos());
    assert_eq!(stream.seek(SeekFrom::Start(1)).unwrap(), 1);
    assert_eq!(stream.seek(SeekFrom::Current(3)).unwrap(), 4);
    assert_eq!(stream.seek(SeekFrom::End(-1)).unwrap(), 4);
    assert!(stream.seek(SeekFrom::Start(6)).is_err());
    assert!(stream.seek(SeekFrom::Current(-100)).is_err());
    assert!(stream.seek(SeekFrom::End(1)).is_err());
}

#[test]
fn alignment_requires_power_of_two_and_stays_in_bounds() {
    let mut bytes = [0, 1, 2, 3, 4];
    let mut stream = Stream::memory(&mut bytes, READ).unwrap();
    for invalid in [3, 63] {
        assert!(stream.seek_to_alignment(invalid).is_err());
    }
    stream.seek_to_alignment(0).unwrap();
    stream.seek_to_alignment(1).unwrap();
    stream.seek_to_alignment(2).unwrap();
    stream.seek(SeekFrom::Start(1)).unwrap();
    stream.seek_to_alignment(2).unwrap();
    assert_eq!(stream.offset(), 2);
    stream.seek_to_alignment(4).unwrap();
    assert_eq!(stream.offset(), 4);
    assert!(stream.seek_to_alignment(16).is_err());
}

#[test]
fn reads_up_to_and_exactly() {
    let mut bytes = [0, 1, 2, 3, 4];
    let mut stream = Stream::memory(&mut bytes, READ).unwrap();
    let mut buffer = [0xDD; 64];
    assert_eq!(stream.read(&mut []).unwrap(), 0);
    assert_eq!(stream.offset(), 0);
    assert_eq!(stream.read(&mut buffer[..1]).unwrap(), 1);
    assert_eq!(&buffer[..2], &[0, 0xDD]);
    buffer.fill(0xDD);
    assert_eq!(stream.read(&mut buffer[..2]).unwrap(), 2);
    assert_eq!(&buffer[..3], &[1, 2, 0xDD]);
    buffer.fill(0xDD);
    assert_eq!(stream.read(&mut buffer).unwrap(), 2);
    assert_eq!(&buffer[..3], &[3, 4, 0xDD]);
    assert!(stream.is_eos());
    assert_eq!(stream.read(&mut buffer).unwrap(), 0);

    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut exact = [0; 5];
    stream.read_exact(&mut exact).unwrap();
    assert_eq!(exact, [0, 1, 2, 3, 4]);
    assert_eq!(stream.read(&mut buffer).unwrap(), 0);
    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut exact = [0; 6];
    assert!(stream.read_exact_atomic(&mut exact).is_err());
    assert_eq!(stream.offset(), 0);
    let mut exact = [0; 5];
    stream.read_exact_atomic(&mut exact).unwrap();
    assert_eq!(exact, [0, 1, 2, 3, 4]);
}

#[test]
fn write_fill_and_bounds() {
    let mut bytes = [0xDD; 16];
    let mut stream = Stream::memory(&mut bytes, RWM).unwrap();
    assert_eq!(stream.write(&[]).unwrap(), 0);
    assert_eq!(stream.write(&[0, 1, 2]).unwrap(), 3);
    stream.seek(SeekFrom::Start(1)).unwrap();
    stream.fill(3, &[0x80]).unwrap();
    stream.seek(SeekFrom::Start(6)).unwrap();
    stream.fill(2, &[0x90, 0xA0]).unwrap();
    stream.seek(SeekFrom::Start(10)).unwrap();
    stream.fill(1, &[0xB0, 0xC0, 0xD0, 0xE0]).unwrap();
    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut all = [0; 16];
    stream.read_exact(&mut all).unwrap();
    assert_eq!(&all[..3], &[0, 0x80, 0x80]);
    assert_eq!(&all[6..10], &[0x90, 0xA0, 0x90, 0xA0]);
    assert_eq!(&all[10..14], &[0xB0, 0xC0, 0xD0, 0xE0]);
    assert!(stream.fill(1, &[0; 3]).is_err());
    assert!(stream.fill(100, &[1]).is_err());
}

#[test]
fn map_read_write_and_copy() {
    let mut bytes = [10, 11, 12, 13, 14];
    let mut stream = Stream::memory(&mut bytes, RWM).unwrap();
    stream.seek(SeekFrom::Start(1)).unwrap();
    assert_eq!(stream.map_read(2).unwrap(), &[11, 12]);
    stream.seek(SeekFrom::End(-1)).unwrap();
    assert_eq!(stream.map_write(1).unwrap().len(), 1);
    stream.seek(SeekFrom::Start(0)).unwrap();
    assert!(stream.map_read(100).is_err());
    assert!(stream.map_write(100).is_err());

    let source_bytes = [0xA0, 0xB0, 0xC0, 0xD0, 0xE0];
    let mut source = Stream::from_owner(source_bytes, READ).unwrap();
    let mut target_bytes = [0xDD; 5];
    let mut target = Stream::memory(&mut target_bytes, RWM).unwrap();
    source.copy_to(&mut target, 2).unwrap();
    target.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(target.map_read(5).unwrap(), &[0xA0, 0xB0, 0xDD, 0xDD, 0xDD]);
    assert!(source.copy_to(&mut target, 100).is_err());
}

#[test]
fn large_copy_crosses_native_blocks() {
    let source_bytes: Vec<u8> = (0..1_048_576).map(|i| i as u8).collect();
    let mut source = Stream::from_owner(source_bytes, READ).unwrap();
    let mut target_bytes = vec![0xDD; 1_048_576];
    let mut target = Stream::memory(&mut target_bytes, RWM).unwrap();
    source.seek(SeekFrom::Start(1)).unwrap();
    target.seek(SeekFrom::Start(2)).unwrap();
    source.copy_to(&mut target, 1_048_566).unwrap();
    target.seek(SeekFrom::Start(0)).unwrap();
    let mapped = target.map_read(1_048_576).unwrap();
    assert_eq!(mapped[0..2], [0xDD; 2]);
    assert_eq!(&mapped[2..32], &(1u8..31).collect::<Vec<_>>());
    assert_eq!(mapped[1_048_568..], [0xDD; 8]);
}

#[test]
fn fill_eight_byte_patterns_unaligned_and_at_end() {
    let pattern = [0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0];
    for offset in [1, 8] {
        let mut bytes = [0xDD; 16];
        let mut stream = Stream::memory(&mut bytes, RWM).unwrap();
        stream.seek(SeekFrom::Start(offset)).unwrap();
        stream.fill(1, &pattern).unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();
        let actual = stream.map_read(16).unwrap();
        let mut expected = [0xDD; 16];
        expected[offset as usize..offset as usize + 8].copy_from_slice(&pattern);
        assert_eq!(actual, &expected[..]);
    }
}
