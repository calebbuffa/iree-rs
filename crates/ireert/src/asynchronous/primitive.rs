//! Owned native platform primitives.
use crate::{Result, check};
use iree_native as n;

/// An owning primitive closed exactly once with the native primitive-close API.
///
/// The C value itself is non-owning. This wrapper deliberately owns its handle;
/// native APIs receiving `as_raw()` borrow it unless explicitly documented.
pub struct Primitive(n::iree_async_primitive_t);
impl Primitive {
    /// Returns an empty primitive with no platform resource.
    pub fn none() -> Self {
        // SAFETY: The parameterless constructor returns the canonical empty primitive value.
        Self(unsafe { n::iree_async_primitive_none() })
    }
    /// Takes ownership of a POSIX descriptor.
    #[cfg(unix)]
    pub fn from_owned_fd(fd: std::os::fd::OwnedFd) -> Self {
        use std::os::fd::IntoRawFd;
        // SAFETY: the owned descriptor transfers without an intervening close.
        Self(unsafe { n::iree_async_primitive_from_fd(fd.into_raw_fd()) })
    }
    /// Takes ownership of a Win32 handle.
    #[cfg(windows)]
    pub fn from_owned_handle(handle: std::os::windows::io::OwnedHandle) -> Self {
        use std::os::windows::io::IntoRawHandle;
        // SAFETY: The owned OS handle transfers into the native primitive without an intervening close.
        Self(unsafe {
            n::iree_async_primitive_from_win32_handle(handle.into_raw_handle() as usize)
        })
    }
    /// Adopts sole ownership of a native primitive's platform resource.
    ///
    /// # Safety
    /// `raw` must contain a live owned handle of the specified native kind,
    /// or be empty. No other owner may close the same handle. Win32 handles
    /// must support `CloseHandle`.
    pub unsafe fn from_raw(raw: n::iree_async_primitive_t) -> Self {
        Self(raw)
    }
    /// Borrows a copy of the non-owning native descriptor.
    pub fn as_raw(&self) -> n::iree_async_primitive_t {
        self.0
    }
    /// Transfers ownership of the platform resource to the caller.
    pub fn into_raw(self) -> n::iree_async_primitive_t {
        let this = core::mem::ManuallyDrop::new(self);
        this.0
    }
    /// Duplicates the underlying resource; each wrapper closes its own handle.
    pub fn try_clone(&self) -> Result<Self> {
        // SAFETY: The parameterless constructor returns the canonical empty primitive value.
        let mut raw = unsafe { n::iree_async_primitive_none() };
        // SAFETY: self owns a live primitive and raw is a writable duplication result slot.
        check(unsafe { n::iree_async_primitive_dup(self.0, &mut raw) })?;
        Ok(Self(raw))
    }
    /// Returns whether this primitive has no resource.
    pub fn is_none(&self) -> bool {
        // SAFETY: The by-value primitive belongs to this live wrapper.
        unsafe { n::iree_async_primitive_is_none(self.0) }
    }
}
impl Default for Primitive {
    fn default() -> Self {
        Self::none()
    }
}
impl Drop for Primitive {
    fn drop(&mut self) {
        // SAFETY: This wrapper uniquely owns the primitive resource being closed.
        unsafe { n::iree_async_primitive_close(&mut self.0) };
    }
}
