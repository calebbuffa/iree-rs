//! Owned VM references.
use crate::{Result, check};
use iree_native as n;
use std::any::Any;
use std::ffi::c_void;

/// Heap object behind a reference created by [`super::RefType::create`].
#[repr(C)]
struct Payload<T> {
    header: n::iree_vm_ref_object_t,
    value: T,
}

pub(super) unsafe extern "C" fn destroy_payload<T>(ptr: *mut c_void) {
    // SAFETY: IREE calls `destroy` once, after the final release, with the
    // pointer passed to `iree_vm_ref_wrap_assign`.
    n::contain(|| drop(unsafe { Box::from_raw(ptr.cast::<Payload<T>>()) }));
}

/// An owned, possibly null `iree_vm_ref_t`.
pub struct Ref {
    raw: n::iree_vm_ref_t,
}

impl Ref {
    /// Creates a null VM reference.
    pub fn null() -> Self {
        Self {
            raw: n::iree_vm_ref_t::default(),
        }
    }

    pub(super) fn wrap<T: Any + Send + Sync>(
        type_: n::iree_vm_ref_type_t,
        value: T,
    ) -> Result<Self> {
        let mut payload = Box::new(Payload {
            header: n::iree_vm_ref_object_t::default(),
            value,
        });
        // SAFETY: the counter is a valid, exclusively borrowed atomic.
        unsafe { n::abi::ref_count::init(&mut payload.header.counter) };
        let ptr = Box::into_raw(payload);
        let mut raw = n::iree_vm_ref_t::default();
        // SAFETY: `ptr` is a live payload whose counter `type_` describes.
        let status = unsafe { n::iree_vm_ref_wrap_assign(ptr.cast(), type_, &mut raw) };
        if !status.is_null() {
            // SAFETY: a failed wrap did not take ownership of the payload.
            drop(unsafe { Box::from_raw(ptr) });
        }
        check(status)?;
        Ok(Self { raw })
    }

    /// Adopts an owned VM reference.
    ///
    /// # Safety
    /// `raw` must be an owned reference or a null reference.
    pub unsafe fn from_raw(raw: n::iree_vm_ref_t) -> Self {
        Self { raw }
    }

    /// Borrows the native reference.
    pub fn as_raw(&self) -> *const n::iree_vm_ref_t {
        &self.raw
    }

    /// Transfers the owned native reference to the caller.
    pub fn into_raw(self) -> n::iree_vm_ref_t {
        std::mem::ManuallyDrop::new(self).raw
    }

    pub(crate) fn raw(&self) -> n::iree_vm_ref_t {
        self.raw
    }

    /// # Safety
    /// The reference must have been created by `wrap::<T>`.
    pub(super) unsafe fn payload<T>(&self) -> &T {
        // SAFETY: per the caller, `ptr` is a `Payload<T>` kept alive by `self`.
        unsafe { &(*self.raw.ptr.cast::<Payload<T>>()).value }
    }

    /// Returns whether two references identify the same object and type.
    pub fn is_equal_to(&self, other: &Self) -> bool {
        // SAFETY: both owned references are live for the comparison.
        unsafe { n::iree_vm_ref_equal(&self.raw, &other.raw) }
    }

    /// Returns whether this reference is null.
    pub fn is_null(&self) -> bool {
        // SAFETY: `self.raw` is a valid reference.
        unsafe { n::iree_vm_ref_is_null(&self.raw) }
    }

    /// Returns the reference's type, or `None` if it is null.
    pub fn ref_type(&self) -> Option<super::RefType> {
        // SAFETY: a live VM reference keeps its type descriptor valid.
        (self.raw.type_ != 0).then(|| unsafe { super::RefType::from_raw(self.raw.type_) })
    }
}

impl Clone for Ref {
    fn clone(&self) -> Self {
        let mut out = n::iree_vm_ref_t::default();
        // SAFETY: retain only increments the counter of a valid reference.
        unsafe { n::iree_vm_ref_retain((&raw const self.raw).cast_mut(), &mut out) };
        Self { raw: out }
    }
}

impl Drop for Ref {
    fn drop(&mut self) {
        // SAFETY: this wrapper owns one reference; releasing null is a no-op.
        unsafe { n::iree_vm_ref_release(&mut self.raw) }
    }
}
