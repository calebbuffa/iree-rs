//! Host-accessible views of file-handle ranges.
//!
//! Laws:
//! - A mapping retains its file handle and never outlives the memory `'a`.
//! - Construction is `unsafe`: file mutations and overlapping writable
//!   mappings must not invalidate Rust references into mapped memory.
use super::file_handle::{FileAccess, FileHandle};
use crate::hal::bitfield::bitfield;
use crate::{Result, check};
use core::marker::PhantomData;
use iree_native as n;

bitfield! {
    /// File-mapping hints.
    MappingFlags: u64 {
        /// Advise sequential access.
        SEQUENTIAL_ACCESS = n::IREE_IO_FILE_MAPPING_FLAG_SEQUENTIAL_ACCESS,
        /// Request large pages.
        LARGE_PAGES = n::IREE_IO_FILE_MAPPING_FLAG_LARGE_PAGES,
        /// Exclude mapped data from process dumps.
        EXCLUDE_FROM_DUMPS = n::IREE_IO_FILE_MAPPING_FLAG_EXCLUDE_FROM_DUMPS,
        /// Create a private copy-on-write mapping.
        PRIVATE = n::IREE_IO_FILE_MAPPING_FLAG_PRIVATE,
    }
}

/// An owned `iree_io_file_mapping_t` reference.
pub struct FileMapping<'a> {
    raw: *mut n::iree_io_file_mapping_t,
    readable: bool,
    writable: bool,
    _memory: PhantomData<FileHandle<'a>>,
}

impl<'a> FileMapping<'a> {
    /// Maps `length` bytes (or the remainder when `None`) at `offset` read-only.
    ///
    /// # Safety
    /// The mapped bytes must not be mutated through any alias for the mapping's
    /// lifetime. This includes other handles or processes modifying the backing
    /// file.
    pub unsafe fn map(
        handle: &FileHandle<'a>,
        offset: u64,
        length: Option<usize>,
        flags: MappingFlags,
    ) -> Result<Self> {
        // SAFETY: the caller excludes mutations through every alias.
        unsafe { Self::map_with_access(handle, FileAccess::READ, offset, length, flags) }
    }

    /// Maps a range with the requested access.
    ///
    /// # Safety
    /// No alias may mutate the mapped bytes for this mapping's lifetime. With
    /// write access, no other mapping of the same range may be accessed while
    /// this mapping's mutable slice is borrowed.
    pub unsafe fn map_with_access(
        handle: &FileHandle<'a>,
        access: FileAccess,
        offset: u64,
        length: Option<usize>,
        flags: MappingFlags,
    ) -> Result<Self> {
        handle.require_access(access)?;
        let mut raw = core::ptr::null_mut();
        // SAFETY: the handle is live and native retains it for the mapping.
        check(unsafe {
            n::iree_io_file_map_view(
                handle.as_raw(),
                access.bits(),
                offset,
                length.unwrap_or(n::iree_host_size_t::MAX),
                flags.bits(),
                crate::system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            readable: access.contains(FileAccess::READ),
            writable: access.contains(FileAccess::WRITE),
            _memory: PhantomData,
        })
    }

    /// Adopts one owned readable mapping reference.
    ///
    /// # Safety
    /// `raw` must be an owned readable mapping reference valid for `'a`.
    pub unsafe fn from_raw(raw: *mut n::iree_io_file_mapping_t) -> Self {
        Self {
            raw,
            readable: true,
            writable: false,
            _memory: PhantomData,
        }
    }

    /// Borrows the native mapping.
    pub fn as_raw(&self) -> *mut n::iree_io_file_mapping_t {
        self.raw
    }

    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_io_file_mapping_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Borrows the bytes when the mapping is readable.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        if !self.readable {
            return None;
        }
        // SAFETY: the mapping is live and readable for its whole length.
        let span = unsafe { n::iree_io_file_mapping_contents_ro(self.raw) };
        // SAFETY: The live readable mapping owns span.data for the returned slice.
        Some(unsafe { n::slice(span.data, span.data_length) })
    }

    /// Mutably borrows the bytes when the mapping is writable.
    pub fn as_mut_slice(&mut self) -> Option<&mut [u8]> {
        if !self.writable {
            return None;
        }
        // SAFETY: the mapping is live and writable; `&mut self` excludes aliases.
        let span = unsafe { n::iree_io_file_mapping_contents_rw(self.raw) };
        // SAFETY: The live writable mapping owns span.data and &mut self excludes aliased access.
        Some(unsafe { n::slice_mut(span.data, span.data_length) })
    }

    /// Retains a read-only mapping; writable mappings cannot be shared.
    pub fn try_clone(&self) -> Option<Self> {
        if self.writable || !self.readable {
            return None;
        }
        // SAFETY: the mapping is live.
        unsafe { n::iree_io_file_mapping_retain(self.raw) };
        Some(Self {
            raw: self.raw,
            readable: true,
            writable: false,
            _memory: PhantomData,
        })
    }
}

impl Drop for FileMapping<'_> {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one reference.
        unsafe { n::iree_io_file_mapping_release(self.raw) }
    }
}
