//! Device-transferable files.
use super::{QueueAffinity, buffer::Buffer, device::Device, flags::MemoryAccess};
use crate::hal::DeviceSize;
use crate::io::{FileAccess, FileHandle};
use crate::{Result, check};
use core::marker::PhantomData;
use iree_native as n;

/// An owned `iree_hal_file_t` reference, retaining its imported file handle.
pub struct File<'a> {
    raw: *mut n::iree_hal_file_t,
    handle: Option<FileHandle<'a>>,
    _not_thread_safe: PhantomData<*mut ()>,
}
impl<'a> File<'a> {
    /// Imports a file handle for use on `device`; the file retains the handle.
    pub fn import(
        device: &Device,
        queue_affinity: QueueAffinity,
        access: MemoryAccess,
        handle: &FileHandle<'a>,
    ) -> Result<Self> {
        require_access(handle, access)?;
        let mut raw = core::ptr::null_mut();
        // SAFETY: the device and handle are live; the handle is retained by the result.
        check(unsafe {
            n::iree_hal_file_import(
                device.as_raw(),
                queue_affinity.bits(),
                access.bits(),
                handle.as_raw(),
                n::IREE_HAL_EXTERNAL_FILE_FLAG_NONE as n::iree_hal_external_file_flags_t,
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            handle: Some(handle.clone()),
            _not_thread_safe: PhantomData,
        })
    }
    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_file_t {
        self.raw
    }
    /// Transfers ownership of the native handle to the caller.
    ///
    /// The native file retains the imported handle itself, so the Rust-side
    /// handle reference is released here rather than leaked.
    pub fn into_raw(mut self) -> *mut n::iree_hal_file_t {
        drop(self.handle.take());
        core::mem::ManuallyDrop::new(self).raw
    }
    /// Returns the access flags allowed by the imported file.
    pub fn allowed_access(&self) -> MemoryAccess {
        // SAFETY: the file is live.
        MemoryAccess::from_bits_retain(unsafe { n::iree_hal_file_allowed_access(self.raw) })
    }
    /// Returns the accessible file length in bytes.
    pub fn len(&self) -> u64 {
        // SAFETY: the file is live.
        unsafe { n::iree_hal_file_length(self.raw) }
    }
    /// Returns whether synchronous I/O is supported.
    pub fn supports_synchronous_io(&self) -> bool {
        // SAFETY: the file is live.
        unsafe { n::iree_hal_file_supports_synchronous_io(self.raw) }
    }
    /// Returns whether the accessible range is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Validates that this file permits the requested access.
    pub fn validate_access(&self, access: MemoryAccess) -> Result<()> {
        // SAFETY: the file is live.
        check(unsafe { n::iree_hal_file_validate_access(self.raw, access.bits()) })
    }
    /// Reads a file range into a host-mappable device buffer synchronously.
    pub fn read(
        &self,
        file_offset: u64,
        buffer: &Buffer,
        buffer_offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        // SAFETY: the file and buffer are live; IREE validates the ranges.
        check(unsafe {
            n::iree_hal_file_read(
                self.raw,
                file_offset,
                buffer.as_raw(),
                buffer_offset,
                length,
            )
        })
    }
    /// Writes a host-mappable device buffer range to this file synchronously.
    pub fn write(
        &self,
        file_offset: u64,
        buffer: &Buffer,
        buffer_offset: DeviceSize,
        length: DeviceSize,
    ) -> Result<()> {
        // SAFETY: the file and buffer are live; IREE validates the ranges.
        check(unsafe {
            n::iree_hal_file_write(
                self.raw,
                file_offset,
                buffer.as_raw(),
                buffer_offset,
                length,
            )
        })
    }
}
impl File<'static> {
    /// Returns a retained storage buffer, if the backend exposes one.
    ///
    /// Only files owning their backing storage can expose an independently
    /// retained buffer; a borrowed host allocation cannot outlive its borrow.
    ///
    /// ```compile_fail
    /// use ireert::hal::file::File;
    /// fn escape(file: &File<'_>) {
    ///     let _ = file.storage_buffer();
    /// }
    /// ```
    pub fn storage_buffer(&self) -> Option<Buffer> {
        // SAFETY: the file is live; the result is borrowed or null.
        let raw = unsafe { n::iree_hal_file_storage_buffer(self.raw) };
        (!raw.is_null()).then(|| {
            // SAFETY: the native buffer retains the file's owned storage.
            unsafe { n::iree_hal_buffer_retain(raw) };
            // SAFETY: the reference was just retained.
            unsafe { Buffer::from_raw(raw) }
        })
    }

    /// Adopts one owned file reference without an imported file-handle dependency.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_file_t) -> Self {
        Self {
            raw,
            handle: None,
            _not_thread_safe: PhantomData,
        }
    }
}
impl Clone for File<'_> {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_file_retain(self.raw) };
        Self {
            raw: self.raw,
            handle: self.handle.clone(),
            _not_thread_safe: PhantomData,
        }
    }
}
impl Drop for File<'_> {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_file_release(self.raw) }
    }
}

/// An owned `iree_hal_file_cache_t`: deduplicates imports of the same file
/// handle per device, queue affinity and access.
pub struct FileCache {
    raw: *mut n::iree_hal_file_cache_t,
    _not_thread_safe: PhantomData<*mut ()>,
}

impl FileCache {
    /// Creates an empty cache using the system allocator.
    pub fn new() -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: `raw` is writable.
        check(unsafe { n::iree_hal_file_cache_create(crate::system(), &mut raw) })?;
        Ok(Self {
            raw,
            _not_thread_safe: PhantomData,
        })
    }

    /// Returns the cached import of `handle`, importing it on a miss.
    pub fn lookup<'a>(
        &self,
        device: &Device,
        queue_affinity: QueueAffinity,
        access: MemoryAccess,
        handle: &FileHandle<'a>,
    ) -> Result<File<'a>> {
        require_access(handle, access)?;
        let mut raw = core::ptr::null_mut();
        // SAFETY: the cache, device and handle are live; the handle is retained by the result.
        check(unsafe {
            n::iree_hal_file_cache_lookup(
                self.raw,
                device.as_raw(),
                queue_affinity.bits(),
                access.bits(),
                handle.as_raw(),
                n::IREE_HAL_EXTERNAL_FILE_FLAG_NONE as n::iree_hal_external_file_flags_t,
                &mut raw,
            )
        })?;
        Ok(File {
            raw,
            handle: Some(handle.clone()),
            _not_thread_safe: PhantomData,
        })
    }

    /// Drops cache entries not referenced outside the cache.
    pub fn trim(&self) {
        // SAFETY: the cache is live.
        unsafe { n::iree_hal_file_cache_trim(self.raw) }
    }

    /// Returns the borrowed native handle.
    pub fn as_raw(&self) -> *mut n::iree_hal_file_cache_t {
        self.raw
    }

    /// Transfers ownership of the native reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_file_cache_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Adopts one owned cache reference.
    ///
    /// # Safety
    /// `raw` must be a non-null owned reference that may be released exactly once.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_file_cache_t) -> Self {
        Self {
            raw,
            _not_thread_safe: PhantomData,
        }
    }
}

impl Clone for FileCache {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_file_cache_retain(self.raw) };
        Self {
            raw: self.raw,
            _not_thread_safe: PhantomData,
        }
    }
}

impl Drop for FileCache {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_file_cache_release(self.raw) }
    }
}

fn require_access(handle: &FileHandle<'_>, access: MemoryAccess) -> Result<()> {
    let mut required = FileAccess::default();
    if access.contains(MemoryAccess::READ) {
        required |= FileAccess::READ;
    }
    if access.contains(MemoryAccess::WRITE) {
        required |= FileAccess::WRITE;
    }
    handle.require_access(required)
}
