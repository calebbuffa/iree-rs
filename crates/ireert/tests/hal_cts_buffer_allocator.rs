//! HAL allocator CTS ports; test panics surface assertion and setup failures.
// Test-only panics surface CTS assertion and setup failures; they are not library behavior.

#[path = "support\\cts.rs"]
mod cts;

use cts::devices;
use ireert::hal::{
    BufferCompatibility, BufferParams, BufferUsage, ExternalBuffer, MemoryAccess, MemoryType,
};
use std::alloc::{Layout, alloc, dealloc};
use std::sync::atomic::{AtomicUsize, Ordering};

const ALLOCATION_SIZE: ireert::hal::DeviceSize = 1024;

#[test]
fn baseline_buffer_compatibility() {
    for (_, _, device) in devices().unwrap() {
        let allocator = device.allocator();
        let host = BufferParams {
            memory_type: MemoryType::HOST_LOCAL | MemoryType::DEVICE_VISIBLE,
            usage: BufferUsage::TRANSFER,
            ..BufferParams::default()
        };
        let (host_caps, _, host_size) = allocator.query_buffer_compatibility(host, ALLOCATION_SIZE);
        assert!(host_size >= ALLOCATION_SIZE);
        let device_local = BufferParams {
            memory_type: MemoryType::HOST_VISIBLE | MemoryType::DEVICE_LOCAL,
            usage: BufferUsage::TRANSFER,
            ..BufferParams::default()
        };
        let (device_caps, _, device_size) =
            allocator.query_buffer_compatibility(device_local, ALLOCATION_SIZE);
        assert!(device_size >= ALLOCATION_SIZE);
        let required = BufferCompatibility::ALLOCATABLE | BufferCompatibility::QUEUE_TRANSFER;
        assert!(host_caps.contains(required) || device_caps.contains(required));
        let dispatch = BufferParams {
            usage: BufferUsage::DISPATCH_STORAGE,
            ..host
        };
        let (dispatch_caps, _, dispatch_size) =
            allocator.query_buffer_compatibility(dispatch, ALLOCATION_SIZE);
        assert!(dispatch_size >= ALLOCATION_SIZE);
        assert!(
            dispatch_caps
                .contains(BufferCompatibility::ALLOCATABLE | BufferCompatibility::QUEUE_DISPATCH)
        );
    }
}

#[test]
fn allocate_buffer() {
    for (_, _, device) in devices().unwrap() {
        let params = BufferParams {
            memory_type: MemoryType::DEVICE_LOCAL,
            usage: BufferUsage::TRANSFER,
            ..BufferParams::default()
        };
        let buffer = device
            .allocator()
            .allocate_buffer(params, ALLOCATION_SIZE)
            .unwrap();
        assert!(buffer.memory_type().contains(params.memory_type));
        assert!(buffer.allowed_usage().contains(params.usage));
        assert!(buffer.allocation_size() >= ALLOCATION_SIZE);
    }
}

#[test]
fn allocate_empty_buffer() {
    for (_, _, device) in devices().unwrap() {
        let params = BufferParams {
            memory_type: MemoryType::DEVICE_LOCAL,
            usage: BufferUsage::TRANSFER,
            ..BufferParams::default()
        };
        let _buffer = device.allocator().allocate_buffer(params, 0).unwrap();
    }
}

fn importable_host_memory(
    device: &ireert::hal::Device,
) -> Result<Option<(Layout, *mut u8, ireert::hal::DeviceSize, BufferParams)>, std::alloc::LayoutError>
{
    let allocator = device.allocator();
    let params = BufferParams {
        memory_type: MemoryType::HOST_LOCAL | MemoryType::DEVICE_VISIBLE,
        access: MemoryAccess::ALL,
        usage: BufferUsage::TRANSFER,
        ..BufferParams::default()
    };
    let (compatibility, adjusted, size) =
        allocator.query_buffer_compatibility(params, ALLOCATION_SIZE);
    if !compatibility.contains(BufferCompatibility::IMPORTABLE) {
        return Ok(None);
    }
    let alignment = if adjusted.min_alignment == 0 {
        64
    } else {
        adjusted.min_alignment
    };
    let layout = Layout::from_size_align(size.max(1), alignment)?;
    // SAFETY: layout has nonzero size and a valid power-of-two alignment.
    let pointer = unsafe { alloc(layout) };
    assert!(!pointer.is_null());
    // SAFETY: pointer is valid for size bytes and is uniquely owned by the test.
    unsafe { pointer.write_bytes(0, size) };
    Ok(Some((layout, pointer, size, adjusted)))
}

#[test]
fn import_host_allocation_null_callback() -> Result<(), std::alloc::LayoutError> {
    for (name, _, device) in devices().unwrap() {
        let Some((layout, pointer, size, params)) = importable_host_memory(&device)? else {
            // Upstream AllocatorTest mirrors this skip when IMPORTABLE is absent.
            eprintln!("skipping {name}: allocator does not support importing host allocations");
            continue;
        };
        {
            // SAFETY: allocation is aligned and lives until both imported wrappers are dropped.
            let memory = unsafe { std::slice::from_raw_parts_mut(pointer, size) };
            let mut external = ExternalBuffer::HostAllocation { memory };
            // SAFETY: the aligned allocation outlives each imported buffer; the null callback leaves freeing to this test.
            let imported = unsafe {
                device
                    .allocator()
                    .import_buffer(params, &mut external, None)
            }
            .unwrap();
            drop(imported);
            // SAFETY: same live, aligned allocation; releasing the first import must unregister it.
            let imported_again = unsafe {
                device
                    .allocator()
                    .import_buffer(params, &mut external, None)
            }
            .unwrap();
            drop(imported_again);
        }
        // SAFETY: both imports have been released and the allocation was created with this layout.
        unsafe { dealloc(pointer, layout) };
    }
    Ok(())
}

#[test]
fn import_host_allocation_with_callback() -> Result<(), std::alloc::LayoutError> {
    for (name, _, device) in devices().unwrap() {
        let Some((layout, pointer, size, params)) = importable_host_memory(&device)? else {
            // Upstream AllocatorTest mirrors this skip when IMPORTABLE is absent.
            eprintln!("skipping {name}: allocator does not support importing host allocations");
            continue;
        };
        let releases = std::sync::Arc::new(AtomicUsize::new(0));
        {
            // SAFETY: allocation is aligned and lives until the imported buffer is released.
            let memory = unsafe { std::slice::from_raw_parts_mut(pointer, size) };
            let mut external = ExternalBuffer::HostAllocation { memory };
            let release_count = releases.clone();
            // SAFETY: backing storage outlives the import; callback state is owned until release.
            let imported = unsafe {
                device.allocator().import_buffer(
                    params,
                    &mut external,
                    Some(Box::new(move || {
                        release_count.fetch_add(1, Ordering::SeqCst);
                    })),
                )
            }
            .unwrap();
            assert_eq!(releases.load(Ordering::SeqCst), 0);
            drop(imported);
            assert_eq!(releases.load(Ordering::SeqCst), 1);
            // SAFETY: buffer release must unregister the allocation before re-import.
            let imported_again = unsafe {
                device
                    .allocator()
                    .import_buffer(params, &mut external, None)
            }
            .unwrap();
            drop(imported_again);
        }
        // SAFETY: both imports have been released and the allocation was created with this layout.
        unsafe { dealloc(pointer, layout) };
    }
    Ok(())
}
