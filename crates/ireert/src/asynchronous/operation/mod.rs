//! Single-shot operation ownership and its public asynchronous operation types.
pub mod semaphore_signal;
pub mod semaphore_wait;

pub use semaphore_signal::SemaphoreSignalOperation;
pub use semaphore_wait::SemaphoreWaitOperation;

use super::{Proactor, Semaphore};
use crate::{Code, Error, Result, check};
use iree_native as n;
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    rc::Rc,
};

/// Native semaphore and value arrays for one operation, plus the retained semaphores.
pub(crate) struct Timeline {
    _owned: Vec<Semaphore>,
    semaphores: Box<[*mut n::iree_async_semaphore_t]>,
    values: Box<[u64]>,
}

impl Timeline {
    fn new(proactor: &Proactor, entries: &[(&Semaphore, u64)]) -> Result<Self> {
        if entries.is_empty()
            || entries
                .iter()
                .any(|(s, _)| s.proactor().as_raw() != proactor.as_raw())
        {
            return Err(Error::new(
                Code::InvalidArgument,
                "empty or foreign semaphore operation",
            ));
        }
        Ok(Self {
            _owned: entries.iter().map(|(s, _)| (*s).clone()).collect(),
            semaphores: entries.iter().map(|(s, _)| s.as_raw()).collect(),
            values: entries.iter().map(|(_, v)| *v).collect(),
        })
    }
}

struct State<T> {
    raw: UnsafeCell<T>,
    complete: Cell<bool>,
    intermediate_error: RefCell<Option<crate::Error>>,
    proactor: Proactor,
    _resources: Box<dyn core::any::Any>,
}
/// Private storage shared by the public C-type owner and one native callback.
pub(crate) struct Operation<T: 'static> {
    state: Rc<State<T>>,
}
struct Payload<T, F> {
    state: Rc<State<T>>,
    callback: F,
}
unsafe extern "C" fn completed<T: 'static, F: FnMut(Result<()>, *mut T)>(
    user_data: *mut core::ffi::c_void,
    operation: *mut n::iree_async_operation_t,
    status: n::iree_status_t,
    flags: n::iree_async_completion_flags_t,
) {
    if flags & n::IREE_ASYNC_COMPLETION_FLAG_MORE as u32 != 0 {
        // MORE is not final, but its status still belongs to the operation.
        // SAFETY: Native invokes this callback with the still-owned Payload pointer registered at submission.
        let payload = unsafe { &*user_data.cast::<Payload<T, F>>() };
        n::contain(|| {
            if let Err(error) = check(status) {
                let mut pending = payload.state.intermediate_error.borrow_mut();
                if pending.is_none() {
                    *pending = Some(error);
                }
            }
        });
        return;
    }
    // SAFETY: Final completion consumes the unique Box pointer transferred to this callback.
    let mut payload = unsafe { Box::from_raw(user_data.cast::<Payload<T, F>>()) };
    payload.state.complete.set(true);
    let completion = match payload.state.intermediate_error.borrow_mut().take() {
        Some(error) => {
            let _ = check(status);
            Err(error)
        }
        None => check(status),
    };
    // Backend dispatch releases resources before this callback and explicitly
    // permits freeing an operation whose pool pointer is NULL (proactor.c).
    n::contain(|| (payload.callback)(completion, operation.cast()));
    // A callback's captures, and even a caught panic payload, may panic on Drop.
    n::contain(|| drop(payload));
}
impl<T: 'static> Operation<T> {
    pub(crate) fn submit<F: FnMut(Result<()>, *mut T) + 'static>(
        proactor: &Proactor,
        raw: T,
        resources: impl core::any::Any,
        callback: F,
    ) -> Result<Self> {
        let state = Rc::new(State {
            raw: UnsafeCell::new(raw),
            complete: Cell::new(false),
            intermediate_error: RefCell::new(None),
            proactor: proactor.clone(),
            _resources: Box::new(resources),
        });
        let payload = Box::into_raw(Box::new(Payload {
            state: state.clone(),
            callback,
        }));
        let base = state.raw.get().cast::<n::iree_async_operation_t>();
        // SAFETY: base points into live UnsafeCell storage and both callback fields are writable before submission.
        unsafe {
            (*base).completion_fn = Some(completed::<T, F>);
            (*base).user_data = payload.cast();
        }
        // SAFETY: The operation storage and callback payload stay live for the submission call.
        let result = check(unsafe { n::iree_async_proactor_submit_one(proactor.as_raw(), base) });
        if let Err(error) = result {
            // Single-operation submission failure transfers no ownership. A
            // synchronous completion, if any, already consumed the user data.
            if !state.complete.replace(true) {
                // SAFETY: Submission failed without consuming payload and no final callback has completed.
                n::contain(|| unsafe { drop(Box::from_raw(payload)) });
            }
            return Err(error);
        }
        Ok(Self { state })
    }
    pub(crate) fn as_raw(&self) -> *mut T {
        self.state.raw.get()
    }
    pub(crate) fn is_complete(&self) -> bool {
        self.state.complete.get()
    }
    pub(crate) fn cancel(&self) -> Result<()> {
        if self.is_complete() {
            return Ok(());
        }
        // SAFETY: State retains the live proactor while this operation is cancellable.
        check(unsafe {
            n::iree_async_proactor_cancel(self.state.proactor.as_raw(), self.as_raw().cast())
        })
    }
}
impl<T> Drop for Operation<T> {
    fn drop(&mut self) {
        // Cancellation is only a request. Native callback ownership keeps all
        // storage alive until final completion, even if cancellation loses a
        // race or is unsupported. Users must keep driving the proactor.
        let _ = self.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::super::{LocalProactorPool, Semaphore, semaphore::WaitMode};
    use super::*;
    use std::{
        cell::Cell,
        rc::Rc,
        time::{Duration, Instant},
    };

    fn poll_until(proactor: &Proactor, condition: impl Fn() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !condition() {
            assert!(
                Instant::now() < deadline,
                "native operation did not complete"
            );
            match proactor.poll(Some(Duration::from_millis(10))) {
                Ok(_) => {}
                Err(error) if error.code() == crate::Code::DeadlineExceeded => {}
                Err(error) => panic!("poll failed: {error}"),
            }
        }
    }

    struct CountDrop(Rc<Cell<usize>>);
    impl Drop for CountDrop {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    // Law: dropping a pending operation only requests cancellation; the
    // captured payload is destroyed exactly once, at final completion.
    #[test]
    fn dropping_pending_operation_keeps_payload_until_one_completion() {
        let pool = LocalProactorPool::with_nodes(&[0]).unwrap();
        let proactor = pool.get(0).unwrap();
        let semaphore = Semaphore::new(&proactor, 0, 0).unwrap();
        let drops = Rc::new(Cell::new(0));
        let calls = Rc::new(Cell::new(0));
        let called = calls.clone();
        let guard = CountDrop(drops.clone());
        let operation = SemaphoreWaitOperation::submit(
            &proactor,
            &[(&semaphore, 1)],
            WaitMode::All,
            move |_| {
                let _ = &guard;
                called.set(called.get() + 1);
            },
        )
        .unwrap();
        drop(operation);
        assert_eq!(drops.get(), 0);
        semaphore.signal(1, None).unwrap();
        poll_until(&proactor, || calls.get() == 1);
        assert_eq!(drops.get(), 1);
    }

    // Law: a panicking callback and a panicking capture destructor are each
    // contained, and the payload is still dropped exactly once.
    #[test]
    fn callback_panic_and_capture_drop_panic_are_independently_contained() {
        struct Bomb(Rc<Cell<usize>>);
        impl Drop for Bomb {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
                panic!("capture drop panic");
            }
        }
        let pool = LocalProactorPool::with_nodes(&[0]).unwrap();
        let proactor = pool.get(0).unwrap();
        let semaphore = Semaphore::new(&proactor, 1, 0).unwrap();
        let drops = Rc::new(Cell::new(0));
        let guard = Bomb(drops.clone());
        let operation = SemaphoreWaitOperation::submit(
            &proactor,
            &[(&semaphore, 1)],
            WaitMode::All,
            move |_| {
                let _ = &guard;
                panic!("callback invocation panic");
            },
        )
        .unwrap();
        poll_until(&proactor, || operation.is_complete());
        assert_eq!(drops.get(), 1);
    }
}
