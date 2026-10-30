//! Non-owning IREE wait sources and their completion results.

use core::ptr;
use core::{ffi::c_void, marker::PhantomData};
use iree_native as n;
use std::time::Instant;

/// A non-owning IREE wait source borrowed from its native owner.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct WaitSource<'a> {
    raw: n::iree_wait_source_t,
    _owner: PhantomData<&'a ()>,
}

impl<'a> WaitSource<'a> {
    /// Returns an already-resolved wait source.
    pub fn immediate() -> WaitSource<'static> {
        // SAFETY: the parameterless IREE constructor returns a valid immediate source.
        WaitSource {
            // SAFETY: The immediate-source constructor has no arguments or borrowed state.
            raw: unsafe { n::iree_wait_source_immediate() },
            _owner: PhantomData,
        }
    }

    /// Returns a wait source resolved at `deadline`.
    pub fn delay_until(deadline: Instant) -> WaitSource<'static> {
        let deadline = crate::base::time::deadline_from_instant(Some(deadline));
        // SAFETY: deadline is expressed in IREE's monotonic epoch.
        WaitSource {
            // SAFETY: deadline is a valid native by-value monotonic deadline.
            raw: unsafe { n::iree_wait_source_delay(deadline) },
            _owner: PhantomData,
        }
    }

    /// Borrows a by-value C wait source whose backing state belongs to `owner`.
    ///
    /// # Safety
    /// `raw` must be valid while `owner` is borrowed. The native source must not
    /// retain state beyond synchronous calls unless the caller keeps the owner
    /// alive until native completion.
    pub const unsafe fn from_raw<O: ?Sized>(raw: n::iree_wait_source_t, _owner: &'a O) -> Self {
        Self {
            raw,
            _owner: PhantomData,
        }
    }

    /// Returns the underlying C wait source value.
    pub const fn as_raw(&self) -> n::iree_wait_source_t {
        self.raw
    }

    /// Returns whether the source is immediate.
    pub fn is_immediate(self) -> bool {
        // SAFETY: this source was produced by IREE or adopted under from_raw's contract.
        unsafe { n::iree_wait_source_is_immediate(self.raw) }
    }

    /// Returns whether the source is a delay.
    pub fn is_delay(self) -> bool {
        // SAFETY: this source was produced by IREE or adopted under from_raw's contract.
        unsafe { n::iree_wait_source_is_delay(self.raw) }
    }

    /// Queries whether the source is pending, resolved, or failed.
    pub fn query(self) -> crate::Result<WaitStatus> {
        let mut raw_code = n::IREE_STATUS_OK;
        // SAFETY: source validity is guaranteed by construction or from_raw's contract.
        crate::check(unsafe { n::iree_wait_source_query(self.raw, &mut raw_code) })?;
        Ok(if raw_code == n::IREE_STATUS_DEFERRED {
            WaitStatus::Deferred
        } else if raw_code == n::IREE_STATUS_OK {
            WaitStatus::Resolved
        } else {
            WaitStatus::Failed(crate::Code::from_raw(raw_code).unwrap_or(crate::Code::Unknown))
        })
    }

    /// Blocks until the source resolves or the optional timeout expires.
    pub fn wait(self, timeout: Option<std::time::Duration>) -> crate::Result<()> {
        let timeout = n::timeout(timeout)?;
        // SAFETY: the source and timeout are valid native value types.
        crate::check(unsafe { n::iree_wait_source_wait_one(self.raw, timeout) })
    }

    /// Resolves the source synchronously or registers a Rust callback.
    ///
    /// `None` waits indefinitely; a zero duration is an immediate timeout. The
    /// callback may run before this method returns. If registration fails, the
    /// callback is guaranteed not to run.
    ///
    /// # Safety
    /// If resolution is asynchronous, keep the borrowed native owner alive until
    /// the callback runs or native resolution is otherwise guaranteed complete.
    pub unsafe fn resolve(
        self,
        timeout: Option<std::time::Duration>,
        callback: Option<WaitCallback>,
    ) -> crate::Result<()> {
        let timeout = n::timeout(timeout)?;
        let user_data = callback
            .map(|callback| Box::into_raw(Box::new(WaitCallbackState(callback))).cast::<c_void>());
        let result = match self.raw.resolve {
            // SAFETY: `from_raw` requires a valid source function pointer; user_data owns the callback state.
            Some(resolve) => unsafe {
                resolve(
                    self.raw,
                    timeout,
                    user_data.map(|_| {
                        wait_callback as unsafe extern "C" fn(*mut c_void, n::iree_status_t)
                    }),
                    user_data.unwrap_or(ptr::null_mut()),
                )
            },
            None => {
                if let Some(user_data) = user_data {
                    // SAFETY: this pointer was just created from one boxed callback state.
                    unsafe { wait_callback(user_data, ptr::null_mut()) };
                }
                ptr::null_mut()
            }
        };
        match crate::check(result) {
            Ok(()) => Ok(()),
            Err(error) => {
                if let Some(user_data) = user_data {
                    // SAFETY: IREE guarantees failed registration neither invokes the callback
                    // nor retains user_data.
                    let state = unsafe { Box::from_raw(user_data.cast::<WaitCallbackState>()) };
                    n::contain(|| drop(state));
                }
                Err(error)
            }
        }
    }
}

/// A completion callback for an IREE wait source.
pub type WaitCallback = Box<dyn FnOnce(crate::Result<()>) + Send + 'static>;

struct WaitCallbackState(WaitCallback);

unsafe extern "C" fn wait_callback(user_data: *mut c_void, status: n::iree_status_t) {
    // SAFETY: `resolve` transfers exactly one boxed state to the callback on success.
    let state = unsafe { Box::from_raw(user_data.cast::<WaitCallbackState>()) };
    let callback = state.0;
    // SAFETY: callback status ownership is transferred to this callback by IREE.
    let result = unsafe { n::consume(status) };
    n::contain(|| callback(result));
}

/// State returned by querying a wait source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaitStatus {
    /// The wait source has not resolved yet.
    Deferred,
    /// The wait source resolved successfully.
    Resolved,
    /// The wait source resolved with a failure status.
    Failed(crate::Code),
}
