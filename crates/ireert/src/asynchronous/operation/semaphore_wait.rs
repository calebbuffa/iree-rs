//! One native semaphore wait operation.
use super::super::semaphore::WaitMode;
use super::super::{Proactor, Semaphore};
use super::{Operation, Timeline};
use crate::Result;
use iree_native as n;

/// One native multi-timeline operation retaining its arrays and semaphores.
///
/// Drop only requests cancellation: native user-data ownership lasts until final
/// completion, and callers must keep polling. Invocation and payload destruction
/// are separately panic-contained; failed submission reclaims payload immediately.
pub struct SemaphoreWaitOperation(Operation<n::iree_async_semaphore_wait_operation_t>);
impl SemaphoreWaitOperation {
    /// Submits one native timeline operation with owned arrays and dependencies.
    pub fn submit(
        proactor: &Proactor,
        entries: &[(&Semaphore, u64)],
        mode: WaitMode,
        mut callback: impl FnMut(Result<Option<usize>>) + 'static,
    ) -> Result<Self> {
        let mut timeline = Timeline::new(proactor, entries)?;
        // SAFETY: Every field is a zero-valid C ABI integer or pointer before native initialization.
        let mut raw: n::iree_async_semaphore_wait_operation_t = unsafe { core::mem::zeroed() };
        // SAFETY: raw.base is writable and the initializer sets its native operation fields.
        unsafe {
            n::iree_async_operation_initialize(
                &mut raw.base,
                n::IREE_ASYNC_OPERATION_TYPE_SEMAPHORE_WAIT as u8,
                0,
                None,
                core::ptr::null_mut(),
            )
        };
        raw.semaphores = timeline.semaphores.as_mut_ptr();
        raw.values = timeline.values.as_mut_ptr();
        raw.count = entries.len();
        raw.mode = mode.to_raw();
        Ok(Self(Operation::submit(
            proactor,
            raw,
            timeline,
            move |status, operation| {
                callback(status.map(|()| {
                    if mode == WaitMode::Any {
                        // SAFETY: The callback receives a live wait operation after native completion.
                        Some(unsafe { (*operation).satisfied_index })
                    } else {
                        None
                    }
                }));
            },
        )?))
    }
    /// Returns whether final completion has been delivered.
    pub fn is_complete(&self) -> bool {
        self.0.is_complete()
    }
    /// Requests cancellation without freeing in-flight arrays or callback state.
    pub fn cancel(&self) -> Result<()> {
        self.0.cancel()
    }
    /// Borrows the native operation; pending fields must not be accessed.
    pub fn as_raw(&self) -> *mut n::iree_async_semaphore_wait_operation_t {
        self.0.as_raw()
    }
}
