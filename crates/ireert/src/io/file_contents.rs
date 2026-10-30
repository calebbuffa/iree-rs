//! Owned host-readable file contents loaded or mapped by IREE.
use super::file_handle::FileAccess;
use crate::{Result, check};
use core::marker::PhantomData;
use iree_native as n;

/// File contents whose storage is released with the upstream contents API.
pub struct FileContents {
    raw: *mut n::iree_io_file_contents_t,
    readable: bool,
    mutable: bool,
    _not_sync: PhantomData<*mut ()>,
}

impl FileContents {
    /// Reads stdin through EOF into an owned, NUL-terminated buffer.
    pub fn read_stdin() -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: allocator and output storage are valid for the call.
        check(unsafe { n::iree_io_file_contents_read_stdin(crate::system(), &mut raw) })?;
        // SAFETY: successful read returns one owned contents allocation.
        Ok(unsafe { Self::from_raw(raw, true, true) })
    }

    /// Reads a file into an owned copy.
    pub fn read(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view, allocator, and output storage are valid for the call.
        check(unsafe {
            n::iree_io_file_contents_read(crate::view(path), crate::system(), &mut raw)
        })?;
        // SAFETY: successful read returns one owned contents allocation.
        Ok(unsafe { Self::from_raw(raw, true, true) })
    }

    /// Maps a file into host memory with the requested access.
    ///
    /// # Safety
    /// No alias may mutate the mapped file bytes while this value is live.
    /// With write access, no other mapping may access the same file range.
    pub unsafe fn map(path: impl AsRef<std::path::Path>, access: FileAccess) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view, allocator, and output storage are valid for the call.
        check(unsafe {
            n::iree_io_file_contents_map(
                crate::view(path),
                access.bits(),
                crate::system(),
                &mut raw,
            )
        })?;
        // SAFETY: successful mapping returns one owned contents allocation.
        Ok(unsafe {
            Self::from_raw(
                raw,
                access.contains(FileAccess::READ),
                access.contains(FileAccess::WRITE),
            )
        })
    }

    /// Writes all `contents` to `path`, replacing existing data.
    pub fn write(path: impl AsRef<std::path::Path>, contents: &[u8]) -> Result<()> {
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view and byte slice remain live for the synchronous write.
        check(unsafe {
            n::iree_io_file_contents_write(
                crate::view(path),
                n::const_span(contents),
                crate::system(),
            )
        })
    }

    /// Returns the native allocator that frees this contents allocation.
    ///
    /// # Safety
    /// Use the returned allocator only while this `FileContents` is live. If
    /// it is used to free the contents, do not access or drop this wrapper
    /// afterward.
    pub unsafe fn deallocator(&self) -> n::iree_allocator_t {
        // SAFETY: the contents allocation is live and valid for this query.
        unsafe { n::iree_io_file_contents_deallocator(self.raw) }
    }

    /// Borrows the native file contents pointer.
    pub fn as_raw(&self) -> *mut n::iree_io_file_contents_t {
        self.raw
    }

    /// Transfers the owned contents allocation to the caller.
    pub fn into_raw(self) -> *mut n::iree_io_file_contents_t {
        let this = core::mem::ManuallyDrop::new(self);
        this.raw
    }

    /// Adopts one owned native contents allocation.
    ///
    /// # Safety
    /// `raw` must be a live pointer returned by an IREE file-contents API with
    /// exactly one owned allocation reference. `readable` and `mutable` must
    /// reflect the access and storage represented by that allocation.
    pub unsafe fn from_raw(
        raw: *mut n::iree_io_file_contents_t,
        readable: bool,
        mutable: bool,
    ) -> Self {
        Self {
            raw,
            readable,
            mutable,
            _not_sync: PhantomData,
        }
    }

    /// Returns the number of loaded or mapped bytes.
    pub fn len(&self) -> usize {
        // SAFETY: the contents allocation is live and initialized by IREE.
        unsafe {
            if self.mutable {
                (*self.raw).__bindgen_anon_1.buffer.data_length
            } else {
                (*self.raw).__bindgen_anon_1.const_buffer.data_length
            }
        }
    }

    /// Returns whether there are no loaded or mapped bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrows the file bytes for the lifetime of this owner when readable.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        if !self.readable {
            return None;
        }
        // SAFETY: the contents allocation is live and initialized by IREE.
        let (data, length) = unsafe {
            if self.mutable {
                let span = (*self.raw).__bindgen_anon_1.buffer;
                (span.data.cast_const(), span.data_length)
            } else {
                let span = (*self.raw).__bindgen_anon_1.const_buffer;
                (span.data, span.data_length)
            }
        };
        // SAFETY: the contents object owns a readable allocation of this length.
        Some(unsafe { n::slice(data, length) })
    }

    /// Mutably borrows owned bytes or a writable mapping.
    pub fn as_mut_bytes(&mut self) -> Option<&mut [u8]> {
        if !self.mutable {
            return None;
        }
        // SAFETY: mutable access is enabled only for an owned buffer or a write mapping;
        // the exclusive borrow prevents aliases through this wrapper.
        let span = unsafe { (*self.raw).__bindgen_anon_1.buffer };
        // SAFETY: the contents object owns writable storage of this length.
        Some(unsafe { n::slice_mut(span.data, span.data_length) })
    }
}

impl Drop for FileContents {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one `iree_io_file_contents_t` allocation.
        unsafe { n::iree_io_file_contents_free(self.raw) }
    }
}
