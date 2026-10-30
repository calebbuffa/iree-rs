//! Upstream vector-stream contract tests (`vec_stream_test.cc`).
// Integration tests use unwrap/expect to fail immediately when setup or an assertion prerequisite fails.

use ireert::io::{Stream, StreamMode};
use std::io::{Read, Seek, SeekFrom, Write};

const READ: StreamMode =
    StreamMode::from_bits_retain(StreamMode::READABLE.bits() | StreamMode::SEEKABLE.bits());
const RW: StreamMode = StreamMode::from_bits_retain(
    READ.bits() | StreamMode::WRITABLE.bits() | StreamMode::RESIZABLE.bits(),
);

fn populated() -> Result<Stream<'static>, Box<dyn std::error::Error>> {
    let mut stream = Stream::vector(RW, 16)?;
    stream.write_all(&[0, 1, 2, 3, 4])?;
    stream.seek(SeekFrom::Start(0))?;
    Ok(stream)
}

#[test]
fn empty() {
    let stream = Stream::vector(READ, 16).unwrap();
    assert_eq!(stream.mode(), READ);
    assert_eq!(stream.offset(), 0);
    assert_eq!(stream.length(), 0);
    assert!(stream.is_eos());
}

#[test]
fn seeks_extend_resizable_stream() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = populated()?;
    assert_eq!(stream.seek(SeekFrom::Start(0)).unwrap(), 0);
    assert_eq!(stream.seek(SeekFrom::End(0)).unwrap(), 5);
    assert_eq!(stream.seek(SeekFrom::Start(4)).unwrap(), 4);
    assert_eq!(stream.seek(SeekFrom::Current(1)).unwrap(), 5);
    assert_eq!(stream.seek(SeekFrom::Current(-5)).unwrap(), 0);
    assert_eq!(stream.seek(SeekFrom::End(-1)).unwrap(), 4);
    assert!(stream.seek(SeekFrom::Start(u64::MAX)).is_err());
    assert!(stream.seek(SeekFrom::Current(-100)).is_err());
    assert_eq!(stream.seek(SeekFrom::Start(6)).unwrap(), 6);
    assert_eq!(stream.length(), 6);
    assert!(stream.is_eos());
    stream.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(stream.seek(SeekFrom::End(100)).unwrap(), 106);
    assert_eq!(stream.length(), 106);
    Ok(())
}

#[test]
fn alignment_and_bounds() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = populated()?;
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
    stream.seek_to_alignment(16).unwrap();
    assert_eq!(stream.offset(), 16);
    assert_eq!(stream.length(), 16);
    Ok(())
}

#[test]
fn read_write_fill_and_block_enumeration() {
    let mut stream = Stream::vector(RW, 8).unwrap();
    stream.write_all(&[0, 1, 2, 3, 4]).unwrap();
    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = [0xDD; 8];
    assert_eq!(stream.read(&mut buffer).unwrap(), 5);
    assert_eq!(&buffer[..6], &[0, 1, 2, 3, 4, 0xDD]);
    assert_eq!(stream.read(&mut buffer).unwrap(), 0);
    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut exact = [0; 6];
    assert!(stream.read_exact_atomic(&mut exact).is_err());
    assert_eq!(stream.offset(), 0);
    let mut exact = [0; 5];
    stream.read_exact_atomic(&mut exact).unwrap();
    assert_eq!(exact, [0, 1, 2, 3, 4]);
    stream.seek(SeekFrom::Start(1)).unwrap();
    stream.fill(3, &[0x80]).unwrap();
    stream.seek(SeekFrom::Start(8)).unwrap();
    stream
        .fill(1, &[0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xFF])
        .unwrap();
    assert!(stream.fill(1, &[0; 3]).is_err());
    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut joined = Vec::new();
    stream
        .for_each_block(|block| joined.extend_from_slice(block))
        .unwrap();
    assert_eq!(&joined[..4], &[0, 0x80, 0x80, 0x80]);
    assert_eq!(
        &joined[8..],
        &[0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xFF]
    );
}

#[test]
fn vector_stream_reads_and_writes_keep_cursor_on_empty_or_failed_operations() {
    let mut stream = populated().unwrap();
    let mut buffer = [0xDD; 8];

    assert_eq!(stream.read(&mut []).unwrap(), 0);
    assert_eq!(stream.offset(), 0);
    assert!(stream.read_exact_atomic(&mut buffer[..6]).is_err());
    assert_eq!(stream.offset(), 0);
    assert_eq!(buffer, [0xDD; 8]);

    stream.seek(SeekFrom::Start(5)).unwrap();
    assert_eq!(stream.read(&mut buffer).unwrap(), 0);
    assert_eq!(stream.offset(), 5);
    assert!(stream.read_exact_atomic(&mut buffer[..1]).is_err());
    assert_eq!(stream.offset(), 5);

    assert_eq!(stream.write(&[]).unwrap(), 0);
    assert_eq!(stream.offset(), 5);
    stream.write_all(&[5, 6]).unwrap();
    assert_eq!(stream.offset(), 7);
    assert_eq!(stream.length(), 7);
}

#[test]
fn immutable_owner_cannot_be_writable() {
    assert!(Stream::from_owner(vec![1, 2], RW).is_err());
}

#[test]
fn fill_sizes_and_two_four_byte_patterns() {
    let mut stream = Stream::vector(RW, 16).unwrap();
    stream.seek(SeekFrom::Start(16)).unwrap();
    stream.seek(SeekFrom::Start(0)).unwrap();
    let pattern = [0x80, 0x90, 0xA0, 0xB0];
    assert!(stream.fill(1, &[0; 3]).is_err());
    assert!(stream.fill(1, &[0; 9]).is_err());
    assert_eq!(stream.offset(), 0);
    stream.seek(SeekFrom::Start(1)).unwrap();
    stream.fill(3, &pattern[..1]).unwrap();
    stream.seek(SeekFrom::Start(4)).unwrap();
    stream.fill(2, &pattern[..2]).unwrap();
    stream.seek(SeekFrom::Start(8)).unwrap();
    stream.fill(1, &pattern).unwrap();
    stream.seek(SeekFrom::Start(0)).unwrap();
    let mut actual = [0xDD; 16];
    stream.read_exact_atomic(&mut actual).unwrap();
    assert_eq!(
        actual,
        [
            0, 0x80, 0x80, 0x80, 0x80, 0x90, 0x80, 0x90, 0x80, 0x90, 0xA0, 0xB0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn fill_eight_byte_patterns_unaligned_and_at_end() {
    let pattern = [0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0];
    for offset in [1, 8] {
        let mut stream = Stream::vector(RW, 16).unwrap();
        stream.seek(SeekFrom::Start(16)).unwrap();
        stream.seek(SeekFrom::Start(offset)).unwrap();
        stream.fill(1, &pattern).unwrap();
        stream.seek(SeekFrom::Start(0)).unwrap();
        let mut actual = [0xDD; 16];
        stream.read_exact_atomic(&mut actual).unwrap();
        let mut expected = [0; 16];
        expected[offset as usize..offset as usize + 8].copy_from_slice(&pattern);
        assert_eq!(actual, expected);
    }
}
