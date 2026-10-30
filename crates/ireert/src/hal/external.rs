//! External buffer handles and imported-buffer release ownership.

use crate::hal::DeviceSize;
use iree_native as n;

/// The external handle representation requested from a HAL allocator.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExternalBufferType {
    /// No external handle.
    None,
    /// Host memory allocation.
    HostAllocation,
    /// Device allocation address.
    DeviceAllocation,
    /// Opaque file descriptor (Unix platforms).
    OpaqueFd,
    /// Opaque Win32 handle (Windows platforms).
    OpaqueWin32,
}

/// An external buffer handle borrowed for import.
pub enum ExternalBuffer<'a> {
    /// A borrowed host-memory slice.
    HostAllocation {
        /// Mutable memory used as the external allocation.
        memory: &'a mut [u8],
    },
    /// A device allocation address and byte length.
    DeviceAllocation {
        /// Device-specific allocation address.
        address: u64,
        /// Allocation size in bytes.
        size: DeviceSize,
    },
    /// A borrowed Unix file descriptor and byte length.
    #[cfg(unix)]
    OpaqueFd {
        /// Borrowed file descriptor.
        handle: std::os::fd::BorrowedFd<'a>,
        /// Allocation size in bytes.
        size: DeviceSize,
    },
    /// A borrowed Windows handle and byte length.
    #[cfg(windows)]
    OpaqueWin32 {
        /// Borrowed Windows handle.
        handle: std::os::windows::io::BorrowedHandle<'a>,
        /// Allocation size in bytes.
        size: DeviceSize,
    },
}
impl ExternalBuffer<'_> {
    pub(crate) fn raw(&mut self) -> n::iree_hal_external_buffer_t {
        // SAFETY: the native struct is plain data; all-zero is `TYPE_NONE` with no flags.
        let mut raw: n::iree_hal_external_buffer_t = unsafe { core::mem::zeroed() };
        match self {
            Self::HostAllocation { memory } => {
                raw.type_ = n::IREE_HAL_EXTERNAL_BUFFER_TYPE_HOST_ALLOCATION;
                raw.size = memory.len() as DeviceSize;
                raw.handle.host_allocation =
                    n::iree_hal_external_buffer_t__bindgen_ty_1__bindgen_ty_1 {
                        ptr: memory.as_mut_ptr().cast(),
                    };
            }
            Self::DeviceAllocation { address, size } => {
                raw.type_ = n::IREE_HAL_EXTERNAL_BUFFER_TYPE_DEVICE_ALLOCATION;
                raw.size = *size;
                raw.handle.device_allocation =
                    n::iree_hal_external_buffer_t__bindgen_ty_1__bindgen_ty_2 { ptr: *address };
            }
            #[cfg(unix)]
            Self::OpaqueFd { handle, size } => {
                use std::os::fd::AsRawFd;
                raw.type_ = n::IREE_HAL_EXTERNAL_BUFFER_TYPE_OPAQUE_FD;
                raw.size = *size;
                raw.handle.opaque_fd = n::iree_hal_external_buffer_t__bindgen_ty_1__bindgen_ty_3 {
                    fd: handle.as_raw_fd(),
                };
            }
            #[cfg(windows)]
            Self::OpaqueWin32 { handle, size } => {
                use std::os::windows::io::AsRawHandle;
                raw.type_ = n::IREE_HAL_EXTERNAL_BUFFER_TYPE_OPAQUE_WIN32;
                raw.size = *size;
                raw.handle.opaque_win32 =
                    n::iree_hal_external_buffer_t__bindgen_ty_1__bindgen_ty_4 {
                        handle: handle.as_raw_handle(),
                    };
            }
        }
        raw
    }
}

/// An exported external handle kept valid by a retained source buffer.
pub struct ExportedExternalBuffer {
    pub(crate) raw: n::iree_hal_external_buffer_t,
    pub(crate) _buffer: super::buffer::Buffer,
    #[cfg(unix)]
    pub(crate) owned_fd: Option<std::os::fd::OwnedFd>,
    #[cfg(windows)]
    pub(crate) owned_handle: Option<std::os::windows::io::OwnedHandle>,
}
impl ExportedExternalBuffer {
    /// Returns the exported handle representation.
    pub fn buffer_type(&self) -> ExternalBufferType {
        match self.raw.type_ {
            n::IREE_HAL_EXTERNAL_BUFFER_TYPE_HOST_ALLOCATION => ExternalBufferType::HostAllocation,
            n::IREE_HAL_EXTERNAL_BUFFER_TYPE_DEVICE_ALLOCATION => {
                ExternalBufferType::DeviceAllocation
            }
            n::IREE_HAL_EXTERNAL_BUFFER_TYPE_OPAQUE_FD => ExternalBufferType::OpaqueFd,
            n::IREE_HAL_EXTERNAL_BUFFER_TYPE_OPAQUE_WIN32 => ExternalBufferType::OpaqueWin32,
            _ => ExternalBufferType::None,
        }
    }
    /// Returns the exported allocation size in bytes.
    pub fn size(&self) -> DeviceSize {
        self.raw.size
    }
    /// Returns the exported device allocation address, if applicable.
    pub fn device_address(&self) -> Option<u64> {
        if self.buffer_type() != ExternalBufferType::DeviceAllocation {
            return None;
        }
        // SAFETY: the discriminator identifies this union member.
        Some(unsafe { self.raw.handle.device_allocation.ptr })
    }
    /// Borrows the exported Unix file descriptor, if applicable.
    #[cfg(unix)]
    pub fn file_descriptor(&self) -> Option<std::os::fd::BorrowedFd<'_>> {
        use std::os::fd::AsFd;
        self.owned_fd.as_ref().map(AsFd::as_fd)
    }
    /// Borrows the exported Win32 handle, if applicable.
    #[cfg(windows)]
    pub fn windows_handle(&self) -> Option<std::os::windows::io::BorrowedHandle<'_>> {
        use std::os::windows::io::AsHandle;
        self.owned_handle.as_ref().map(AsHandle::as_handle)
    }
    /// Borrows the exported host allocation as mutable bytes.
    ///
    /// # Safety
    /// The caller must ensure no aliases, including other buffer handles, access the allocation for the returned lifetime.
    /// Every byte in the exported allocation must be initialized.
    pub unsafe fn host_allocation(&mut self) -> Option<&mut [u8]> {
        let length = self.raw.size;
        if self.buffer_type() != ExternalBufferType::HostAllocation {
            return None;
        }
        // SAFETY: the discriminator identifies this union member.
        let pointer = unsafe { self.raw.handle.host_allocation.ptr };
        if pointer.is_null() {
            return None;
        }
        // SAFETY: the caller guarantees unique access and the exported pointer spans this size.
        Some(unsafe { n::slice_mut(pointer.cast(), length) })
    }
}
