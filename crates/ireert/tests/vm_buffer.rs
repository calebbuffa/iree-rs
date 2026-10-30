//! Ports of publicly expressible cases in upstream `vm/buffer_test.cc`.
// Inline initialization and module-storage allocator cases need a safe externally-backed buffer wrapper, which is not yet exposed.

use ireert::vm::{Buffer, BufferAccess, Instance};

#[test]
fn buffer_copy_and_mapping() {
    let instance = Instance::new().unwrap();
    let bytes = [0_u8, 1, 2, 3, 4, 5, 6, 7];
    let mut buffer = Buffer::new(&instance, BufferAccess::MUTABLE, bytes.len(), 1).unwrap();
    buffer.write(0, &bytes).unwrap();
    assert_eq!(buffer.len(), bytes.len());
    assert!(buffer.access().contains(BufferAccess::MUTABLE));
    let mut actual = [0; 8];
    buffer.read(0, &mut actual).unwrap();
    assert_eq!(actual, bytes);
    // SAFETY: this test does not write through the buffer while the mapped slice is live.
    let mapped = unsafe { buffer.map(0, 8, 1) }.unwrap();
    assert_eq!(mapped, bytes);
}

#[test]
fn buffer_clone_compare_copy_and_fill() {
    let instance = Instance::new().unwrap();
    let mut source = Buffer::new(&instance, BufferAccess::MUTABLE, 8, 1).unwrap();
    source.write(0, &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    let mut target = source.clone_range(BufferAccess::MUTABLE, 0, 8, 1).unwrap();
    assert!(source.compare(0, &target, 0, 8).unwrap());
    target.fill(0, 8, 0).unwrap();
    assert!(!source.compare(0, &target, 0, 8).unwrap());
    source.copy_to(0, &target, 0, 8).unwrap();
    assert!(source.compare(0, &target, 0, 8).unwrap());
}

#[test]
fn buffer_rejects_out_of_range_and_invalid_alignment() {
    let instance = Instance::new().unwrap();
    let mut buffer = Buffer::new(&instance, BufferAccess::MUTABLE, 8, 1).unwrap();
    assert!(buffer.read(usize::MAX, &mut [0]).is_err());
    assert!(buffer.write(8, &[1]).is_err());
    assert!(buffer.clone_range(BufferAccess::MUTABLE, 7, 2, 1).is_err());
    assert!(buffer.clone_range(BufferAccess::MUTABLE, 0, 4, 3).is_err());
}

#[test]
fn buffer_round_trips_through_vm_ref() {
    let instance = Instance::new().unwrap();
    let buffer = Buffer::new(&instance, BufferAccess::MUTABLE, 4, 1).unwrap();
    let reference = buffer.to_ref().unwrap();
    let recovered = Buffer::from_ref(&reference).unwrap();
    assert_eq!(recovered.len(), 4);
}
