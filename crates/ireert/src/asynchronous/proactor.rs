//! Caller-driven proactor ownership.
use crate::{Code, Error, Result, check};
use iree_native as n;
use std::time::Duration;
use std::{cell::Cell, rc::Rc};

struct Inner {
    raw: *mut n::iree_async_proactor_t,
    polling: Cell<bool>,
}
impl Drop for Inner {
    fn drop(&mut self) {
        // SAFETY: Inner owns one live proactor reference.
        unsafe { n::iree_async_proactor_release(self.raw) };
    }
}

crate::hal::bitfield::bitfield! {
    /// Native proactor backend capability bits.
    Capabilities: u64 {
        /// Native repeated-completion operations.
        MULTISHOT = n::IREE_ASYNC_PROACTOR_CAPABILITY_MULTISHOT,
        /// Fixed file tables.
        FIXED_FILES = n::IREE_ASYNC_PROACTOR_CAPABILITY_FIXED_FILES,
        /// Registered zero-copy memory.
        REGISTERED_BUFFERS = n::IREE_ASYNC_PROACTOR_CAPABILITY_REGISTERED_BUFFERS,
        /// Native linked operation chains.
        LINKED_OPERATIONS = n::IREE_ASYNC_PROACTOR_CAPABILITY_LINKED_OPERATIONS,
        /// Zero-copy socket sends.
        ZERO_COPY_SEND = n::IREE_ASYNC_PROACTOR_CAPABILITY_ZERO_COPY_SEND,
        /// DMA buffer import.
        DMABUF = n::IREE_ASYNC_PROACTOR_CAPABILITY_DMABUF,
        /// Device fence import/export.
        DEVICE_FENCE = n::IREE_ASYNC_PROACTOR_CAPABILITY_DEVICE_FENCE,
        /// Absolute native timer deadlines.
        ABSOLUTE_TIMEOUT = n::IREE_ASYNC_PROACTOR_CAPABILITY_ABSOLUTE_TIMEOUT,
        /// Native futex waits and wakes.
        FUTEX_OPERATIONS = n::IREE_ASYNC_PROACTOR_CAPABILITY_FUTEX_OPERATIONS,
        /// Proactor-to-proactor message delivery.
        PROACTOR_MESSAGING = n::IREE_ASYNC_PROACTOR_CAPABILITY_PROACTOR_MESSAGING,
        /// Windows wait-completion packets.
        WAIT_COMPLETION_PACKET = n::IREE_ASYNC_PROACTOR_CAPABILITY_WAIT_COMPLETION_PACKET,
    }
}

/// One retained native proactor, bound to the constructing thread.
///
/// Clones share a poll-reentrancy guard. No `Send`/`Sync` implementation is
/// provided: `proactor.h` assigns polling permanently to its first poll thread.
#[derive(Clone)]
pub struct Proactor(Rc<Inner>);
impl Proactor {
    /// Adopts one owned proactor reference on its polling thread.
    ///
    /// # Safety
    /// `raw` must be live, exclusively driven by this family of wrappers, and
    /// either unpolled or already owned by the current thread. No native poll
    /// runner may be active. This thread must remain its poll owner.
    pub unsafe fn from_raw(raw: *mut n::iree_async_proactor_t) -> Self {
        Self(Rc::new(Inner {
            raw,
            polling: Cell::new(false),
        }))
    }
    /// Borrows the native proactor.
    pub fn as_raw(&self) -> *mut n::iree_async_proactor_t {
        self.0.raw
    }
    /// Transfers one native reference, retaining when other clones exist.
    pub fn into_raw(self) -> *mut n::iree_async_proactor_t {
        let raw = self.as_raw();
        // SAFETY: The wrapper holds a live proactor reference while retaining one for the caller.
        unsafe { n::iree_async_proactor_retain(raw) };
        raw
    }
    /// Returns the backend's capability bit mask.
    pub fn capabilities(&self) -> Capabilities {
        // SAFETY: the proactor is live.
        Capabilities::from_bits_retain(unsafe {
            n::iree_async_proactor_query_capabilities(self.as_raw())
        } as u64)
    }
    /// Delivers completions on this thread, rejecting reentrant polling.
    pub fn poll(&self, timeout: Option<Duration>) -> Result<usize> {
        if self.0.polling.replace(true) {
            return Err(Error::new(
                Code::FailedPrecondition,
                "reentrant proactor poll",
            ));
        }
        struct Guard<'a>(&'a Cell<bool>);
        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                self.0.set(false);
            }
        }
        let guard = Guard(&self.0.polling);
        let mut count = 0;
        // SAFETY: self retains the live proactor and the completion count is writable.
        let result = check(unsafe {
            n::iree_async_proactor_poll(self.as_raw(), n::timeout(timeout)?, &mut count)
        });
        drop(guard);
        result?;
        Ok(count)
    }
    /// Wakes a blocked native poll.
    pub fn wake(&self) {
        // SAFETY: self retains the live proactor being woken.
        unsafe { n::iree_async_proactor_wake(self.as_raw()) };
    }
}
