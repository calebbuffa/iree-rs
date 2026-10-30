//! Native frontier progress tables.
use super::{Axis, Semaphore};
use crate::{Code, Result, check, system};
use iree_native as n;
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

struct Inner {
    raw: *mut n::iree_async_frontier_tracker_t,
    bridges: RefCell<BTreeMap<Axis, Semaphore>>,
}
impl Drop for Inner {
    fn drop(&mut self) {
        // frontier_tracker.h borrows bridge semaphores until axis retirement.
        for axis in self.bridges.get_mut().keys() {
            // SAFETY: Inner retains the tracker and each bridge until this axis is retired.
            unsafe {
                n::iree_async_frontier_tracker_retire_axis(
                    self.raw,
                    axis.as_raw(),
                    n::abi::status::from_code(n::IREE_STATUS_CANCELLED),
                )
            };
        }
        // SAFETY: Inner owns exactly one live tracker reference.
        unsafe { n::iree_async_frontier_tracker_release(self.raw) };
    }
}
/// A retained native frontier tracker retaining its borrowed bridge semaphores.
#[derive(Clone)]
pub struct FrontierTracker(Rc<Inner>);
impl FrontierTracker {
    /// Creates a bounded axis table with deterministic session/machine identity.
    pub fn new(capacity: u32, session: u8, machine: u8) -> Result<Self> {
        // SAFETY: The parameterless options constructor returns a valid by-value options struct.
        let mut options = unsafe { n::iree_async_frontier_tracker_options_default() };
        options.axis_table_capacity = capacity;
        options.session_epoch = session;
        options.machine_index = machine;
        let mut raw = core::ptr::null_mut();
        // SAFETY: options is valid and raw is a writable output slot.
        check(unsafe { n::iree_async_frontier_tracker_create(options, system(), &mut raw) })?;
        // SAFETY: create returned one owned tracker reference on success.
        Ok(unsafe { Self::from_raw(raw) })
    }
    /// Adopts one native tracker reference with no externally registered bridges.
    ///
    /// # Safety
    /// `raw` must be live, owned, and exclusively used on this thread.
    /// External bridge registrations and waiters
    /// must remain valid until native release, independently of this wrapper.
    pub unsafe fn from_raw(raw: *mut n::iree_async_frontier_tracker_t) -> Self {
        Self(Rc::new(Inner {
            raw,
            bridges: RefCell::new(BTreeMap::new()),
        }))
    }
    /// Borrows the native tracker.
    pub fn as_raw(&self) -> *mut n::iree_async_frontier_tracker_t {
        self.0.raw
    }
    /// Transfers one retained tracker reference after retiring owned bridges.
    ///
    /// Registrations made through this wrapper are retired when its last Rust
    /// clone is dropped; the returned C reference does not keep them registered.
    pub fn into_raw(self) -> *mut n::iree_async_frontier_tracker_t {
        let raw = self.as_raw();
        // SAFETY: This wrapper holds a live tracker reference while retaining it for the caller.
        unsafe { n::iree_async_frontier_tracker_retain(raw) };
        raw
    }
    /// Returns the configured session epoch.
    pub fn session_epoch(&self) -> u8 {
        // SAFETY: The Rc-owned Inner keeps the tracker live for this query.
        unsafe { n::iree_async_frontier_tracker_session_epoch(self.as_raw()) }
    }
    /// Returns the configured machine index.
    pub fn machine_index(&self) -> u8 {
        // SAFETY: The Rc-owned Inner keeps the tracker live for this query.
        unsafe { n::iree_async_frontier_tracker_machine_index(self.as_raw()) }
    }
    /// Registers an axis, retaining its optional borrowed bridge semaphore.
    pub fn register_axis(&self, axis: Axis, semaphore: Option<&Semaphore>) -> Result<()> {
        // SAFETY: The tracker is live and the optional semaphore remains borrowed through registration.
        check(unsafe {
            n::iree_async_frontier_tracker_register_axis(
                self.as_raw(),
                axis.as_raw(),
                semaphore.map_or(core::ptr::null_mut(), Semaphore::as_raw),
            )
        })?;
        if let Some(semaphore) = semaphore {
            self.0.bridges.borrow_mut().insert(axis, semaphore.clone());
        }
        Ok(())
    }
    /// Permanently retires an axis before releasing its bridge semaphore.
    pub fn retire_axis(&self, axis: Axis, reason: Code) {
        // SAFETY: The Rc-owned Inner keeps the tracker live through axis retirement.
        unsafe {
            n::iree_async_frontier_tracker_retire_axis(
                self.as_raw(),
                axis.as_raw(),
                n::abi::status::from_code(reason.to_raw()),
            )
        };
        self.0.bridges.borrow_mut().remove(&axis);
    }
    /// Tests whether an axis has reached the requested epoch.
    pub fn query_epoch(&self, axis: Axis, epoch: u64) -> bool {
        // SAFETY: The Rc-owned Inner keeps the tracker live through the epoch query.
        unsafe { n::iree_async_frontier_tracker_query_epoch(self.as_raw(), axis.as_raw(), epoch) }
    }
    /// Monotonically advances an axis and returns the dispatched waiter count.
    pub fn advance(&self, axis: Axis, epoch: u64) -> usize {
        // SAFETY: The Rc-owned Inner keeps the tracker live through the epoch advance.
        unsafe { n::iree_async_frontier_tracker_advance(self.as_raw(), axis.as_raw(), epoch) }
    }
    /// Fails all pending and future waits involving an axis.
    pub fn fail_axis(&self, axis: Axis, reason: Code) {
        // SAFETY: The Rc-owned Inner keeps the tracker live through failure propagation.
        unsafe {
            n::iree_async_frontier_tracker_fail_axis(
                self.as_raw(),
                axis.as_raw(),
                n::abi::status::from_code(reason.to_raw()),
            )
        };
    }
}

#[cfg(test)]
mod tests {
    use super::super::CausalDomain;
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn epochs_never_regress(a in 0u64..10000, b in 0u64..10000) {
            let tracker = FrontierTracker::new(16, 1, 0).unwrap();
            let axis = Axis::new(1, 0, CausalDomain::Host, 1);
            tracker.register_axis(axis, None).unwrap();
            tracker.advance(axis, a);
            tracker.advance(axis, b);
            prop_assert!(tracker.query_epoch(axis, a.max(b)));
            prop_assert!(!tracker.query_epoch(axis, a.max(b) + 1));
        }
    }
}
