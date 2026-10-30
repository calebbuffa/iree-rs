//! Ports of publicly expressible cases in upstream `vm/ref_test.cc`.
// Raw C retain-or-move/assign variants are replaced by Rust clone, move, null, identity, and Drop laws.

use ireert::vm::{Instance, RefType};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct DropProbe(Arc<AtomicUsize>);
impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn type_registration_and_lookup() {
    let instance = Instance::new().unwrap();
    let ty = RefType::register::<i32>(&instance, "test.ref.type_registration").unwrap();
    assert_eq!(
        RefType::lookup(&instance, "test.ref.type_registration"),
        Some(ty)
    );
    assert!(RefType::lookup(&instance, "test.ref.type_registration.missing").is_none());
}

#[test]
fn wrapping_and_retaining_release_payload_once() {
    let instance = Instance::new().unwrap();
    let ty = RefType::register::<DropProbe>(&instance, "test.ref.retention").unwrap();
    let drops = Arc::new(AtomicUsize::new(0));
    let reference = ty.create(DropProbe(drops.clone())).unwrap();
    let retained = reference.clone();
    drop(reference);
    assert_eq!(drops.load(Ordering::SeqCst), 0);
    assert!(ty.downcast::<DropProbe>(&retained).is_some());
    drop(retained);
    assert_eq!(drops.load(Ordering::SeqCst), 1);
}

#[test]
fn registration_rejects_different_payload_type_for_same_name() {
    let instance = Instance::new().unwrap();
    let name = "test.ref.registration_payload_conflict";
    RefType::register::<i32>(&instance, name).unwrap();
    assert!(RefType::register::<u32>(&instance, name).is_err());
}

#[test]
fn wrong_type_downcast_is_none() {
    let instance = Instance::new().unwrap();
    let ty = RefType::register::<i32>(&instance, "test.ref.downcast").unwrap();
    let reference = ty.create(1).unwrap();
    assert!(ty.downcast::<u32>(&reference).is_none());
}

#[test]
fn null_and_identity_operations() {
    let instance = Instance::new().unwrap();
    let ty = RefType::register::<i32>(&instance, "test.ref.identity").unwrap();
    let null = ireert::vm::Ref::null();
    assert!(null.is_null());
    assert!(null.is_equal_to(&ireert::vm::Ref::null()));
    let value = ty.create(5).unwrap();
    let retained = value.clone();
    assert!(value.is_equal_to(&retained));
    assert!(!value.is_equal_to(&null));
}
