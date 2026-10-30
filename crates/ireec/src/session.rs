//! Compiler session setup and resource ownership.
use crate::{Compiler, Invocation, Options, Result, Source, cstring, lock, take};
use iree_native as n;
use std::{
    ffi::{c_char, c_int, c_void},
    path::Path,
    ptr::{self, NonNull},
};

/// Compiler configuration scope (`iree_compiler_session_t`).
pub struct Session {
    compiler: Compiler,
    raw: NonNull<n::iree_compiler_session_t>,
}

impl Session {
    pub(crate) fn create(compiler: Compiler) -> Result<Session> {
        let _guard = lock();
        // SAFETY: the compiler was initialized and session creation is serialized.
        let raw = unsafe { n::ireeCompilerSessionCreate() };
        let raw = NonNull::new(raw).ok_or_else(|| {
            crate::Error::Native(n::Error::new(
                n::Code::Unknown,
                "compiler returned a null session",
            ))
        })?;
        Ok(Session { compiler, raw })
    }

    /// The underlying compiler session handle for interop with companion crates.
    ///
    /// The handle is borrowed and remains valid only while this session is alive.
    pub fn as_raw(&self) -> *mut n::iree_compiler_session_t {
        self.raw.as_ptr()
    }

    /// Set supported session-scoped compiler options such as `--iree-opt-level=O2`.
    ///
    /// This is not a general `iree-compile` argument parser: driver-only output
    /// flags and LLVM/MLIR process-global options are not session options. Use
    /// an `Invocation::output_*` method to choose an output, and
    /// [`Compiler::setup_global_cl`](crate::Compiler::setup_global_cl) only
    /// when a feature specifically requires process-global command-line setup.
    pub fn set_flags<S: AsRef<str>>(&mut self, flags: &[S]) -> Result<()> {
        let count = c_int::try_from(flags.len()).map_err(|_| crate::Error::ArgumentCount)?;
        let owned = flags
            .iter()
            .map(|f| cstring(f.as_ref()))
            .collect::<Result<Vec<_>>>()?;
        let mut pointers: Vec<*const c_char> = owned.iter().map(|s| s.as_ptr()).collect();
        let error = {
            let _guard = lock();
            // SAFETY: self owns a live session; pointers reference live NUL-terminated flags.
            unsafe {
                n::ireeCompilerSessionSetFlags(self.raw.as_ptr(), count, pointers.as_mut_ptr())
            }
        };
        // SAFETY: the native setter returns a null or owned compiler error.
        unsafe { take(error) }
    }

    /// Apply typed options after checking that every requested backend is registered.
    ///
    /// Backend validation happens before `ireeCompilerSessionSetFlags`, so a
    /// failed validation leaves this session unchanged.
    pub fn configure(&mut self, options: &Options) -> Result<()> {
        let (flags, requested_backends) = options.flags()?;
        if flags.is_empty() {
            return Ok(());
        }
        let registered = self.compiler.target_backends();
        if let Some(missing) = requested_backends
            .into_iter()
            .find(|requested| !registered.iter().any(|name| name == requested))
        {
            return Err(crate::Error::UnregisteredBackend(missing));
        }
        self.set_flags(&flags)
    }

    /// The flags of this session, one per entry; `non_default_only` filters
    /// to flags that differ from their defaults.
    pub fn flags(&self, non_default_only: bool) -> Vec<String> {
        unsafe extern "C" fn push(text: *const c_char, len: usize, out: *mut c_void) {
            // SAFETY: `flags` passes its vector; the compiler supplies `len` bytes.
            let out = unsafe { &mut *out.cast::<Vec<String>>() };
            // SAFETY: the compiler supplies len readable bytes for this callback.
            let text = unsafe { crate::lossy(text, len) };
            n::contain(|| out.extend(text));
        }
        let mut out: Vec<String> = Vec::new();
        let _guard = lock();
        // SAFETY: self owns a live session and out remains valid for the synchronous callback call.
        unsafe {
            n::ireeCompilerSessionGetFlags(
                self.raw.as_ptr(),
                non_default_only,
                Some(push),
                ptr::from_mut(&mut out).cast(),
            )
        };
        out
    }

    /// Open a source file.
    pub fn source_file(&self, path: impl AsRef<Path>) -> Result<Source<'_>> {
        Source::open(self.raw.as_ptr(), path.as_ref())
    }

    /// Wrap `bytes` as a source named `name`.
    pub fn source_buffer<'a>(&'a self, name: &str, bytes: &'a [u8]) -> Result<Source<'a>> {
        Source::buffer(self.raw.as_ptr(), name, bytes)
    }

    /// A new invocation on this session.
    pub fn invocation(&self) -> Result<Invocation<'_>> {
        Invocation::create(self.raw.as_ptr())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _guard = lock();
        // SAFETY: drop uniquely owns this live compiler session.
        unsafe { n::ireeCompilerSessionDestroy(self.raw.as_ptr()) }
    }
}
