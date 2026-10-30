//! VM reference type registration and list element types.
use super::{Instance, Ref, ValueType};
use crate::{Code, Error, Result, check};
use iree_native as n;
use std::any::{Any, TypeId};
use std::sync::{Mutex, PoisonError};

/// Rust-side record of a type registered through [`RefType::register`].
///
/// Native instances and modules can retain raw descriptor pointers whose
/// lifetimes Rust cannot observe, so descriptor storage lives for the process
/// even when a native registration is explicitly unregistered.
// `align(8)` satisfies the `IREE_VM_REF_TYPE_TAG_BITS` descriptor alignment.
#[repr(C, align(8))]
struct Registration {
    descriptor: n::iree_vm_ref_type_descriptor_t,
    payload: TypeId,
    name: Box<str>,
}

// SAFETY: a registration is immutable once leaked, and its descriptor only
// points at the process-lived name and a stateless destroy function.
unsafe impl Send for Registration {}
// SAFETY: as above; shared reads of immutable data are race-free.
unsafe impl Sync for Registration {}

/// A VM reference type identity.
///
/// Equality and hashing compare only the native identity. Handles returned by
/// [`Self::register`] also know their Rust payload type, which
/// [`Self::create`] and [`Self::downcast`] require.
#[derive(Clone, Copy)]
pub struct RefType {
    id: n::iree_vm_ref_type_t,
    registration: Option<&'static Registration>,
}

impl RefType {
    /// Registers `T` under a VM type name in `instance`.
    ///
    /// Registering the same name with the same `T` again returns the same
    /// type and adds one native registration count, matching upstream's
    /// descriptor deduplication. Fails with [`Code::AlreadyExists`] if the
    /// name is registered for another payload type. Payloads may be destroyed
    /// on any native thread, so `T` must be `Send + Sync`.
    pub fn register<T: Any + Send + Sync>(instance: &Instance, name: &str) -> Result<Self> {
        // Every registration this process made, so a name keeps one descriptor.
        static REGISTRY: Mutex<Vec<&'static Registration>> = Mutex::new(Vec::new());
        let mut registry = REGISTRY.lock().unwrap_or_else(PoisonError::into_inner);
        let payload = TypeId::of::<T>();
        if let Some(registration) = registry
            .iter()
            .copied()
            .find(|known| known.name.as_ref() == name)
        {
            if registration.payload != payload {
                return Err(Error::new(
                    Code::AlreadyExists,
                    "VM reference type name is registered for another Rust type",
                ));
            }
            // Upstream deduplicates by descriptor address only, so a name already
            // bound to another descriptor would otherwise be registered twice.
            if Self::lookup(instance, name).is_some_and(|existing| {
                !std::ptr::eq(existing.descriptor(), &registration.descriptor)
            }) {
                return Err(Error::new(
                    Code::AlreadyExists,
                    "VM reference type name is already registered",
                ));
            }
            let mut id = 0;
            // SAFETY: the instance is live and the descriptor lives for the process.
            check(unsafe {
                n::iree_vm_instance_register_type(
                    instance.as_raw(),
                    &registration.descriptor,
                    &mut id,
                )
            })?;
            return Ok(Self {
                id,
                registration: Some(registration),
            });
        }

        let name = name.to_owned().into_boxed_str();
        let mut registration = Box::new(Registration {
            descriptor: n::iree_vm_ref_type_descriptor_t {
                destroy: Some(super::reference::destroy_payload::<T>),
                type_name: crate::view(&name),
                ..Default::default()
            },
            payload,
            name,
        });
        // The counter is the first field of `Payload<T>`.
        registration.descriptor.set_offsetof_counter(0);
        if Self::lookup(instance, registration.name.as_ref())
            .is_some_and(|existing| !std::ptr::eq(existing.descriptor(), &registration.descriptor))
        {
            return Err(Error::new(
                Code::AlreadyExists,
                "VM reference type name is already registered",
            ));
        }
        let mut id = 0;
        // SAFETY: the instance and candidate are live; IREE retains the descriptor only on success.
        check(unsafe {
            n::iree_vm_instance_register_type(instance.as_raw(), &registration.descriptor, &mut id)
        })?;
        let registration = Box::leak(registration);
        registry.push(registration);
        Ok(Self {
            id,
            registration: Some(registration),
        })
    }

    /// Unregisters this type from `instance`.
    ///
    /// The descriptor stays allocated, so later native lookups remain memory-safe.
    ///
    /// # Safety
    /// No reference of this type may be live in `instance`, and no module or
    /// context may use the type afterwards.
    pub unsafe fn unregister(self, instance: &Instance) -> Result<()> {
        let registration = self.registration.ok_or_else(|| {
            Error::new(
                Code::FailedPrecondition,
                "only handles returned by RefType::register can be unregistered",
            )
        })?;
        // SAFETY: the caller guarantees no live references; the descriptor is process-lived.
        unsafe { n::iree_vm_instance_unregister_type(instance.as_raw(), &registration.descriptor) };
        Ok(())
    }

    /// Looks up a registered type by full name.
    ///
    /// The result has no Rust payload type; use the handle returned by
    /// [`Self::register`] to create or downcast references.
    pub fn lookup(instance: &Instance, name: &str) -> Option<Self> {
        // SAFETY: the instance and name view are valid for the call.
        let id = unsafe { n::iree_vm_instance_lookup_type(instance.as_raw(), crate::view(name)) };
        (id != 0).then_some(Self {
            id,
            registration: None,
        })
    }

    /// Creates a reference owning `value`.
    ///
    /// Fails unless this handle came from [`Self::register`] with the same `T`.
    pub fn create<T: Any + Send + Sync>(&self, value: T) -> Result<Ref> {
        if !self.holds::<T>() {
            return Err(Error::new(
                Code::InvalidArgument,
                "reference type was not registered for this Rust type",
            ));
        }
        Ref::wrap(self.id, value)
    }

    /// Borrows the payload of `reference` if it was created from this registration as `T`.
    pub fn downcast<'a, T: Any + Send + Sync>(&self, reference: &'a Ref) -> Option<&'a T> {
        (self.holds::<T>() && !reference.is_null() && reference.raw().type_ == self.id)
            // SAFETY: the type id names this registration, whose payloads are `Payload<T>`.
            .then(|| unsafe { reference.payload::<T>() })
    }

    /// Returns the name given to [`Self::register`]; `None` for looked-up handles.
    pub fn name(&self) -> Option<&'static str> {
        self.registration
            .map(|registration| registration.name.as_ref())
    }

    /// Returns the native type identifier.
    pub fn as_raw(&self) -> n::iree_vm_ref_type_t {
        self.id
    }

    /// Wraps a native type identifier without a Rust payload type.
    ///
    /// # Safety
    /// `id` must identify a valid native reference type whose descriptor
    /// remains live for every use of this handle and every list storing it.
    pub unsafe fn from_raw(id: n::iree_vm_ref_type_t) -> Self {
        Self {
            id,
            registration: None,
        }
    }

    fn holds<T: Any>(&self) -> bool {
        self.registration
            .is_some_and(|registration| registration.payload == TypeId::of::<T>())
    }

    /// The descriptor address encoded in the identifier above its tag bits.
    fn descriptor(&self) -> *const n::iree_vm_ref_type_descriptor_t {
        let tag_mask: n::iree_vm_ref_type_t = (1 << n::IREE_VM_REF_TYPE_TAG_BITS) - 1;
        (self.id & !tag_mask) as *const _
    }
}

impl std::fmt::Debug for RefType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RefType").field(&self.id).finish()
    }
}

impl PartialEq for RefType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for RefType {}

impl std::hash::Hash for RefType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// VM list element type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypeDef {
    /// Any value or reference.
    Any,
    /// A specific scalar value.
    Value(ValueType),
    /// A specific reference type.
    Ref(RefType),
}

impl TypeDef {
    pub(crate) fn raw(self) -> n::iree_vm_type_def_t {
        // SAFETY: these constructors only pack their arguments.
        unsafe {
            match self {
                Self::Any => n::iree_vm_make_undefined_type_def(),
                Self::Value(value_type) => n::iree_vm_make_value_type_def(value_type.raw()),
                Self::Ref(ref_type) => n::iree_vm_make_ref_type_def(ref_type.id),
            }
        }
    }

    pub(crate) fn from_raw(raw: n::iree_vm_type_def_t) -> Option<Self> {
        if n::abi::vm::type_def_is_undefined(raw) {
            Some(Self::Any)
        } else if n::abi::vm::type_def_is_value(raw) {
            ValueType::from_raw(n::abi::vm::type_def_as_value(raw)).map(Self::Value)
        } else if n::abi::vm::type_def_is_ref(raw) {
            // SAFETY: callers provide type definitions from live native lists
            // and variants, whose reference descriptors remain valid.
            Some(Self::Ref(unsafe {
                RefType::from_raw(n::abi::vm::type_def_as_ref(raw))
            }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn duplicate_names_are_rejected() {
        let instance = Instance::new().unwrap();
        RefType::register::<i64>(&instance, "test.duplicate").unwrap();
        let error = RefType::register::<u8>(&instance, "test.duplicate").unwrap_err();
        assert_eq!(error.code(), Code::AlreadyExists);
    }

    proptest! {
        #[test]
        fn create_downcast_and_clone_preserve_value(value in any::<i64>(), count in 0usize..32) {
            let instance = Instance::new().unwrap();
            let ty = RefType::register::<i64>(&instance, "test.integer").unwrap();
            let reference = ty.create(value).unwrap();
            let clones: Vec<_> = (0..count).map(|_| reference.clone()).collect();
            prop_assert_eq!(ty.downcast::<i64>(&reference), Some(&value));
            for clone in &clones {
                prop_assert!(std::ptr::eq(
                    ty.downcast::<i64>(&reference).unwrap(),
                    ty.downcast::<i64>(clone).unwrap(),
                ));
            }
            prop_assert!(ty.downcast::<String>(&reference).is_none());
            prop_assert!(ty.create(String::new()).is_err());
            let looked_up = RefType::lookup(&instance, "test.integer").unwrap();
            prop_assert_eq!(looked_up, ty);
            prop_assert!(looked_up.downcast::<i64>(&reference).is_none());
        }

        #[test]
        fn final_release_drops_exactly_once(count in 0usize..32) {
            struct Counted(Arc<AtomicUsize>);
            impl Drop for Counted {
                fn drop(&mut self) {
                    self.0.fetch_add(1, Ordering::SeqCst);
                }
            }
            let drops = Arc::new(AtomicUsize::new(0));
            let instance = Instance::new().unwrap();
            let ty = RefType::register::<Counted>(&instance, "test.counted").unwrap();
            let reference = ty.create(Counted(drops.clone())).unwrap();
            let clones: Vec<_> = (0..count).map(|_| reference.clone()).collect();
            drop(clones);
            prop_assert_eq!(drops.load(Ordering::SeqCst), 0);
            drop(instance);
            drop(reference);
            prop_assert_eq!(drops.load(Ordering::SeqCst), 1);
        }
    }
}
