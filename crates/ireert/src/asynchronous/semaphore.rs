//! Native asynchronous timeline semaphores.
use super::{Frontier, Proactor};
use crate::{Code, Error, Result, check, system};
use iree_native as n;
use std::time::Duration;

/// Native multi-semaphore wait policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitMode {
    /// Every timeline target must be reached.
    All,
    /// At least one timeline target must be reached.
    Any,
}

impl WaitMode {
    pub(crate) fn to_raw(self) -> n::iree_async_wait_mode_t {
        (match self {
            Self::All => n::IREE_ASYNC_WAIT_MODE_ALL,
            Self::Any => n::IREE_ASYNC_WAIT_MODE_ANY,
        }) as n::iree_async_wait_mode_t
    }
}

/// A retained async timeline semaphore, distinct from a HAL semaphore.
pub struct Semaphore {
    raw: *mut n::iree_async_semaphore_t,
    proactor: Proactor,
}
impl Semaphore {
    /// Creates a timeline with a bounded causal-frontier capacity.
    pub fn new(proactor: &Proactor, initial_value: u64, frontier_capacity: u8) -> Result<Self> {
        let mut raw = core::ptr::null_mut();
        // SAFETY: The proactor is live and raw is a writable semaphore output slot.
        check(unsafe {
            n::iree_async_semaphore_create(
                proactor.as_raw(),
                initial_value,
                frontier_capacity,
                system(),
                &mut raw,
            )
        })?;
        // SAFETY: create returned one owned semaphore reference bound to this live proactor.
        Ok(unsafe { Self::from_raw(raw, proactor) })
    }
    /// Adopts one owned reference and retains its proactor dependency.
    ///
    /// # Safety
    /// `raw` must be live, owned, and bound to `proactor`.
    pub unsafe fn from_raw(raw: *mut n::iree_async_semaphore_t, proactor: &Proactor) -> Self {
        Self {
            raw,
            proactor: proactor.clone(),
        }
    }
    /// Borrows the native semaphore.
    pub fn as_raw(&self) -> *mut n::iree_async_semaphore_t {
        self.raw
    }
    /// Transfers one native reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_async_semaphore_t {
        let raw = self.raw;
        // SAFETY: The wrapper holds a live semaphore reference while retaining one for the caller.
        unsafe { n::iree_async_semaphore_retain(raw) };
        raw
    }
    pub(crate) fn proactor(&self) -> &Proactor {
        &self.proactor
    }
    /// Runs `callback` once this semaphore reaches `value` or fails.
    ///
    /// The callback may run immediately, or later on any thread, and must not block.
    pub fn on_value<F>(&self, value: u64, callback: F) -> Result<crate::asynchronous::Timepoint>
    where
        F: FnOnce(Result<()>) + Send + 'static,
    {
        // SAFETY: the semaphore is live.
        unsafe { crate::asynchronous::Timepoint::register(self.raw, value, Box::new(callback)) }
    }

    /// Signals `target` to `target_value` once this semaphore reaches `value`,
    /// propagating failure. Dropping the returned timepoint cancels the relay.
    pub fn link(
        &self,
        value: u64,
        target: &Self,
        target_value: u64,
    ) -> Result<crate::asynchronous::Timepoint> {
        // SAFETY: both semaphores are live.
        unsafe { crate::asynchronous::timepoint::link(self.raw, value, target.raw, target_value) }
    }

    /// Signals this semaphore to `value` once the platform `fence` signals;
    /// `proactor` takes ownership of the fence on success.
    ///
    /// # Safety
    /// Keep the proactor and the semaphore's signal-path dependencies alive
    /// until the import completes or is cancelled by proactor shutdown.
    pub unsafe fn import_fence(
        &self,
        proactor: &crate::asynchronous::Proactor,
        fence: crate::asynchronous::Primitive,
        value: u64,
    ) -> Result<()> {
        // SAFETY: the semaphore is live; the caller preserves signal dependencies.
        unsafe { crate::asynchronous::timepoint::import_fence(self.raw, proactor, fence, value) }
    }

    /// Exports a platform fence (eventfd, sync file or Win32 event) that
    /// signals once this semaphore reaches `value`.
    ///
    /// # Safety
    /// The returned primitive must remain open until the semaphore reaches
    /// `value` or fails: a pending native callback may write to its handle.
    pub unsafe fn export_fence(
        &self,
        proactor: &crate::asynchronous::Proactor,
        value: u64,
    ) -> Result<crate::asynchronous::Primitive> {
        // SAFETY: the semaphore is live; the caller preserves the pending handle.
        unsafe { crate::asynchronous::timepoint::export_fence(self.raw, proactor, value) }
    }

    /// Returns the current payload, including the native failure sentinel.
    pub fn query(&self) -> u64 {
        // SAFETY: self owns a live semaphore.
        unsafe { n::iree_async_semaphore_query(self.raw) }
    }
    /// Advances the timeline and optionally merges a causal frontier.
    pub fn signal(&self, value: u64, frontier: Option<&Frontier>) -> Result<()> {
        // SAFETY: self is live and frontier, when present, stays borrowed for this call.
        check(unsafe {
            n::iree_async_semaphore_signal(
                self.raw,
                value,
                frontier.map_or(core::ptr::null(), |f| f.as_raw()),
            )
        })
    }
    /// Copies the complete accumulated causal frontier without truncation.
    pub fn frontier(&self) -> Frontier {
        let mut frontier = Frontier::default();
        // SAFETY: self is live and frontier has writable capacity for all 255 entries.
        unsafe { n::iree_async_semaphore_query_frontier(self.raw, frontier.as_mut_ptr(), u8::MAX) };
        frontier
    }
    /// Merges causal context without advancing the timeline.
    ///
    /// Returns false on capacity overflow, preserving the previous frontier.
    pub fn merge_frontier(&self, frontier: &Frontier) -> bool {
        // SAFETY: Both the owned semaphore and borrowed frontier are valid for the merge.
        unsafe { n::iree_async_semaphore_merge_frontier(self.raw, frontier.as_raw()) }
    }
    /// Fails this timeline, consuming a newly-created native status.
    pub fn fail(&self, code: Code) {
        // SAFETY: self owns a live semaphore and the status code is passed by value.
        unsafe { n::iree_async_semaphore_fail(self.raw, n::abi::status::from_code(code.to_raw())) };
    }
    /// Reports a stored failure, or succeeds for a healthy semaphore.
    pub fn status(&self) -> Result<()> {
        // SAFETY: self owns a live semaphore queried synchronously.
        let code = unsafe { n::iree_async_semaphore_query_status(self.raw) };
        if code == n::IREE_STATUS_OK {
            Ok(())
        } else {
            Err(Error::new(
                Code::from_raw(code).unwrap_or(Code::Unknown),
                "asynchronous semaphore failed",
            ))
        }
    }
    /// Blocks until the payload reaches `value`, independently of polling.
    pub fn wait(&self, value: u64, timeout: Option<Duration>) -> Result<()> {
        Self::wait_many(&[(self, value)], WaitMode::All, timeout)
    }
    /// Waits for any or all timeline targets; an empty all-wait succeeds.
    pub fn wait_many(
        entries: &[(&Self, u64)],
        mode: WaitMode,
        timeout: Option<Duration>,
    ) -> Result<()> {
        let mut semaphores: Vec<_> = entries.iter().map(|(s, _)| s.raw).collect();
        let values: Vec<_> = entries.iter().map(|(_, v)| *v).collect();
        // SAFETY: The arrays borrow live entry values and the semaphore owners for this synchronous wait.
        check(unsafe {
            n::iree_async_semaphore_multi_wait(
                mode.to_raw(),
                semaphores.as_mut_ptr(),
                values.as_ptr(),
                entries.len(),
                n::timeout(timeout)?,
                0,
                system(),
            )
        })
    }
}
impl Clone for Semaphore {
    fn clone(&self) -> Self {
        // SAFETY: Clone retains the live semaphore before returning another owner.
        unsafe { n::iree_async_semaphore_retain(self.raw) };
        Self {
            raw: self.raw,
            proactor: self.proactor.clone(),
        }
    }
}
impl Drop for Semaphore {
    fn drop(&mut self) {
        // SAFETY: Drop releases the one semaphore reference owned by self.
        unsafe { n::iree_async_semaphore_release(self.raw) };
    }
}

#[cfg(test)]
mod tests {
    use super::super::LocalProactorPool;
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(16))]
        #[test]
        fn signal_query_law(initial in 0u64..10000, increment in 1u64..10000) {
            let pool = LocalProactorPool::with_nodes(&[0]).unwrap();
            let proactor = pool.get(0).unwrap();
            let semaphore = Semaphore::new(&proactor, initial, 0).unwrap();
            semaphore.signal(initial + increment, None).unwrap();
            prop_assert_eq!(semaphore.query(), initial + increment);
            semaphore.wait(initial + increment, Some(Duration::ZERO)).unwrap();
        }
    }
    #[test]
    fn causal_frontier_is_retained_and_copied() {
        use super::super::{Axis, CausalDomain, Comparison};
        let pool = LocalProactorPool::with_nodes(&[0]).unwrap();
        let proactor = pool.get(0).unwrap();
        let semaphore = Semaphore::new(&proactor, 0, 4).unwrap();
        let frontier = Frontier::new(&[(Axis::new(1, 0, CausalDomain::Host, 1), 7)]).unwrap();
        semaphore.signal(1, Some(&frontier)).unwrap();
        assert_eq!(semaphore.frontier().compare(&frontier), Comparison::Equal);
        assert!(semaphore.merge_frontier(&frontier));
        drop(frontier);
        assert_eq!(semaphore.frontier().len(), 1);
    }
}
