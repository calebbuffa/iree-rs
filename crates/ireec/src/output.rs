//! Compiler outputs and output-path configuration.
use crate::{Compiler, Result, lock, path_cstring, take};
use iree_native as n;
#[cfg(unix)]
use std::os::fd::{IntoRawFd, OwnedFd};
use std::{
    ffi::{c_int, c_void},
    io,
    path::Path,
    ptr::{self, NonNull},
};

/// A compilation result sink (`iree_compiler_output_t`).
///
/// File outputs are deleted on drop unless [`Output::keep`] was called.
pub struct Output {
    raw: NonNull<n::iree_compiler_output_t>,
}

impl Output {
    fn open(
        open: impl FnOnce(*mut *mut n::iree_compiler_output_t) -> *mut n::iree_compiler_error_t,
    ) -> Result<Output> {
        let mut raw = ptr::null_mut();
        let error = {
            let _guard = lock();
            open(&mut raw)
        };
        // SAFETY: the native output opener returns a null or owned compiler error.
        if let Err(error) = unsafe { take(error) } {
            if !raw.is_null() {
                let _guard = lock();
                // SAFETY: the opener initialized `raw` with an owned output before failing.
                unsafe { n::ireeCompilerOutputDestroy(raw) };
            }
            return Err(error);
        }
        let raw = NonNull::new(raw).ok_or_else(|| {
            crate::Error::Native(n::Error::new(
                n::Code::Unknown,
                "compiler returned a null output",
            ))
        })?;
        Ok(Output { raw })
    }

    /// An in-memory output; read it back with [`Output::bytes`].
    ///
    /// The compiler handle proves process-wide initialization.
    pub fn memory(_compiler: &Compiler) -> Result<Output> {
        // SAFETY: Output::open supplies a writable output slot for this call.
        Output::open(|out| unsafe { n::ireeCompilerOutputOpenMembuffer(out) })
    }

    /// An output to `path`; dropping it without [`Output::keep`] removes any partial file.
    pub fn file(_compiler: &Compiler, path: impl AsRef<Path>) -> Result<Output> {
        let path = path_cstring(path.as_ref())?;
        // SAFETY: path is NUL-terminated and Output::open supplies a writable output slot.
        Output::open(|out| unsafe { n::ireeCompilerOutputOpenFile(path.as_ptr(), out) })
    }

    /// An output that takes ownership of a file descriptor.
    ///
    /// The output closes the descriptor when dropped.
    #[cfg(unix)]
    pub fn from_fd(_compiler: &Compiler, fd: OwnedFd) -> Result<Output> {
        // SAFETY: `OwnedFd` guarantees an open descriptor and ownership is transferred here.
        unsafe { open_fd(fd.into_raw_fd()) }
    }

    /// An output that takes ownership of a raw file descriptor.
    ///
    /// # Safety
    /// `fd` must be a valid open descriptor owned by the caller. Ownership is
    /// transferred to the output, which closes it when dropped.
    pub unsafe fn from_raw_fd(_compiler: &Compiler, fd: i32) -> Result<Output> {
        // SAFETY: the caller guarantees the raw descriptor's validity and ownership.
        unsafe { open_fd(fd) }
    }

    pub(crate) fn as_raw(&self) -> *mut n::iree_compiler_output_t {
        self.raw.as_ptr()
    }

    fn write_native(&mut self, bytes: &[u8]) -> Result<()> {
        let error = {
            let _guard = lock();
            // SAFETY: self owns a live output and bytes remain readable for the synchronous write.
            unsafe {
                n::ireeCompilerOutputWrite(
                    self.raw.as_ptr(),
                    bytes.as_ptr().cast::<c_void>(),
                    bytes.len(),
                )
            }
        };
        // SAFETY: the native write returns a null or owned compiler error.
        unsafe { take(error) }
    }

    /// Commit a file output so it survives drop.
    pub fn keep(&mut self) {
        let _guard = lock();
        // SAFETY: self owns a live output and compiler calls are serialized.
        unsafe { n::ireeCompilerOutputKeep(self.raw.as_ptr()) }
    }

    /// The contents of an in-memory output, valid until it is written again.
    pub fn bytes(&mut self) -> Result<&[u8]> {
        let (mut data, mut size) = (ptr::null_mut(), 0u64);
        let error = {
            let _guard = lock();
            // SAFETY: self owns the output and both mapping result slots are writable.
            unsafe { n::ireeCompilerOutputMapMemory(self.raw.as_ptr(), &mut data, &mut size) }
        };
        // SAFETY: the native mapping call returns a null or owned compiler error.
        unsafe { take(error)? };
        let size = usize::try_from(size).map_err(|_| {
            crate::Error::Native(iree_native::Error::new(
                iree_native::Code::Unknown,
                "compiler output exceeds the addressable Rust slice size",
            ))
        })?;
        if size > isize::MAX as usize {
            return Err(crate::Error::Native(iree_native::Error::new(
                iree_native::Code::Unknown,
                "compiler output exceeds the maximum Rust slice size",
            )));
        }
        if size == 0 {
            return Ok(&[]);
        }
        if data.is_null() {
            return Err(crate::Error::Native(iree_native::Error::new(
                iree_native::Code::Unknown,
                "compiler returned a null output mapping",
            )));
        }
        // SAFETY: the native mapping is `size` bytes and lives as long as the output.
        Ok(unsafe { n::slice(data.cast::<u8>(), size) })
    }
}

unsafe fn open_fd(fd: c_int) -> Result<Output> {
    // SAFETY: the caller transfers ownership of a valid descriptor to the output.
    Output::open(|out| unsafe { n::ireeCompilerOutputOpenFD(fd, out) })
}

impl io::Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.write_native(bytes).map_err(io::Error::other)?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        let _guard = lock();
        // SAFETY: drop uniquely owns this live compiler output.
        unsafe { n::ireeCompilerOutputDestroy(self.raw.as_ptr()) }
    }
}
