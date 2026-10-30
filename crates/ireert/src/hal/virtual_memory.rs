//! Virtual address reservations and physical memory mapping.
//!
//! Reservations, physical allocations and mappings release in dependency order:
//! a [`VirtualMapping`] borrows both sides, so it always unmaps first.
use super::{Allocator, Buffer, BufferParams, QueueAffinity};
use crate::hal::{DeviceSize, MemoryAdvice};
use crate::{Result, check};
use core::{mem::ManuallyDrop, ptr::NonNull};
use iree_native as n;

super::bitfield::bitfield! {
    /// Access permitted to a virtual address range (`iree_hal_memory_protection_t`).
    MemoryProtection: n::iree_hal_memory_protection_t {
        /// No access.
        NONE = n::IREE_HAL_MEMORY_PROTECTION_NONE,
        /// Read access.
        READ = n::IREE_HAL_MEMORY_PROTECTION_READ,
        /// Write access.
        WRITE = n::IREE_HAL_MEMORY_PROTECTION_WRITE,
        /// Read and write access.
        READ_WRITE = n::IREE_HAL_MEMORY_PROTECTION_READ_WRITE,
    }
}

/// Minimum and recommended page sizes for virtual-memory operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Granularity {
    /// Alignment required of every size and offset.
    pub minimum: DeviceSize,
    /// Alignment recommended for performance.
    pub recommended: DeviceSize,
}

impl Allocator {
    /// Creates a host-local heap allocator for buffers that never touch a device.
    pub fn heap(identifier: &str) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the identifier is copied and the output is writable.
        check(unsafe {
            n::iree_hal_allocator_create_heap(
                crate::view(identifier),
                crate::system(),
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: successful creation returns one owned allocator reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Returns minimum and recommended virtual-memory page sizes.
    pub fn virtual_memory_granularity(&self, params: BufferParams) -> Result<Granularity> {
        let (mut minimum, mut recommended) = (0, 0);
        // SAFETY: both outputs are writable.
        check(unsafe {
            n::iree_hal_allocator_virtual_memory_query_granularity(
                self.as_raw(),
                params.raw(),
                &mut minimum,
                &mut recommended,
            )
        })?;
        Ok(Granularity {
            minimum,
            recommended,
        })
    }

    /// Reserves a virtual address range without backing memory.
    pub fn reserve_virtual_memory(
        &self,
        affinity: QueueAffinity,
        size: DeviceSize,
    ) -> Result<VirtualReservation> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the allocator is live and the output is writable.
        check(unsafe {
            n::iree_hal_allocator_virtual_memory_reserve(
                self.as_raw(),
                affinity.bits(),
                size,
                &mut raw,
            )
        })?;
        Ok(VirtualReservation {
            allocator: self.clone(),
            // SAFETY: the reservation buffer is released only by `virtual_memory_release`.
            buffer: ManuallyDrop::new(unsafe { Buffer::from_raw(raw) }),
        })
    }

    /// Allocates physical memory that is not yet mapped anywhere.
    pub fn allocate_physical_memory(
        &self,
        params: BufferParams,
        size: DeviceSize,
    ) -> Result<PhysicalMemory> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: the allocator is live and the output is writable.
        check(unsafe {
            n::iree_hal_allocator_physical_memory_allocate(
                self.as_raw(),
                params.raw(),
                size,
                crate::system(),
                &mut raw,
            )
        })?;
        let raw = NonNull::new(raw).ok_or_else(|| {
            crate::Error::new(crate::Code::Internal, "physical allocation returned null")
        })?;
        Ok(PhysicalMemory {
            allocator: self.clone(),
            raw,
        })
    }
}

/// A uniquely owned virtual address reservation.
///
/// Dropping it releases the reservation; release failures are discarded.
pub struct VirtualReservation {
    allocator: Allocator,
    buffer: ManuallyDrop<Buffer>,
}
impl VirtualReservation {
    /// Returns the native reservation buffer.
    pub fn as_raw(&self) -> *mut n::iree_hal_buffer_t {
        self.buffer.as_raw()
    }

    /// Borrows the reservation as a buffer for queue and command-buffer use.
    ///
    /// # Safety
    /// Releasing a reservation destroys its buffer regardless of reference count.
    /// No clone of the returned buffer, and no command buffer or queue operation
    /// that retains it, may outlive this reservation.
    pub unsafe fn as_buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Maps `size` bytes of `physical` at `physical_offset` into this range at `offset`.
    pub fn map<'a>(
        &'a self,
        offset: DeviceSize,
        physical: &'a PhysicalMemory,
        physical_offset: DeviceSize,
        size: DeviceSize,
    ) -> Result<VirtualMapping<'a>> {
        // SAFETY: both handles are live and outlive the returned mapping guard.
        check(unsafe {
            n::iree_hal_allocator_virtual_memory_map(
                self.allocator.as_raw(),
                self.as_raw(),
                offset,
                physical.raw.as_ptr(),
                physical_offset,
                size,
            )
        })?;
        Ok(VirtualMapping {
            reservation: self,
            _physical: physical,
            offset,
            size,
        })
    }

    /// Sets access permissions on a mapped range for the given queues.
    pub fn protect(
        &self,
        offset: DeviceSize,
        size: DeviceSize,
        affinity: QueueAffinity,
        protection: MemoryProtection,
    ) -> Result<()> {
        // SAFETY: the reservation is live for the call.
        check(unsafe {
            n::iree_hal_allocator_virtual_memory_protect(
                self.allocator.as_raw(),
                self.as_raw(),
                offset,
                size,
                affinity.bits(),
                protection.bits(),
            )
        })
    }

    /// Provides an advisory usage hint for a range.
    pub fn advise(
        &self,
        offset: DeviceSize,
        size: DeviceSize,
        affinity: QueueAffinity,
        advice: MemoryAdvice,
    ) -> Result<()> {
        // SAFETY: the reservation is live for the call.
        check(unsafe {
            n::iree_hal_allocator_virtual_memory_advise(
                self.allocator.as_raw(),
                self.as_raw(),
                offset,
                size,
                affinity.bits(),
                advice.bits(),
            )
        })
    }

    /// Releases the reservation and reports any failure.
    pub fn release(self) -> Result<()> {
        let this = ManuallyDrop::new(self);
        // SAFETY: the reservation is released once and its allocator dropped once.
        let status = unsafe {
            n::iree_hal_allocator_virtual_memory_release(this.allocator.as_raw(), this.as_raw())
        };
        // SAFETY: `this` is never used again.
        drop(unsafe { core::ptr::read(&this.allocator) });
        check(status)
    }
}
impl Drop for VirtualReservation {
    fn drop(&mut self) {
        // SAFETY: no mapping guard can outlive the borrow of this reservation.
        let status = unsafe {
            n::iree_hal_allocator_virtual_memory_release(self.allocator.as_raw(), self.as_raw())
        };
        // SAFETY: the status is owned here and discarded.
        let _ = unsafe { n::consume(status) };
    }
}
// SAFETY: allocator virtual-memory entry points are thread-safe.
unsafe impl Send for VirtualReservation {}
// SAFETY: see `Send`; shared methods only issue thread-safe native calls.
unsafe impl Sync for VirtualReservation {}

/// A physical allocation that can be mapped into virtual reservations.
///
/// Dropping it frees the allocation; free failures are discarded.
pub struct PhysicalMemory {
    allocator: Allocator,
    raw: NonNull<n::iree_hal_physical_memory_t>,
}
impl PhysicalMemory {
    /// Returns the native physical-memory handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_physical_memory_t {
        self.raw.as_ptr()
    }

    /// Frees the allocation and reports any failure.
    pub fn free(self) -> Result<()> {
        let this = ManuallyDrop::new(self);
        // SAFETY: the handle is freed once and its allocator dropped once.
        let status = unsafe {
            n::iree_hal_allocator_physical_memory_free(this.allocator.as_raw(), this.as_raw())
        };
        // SAFETY: `this` is never used again.
        drop(unsafe { core::ptr::read(&this.allocator) });
        check(status)
    }
}
impl Drop for PhysicalMemory {
    fn drop(&mut self) {
        // SAFETY: no mapping guard can outlive the borrow of this allocation.
        let status = unsafe {
            n::iree_hal_allocator_physical_memory_free(self.allocator.as_raw(), self.as_raw())
        };
        // SAFETY: the status is owned here and discarded.
        let _ = unsafe { n::consume(status) };
    }
}
// SAFETY: allocator virtual-memory entry points are thread-safe.
unsafe impl Send for PhysicalMemory {}
// SAFETY: see `Send`; the handle is only passed to thread-safe native calls.
unsafe impl Sync for PhysicalMemory {}

/// A mapped range that unmaps when dropped; unmap failures are discarded.
pub struct VirtualMapping<'a> {
    reservation: &'a VirtualReservation,
    _physical: &'a PhysicalMemory,
    offset: DeviceSize,
    size: DeviceSize,
}
impl VirtualMapping<'_> {
    /// Returns the mapped offset within the reservation.
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }

    /// Returns the mapped byte length.
    pub fn size(&self) -> DeviceSize {
        self.size
    }

    /// Unmaps the range and reports any failure.
    pub fn unmap(self) -> Result<()> {
        let this = ManuallyDrop::new(self);
        check(this.raw_unmap())
    }

    fn raw_unmap(&self) -> n::iree_status_t {
        // SAFETY: the range was mapped by `map` and is unmapped once.
        unsafe {
            n::iree_hal_allocator_virtual_memory_unmap(
                self.reservation.allocator.as_raw(),
                self.reservation.as_raw(),
                self.offset,
                self.size,
            )
        }
    }
}
impl Drop for VirtualMapping<'_> {
    fn drop(&mut self) {
        // SAFETY: the status is owned here and discarded.
        let _ = unsafe { n::consume(self.raw_unmap()) };
    }
}
