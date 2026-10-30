//! HAL buffer allocators and their allocation properties.

use super::flags::{BufferParams, BufferUsage};
use crate::hal::DeviceSize;
use crate::{Result, check};
use core::ptr;
use iree_native as n;

/// A retained device buffer allocator.
pub struct Allocator(*mut n::iree_hal_allocator_t);
impl Allocator {
    /// Adopts one owned allocator reference.
    ///
    /// # Safety
    /// `raw` must be a live allocator pointer whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_allocator_t) -> Self {
        Self(raw)
    }
    /// Borrows the native allocator pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_allocator_t {
        self.0
    }
    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_allocator_t {
        core::mem::ManuallyDrop::new(self).0
    }
    /// Allocates a buffer with the requested parameters and byte length.
    pub fn allocate_buffer(
        &self,
        params: BufferParams,
        length: DeviceSize,
    ) -> Result<super::buffer::Buffer> {
        let mut out = ptr::null_mut();
        // SAFETY: output pointer is valid; the allocation parameters are plain values.
        check(unsafe {
            n::iree_hal_allocator_allocate_buffer(self.0, params.raw(), length, &mut out)
        })?;
        // SAFETY: successful allocation returns one owned buffer reference.
        Ok(unsafe { super::buffer::Buffer::from_raw(out) })
    }
    /// Imports an external buffer into this allocator.
    ///
    /// # Safety
    /// The external storage and handle must remain valid until the returned buffer is released. The Rust wrapper cannot encode the native buffer's potentially longer lifetime.
    pub unsafe fn import_buffer(
        &self,
        params: BufferParams,
        external: &mut super::ExternalBuffer<'_>,
        release: Option<Box<dyn FnOnce() + Send>>,
    ) -> Result<super::buffer::Buffer> {
        let mut out = ptr::null_mut();
        let mut external = external.raw();
        // Double boxing yields a thin pointer for the C `user_data` slot.
        let mut callback = release.map(|release| Box::into_raw(Box::new(release)));
        let raw_callback = n::iree_hal_buffer_release_callback_t {
            fn_: callback.map(|_| buffer_release_trampoline as _),
            user_data: callback.map_or(ptr::null_mut(), |user_data| user_data.cast()),
        };
        // SAFETY: caller upholds the borrowed external handle's native lifetime rules; output is writable.
        let status = unsafe {
            n::iree_hal_allocator_import_buffer(
                self.0,
                params.raw(),
                &mut external,
                raw_callback,
                &mut out,
            )
        };
        if let Err(error) = check(status) {
            if let Some(user_data) = callback.take() {
                // SAFETY: failed import does not take ownership of its release callback.
                unsafe {
                    drop(Box::from_raw(user_data));
                }
            }
            return Err(error);
        }
        // SAFETY: successful import returns one owned buffer reference.
        Ok(unsafe { super::buffer::Buffer::from_raw(out) })
    }
    /// Exports a buffer to a driver-specific external handle.
    ///
    /// The returned wrapper keeps the source buffer alive and closes an exported OS handle on drop.
    pub fn export_buffer(
        &self,
        buffer: &super::buffer::Buffer,
        requested_type: super::ExternalBufferType,
    ) -> Result<super::ExportedExternalBuffer> {
        // SAFETY: the native struct is plain data and is fully written on success.
        let mut out: n::iree_hal_external_buffer_t = unsafe { core::mem::zeroed() };
        // SAFETY: the source buffer is borrowed for the call and `out` is writable.
        check(unsafe {
            n::iree_hal_allocator_export_buffer(
                self.0,
                buffer.as_raw(),
                requested_type as n::iree_hal_external_buffer_type_t,
                n::IREE_HAL_EXTERNAL_BUFFER_FLAG_NONE as n::iree_hal_external_buffer_flags_t,
                &mut out,
            )
        })?;
        #[cfg(unix)]
        let owned_fd = if out.type_ == n::IREE_HAL_EXTERNAL_BUFFER_TYPE_OPAQUE_FD {
            use std::os::fd::FromRawFd;
            // SAFETY: successful export transfers responsibility for the returned descriptor to the caller.
            Some(unsafe { std::os::fd::OwnedFd::from_raw_fd(out.handle.opaque_fd.fd) })
        } else {
            None
        };
        #[cfg(windows)]
        let owned_handle = if out.type_ == n::IREE_HAL_EXTERNAL_BUFFER_TYPE_OPAQUE_WIN32 {
            use std::os::windows::io::FromRawHandle;
            // SAFETY: successful export transfers responsibility for the returned handle to the caller.
            Some(unsafe {
                std::os::windows::io::OwnedHandle::from_raw_handle(out.handle.opaque_win32.handle)
            })
        } else {
            None
        };
        Ok(super::ExportedExternalBuffer {
            raw: out,
            _buffer: buffer.clone(),
            #[cfg(unix)]
            owned_fd,
            #[cfg(windows)]
            owned_handle,
        })
    }
    /// Lists the allocator's memory heaps in preference order.
    pub fn memory_heaps(&self) -> Result<Vec<super::MemoryHeap>> {
        let mut count = 0usize;
        // SAFETY: null heap pointer with zero capacity is the documented count query.
        let query = check(unsafe {
            n::iree_hal_allocator_query_memory_heaps(self.0, 0, ptr::null_mut(), &mut count)
        });
        match query {
            Ok(()) => {}
            Err(error) if error.code() == crate::Code::OutOfRange => {}
            Err(error) => return Err(error),
        }
        let mut heaps = vec![n::iree_hal_allocator_memory_heap_t::default(); count];
        // SAFETY: allocated vector provides writable capacity for `count` records.
        check(unsafe {
            n::iree_hal_allocator_query_memory_heaps(
                self.0,
                heaps.len(),
                heaps.as_mut_ptr(),
                &mut count,
            )
        })?;
        heaps.truncate(count);
        Ok(heaps
            .into_iter()
            .map(|h| MemoryHeap {
                memory_type: super::flags::MemoryType::from_bits_retain(h.type_),
                allowed_usage: BufferUsage::from_bits_retain(h.allowed_usage),
                max_allocation_size: h.max_allocation_size,
                min_alignment: h.min_alignment,
            })
            .collect())
    }
    /// Returns whether this allocator supports virtual-memory operations.
    pub fn supports_virtual_memory(&self) -> bool {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_allocator_supports_virtual_memory(self.0) }
    }
    /// Trims cached allocations.
    pub fn trim(&self) -> Result<()> {
        // SAFETY: the wrapper owns a live reference.
        check(unsafe { n::iree_hal_allocator_trim(self.0) })
    }
    /// Returns a snapshot of allocator statistics.
    pub fn statistics(&self) -> AllocatorStatistics {
        let mut stats = n::iree_hal_allocator_statistics_t::default();
        // SAFETY: stats is writable and the function captures an instantaneous snapshot.
        unsafe { n::iree_hal_allocator_query_statistics(self.0, &mut stats) };
        AllocatorStatistics::from_raw(stats)
    }
    /// Queries whether parameters and size are compatible with the allocator.
    pub fn query_buffer_compatibility(
        &self,
        params: BufferParams,
        length: DeviceSize,
    ) -> (BufferCompatibility, BufferParams, DeviceSize) {
        let mut out = params.raw();
        let mut out_size = length;
        // SAFETY: output values are writable and the inputs are plain values.
        let compatibility = unsafe {
            n::iree_hal_allocator_query_buffer_compatibility(
                self.0,
                params.raw(),
                length,
                &mut out,
                &mut out_size,
            )
        };
        (
            BufferCompatibility::from_bits_retain(compatibility),
            BufferParams::from_raw(out),
            out_size,
        )
    }
}
unsafe extern "C" fn buffer_release_trampoline(
    user_data: *mut core::ffi::c_void,
    _buffer: *mut n::iree_hal_buffer_t,
) {
    n::contain(|| {
        // SAFETY: user_data points to the boxed callback transferred by import_buffer.
        let release = unsafe { Box::from_raw(user_data.cast::<Box<dyn FnOnce() + Send>>()) };
        release();
    });
}

impl Clone for Allocator {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_allocator_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for Allocator {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_allocator_release(self.0) }
    }
}
// SAFETY: HAL allocators are reference counted atomically and their public
// entry points are documented as thread-safe.
unsafe impl Send for Allocator {}
// SAFETY: see `Send`; every method takes `&self` and calls thread-safe natives.
unsafe impl Sync for Allocator {}

super::bitfield::bitfield! {
    /// HAL buffer compatibility capabilities (`iree_hal_buffer_compatibility_t`).
    BufferCompatibility: n::iree_hal_buffer_compatibility_t {
        /// New allocation is supported.
        ALLOCATABLE = n::IREE_HAL_BUFFER_COMPATIBILITY_ALLOCATABLE,
        /// Import is supported.
        IMPORTABLE = n::IREE_HAL_BUFFER_COMPATIBILITY_IMPORTABLE,
        /// Export is supported.
        EXPORTABLE = n::IREE_HAL_BUFFER_COMPATIBILITY_EXPORTABLE,
        /// Queue transfers are supported.
        QUEUE_TRANSFER = n::IREE_HAL_BUFFER_COMPATIBILITY_QUEUE_TRANSFER,
        /// Queue dispatch is supported.
        QUEUE_DISPATCH = n::IREE_HAL_BUFFER_COMPATIBILITY_QUEUE_DISPATCH,
        /// The buffer may be slower than native allocations.
        LOW_PERFORMANCE = n::IREE_HAL_BUFFER_COMPATIBILITY_LOW_PERFORMANCE,
    }
}

/// Host allocation statistics captured by a HAL allocator.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AllocatorStatistics {
    /// Peak host bytes allocated.
    pub host_bytes_peak: DeviceSize,
    /// Current host bytes allocated.
    pub host_bytes_allocated: DeviceSize,
    /// Total host bytes freed.
    pub host_bytes_freed: DeviceSize,
    /// Peak device bytes allocated.
    pub device_bytes_peak: DeviceSize,
    /// Current device bytes allocated.
    pub device_bytes_allocated: DeviceSize,
    /// Total device bytes freed.
    pub device_bytes_freed: DeviceSize,
}
impl AllocatorStatistics {
    fn from_raw(v: n::iree_hal_allocator_statistics_t) -> Self {
        Self {
            host_bytes_peak: v.host_bytes_peak,
            host_bytes_allocated: v.host_bytes_allocated,
            host_bytes_freed: v.host_bytes_freed,
            device_bytes_peak: v.device_bytes_peak,
            device_bytes_allocated: v.device_bytes_allocated,
            device_bytes_freed: v.device_bytes_freed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_query_accepts_the_count_only_out_of_range_status() {
        let allocator = Allocator::heap("heap-query").unwrap();
        let heaps = allocator.memory_heaps().unwrap();
        assert_eq!(heaps.len(), 1);
        assert!(heaps[0].max_allocation_size > 0);
    }
}

/// Properties of one allocator memory heap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryHeap {
    /// Memory properties exposed by the heap.
    pub memory_type: super::flags::MemoryType,
    /// Permitted buffer usages.
    pub allowed_usage: super::flags::BufferUsage,
    /// Largest supported allocation, in bytes.
    pub max_allocation_size: DeviceSize,
    /// Minimum allocation alignment, in bytes.
    pub min_alignment: DeviceSize,
}
