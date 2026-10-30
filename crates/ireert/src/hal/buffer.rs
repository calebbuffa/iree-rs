//! HAL buffer ownership, mapping, and byte-range operations.
use super::flags::{BufferUsage, MemoryAccess, MemoryType};
use crate::hal::DeviceSize;
use crate::{Code, Error, Result, check, system};
use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ptr,
};
use iree_native as n;

/// How two buffer ranges overlap (`iree_hal_buffer_overlap_t`).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Overlap {
    /// The ranges share no bytes.
    Disjoint,
    /// The ranges share some bytes.
    Partial,
    /// The ranges are identical.
    Complete,
}

super::bitfield::bitfield! {
    /// How an allocation was placed (`iree_hal_buffer_placement_flags_t`).
    PlacementFlags: n::iree_hal_buffer_placement_flags_t {
        /// Allocated, and deallocatable, with queue-ordered operations.
        ASYNCHRONOUS = n::IREE_HAL_BUFFER_PLACEMENT_FLAG_ASYNCHRONOUS,
        /// Explicit deallocation is ignored; release happens with the last reference.
        INDETERMINATE_LIFETIME = n::IREE_HAL_BUFFER_PLACEMENT_FLAG_INDETERMINATE_LIFETIME,
    }
}

/// Where an allocation was originally placed (`iree_hal_buffer_placement_t`).
#[derive(Clone)]
pub struct Placement {
    /// Device that allocated the buffer, absent for host or imported allocations.
    pub device: Option<super::device::RetainedDevice>,
    /// Queues the allocation is usable on.
    pub queue_affinity: super::QueueAffinity,
    /// Placement flags.
    pub flags: PlacementFlags,
}

/// A retained HAL byte buffer.
pub struct Buffer(*mut n::iree_hal_buffer_t);
impl Buffer {
    /// Adopts one owned buffer reference.
    ///
    /// # Safety
    /// `raw` must be a live buffer pointer whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_buffer_t) -> Self {
        Self(raw)
    }
    /// Retains a borrowed buffer pointer into a new owned reference.
    ///
    /// # Safety
    /// `raw` must be a live buffer pointer.
    pub unsafe fn from_borrowed(raw: *mut n::iree_hal_buffer_t) -> Self {
        // SAFETY: guaranteed live by the caller; the new reference is released on drop.
        unsafe { n::iree_hal_buffer_retain(raw) };
        Self(raw)
    }
    /// Borrows the native buffer pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_buffer_t {
        self.0
    }
    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_buffer_t {
        ManuallyDrop::new(self).0
    }
    /// Returns the absolute byte offset in the underlying allocation.
    pub fn byte_offset(&self) -> DeviceSize {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_byte_offset(self.0) }
    }
    /// Returns this buffer's byte length.
    pub fn byte_length(&self) -> DeviceSize {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_byte_length(self.0) }
    }
    /// Returns memory capabilities of this buffer.
    pub fn memory_type(&self) -> MemoryType {
        // SAFETY: the wrapper owns a live reference.
        MemoryType::from_bits_retain(unsafe { n::iree_hal_buffer_memory_type(self.0) })
    }
    /// Returns permitted access modes.
    pub fn allowed_access(&self) -> MemoryAccess {
        // SAFETY: the wrapper owns a live reference.
        MemoryAccess::from_bits_retain(unsafe { n::iree_hal_buffer_allowed_access(self.0) })
    }
    /// Returns declared usage modes.
    pub fn allowed_usage(&self) -> BufferUsage {
        // SAFETY: the wrapper owns a live reference.
        BufferUsage::from_bits_retain(unsafe { n::iree_hal_buffer_allowed_usage(self.0) })
    }
    /// Returns the parent allocation buffer if this is a subspan.
    pub fn allocated_buffer(&self) -> Option<Buffer> {
        // SAFETY: the wrapper owns a live reference.
        let raw = unsafe { n::iree_hal_buffer_allocated_buffer(self.0) };
        // SAFETY: the parent is kept live by this buffer; retain our own reference.
        (!raw.is_null()).then(|| unsafe { Self::from_borrowed(raw) })
    }
    /// Returns the size of the underlying allocation, possibly rounded up from the request.
    pub fn allocation_size(&self) -> DeviceSize {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_allocation_size(self.0) }
    }
    /// Returns the original placement of the underlying allocation.
    pub fn allocation_placement(&self) -> Placement {
        // SAFETY: the wrapper owns a live reference.
        let raw = unsafe { n::iree_hal_buffer_allocation_placement(self.0) };
        Placement {
            // SAFETY: the buffer's placement exposes a borrowed live device.
            device: (!raw.device.is_null())
                .then(|| unsafe { super::device::RetainedDevice::from_borrowed(raw.device) }),
            queue_affinity: super::QueueAffinity::from_bits_retain(raw.queue_affinity),
            flags: PlacementFlags::from_bits_retain(raw.flags),
        }
    }
    /// Takes allocation ownership, preventing other owners from deallocating it.
    pub fn preserve(&self) -> PreservedAllocation {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_allocation_preserve(self.0) };
        PreservedAllocation(ManuallyDrop::new(self.clone()))
    }
    /// Returns true when no other owner preserves the allocation, so it may be reused.
    pub fn allocation_is_terminal(&self) -> bool {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_buffer_allocation_is_terminal(self.0) }
    }
    /// Creates an owned subspan of the buffer.
    pub fn subspan(&self, offset: DeviceSize, length: DeviceSize) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: the output pointer is writable; range validation is performed by IREE.
        check(unsafe { n::iree_hal_buffer_subspan(self.0, offset, length, system(), &mut out) })?;
        // SAFETY: a successful call returns a new owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }
    /// Tests how `[offset, offset + length)` overlaps a range of `other`,
    /// accounting for subspans of the same allocation.
    pub fn overlap(
        &self,
        offset: DeviceSize,
        length: DeviceSize,
        other: &Buffer,
        other_offset: DeviceSize,
        other_length: DeviceSize,
    ) -> Overlap {
        // SAFETY: both wrappers own live references.
        match unsafe {
            n::iree_hal_buffer_test_overlap(
                self.0,
                offset,
                length,
                other.0,
                other_offset,
                other_length,
            )
        } {
            n::IREE_HAL_BUFFER_OVERLAP_DISJOINT => Overlap::Disjoint,
            n::IREE_HAL_BUFFER_OVERLAP_COMPLETE => Overlap::Complete,
            _ => Overlap::Partial,
        }
    }
    /// Maps a byte range until the returned guard is dropped or unmapped.
    ///
    /// # Safety
    /// While the mapping is live, no host transfer or device operation may
    /// access the range. Multiple mapping guards may overlap, as supported by
    /// the backend, but callers must never access aliased mapped ranges
    /// simultaneously or keep a Rust reference to one mapping while accessing
    /// another. If `access` permits reading, every mapped byte must already be
    /// initialized.
    pub unsafe fn map_range(
        &self,
        offset: DeviceSize,
        length: DeviceSize,
        access: MemoryAccess,
    ) -> Result<MappedRange<'_>> {
        let mut mapping = n::iree_hal_buffer_mapping_t::default();
        // SAFETY: the mapping record is writable; exclusivity is guaranteed by the caller.
        check(unsafe {
            n::iree_hal_buffer_map_range(
                self.0,
                n::IREE_HAL_MAPPING_MODE_SCOPED as n::iree_hal_mapping_mode_t,
                access.bits(),
                offset,
                length,
                &mut mapping,
            )
        })?;
        Ok(MappedRange {
            mapping,
            access,
            _buffer: PhantomData,
        })
    }
    /// Zeroes a byte range.
    pub fn zero(&self, offset: DeviceSize, length: DeviceSize) -> Result<()> {
        // SAFETY: the wrapper owns a live reference; IREE validates the range.
        check(unsafe { n::iree_hal_buffer_map_zero(self.0, offset, length) })
    }
    /// Fills a byte range with a repeated 1, 2, or 4-byte pattern.
    pub fn fill(&self, offset: DeviceSize, length: DeviceSize, pattern: &[u8]) -> Result<()> {
        // SAFETY: the pattern is borrowed for the synchronous call; IREE validates its length.
        check(unsafe {
            n::iree_hal_buffer_map_fill(
                self.0,
                offset,
                length,
                pattern.as_ptr().cast(),
                pattern.len(),
            )
        })
    }
    /// Reads bytes from a range whose contents are already initialized.
    ///
    /// # Safety
    /// Every byte in the source range must have been initialized by the host or device.
    pub unsafe fn read_unchecked(&self, offset: DeviceSize, target: &mut [u8]) -> Result<()> {
        let length = DeviceSize::try_from(target.len())
            .map_err(|_| Error::new(Code::OutOfRange, "buffer read length exceeds native size"))?;
        // SAFETY: the target is writable for `length` bytes; initialization is guaranteed by the caller.
        check(unsafe {
            n::iree_hal_buffer_map_read(self.0, offset, target.as_mut_ptr().cast(), length)
        })
    }
    /// Reads initialized elements into a host slice with checked alignment.
    ///
    /// # Safety
    /// Every byte in the source range must have been initialized by the host or device.
    pub unsafe fn read_typed_unchecked<T: bytemuck::Pod>(
        &self,
        offset: DeviceSize,
        target: &mut [T],
    ) -> Result<()> {
        super::pod_element_size::<T>()?;
        let bytes = bytemuck::try_cast_slice_mut(target).map_err(super::pod_cast_error)?;
        // SAFETY: guaranteed by the caller.
        unsafe { self.read_unchecked(offset, bytes) }
    }
    /// Writes bytes from a host slice into a buffer range.
    pub fn write(&self, offset: DeviceSize, source: &[u8]) -> Result<()> {
        let length = DeviceSize::try_from(source.len())
            .map_err(|_| Error::new(Code::OutOfRange, "buffer write length exceeds native size"))?;
        // SAFETY: the source is readable for `length` bytes through the synchronous call.
        check(unsafe {
            n::iree_hal_buffer_map_write(self.0, offset, source.as_ptr().cast(), length)
        })
    }
    /// Writes plain-data elements from a host slice into a buffer range.
    pub fn write_typed<T: bytemuck::Pod>(&self, offset: DeviceSize, source: &[T]) -> Result<()> {
        super::pod_element_size::<T>()?;
        self.write(
            offset,
            bytemuck::try_cast_slice(source).map_err(super::pod_cast_error)?,
        )
    }
    /// Copies a byte range from another buffer; overlapping ranges are rejected by IREE.
    pub fn copy_from(
        &self,
        target_offset: DeviceSize,
        source: &Buffer,
        source_offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        // SAFETY: both buffers remain live for the synchronous copy.
        check(unsafe {
            n::iree_hal_buffer_map_copy(source.0, source_offset, self.0, target_offset, length)
        })
    }
}
impl Clone for Buffer {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { Self::from_borrowed(self.0) }
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_buffer_release(self.0) }
    }
}

/// Allocation ownership taken by [`Buffer::preserve`]; discarded on drop.
pub struct PreservedAllocation(ManuallyDrop<Buffer>);
impl PreservedAllocation {
    /// The preserved buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.0
    }
    /// Releases ownership, returning true when the caller was the last owner and
    /// should deallocate, e.g. with a queue deallocation for asynchronous placements.
    pub fn discard(self) -> bool {
        let mut this = ManuallyDrop::new(self);
        // SAFETY: `this` is never dropped, so the buffer is taken exactly once.
        let buffer = unsafe { ManuallyDrop::take(&mut this.0) };
        // SAFETY: this guard holds exactly one preservation of a live buffer.
        unsafe { n::iree_hal_buffer_allocation_discard(buffer.0) }
    }
}
impl Drop for PreservedAllocation {
    fn drop(&mut self) {
        // SAFETY: this guard holds exactly one preservation of a live buffer.
        let _last = unsafe { n::iree_hal_buffer_allocation_discard(self.0.0) };
        // SAFETY: the buffer is dropped once, here.
        unsafe { ManuallyDrop::drop(&mut self.0) };
    }
}

/// Rejects queue or transfer ranges that alias, which IREE leaves undefined.
pub(crate) fn ensure_disjoint(
    source: &Buffer,
    source_offset: DeviceSize,
    target: &Buffer,
    target_offset: DeviceSize,
    length: DeviceSize,
) -> Result<()> {
    if source.overlap(source_offset, length, target, target_offset, length) == Overlap::Disjoint {
        Ok(())
    } else {
        Err(Error::new(
            Code::InvalidArgument,
            "source and target ranges must not overlap",
        ))
    }
}

/// Scoped host mapping of a buffer range; unmapped on drop.
pub struct MappedRange<'a> {
    mapping: n::iree_hal_buffer_mapping_t,
    access: MemoryAccess,
    _buffer: PhantomData<&'a Buffer>,
}
impl MappedRange<'_> {
    /// Returns the mapped bytes when read access was requested.
    pub fn as_bytes(&self) -> Result<&[u8]> {
        self.require(MemoryAccess::READ, "mapping was not opened for reading")?;
        // SAFETY: the guard keeps the mapping live; `map_range`'s contract makes it initialized and unaliased.
        Ok(unsafe {
            n::slice(
                self.mapping.contents.data,
                self.mapping.contents.data_length,
            )
        })
    }
    /// Returns initialized mapped bytes mutably when read and write access
    /// were requested.
    ///
    /// Write-only mappings may contain uninitialized bytes. Use
    /// [`Self::as_uninit_bytes_mut`] to initialize those without forming
    /// references to invalid `u8` values.
    pub fn as_bytes_mut(&mut self) -> Result<&mut [u8]> {
        self.require(
            MemoryAccess::READ | MemoryAccess::WRITE,
            "initialized mutable bytes require read and write access",
        )?;
        let contents = self.mapping.contents;
        if contents.data_length == 0 {
            return Ok(&mut []);
        }
        // SAFETY: read access requires initialization; map_range requires exclusivity.
        Ok(unsafe { n::slice_mut(contents.data, contents.data_length) })
    }
    /// Returns writable mapped storage without assuming its bytes initialized.
    pub fn as_uninit_bytes_mut(&mut self) -> Result<&mut [MaybeUninit<u8>]> {
        self.require(MemoryAccess::WRITE, "mapping was not opened for writing")?;
        let contents = self.mapping.contents;
        if contents.data_length == 0 {
            return Ok(&mut []);
        }
        // SAFETY: map_range guarantees exclusive access to this writable span;
        // MaybeUninit does not require the storage to contain initialized bytes.
        Ok(unsafe { core::slice::from_raw_parts_mut(contents.data.cast(), contents.data_length) })
    }
    /// Views mapped bytes as plain-data elements, checking alignment and length.
    pub fn as_slice<T: bytemuck::Pod>(&self) -> Result<&[T]> {
        super::pod_element_size::<T>()?;
        bytemuck::try_cast_slice(self.as_bytes()?).map_err(super::pod_cast_error)
    }
    /// Mutably views mapped bytes as plain-data elements, checking alignment and length.
    pub fn as_slice_mut<T: bytemuck::Pod>(&mut self) -> Result<&mut [T]> {
        super::pod_element_size::<T>()?;
        bytemuck::try_cast_slice_mut(self.as_bytes_mut()?).map_err(super::pod_cast_error)
    }
    /// Flushes a mapped byte range to device-visible memory.
    pub fn flush(&mut self, offset: DeviceSize, length: DeviceSize) -> Result<()> {
        // SAFETY: the guard owns the live mapping record.
        check(unsafe { n::iree_hal_buffer_mapping_flush_range(&mut self.mapping, offset, length) })
    }
    /// Invalidates a mapped range to make device writes visible to the host.
    pub fn invalidate(&mut self, offset: DeviceSize, length: DeviceSize) -> Result<()> {
        // SAFETY: the guard owns the live mapping record.
        check(unsafe {
            n::iree_hal_buffer_mapping_invalidate_range(&mut self.mapping, offset, length)
        })
    }
    /// Unmaps the range and reports any native failure.
    pub fn unmap(self) -> Result<()> {
        let mut this = ManuallyDrop::new(self);
        // SAFETY: the guard owns the live mapping and is not dropped afterwards.
        check(unsafe { n::iree_hal_buffer_unmap_range(&mut this.mapping) })
    }
    fn require(&self, access: MemoryAccess, message: &'static str) -> Result<()> {
        if self.access.contains(access) {
            Ok(())
        } else {
            Err(Error::new(Code::FailedPrecondition, message))
        }
    }
}
impl Drop for MappedRange<'_> {
    fn drop(&mut self) {
        // SAFETY: the guard owns the live mapping and releases it once; `unmap` reports errors.
        let _ = unsafe { n::iree_hal_buffer_unmap_range(&mut self.mapping) };
    }
}
