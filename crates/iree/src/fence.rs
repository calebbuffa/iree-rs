use crate::{Error, Result, Semaphore, Timeout, Wait, runtime::check, sys};
use std::{
    future::Future,
    pin::Pin,
    ptr,
    task::{Context, Poll},
};

impl Semaphore {
    /// Create an immutable fence for this semaphore reaching a value.
    pub fn at(&self, value: u64) -> Result<Fence> {
        if value > sys::IREE_HAL_SEMAPHORE_MAX_VALUE as u64 {
            return Err(Error::invalid(
                "fence timepoint uses a reserved failure value",
            ));
        }
        let mut raw = ptr::null_mut();
        check(unsafe {
            sys::iree_hal_fence_create_at(
                self.as_raw(),
                value,
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Fence {
            raw,
            timepoints: vec![(self.clone(), value)],
        })
    }
}

/// An immutable native wait-all fence retaining its device semaphore owners.
///
/// Construct timepoints with [`Semaphore::at`] and combine them with
/// [`Self::join`]. A consumed fence cannot be mutated in place.
pub struct Fence {
    raw: *mut sys::iree_hal_fence_t,
    timepoints: Vec<(Semaphore, u64)>,
}

impl Fence {
    pub(crate) fn validate_device(&self, device: &crate::Device) -> Result<()> {
        if self
            .timepoints
            .iter()
            .any(|(semaphore, _)| semaphore.device.as_raw() != device.as_raw())
        {
            return Err(Error::invalid("fence belongs to a different device"));
        }
        Ok(())
    }
    /// An already-satisfied fence with no dependencies.
    pub fn empty() -> Self {
        Self {
            raw: ptr::null_mut(),
            timepoints: Vec::new(),
        }
    }

    /// Join all input fences and retain each source device.
    pub fn join(fences: &[Self]) -> Result<Self> {
        let mut raw = ptr::null_mut();
        let mut pointers: Vec<_> = fences.iter().map(Self::as_raw).collect();
        check(unsafe {
            sys::iree_hal_fence_join(
                pointers.len(),
                pointers.as_mut_ptr(),
                sys::iree_allocator_system(),
                &mut raw,
            )
        })?;
        Ok(Self {
            raw,
            timepoints: fences
                .iter()
                .flat_map(|fence| fence.timepoints.iter().cloned())
                .collect(),
        })
    }

    /// Query readiness without blocking, preserving any native failure.
    pub fn is_ready(&self) -> Result<bool> {
        let status = unsafe { sys::iree_hal_fence_query(self.raw) };
        let code = unsafe { sys::iree_rs_status_code(status) };
        if code == sys::IREE_STATUS_DEFERRED {
            unsafe { sys::iree_rs_status_free(status) };
            return Ok(false);
        }
        check(status)?;
        Ok(true)
    }

    /// Block until every timepoint completes or the timeout elapses.
    pub fn wait_blocking(&self, timeout: Timeout) -> Result<()> {
        check(unsafe { sys::iree_hal_fence_wait(self.raw, timeout.native()?, 0) })
    }

    /// Await every timepoint using native callbacks, without a blocking worker.
    pub fn wait(&self) -> Result<FenceWait> {
        let waits = self
            .timepoints
            .iter()
            .map(|(semaphore, value)| semaphore.wait(*value).map(Some))
            .collect::<Result<_>>()?;
        Ok(FenceWait {
            waits,
            completed: false,
        })
    }

    /// Borrow the native fence; null denotes the already-satisfied empty fence.
    pub fn as_raw(&self) -> *mut sys::iree_hal_fence_t {
        self.raw
    }
}

impl Clone for Fence {
    fn clone(&self) -> Self {
        unsafe { sys::iree_hal_fence_retain(self.raw) };
        Self {
            raw: self.raw,
            timepoints: self.timepoints.clone(),
        }
    }
}
impl Default for Fence {
    fn default() -> Self {
        Self::empty()
    }
}
impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { sys::iree_hal_fence_release(self.raw) };
    }
}

/// Cancellation-safe completion of all timepoints in an immutable fence.
pub struct FenceWait {
    waits: Vec<Option<Wait<Semaphore>>>,
    completed: bool,
}

impl Future for FenceWait {
    type Output = Result<()>;
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            return Poll::Ready(Err(Error::invalid("fence future was already consumed")));
        }
        let mut pending = false;
        for entry in &mut self.waits {
            if let Some(wait) = entry {
                match Pin::new(wait).poll(context) {
                    Poll::Ready(Ok(())) => {
                        *entry = None;
                    }
                    Poll::Ready(Err(error)) => {
                        self.completed = true;
                        self.waits.clear();
                        return Poll::Ready(Err(error));
                    }
                    Poll::Pending => pending = true,
                }
            }
        }
        if pending {
            Poll::Pending
        } else {
            self.completed = true;
            Poll::Ready(Ok(()))
        }
    }
}
