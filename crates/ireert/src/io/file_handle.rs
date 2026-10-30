//! Reference-counted I/O file handles over platform files or host memory.
//!
//! Laws:
//! - `FileHandle<'a>` never outlives the host memory it wraps; owned sources
//!   (`create`, `open`, `preload`, `from_bytes`, `from_owner`) are `'static`.
//! - APIs that retain a handle beyond the call require `FileHandle<'static>`.
use crate::hal::bitfield::bitfield;
use crate::{Result, check, system};
use core::marker::PhantomData;
use iree_native as n;
#[cfg(unix)]
use std::os::fd::{AsRawFd, BorrowedFd, OwnedFd};

bitfield! {
    /// File-handle access permissions.
    FileAccess: u32 {
        /// Read access.
        READ = n::IREE_IO_FILE_ACCESS_READ,
        /// Write access.
        WRITE = n::IREE_IO_FILE_ACCESS_WRITE,
    }
}

bitfield! {
    /// Platform file open and creation flags.
    FileMode: u64 {
        /// Read access.
        READ = n::IREE_IO_FILE_MODE_READ,
        /// Write access.
        WRITE = n::IREE_IO_FILE_MODE_WRITE,
        /// Random-access hint.
        RANDOM_ACCESS = n::IREE_IO_FILE_MODE_RANDOM_ACCESS,
        /// Sequential-scan hint.
        SEQUENTIAL_SCAN = n::IREE_IO_FILE_MODE_SEQUENTIAL_SCAN,
        /// Direct-I/O hint.
        DIRECT = n::IREE_IO_FILE_MODE_DIRECT,
        /// Delete the file when it is closed.
        TEMPORARY = n::IREE_IO_FILE_MODE_TEMPORARY,
        /// Permit readers to open while the creator holds the file.
        SHARE_READ = n::IREE_IO_FILE_MODE_SHARE_READ,
        /// Permit writers to open while the creator holds the file.
        SHARE_WRITE = n::IREE_IO_FILE_MODE_SHARE_WRITE,
        /// Truncate an existing file on creation.
        OVERWRITE = n::IREE_IO_FILE_MODE_OVERWRITE,
        /// Open with platform asynchronous-I/O support.
        ASYNC = n::IREE_IO_FILE_MODE_ASYNC,
    }
}

/// The platform primitive backing a file handle.
#[derive(Debug)]
pub enum FileHandlePrimitive<'a> {
    /// Host memory of the given byte length.
    HostAllocation {
        /// Length of the allocation in bytes.
        length: usize,
    },
    /// A borrowed platform file descriptor.
    #[cfg(unix)]
    FileDescriptor(BorrowedFd<'a>),
    /// A C runtime file descriptor.
    #[cfg(not(unix))]
    FileDescriptor(i32),
    /// A primitive type unknown to this binding version.
    Unknown(n::iree_io_file_handle_type_t),
    /// Uninhabited; carries the borrow on platforms without borrowed descriptors.
    #[cfg(not(unix))]
    #[doc(hidden)]
    _Borrow(core::marker::PhantomData<&'a ()>, core::convert::Infallible),
}

/// An owned `iree_io_file_handle_t` reference.
pub struct FileHandle<'a> {
    raw: *mut n::iree_io_file_handle_t,
    _memory: PhantomData<&'a mut [u8]>,
}

impl FileHandle<'static> {
    /// Creates a file at `path` with the given initial byte length.
    pub fn create(
        path: impl AsRef<std::path::Path>,
        mode: FileMode,
        initial_size: u64,
    ) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view, allocator, and output pointer are valid for the call.
        check(unsafe {
            n::iree_io_file_handle_create(
                mode.bits(),
                crate::view(path),
                initial_size,
                system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Opens an existing file.
    pub fn open(path: impl AsRef<std::path::Path>, mode: FileMode) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view, allocator, and output pointer are valid for the call.
        check(unsafe {
            n::iree_io_file_handle_open(mode.bits(), crate::view(path), system(), &mut raw)
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Opens a file and preloads its contents into wired host memory.
    pub fn preload(path: impl AsRef<std::path::Path>, mode: FileMode) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        let path = super::path_str(path.as_ref())?;
        // SAFETY: path view, allocator, and output pointer are valid for the call.
        check(unsafe {
            n::iree_io_file_handle_preload(mode.bits(), crate::view(path), system(), &mut raw)
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Duplicates a borrowed platform file descriptor.
    #[cfg(unix)]
    pub fn open_fd(fd: BorrowedFd<'_>, mode: FileMode) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: native duplicates the descriptor, which is live for the call.
        check(unsafe {
            n::iree_io_file_handle_open_fd(mode.bits(), fd.as_raw_fd(), system(), &mut raw)
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Transfers ownership of a descriptor; it is closed on final release.
    #[cfg(unix)]
    pub fn from_owned_fd(fd: OwnedFd, access: FileAccess) -> Result<Self> {
        let primitive = n::iree_io_file_handle_primitive_t {
            type_: n::IREE_IO_FILE_HANDLE_TYPE_FD,
            value: n::iree_io_file_handle_primitive_value_t { fd: fd.as_raw_fd() },
        };
        Self::wrap(access, primitive, fd)
    }

    /// Takes ownership of host memory, freed on final native release.
    pub fn from_bytes(mut bytes: Box<[u8]>, access: FileAccess) -> Result<Self> {
        // Moving the box keeps its heap allocation in place.
        let primitive = host_allocation(bytes.as_mut_ptr(), bytes.len());
        Self::wrap(access, primitive, bytes)
    }

    /// Wraps immutable bytes read-only without copying; `owner` is dropped on
    /// final native release.
    pub fn from_owner<T: AsRef<[u8]> + Send + 'static>(owner: T) -> Result<Self> {
        let owner = Box::new(owner);
        let bytes = (*owner).as_ref();
        // Moving the outer box keeps `T` and the bytes it exposes in place.
        let primitive = host_allocation(bytes.as_ptr().cast_mut(), bytes.len());
        Self::wrap(FileAccess::READ, primitive, owner)
    }

    fn wrap<T: Send + 'static>(
        access: FileAccess,
        primitive: n::iree_io_file_handle_primitive_t,
        owner: T,
    ) -> Result<Self> {
        let owner = Box::into_raw(Box::new(owner));
        let mut raw = core::ptr::null_mut();
        // SAFETY: `owner` keeps the primitive live until the release callback
        // drops it; native never calls the callback when wrapping fails.
        let status = check(unsafe {
            n::iree_io_file_handle_wrap(
                access.bits(),
                n::IREE_IO_FILE_MODE_NONE as _,
                primitive,
                n::iree_io_file_handle_release_callback_t {
                    fn_: Some(release_owner::<T>),
                    user_data: owner.cast(),
                },
                system(),
                &mut raw,
            )
        });
        if let Err(error) = status {
            // SAFETY: wrapping failed, so ownership never left this function.
            drop(unsafe { Box::from_raw(owner) });
            return Err(error);
        }
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }
}

fn host_allocation(data: *mut u8, data_length: usize) -> n::iree_io_file_handle_primitive_t {
    n::iree_io_file_handle_primitive_t {
        type_: n::IREE_IO_FILE_HANDLE_TYPE_HOST_ALLOCATION,
        value: n::iree_io_file_handle_primitive_value_t {
            host_allocation: n::iree_byte_span_t { data, data_length },
        },
    }
}

unsafe extern "C" fn release_owner<T>(
    user_data: *mut core::ffi::c_void,
    _primitive: n::iree_io_file_handle_primitive_t,
) {
    // SAFETY: native calls this once on final release with the boxed owner.
    n::contain(|| drop(unsafe { Box::from_raw(user_data.cast::<T>()) }));
}

impl<'a> FileHandle<'a> {
    /// Wraps borrowed host memory for the lifetime of the slice.
    pub fn wrap_host_allocation(memory: &'a mut [u8], access: FileAccess) -> Result<Self> {
        let span = n::iree_byte_span_t {
            data: memory.as_mut_ptr(),
            data_length: memory.len(),
        };
        let mut raw = core::ptr::null_mut();
        // SAFETY: the handle's lifetime is tied to the exclusive borrow.
        check(unsafe {
            n::iree_io_file_handle_wrap_host_allocation(
                access.bits(),
                span,
                n::iree_io_file_handle_release_callback_null(),
                system(),
                &mut raw,
            )
        })?;
        // SAFETY: success returns one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Adopts one owned file-handle reference.
    ///
    /// # Safety
    /// `raw` must be an owned reference whose backing resources remain valid
    /// for `'a`.
    pub unsafe fn from_raw(raw: *mut n::iree_io_file_handle_t) -> Self {
        Self {
            raw,
            _memory: PhantomData,
        }
    }

    /// Borrows the native handle.
    pub fn as_raw(&self) -> *mut n::iree_io_file_handle_t {
        self.raw
    }

    /// Transfers ownership of the native handle to the caller.
    pub fn into_raw(self) -> *mut n::iree_io_file_handle_t {
        core::mem::ManuallyDrop::new(self).raw
    }

    /// Returns the platform primitive backing this handle.
    pub fn primitive(&self) -> FileHandlePrimitive<'_> {
        // SAFETY: `self` owns a live handle.
        let primitive = unsafe { n::iree_io_file_handle_primitive(self.raw) };
        match primitive.type_ {
            n::IREE_IO_FILE_HANDLE_TYPE_HOST_ALLOCATION => FileHandlePrimitive::HostAllocation {
                // SAFETY: the type tag selects the active union member.
                length: unsafe { primitive.value.host_allocation.data_length },
            },
            n::IREE_IO_FILE_HANDLE_TYPE_FD => {
                // SAFETY: the type tag selects the active union member.
                let fd = unsafe { primitive.value.fd };
                // SAFETY: the descriptor stays open while `self` is borrowed.
                #[cfg(unix)]
                let fd = unsafe { BorrowedFd::borrow_raw(fd) };
                FileHandlePrimitive::FileDescriptor(fd)
            }
            other => FileHandlePrimitive::Unknown(other),
        }
    }

    /// Returns the access granted to this handle.
    pub fn access(&self) -> FileAccess {
        // SAFETY: `self` owns a live handle.
        FileAccess::from_bits_retain(unsafe { n::iree_io_file_handle_access(self.raw) })
    }

    pub(crate) fn require_access(&self, access: FileAccess) -> Result<()> {
        if self.access().contains(access) {
            Ok(())
        } else {
            Err(crate::Error::new(
                crate::Code::PermissionDenied,
                "requested access exceeds the file handle permissions",
            ))
        }
    }

    /// Returns whether the handle uses platform asynchronous I/O.
    pub fn uses_async_io(&self) -> bool {
        // SAFETY: `self` owns a live handle.
        unsafe { n::iree_io_file_handle_uses_async_io(self.raw) }
    }

    /// Flushes pending writes to the backing file.
    pub fn flush(&self) -> Result<()> {
        // SAFETY: `self` owns a live handle.
        check(unsafe { n::iree_io_file_handle_flush(self.raw) })
    }
}

impl Clone for FileHandle<'_> {
    fn clone(&self) -> Self {
        // SAFETY: `self` owns a live reference.
        unsafe { n::iree_io_file_handle_retain(self.raw) };
        Self {
            raw: self.raw,
            _memory: PhantomData,
        }
    }
}

impl Drop for FileHandle<'_> {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns exactly one reference.
        unsafe { n::iree_io_file_handle_release(self.raw) }
    }
}
