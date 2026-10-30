//! HAL fence composition and synchronization.

use super::{WaitFlags, semaphore::Semaphore};
use crate::{Code, Result, check, system};
use core::{ptr, time::Duration};
use iree_native as n;

/// A retained HAL fence: a set of semaphore timepoints.
pub struct Fence(*mut n::iree_hal_fence_t);
impl Fence {
    /// Creates an empty fence with capacity for `capacity` unique semaphores.
    pub fn new(capacity: usize) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: `out` is writable.
        check(unsafe { n::iree_hal_fence_create(capacity, system(), &mut out) })?;
        // SAFETY: successful creation returns one owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }

    /// Creates a fence containing one semaphore timepoint.
    pub fn at(semaphore: &Semaphore, value: u64) -> Result<Self> {
        let mut out = ptr::null_mut();
        // SAFETY: the semaphore is live and `out` is writable.
        check(unsafe {
            n::iree_hal_fence_create_at(semaphore.as_raw(), value, system(), &mut out)
        })?;
        // SAFETY: successful creation returns one owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }

    /// Joins fences into one fence reached when all of them are.
    pub fn join(fences: &[&Fence]) -> Result<Self> {
        let mut refs: Vec<_> = fences.iter().map(|f| f.0).collect();
        let mut out = ptr::null_mut();
        // SAFETY: every fence is live for the call and `out` is writable.
        check(unsafe {
            n::iree_hal_fence_join(refs.len(), refs.as_mut_ptr(), system(), &mut out)
        })?;
        // SAFETY: successful join returns one owned reference.
        Ok(unsafe { Self::from_raw(out) })
    }

    /// Returns the fence held by `reference`.
    pub fn from_ref(reference: &crate::vm::Ref) -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: the reference is live and the output pointer is writable.
        check(unsafe { n::iree_hal_fence_check_deref(*reference.as_raw(), &mut raw) })?;
        // SAFETY: the reference keeps the fence alive; retain gives this wrapper its own reference.
        unsafe { n::iree_hal_fence_retain(raw) };
        // SAFETY: the retain above produced one owned reference.
        Ok(unsafe { Self::from_raw(raw) })
    }

    /// Returns a new VM reference to this fence.
    pub fn to_ref(&self) -> crate::vm::Ref {
        // SAFETY: this live fence is retained into the returned VM reference.
        let raw = unsafe { n::iree_hal_fence_retain_ref(self.0) };
        // SAFETY: retain_ref returns an owned VM reference.
        unsafe { crate::vm::Ref::from_raw(raw) }
    }
    /// Adopts one owned fence reference.
    ///
    /// # Safety
    /// `raw` must be a live fence pointer whose ownership is transferred here.
    pub unsafe fn from_raw(raw: *mut n::iree_hal_fence_t) -> Self {
        Self(raw)
    }

    /// Borrows the native fence pointer.
    pub fn as_raw(&self) -> *mut n::iree_hal_fence_t {
        self.0
    }

    /// Transfers the owned reference to the caller.
    pub fn into_raw(self) -> *mut n::iree_hal_fence_t {
        core::mem::ManuallyDrop::new(self).0
    }

    /// Returns the number of unique semaphore timepoints.
    pub fn timepoint_count(&self) -> usize {
        // SAFETY: the fence is live.
        unsafe { n::iree_hal_fence_timepoint_count(self.0) }
    }

    /// Returns the fence's semaphore timepoints, each semaphore retained.
    pub fn timepoints(&self) -> Vec<(Semaphore, u64)> {
        // SAFETY: the fence is live; the list borrows its storage.
        let list = unsafe { n::iree_hal_fence_semaphore_list(self.0) };
        if list.count == 0 {
            return Vec::new();
        }
        // SAFETY: the list holds `count` initialized entries owned by the fence.
        let (semaphores, values) = unsafe {
            (
                n::slice(list.semaphores, list.count),
                n::slice(list.payload_values, list.count),
            )
        };
        semaphores
            .iter()
            .zip(values)
            .map(|(&raw, &value)| {
                // SAFETY: the semaphore is live in the fence; retain it for the caller.
                unsafe { n::iree_hal_semaphore_retain(raw) };
                // SAFETY: the reference retained above is owned by the wrapper.
                (unsafe { Semaphore::from_raw(raw) }, value)
            })
            .collect()
    }

    /// Inserts a semaphore timepoint, keeping the maximum value if already present.
    pub fn insert(&self, semaphore: &Semaphore, value: u64) -> Result<()> {
        // SAFETY: the fence and semaphore are live.
        check(unsafe { n::iree_hal_fence_insert(self.0, semaphore.as_raw(), value) })
    }

    /// Extends this fence with every timepoint from another fence.
    pub fn extend(&self, other: &Fence) -> Result<()> {
        // SAFETY: both fences are live.
        check(unsafe { n::iree_hal_fence_extend(self.0, other.0) })
    }

    /// Returns whether every timepoint has been reached, or the first failure.
    pub fn query(&self) -> Result<bool> {
        // SAFETY: the fence is live.
        match check(unsafe { n::iree_hal_fence_query(self.0) }) {
            Ok(()) => Ok(true),
            Err(error) if error.code() == Code::Deferred => Ok(false),
            Err(error) => Err(error),
        }
    }

    /// Signals all fence timepoints.
    pub fn signal(&self) -> Result<()> {
        // SAFETY: the fence is live.
        check(unsafe { n::iree_hal_fence_signal(self.0) })
    }

    /// Fails all fence timepoints with `code`.
    pub fn fail(&self, code: Code) {
        // SAFETY: the fence is live; ownership of the status passes to IREE.
        unsafe { n::iree_hal_fence_fail(self.0, n::abi::status::from_code(code.to_raw())) }
    }

    /// Blocks until the fence is reached or `timeout` (`None` waits forever) expires.
    pub fn wait(&self, timeout: Option<Duration>, flags: WaitFlags) -> Result<()> {
        let timeout = n::timeout(timeout)?;
        // SAFETY: the fence is live.
        check(unsafe { n::iree_hal_fence_wait(self.0, timeout, flags.bits()) })
    }
}
impl Clone for Fence {
    fn clone(&self) -> Self {
        // SAFETY: the wrapper owns a live reference.
        unsafe { n::iree_hal_fence_retain(self.0) };
        Self(self.0)
    }
}
impl Drop for Fence {
    fn drop(&mut self) {
        // SAFETY: the wrapper owns exactly one reference.
        unsafe { n::iree_hal_fence_release(self.0) }
    }
}

// SAFETY: IREE HAL fence reference counting and host wait/signal operations are thread-safe.
unsafe impl Send for Fence {}
