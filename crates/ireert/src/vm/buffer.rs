//! VM byte buffers.
use super::{Instance, Ref};
use crate::hal::bitfield::bitfield;
use crate::{Code, Error, Result, check};
use iree_native as n;
use n::{slice, slice_mut};

bitfield! {
    /// VM buffer access and origin flags.
    BufferAccess: n::iree_vm_buffer_access_t {
        /// Allows writes.
        MUTABLE = n::IREE_VM_BUFFER_ACCESS_MUTABLE,
        /// Storage originates from a module.
        ORIGIN_MODULE = n::IREE_VM_BUFFER_ACCESS_ORIGIN_MODULE,
        /// Storage originates from guest code.
        ORIGIN_GUEST = n::IREE_VM_BUFFER_ACCESS_ORIGIN_GUEST,
        /// Storage originates from the host.
        ORIGIN_HOST = n::IREE_VM_BUFFER_ACCESS_ORIGIN_HOST,
    }
}

/// A reference-counted VM byte buffer.
///
/// Clones share one native buffer.
pub struct Buffer(*mut n::iree_vm_buffer_t);

impl Buffer {
    /// Allocates a zeroed buffer of `length` bytes.
    ///
    /// `instance` proves the `vm.buffer` type is registered; upstream
    /// registers it per instance.
    pub fn new(
        _instance: &Instance,
        access: BufferAccess,
        length: usize,
        alignment: usize,
    ) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the output is writable.
        check(unsafe {
            n::iree_vm_buffer_create(access.bits(), length, alignment, crate::system(), &mut raw)
        })?;
        Ok(Self(raw))
    }

    /// Adopts an owned VM buffer pointer.
    ///
    /// # Safety
    /// `raw` must be a live, owned IREE VM buffer pointer.
    pub unsafe fn from_raw(raw: *mut n::iree_vm_buffer_t) -> Self {
        Self(raw)
    }

    /// Borrows the native buffer pointer.
    pub fn as_raw(&self) -> *mut n::iree_vm_buffer_t {
        self.0
    }

    /// Transfers the owned pointer to the caller.
    pub fn into_raw(self) -> *mut n::iree_vm_buffer_t {
        std::mem::ManuallyDrop::new(self).0
    }

    /// Returns the buffer held by `reference`.
    pub fn from_ref(reference: &Ref) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        // SAFETY: the reference is valid; the result is type-checked.
        check(unsafe { n::iree_vm_buffer_check_deref(reference.raw(), &mut raw) })?;
        // SAFETY: `raw` is live; retaining gives the result its own reference.
        unsafe { n::iree_vm_buffer_retain(raw) };
        Ok(Self(raw))
    }

    /// Returns a new VM reference to this buffer.
    pub fn to_ref(&self) -> Result<Ref> {
        let mut raw = n::iree_vm_ref_t::default();
        // SAFETY: the buffer is live; the output receives a retained reference.
        check(unsafe { n::iree_vm_buffer_wrap_retain(self.0, &mut raw) })?;
        // SAFETY: the reference was retained for us.
        Ok(unsafe { Ref::from_raw(raw) })
    }

    /// Returns the access and origin flags.
    pub fn access(&self) -> BufferAccess {
        // SAFETY: the buffer is live and `access` is immutable after creation.
        BufferAccess::from_bits_retain(unsafe { (*self.0).access })
    }

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        // SAFETY: the buffer is live.
        unsafe { n::iree_vm_buffer_length(self.0) }
    }

    /// Returns whether the buffer has no bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies `length` bytes at `offset` into a new buffer.
    pub fn clone_range(
        &self,
        access: BufferAccess,
        offset: usize,
        length: usize,
        alignment: usize,
    ) -> Result<Self> {
        end(offset, length)?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: the buffer is live and the range does not overflow.
        check(unsafe {
            n::iree_vm_buffer_clone(
                access.bits(),
                self.0,
                offset,
                length,
                alignment,
                crate::system(),
                &mut raw,
            )
        })?;
        Ok(Self(raw))
    }

    /// Copies `length` bytes into `target`.
    pub fn copy_to(
        &self,
        source_offset: usize,
        target: &Buffer,
        target_offset: usize,
        length: usize,
    ) -> Result<()> {
        end(source_offset, length)?;
        end(target_offset, length)?;
        // SAFETY: both buffers are live and the ranges do not overflow.
        check(unsafe {
            n::iree_vm_buffer_copy_bytes(self.0, source_offset, target.0, target_offset, length)
        })
    }

    /// Returns whether two byte ranges are equal.
    pub fn compare(
        &self,
        offset: usize,
        other: &Buffer,
        other_offset: usize,
        length: usize,
    ) -> Result<bool> {
        end(offset, length)?;
        end(other_offset, length)?;
        let mut equal = false;
        // SAFETY: both buffers are live and the ranges do not overflow.
        check(unsafe {
            n::iree_vm_buffer_compare_bytes(
                self.0,
                offset,
                other.0,
                other_offset,
                length,
                &mut equal,
            )
        })?;
        Ok(equal)
    }

    /// Borrows `length` bytes at `offset`.
    ///
    /// `offset` and `length` must be multiples of the power-of-two `alignment`.
    ///
    /// # Safety
    /// No clone of this buffer, and no VM code, may write it while the slice lives.
    pub unsafe fn map(&self, offset: usize, length: usize, alignment: usize) -> Result<&[u8]> {
        aligned(offset, length, alignment)?;
        let mut span = n::iree_const_byte_span_t::default();
        // SAFETY: the buffer is live and the range is aligned and does not overflow.
        check(unsafe { n::iree_vm_buffer_map_ro(self.0, offset, length, alignment, &mut span) })?;
        // SAFETY: the span lies inside the buffer; the caller excludes writers.
        Ok(unsafe { slice(span.data, span.data_length) })
    }

    /// Mutably borrows `length` bytes at `offset` of a [`BufferAccess::MUTABLE`] buffer.
    ///
    /// `offset` and `length` must be multiples of the power-of-two `alignment`.
    ///
    /// # Safety
    /// No clone of this buffer, and no VM code, may access it while the slice lives.
    pub unsafe fn map_mut(
        &mut self,
        offset: usize,
        length: usize,
        alignment: usize,
    ) -> Result<&mut [u8]> {
        aligned(offset, length, alignment)?;
        let mut span = n::iree_byte_span_t::default();
        // SAFETY: the buffer is live and the range is aligned and does not overflow.
        check(unsafe { n::iree_vm_buffer_map_rw(self.0, offset, length, alignment, &mut span) })?;
        // SAFETY: the span lies inside the buffer; the caller excludes other access.
        Ok(unsafe { slice_mut(span.data, span.data_length) })
    }

    /// Copies bytes at `offset` into `output`.
    pub fn read(&self, offset: usize, output: &mut [u8]) -> Result<()> {
        end(offset, output.len())?;
        // SAFETY: the buffer is live, the range does not overflow, and
        // `output` is writable for its length.
        check(unsafe {
            n::iree_vm_buffer_read_elements(
                self.0,
                offset,
                output.as_mut_ptr().cast(),
                output.len(),
                1,
            )
        })
    }

    /// Copies `input` into the buffer at `offset`.
    pub fn write(&mut self, offset: usize, input: &[u8]) -> Result<()> {
        end(offset, input.len())?;
        // SAFETY: the buffer is live, the range does not overflow, and
        // `input` is readable for its length.
        check(unsafe {
            n::iree_vm_buffer_write_elements(input.as_ptr().cast(), self.0, offset, input.len(), 1)
        })
    }

    /// Sets `length` bytes at `offset` to `value`.
    pub fn fill(&mut self, offset: usize, length: usize, value: u8) -> Result<()> {
        end(offset, length)?;
        // SAFETY: the buffer is live and the range does not overflow.
        check(unsafe { n::iree_vm_buffer_fill_bytes(self.0, offset, length, value) })
    }

    /// Repeats a 1-, 2-, 4- or 8-byte `pattern` over `count` elements,
    /// starting at element `offset`.
    pub fn fill_elements(&mut self, offset: usize, count: usize, pattern: &[u8]) -> Result<()> {
        let width = pattern.len();
        if !matches!(width, 1 | 2 | 4 | 8) {
            return Err(Error::new(
                Code::InvalidArgument,
                "fill patterns must be 1, 2, 4 or 8 bytes",
            ));
        }
        let overflow = || Error::new(Code::OutOfRange, "VM buffer range overflows");
        end(
            offset.checked_mul(width).ok_or_else(overflow)?,
            count.checked_mul(width).ok_or_else(overflow)?,
        )?;
        // Native reads the pattern as an aligned integer of `width` bytes.
        let mut aligned = [0u64];
        let bytes: &mut [u8] = bytemuck::cast_slice_mut(&mut aligned);
        bytes[..width].copy_from_slice(pattern);
        // SAFETY: the buffer is live, the range does not overflow, and the
        // pattern is aligned and readable for `width` bytes.
        check(unsafe {
            n::iree_vm_buffer_fill_elements(self.0, offset, count, width, aligned.as_ptr().cast())
        })
    }

    /// Returns IREE's hash of `length` bytes at `offset`.
    pub fn hash(&self, offset: usize, length: usize) -> Result<i64> {
        end(offset, length)?;
        let mut hash = 0;
        // SAFETY: the buffer is live and the range does not overflow.
        check(unsafe { n::iree_vm_buffer_hash(self.0, offset, length, &mut hash) })?;
        Ok(hash)
    }
}

/// Upstream adds offsets and lengths without overflow checks before bounds
/// checking, so a wrapped end would pass.
fn end(offset: usize, length: usize) -> Result<usize> {
    offset
        .checked_add(length)
        .ok_or_else(|| Error::new(Code::OutOfRange, "VM buffer range overflows"))
}

/// Upstream silently rounds unaligned ranges down and does not validate
/// the alignment.
fn aligned(offset: usize, length: usize, alignment: usize) -> Result<()> {
    if !alignment.is_power_of_two()
        || !offset.is_multiple_of(alignment)
        || !length.is_multiple_of(alignment)
    {
        return Err(Error::new(
            Code::InvalidArgument,
            "VM buffer ranges must be multiples of a power-of-two alignment",
        ));
    }
    end(offset, length).map(drop)
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        // SAFETY: `self.0` is a live ref-counted buffer.
        unsafe { n::iree_vm_buffer_retain(self.0) };
        Self(self.0)
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one buffer reference.
        unsafe { n::iree_vm_buffer_release(self.0) }
    }
}
