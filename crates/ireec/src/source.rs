//! Source inputs and compiler source configuration.
use crate::{Error, Result, cstring, lock, path_cstring, take};
use iree_native as n;
use std::{
    ffi::{c_char, c_void},
    marker::PhantomData,
    path::Path,
    ptr::{self, NonNull},
};

/// A compilation input (`iree_compiler_source_t`).
///
/// Sources are single-use: parsing transfers the backing buffer. A source
/// borrows the session that created it and, for in-memory sources, the bytes.
pub struct Source<'a> {
    raw: NonNull<n::iree_compiler_source_t>,
    _borrow: PhantomData<&'a ()>,
}

impl<'a> Source<'a> {
    /// Open a source file.
    pub(crate) fn open(
        session: *mut n::iree_compiler_session_t,
        path: &Path,
    ) -> Result<Source<'a>> {
        let path = path_cstring(path)?;
        let mut raw = ptr::null_mut();
        let error = {
            let _guard = lock();
            // SAFETY: the session is live, path is NUL-terminated, and raw is writable.
            unsafe { n::ireeCompilerSourceOpenFile(session, path.as_ptr(), &mut raw) }
        };
        Source::from_open(raw, error)
    }

    /// Wrap bytes that must outlive the source.
    pub(crate) fn buffer(
        session: *mut n::iree_compiler_session_t,
        name: &str,
        bytes: &'a [u8],
    ) -> Result<Source<'a>> {
        let name = cstring(name)?;
        let mut raw = ptr::null_mut();
        let error = {
            let _guard = lock();
            // SAFETY: session is live; name and bytes remain valid and raw is writable for this call.
            unsafe {
                n::ireeCompilerSourceWrapBuffer(
                    session,
                    name.as_ptr(),
                    bytes.as_ptr().cast::<c_char>(),
                    bytes.len(),
                    false,
                    &mut raw,
                )
            }
        };
        Source::from_open(raw, error)
    }

    fn from_open(
        raw: *mut n::iree_compiler_source_t,
        error: *mut n::iree_compiler_error_t,
    ) -> Result<Self> {
        // SAFETY: native source openers return a null or owned compiler error.
        if let Err(error) = unsafe { take(error) } {
            if !raw.is_null() {
                let _guard = lock();
                // SAFETY: the opener initialized `raw` with an owned source before failing.
                unsafe { n::ireeCompilerSourceDestroy(raw) };
            }
            return Err(error);
        }
        Source::wrap(raw)
    }

    fn wrap(raw: *mut n::iree_compiler_source_t) -> Result<Source<'a>> {
        let raw = NonNull::new(raw).ok_or_else(|| {
            Error::Native(n::Error::new(
                n::Code::Unknown,
                "compiler returned a null source",
            ))
        })?;
        Ok(Source {
            raw,
            _borrow: PhantomData,
        })
    }

    pub(crate) fn as_raw(&self) -> *mut n::iree_compiler_source_t {
        self.raw.as_ptr()
    }

    /// Split at MLIR `// -----` markers into independent sources.
    pub fn split(self) -> Result<Vec<Source<'a>>> {
        struct SplitState {
            raws: Vec<*mut n::iree_compiler_source_t>,
            failed: bool,
        }

        unsafe extern "C" fn push(source: *mut n::iree_compiler_source_t, out: *mut c_void) {
            // SAFETY: `split` passes its live state for this synchronous call.
            let state = unsafe { &mut *out.cast::<SplitState>() };
            // Reserving first makes the push infallible, so nothing here can unwind.
            if source.is_null() || state.raws.try_reserve(1).is_err() {
                state.failed = true;
                if !source.is_null() {
                    // SAFETY: the split source is owned and not retained on this path.
                    unsafe { n::ireeCompilerSourceDestroy(source) };
                }
                return;
            }
            state.raws.push(source);
        }

        let mut state = SplitState {
            raws: Vec::new(),
            failed: false,
        };
        let error = {
            let _guard = lock();
            // SAFETY: self owns a live source and state is a writable callback context for this call.
            unsafe {
                n::ireeCompilerSourceSplit(
                    self.raw.as_ptr(),
                    Some(push),
                    ptr::from_mut(&mut state).cast(),
                )
            }
        };
        if state.failed {
            let _guard = lock();
            for raw in state.raws.drain(..) {
                // SAFETY: split returned each raw source as an owned handle.
                unsafe { n::ireeCompilerSourceDestroy(raw) };
            }
            drop(_guard);
            // SAFETY: the native split call returns a null or owned compiler error.
            unsafe { take(error)? };
            return Err(n::Error::new(
                n::Code::ResourceExhausted,
                "failed to collect split compiler sources",
            )
            .into());
        }
        let mut raws = state.raws.into_iter();
        let mut parts = Vec::new();
        while let Some(raw) = raws.next() {
            match Source::wrap(raw) {
                Ok(part) => parts.push(part),
                Err(failure) => {
                    let _guard = lock();
                    for raw in raws {
                        if !raw.is_null() {
                            // SAFETY: each remaining split source is owned by this iterator.
                            unsafe { n::ireeCompilerSourceDestroy(raw) };
                        }
                    }
                    drop(_guard);
                    // SAFETY: the native split call returns a null or owned compiler error.
                    unsafe { take(error)? };
                    return Err(failure);
                }
            }
        }
        // SAFETY: the native split call returns a null or owned compiler error.
        unsafe { take(error)? };
        Ok(parts)
    }
}

impl Drop for Source<'_> {
    fn drop(&mut self) {
        let _guard = lock();
        // SAFETY: drop uniquely owns this live compiler source.
        unsafe { n::ireeCompilerSourceDestroy(self.raw.as_ptr()) }
    }
}
