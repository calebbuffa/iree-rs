//! Behavioural laws of the runtime bindings.
// Test fixtures outside `#[test]` functions assert by panicking.

use ireert::vm::{Buffer, BufferAccess, Instance, List, RefType, TypeDef, Value};
use proptest::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

proptest! {
    #[test]
    fn list_push_then_get_preserves_immediate_value(value in any::<i64>()) {
        let mut list = List::new( TypeDef::Any, 0).unwrap();
        list.push(Value::I64(value)).unwrap();
        prop_assert_eq!(list.len(), 1);
        prop_assert_eq!(list.get_value(0).unwrap(), Value::I64(value));
    }
}

struct DropProbe(Arc<AtomicUsize>);

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

proptest! {
    #[test]
    fn vm_buffer_clone_and_copy_preserve_bytes(bytes in prop::collection::vec(any::<u8>(), 0..128)) {
        let instance = Instance::new().unwrap();
        let mut source = Buffer::new(&instance, BufferAccess::MUTABLE, bytes.len(), 1).unwrap();
        source.write(0, &bytes).unwrap();
        let mut clone = source
            .clone_range(BufferAccess::MUTABLE, 0, bytes.len(), 1)
            .unwrap();
        prop_assert!(source.compare(0, &clone, 0, bytes.len()).unwrap());
        clone.fill(0, bytes.len(), 0).unwrap();
        source.copy_to(0, &clone, 0, bytes.len()).unwrap();
        let mut round_trip = vec![0; bytes.len()];
        clone.read(0, &mut round_trip).unwrap();
        prop_assert_eq!(round_trip, bytes);
    }

    #[test]
    fn retained_custom_refs_destroy_payload_once(clones in 1usize..32) {
        let drops = Arc::new(AtomicUsize::new(0));
        let instance = Instance::new().unwrap();
        let ty = RefType::register::<DropProbe>(&instance, "test.drop_probe").unwrap();
        let value = ty.create(DropProbe(drops.clone())).unwrap();
        let retained: Vec<_> = (0..clones).map(|_| value.clone()).collect();
        drop(value);
        prop_assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(retained);
        prop_assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}

proptest! {
    #[test]
    fn list_reference_retain_and_release_is_balanced(clones in 0usize..16) {
        let drops = Arc::new(AtomicUsize::new(0));
        let instance = Instance::new().unwrap();
        let ty = RefType::register::<DropProbe>(&instance, "test.list_drop_probe").unwrap();
        let value = ty.create(DropProbe(drops.clone())).unwrap();
        let mut list = List::new( TypeDef::Any, 0).unwrap();
        list.push(value.clone()).unwrap();
        let retained: Vec<_> = (0..clones).map(|_| value.clone()).collect();
        drop(value);
        let read_back = list.get_ref(0).unwrap();
        list.clear();
        prop_assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(retained);
        prop_assert_eq!(drops.load(Ordering::SeqCst), 0);
        drop(read_back);
        prop_assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}
