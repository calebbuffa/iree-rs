//! VM instance ownership.
use crate::{Code, Error, Result, check};
use iree_native as n;
use std::sync::{Mutex, PoisonError};

/// Shared VM type registry and allocator state.
///
/// Every handle refers to one process-wide native instance, created on first
/// use and kept for the rest of the process. Upstream recommends sharing one
/// instance, and it is also required for soundness: registering a builtin
/// or HAL type first writes 0 to its process-global type id, so creating a
/// second native instance would briefly invalidate type ids that other
/// threads are using. Contexts provide isolation between workloads.
pub struct Instance {
    raw: *mut n::iree_vm_instance_t,
}

struct SharedState {
    instance: Option<(Instance, usize)>,
    #[cfg(feature = "cpu")]
    isolated_instances: usize,
}

/// The process instance and any native isolated instances that rewrite global VM type IDs.
static SHARED: Mutex<SharedState> = Mutex::new(SharedState {
    instance: None,
    #[cfg(feature = "cpu")]
    isolated_instances: 0,
});

/// Prevents safe creation of the shared instance while a native isolated VM instance is alive.
#[cfg(feature = "cpu")]
pub(crate) struct IsolatedTypeRegistryGuard;

#[cfg(feature = "cpu")]
impl IsolatedTypeRegistryGuard {
    pub(crate) fn begin() -> Result<Self> {
        let mut state = SHARED.lock().unwrap_or_else(PoisonError::into_inner);
        if state.instance.is_some() || state.isolated_instances != 0 {
            return Err(Error::new(
                Code::FailedPrecondition,
                "an isolated VM instance cannot coexist with another VM instance",
            ));
        }
        state.isolated_instances = 1;
        Ok(Self)
    }
}

#[cfg(feature = "cpu")]
impl Drop for IsolatedTypeRegistryGuard {
    fn drop(&mut self) {
        let mut state = SHARED.lock().unwrap_or_else(PoisonError::into_inner);
        state.isolated_instances -= 1;
    }
}

impl Instance {
    /// Returns the process instance, creating it with upstream's default type
    /// capacity and the HAL types registered if it does not exist yet.
    ///
    /// Returns [`Code::FailedPrecondition`] while an isolated VMVX loader is
    /// alive, because it owns a VM instance with conflicting process-global
    /// type IDs.
    pub fn new() -> Result<Self> {
        Self::shared(None)
    }

    /// Returns the process instance, creating it with room for
    /// `type_capacity` reference types in total if it does not exist yet.
    ///
    /// The builtin and HAL types count against the capacity; size it
    /// explicitly to bound memory on constrained targets. Fails with
    /// [`Code::FailedPrecondition`] if the instance already exists with a
    /// different capacity.
    pub fn with_type_capacity(type_capacity: usize) -> Result<Self> {
        Self::shared(Some(type_capacity))
    }

    fn shared(type_capacity: Option<usize>) -> Result<Self> {
        let mut shared = SHARED.lock().unwrap_or_else(PoisonError::into_inner);
        #[cfg(feature = "cpu")]
        if shared.isolated_instances != 0 {
            return Err(Error::new(
                Code::FailedPrecondition,
                "the process VM type registry is in use by an isolated VM instance",
            ));
        }
        if let Some((instance, capacity)) = shared.instance.as_ref() {
            if type_capacity.is_some_and(|requested| requested != *capacity) {
                return Err(Error::new(
                    Code::FailedPrecondition,
                    "the process VM instance already exists with a different type capacity",
                ));
            }
            return Ok(instance.clone());
        }
        let capacity = type_capacity.unwrap_or(n::IREE_VM_TYPE_CAPACITY_DEFAULT as usize);
        let mut raw = std::ptr::null_mut();
        // SAFETY: the output is writable, and the lock above guarantees this
        // is the only native instance creation in the process.
        check(unsafe { n::iree_vm_instance_create(capacity, crate::system(), &mut raw) })?;
        let instance = Self { raw };
        // SAFETY: the instance is live, and no other instance exists yet whose
        // users could observe the global type ids being rewritten.
        check(unsafe { n::iree_hal_module_register_all_types(raw) })?;
        shared.instance = Some((instance.clone(), capacity));
        Ok(instance)
    }

    /// Adopts an owned instance pointer.
    ///
    /// # Safety
    /// `raw` must be a live, owned IREE VM instance pointer. Creating native
    /// instances outside [`Self::new`] while any instance is in use races on
    /// IREE's global type ids.
    pub unsafe fn from_raw(raw: *mut n::iree_vm_instance_t) -> Self {
        Self { raw }
    }

    /// Borrows the native instance pointer.
    pub fn as_raw(&self) -> *mut n::iree_vm_instance_t {
        self.raw
    }

    /// Transfers the owned pointer to the caller.
    pub fn into_raw(self) -> *mut n::iree_vm_instance_t {
        std::mem::ManuallyDrop::new(self).raw
    }
}

impl Clone for Instance {
    fn clone(&self) -> Self {
        // SAFETY: `self.raw` is a live ref-counted instance.
        unsafe { n::iree_vm_instance_retain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one instance reference.
        unsafe { n::iree_vm_instance_release(self.raw) }
    }
}

// SAFETY: the instance is atomically reference counted and its type table is
// guarded by `type_mutex` in instance.c.
unsafe impl Send for Instance {}
// SAFETY: every native instance operation takes `type_mutex` or only reads
// immutable fields.
unsafe impl Sync for Instance {}
