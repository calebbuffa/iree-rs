//! Ports of public cases in upstream `vm/list_test.cc`.
// SwapStorageSelf is omitted because aliasing the same list twice as `&mut` would be unsound.

use ireert::vm::{Element, Instance, List, RefType, TypeDef, Value, ValueType};

fn instance() -> ireert::Result<Instance> {
    Instance::new()
}

#[test]
fn usage_i32() {
    let mut list = List::new(TypeDef::Value(ValueType::I32), 123).unwrap();
    assert_eq!(list.element_type().unwrap(), TypeDef::Value(ValueType::I32));
    assert!(list.capacity() >= 123);
    assert!(list.is_empty());
    list.resize(5).unwrap();
    for i in 0..5 {
        list.set(i, Value::I32(i as i32)).unwrap();
    }
    for i in 0..5 {
        assert_eq!(
            list.get_value_as(i, ValueType::I32).unwrap(),
            Value::I32(i as i32)
        );
    }
}

#[test]
fn usage_ref() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.usage_ref").unwrap();
    let mut list = List::new(TypeDef::Ref(ty), 123).unwrap();
    assert_eq!(list.element_type().unwrap(), TypeDef::Ref(ty));
    assert!(list.capacity() >= 123);
    list.resize(5).unwrap();
    for i in 0..5 {
        list.set(i, ty.create(i as i32).unwrap()).unwrap();
    }
    for i in 0..5 {
        let value = list.get_ref(i).unwrap();
        assert_eq!(ty.downcast::<i32>(&value), Some(&(i as i32)));
    }
    Ok(())
}

#[test]
fn get_ref_retain_or_move() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.get_ref_retain_or_move").unwrap();
    let mut list = List::new(TypeDef::Ref(ty), 1).unwrap();
    list.push(ty.create(7).unwrap()).unwrap();
    let retained = list.get_ref(0).unwrap();
    let moved = list.take_ref(0).unwrap();
    assert_eq!(ty.downcast::<i32>(&retained), Some(&7));
    assert_eq!(ty.downcast::<i32>(&moved), Some(&7));
    assert!(list.get_ref(0).unwrap().is_null());
    Ok(())
}

#[test]
fn variant_list_ref_move_marks_slot_empty() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.variant_move").unwrap();
    let mut list = List::new(TypeDef::Any, 1).unwrap();
    list.push(ty.create(9).unwrap()).unwrap();
    assert!(matches!(list.take(0).unwrap(), Some(Element::Ref(_))));
    assert!(list.get(0).unwrap().is_none());
    Ok(())
}

#[test]
fn usage_variant() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.variant_usage").unwrap();
    let mut list = List::new(TypeDef::Any, 2).unwrap();
    list.push(Value::I32(1)).unwrap();
    list.push(ty.create(2).unwrap()).unwrap();
    assert!(matches!(
        list.get(0).unwrap(),
        Some(Element::Value(Value::I32(1)))
    ));
    let Some(Element::Ref(reference)) = list.get(1).unwrap() else {
        panic!("expected ref")
    };
    assert_eq!(ty.downcast::<i32>(&reference), Some(&2));
    Ok(())
}

#[test]
fn clone_values_empty() {
    let list = List::new(TypeDef::Value(ValueType::I32), 0).unwrap();
    let clone = list.try_clone().unwrap();
    assert!(clone.is_empty());
    assert_eq!(clone.element_type().unwrap(), list.element_type().unwrap());
}

#[test]
fn clone_values() {
    let mut list = List::new(TypeDef::Value(ValueType::I32), 2).unwrap();
    list.push(Value::I32(3)).unwrap();
    list.push(Value::I32(4)).unwrap();
    let clone = list.try_clone().unwrap();
    assert_eq!(clone.get_value(0).unwrap(), Value::I32(3));
    assert_eq!(clone.get_value(1).unwrap(), Value::I32(4));
}

#[test]
fn clone_refs_empty() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.clone_refs_empty").unwrap();
    let list = List::new(TypeDef::Ref(ty), 0).unwrap();
    assert!(list.try_clone().unwrap().is_empty());
    Ok(())
}

#[test]
fn clone_refs() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.clone_refs").unwrap();
    let mut list = List::new(TypeDef::Ref(ty), 1).unwrap();
    list.push(ty.create(42).unwrap()).unwrap();
    let clone = list.try_clone().unwrap();
    assert_eq!(ty.downcast::<i32>(&clone.get_ref(0).unwrap()), Some(&42));
    Ok(())
}

#[test]
fn clone_variants_empty() {
    let list = List::new(TypeDef::Any, 0).unwrap();
    assert!(list.try_clone().unwrap().is_empty());
}

#[test]
fn clone_variants() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.clone_variants").unwrap();
    let mut list = List::new(TypeDef::Any, 2).unwrap();
    list.push(Value::I32(1)).unwrap();
    list.push(ty.create(2).unwrap()).unwrap();
    let clone = list.try_clone().unwrap();
    assert!(matches!(
        clone.get(0).unwrap(),
        Some(Element::Value(Value::I32(1)))
    ));
    let Some(Element::Ref(reference)) = clone.get(1).unwrap() else {
        panic!("expected ref")
    };
    assert_eq!(ty.downcast::<i32>(&reference), Some(&2));
    Ok(())
}

#[test]
fn reserve() {
    let mut list = List::new(TypeDef::Any, 0).unwrap();
    list.reserve(64).unwrap();
    assert!(list.capacity() >= 64);
}

#[test]
fn reserve_overflow() {
    let mut list = List::new(TypeDef::Any, 0).unwrap();
    assert!(list.reserve(usize::MAX).is_err());
}

#[test]
fn resize_overflow() {
    let mut list = List::new(TypeDef::Any, 0).unwrap();
    assert!(list.resize(usize::MAX).is_err());
}

#[test]
fn resize_i32() {
    let mut list = List::new(TypeDef::Value(ValueType::I32), 0).unwrap();
    list.resize(3).unwrap();
    assert_eq!(list.len(), 3);
    assert_eq!(list.get_value(1).unwrap(), Value::I32(0));
    list.resize(1).unwrap();
    assert_eq!(list.len(), 1);
}

#[test]
fn resize_ref() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.resize_ref").unwrap();
    let mut list = List::new(TypeDef::Ref(ty), 0).unwrap();
    list.resize(3).unwrap();
    assert!(list.get_ref(2).unwrap().is_null());
    list.clear();
    assert!(list.is_empty());
    Ok(())
}

#[test]
fn resize_variant() {
    let mut list = List::new(TypeDef::Any, 0).unwrap();
    list.resize(3).unwrap();
    assert!(list.get(1).unwrap().is_none());
}

// Upstream SwapStorageSelf requires aliasing the same mutable list; safe Rust rejects it and manufacturing aliases would be undefined behavior.
#[test]
fn swap_storage() {
    let mut a = List::new(TypeDef::Value(ValueType::I32), 1).unwrap();
    let mut b = List::new(TypeDef::Any, 1).unwrap();
    a.push(Value::I32(1)).unwrap();
    b.push(Value::I32(2)).unwrap();
    a.swap_storage(&mut b);
    assert_eq!(a.element_type().unwrap(), TypeDef::Any);
    assert_eq!(a.get_value(0).unwrap(), Value::I32(2));
    assert_eq!(b.element_type().unwrap(), TypeDef::Value(ValueType::I32));
    assert_eq!(b.get_value(0).unwrap(), Value::I32(1));
}

#[test]
fn copy_out_of_range() {
    let source = List::new(TypeDef::Value(ValueType::I32), 1).unwrap();
    let mut target = List::new(TypeDef::Value(ValueType::I32), 1).unwrap();
    target.resize(1).unwrap();
    assert!(source.copy_to(1, &mut target, 0, 1).is_err());
}

#[test]
fn copy_values() {
    let mut source = List::new(TypeDef::Value(ValueType::I32), 2).unwrap();
    source.push(Value::I32(1)).unwrap();
    source.push(Value::I32(2)).unwrap();
    let mut target = List::new(TypeDef::Value(ValueType::I32), 2).unwrap();
    target.resize(2).unwrap();
    source.copy_to(0, &mut target, 0, 2).unwrap();
    assert_eq!(target.get_value(1).unwrap(), Value::I32(2));
}

#[test]
fn copy_wrong_values() {
    let mut source = List::new(TypeDef::Value(ValueType::I32), 1).unwrap();
    source.push(Value::I32(1)).unwrap();
    let mut target = List::new(TypeDef::Value(ValueType::I64), 1).unwrap();
    target.resize(1).unwrap();
    assert!(source.copy_to(0, &mut target, 0, 1).is_err());
}

#[test]
fn copy_refs() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.copy_refs").unwrap();
    let mut source = List::new(TypeDef::Ref(ty), 1).unwrap();
    source.push(ty.create(3).unwrap()).unwrap();
    let mut target = List::new(TypeDef::Ref(ty), 1).unwrap();
    target.resize(1).unwrap();
    source.copy_to(0, &mut target, 0, 1).unwrap();
    assert_eq!(ty.downcast::<i32>(&target.get_ref(0).unwrap()), Some(&3));
    Ok(())
}

#[test]
fn copy_wrong_refs() -> ireert::Result<()> {
    let instance = instance()?;
    let a = RefType::register::<i32>(&instance, "test.list.copy_wrong_refs_a").unwrap();
    let b = RefType::register::<i32>(&instance, "test.list.copy_wrong_refs_b").unwrap();
    let mut source = List::new(TypeDef::Ref(a), 1).unwrap();
    source.push(a.create(3).unwrap()).unwrap();
    let mut target = List::new(TypeDef::Ref(b), 1).unwrap();
    target.resize(1).unwrap();
    assert!(source.copy_to(0, &mut target, 0, 1).is_err());
    Ok(())
}

#[test]
fn copy_variants() {
    let mut source = List::new(TypeDef::Any, 1).unwrap();
    source.push(Value::I32(5)).unwrap();
    let mut target = List::new(TypeDef::Any, 1).unwrap();
    target.resize(1).unwrap();
    source.copy_to(0, &mut target, 0, 1).unwrap();
    assert!(matches!(
        target.get(0).unwrap(),
        Some(Element::Value(Value::I32(5)))
    ));
}

#[test]
fn copy_from_variants() {
    let mut source = List::new(TypeDef::Any, 1).unwrap();
    source.push(Value::I32(6)).unwrap();
    let mut target = List::new(TypeDef::Value(ValueType::I32), 1).unwrap();
    target.resize(1).unwrap();
    source.copy_to(0, &mut target, 0, 1).unwrap();
    assert_eq!(target.get_value(0).unwrap(), Value::I32(6));
}

#[test]
fn copy_to_variants() {
    let mut source = List::new(TypeDef::Value(ValueType::I32), 1).unwrap();
    source.push(Value::I32(7)).unwrap();
    let mut target = List::new(TypeDef::Any, 1).unwrap();
    target.resize(1).unwrap();
    source.copy_to(0, &mut target, 0, 1).unwrap();
    assert!(matches!(
        target.get(0).unwrap(),
        Some(Element::Value(Value::I32(7)))
    ));
}

#[test]
fn push_pop_ref() -> ireert::Result<()> {
    let instance = instance()?;
    let ty = RefType::register::<i32>(&instance, "test.list.push_pop_ref").unwrap();
    let mut list = List::new(TypeDef::Ref(ty), 1).unwrap();
    assert!(list.pop_front_ref().unwrap().is_none());
    list.push(ty.create(8).unwrap()).unwrap();
    let popped = list.pop_front_ref().unwrap().unwrap();
    assert_eq!(ty.downcast::<i32>(&popped), Some(&8));
    assert!(list.is_empty());
    Ok(())
}

#[test]
fn f64_to_f32_conversion() {
    let mut list = List::new(TypeDef::Value(ValueType::F64), 1).unwrap();
    list.push(Value::F64(1.25)).unwrap();
    assert_eq!(
        list.get_value_as(0, ValueType::F32).unwrap(),
        Value::F32(1.25)
    );
}

#[test]
fn f32_to_f64_conversion() {
    let mut list = List::new(TypeDef::Value(ValueType::F32), 1).unwrap();
    list.push(Value::F32(2.5)).unwrap();
    assert_eq!(
        list.get_value_as(0, ValueType::F64).unwrap(),
        Value::F64(2.5)
    );
}
