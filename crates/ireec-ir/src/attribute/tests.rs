//! Laws for attribute construction, parsing, and typed round trips.
use super::*;
use crate::{
    Context,
    types::{IntegerType, RankedTensorType, Signedness},
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn integer_value_and_assembly_round_trip(value in any::<i64>()) {
        let context = Context::new().unwrap();
        let ty = IntegerType::new(&context, 64, Signedness::Signless).unwrap();
        let attr = IntegerAttr::new(ty.into(), value).unwrap();
        prop_assert_eq!(attr.value().unwrap(), value);
        prop_assert_eq!(Attribute::parse(&context, &attr.print()).unwrap(), Attribute::from(attr));
    }

    #[test]
    fn arbitrary_width_words_round_trip(low in any::<u64>(), high in any::<u64>()) {
        let context = Context::new().unwrap();
        let ty = IntegerType::new(&context, 128, Signedness::Signless).unwrap();
        let attr = IntegerAttr::from_words(ty, &[low, high]).unwrap();
        prop_assert_eq!(attr.words(), vec![low, high]);
        prop_assert!(attr.value().is_err());
    }

    #[test]
    fn dense_values_round_trip(values in prop::collection::vec(any::<i64>(), 0..24)) {
        let context = Context::new().unwrap();
        let element = IntegerType::new(&context, 64, Signedness::Signless).unwrap();
        let ty = RankedTensorType::new(&[values.len() as i64], element.into(), None).unwrap();
        let attr = DenseElementsAttr::from_i64(ty.into(), &values).unwrap();
        for (index, value) in values.iter().enumerate() {
            prop_assert_eq!(attr.get_i64(index).unwrap(), Some(*value));
        }
        prop_assert_eq!(attr.get_i64(values.len()).unwrap(), None);
        prop_assert_eq!(Attribute::parse(&context, &attr.print()).unwrap(), Attribute::from(attr));
    }

    #[test]
    fn strings_round_trip(text in ".*") {
        let context = Context::new().unwrap();
        let attr = StringAttr::new(&context, &text).unwrap();
        prop_assert_eq!(attr.value().unwrap(), text);
        prop_assert_eq!(Attribute::parse(&context, &attr.print()).unwrap(), Attribute::from(attr));
    }

    #[test]
    fn dense_arrays_preserve_values(values in prop::collection::vec(any::<i32>(), 0..24)) {
        let context = Context::new().unwrap();
        let array = DenseArrayAttr::from_i32(&context, &values).unwrap();
        prop_assert_eq!(array.len(), values.len());
        for (index, value) in values.iter().enumerate() {
            prop_assert_eq!(array.get_i32(index).unwrap(), Some(*value));
        }
        prop_assert!(array.get_f32(0).is_err());
    }

    #[test]
    fn strided_layout_preserves_signed_values(offset in any::<i64>(), strides in prop::collection::vec(any::<i64>(), 0..8)) {
        let context = Context::new().unwrap();
        let layout = StridedLayoutAttr::new(&context, offset, &strides).unwrap();
        prop_assert_eq!(layout.offset(), offset);
        prop_assert_eq!(layout.strides(), strides);
    }
}

#[test]
fn invalid_downcasts_and_cross_context_arrays_are_rejected() {
    let context = Context::new().unwrap();
    let other = Context::new().unwrap();
    let string = StringAttr::new(&context, "value").unwrap();
    assert!(IntegerAttr::try_from(Attribute::from(string)).is_err());
    assert!(ArrayAttr::new(&other, &[string.into()]).is_err());
    let entry = NamedAttribute::new("same", string.into()).unwrap();
    assert!(DictionaryAttr::new(&context, &[entry, entry]).is_err());
}

#[test]
fn one_dense_value_constructs_a_splat() {
    let context = Context::new().unwrap();
    let element = IntegerType::new(&context, 32, Signedness::Signless).unwrap();
    let ty = RankedTensorType::new(&[4], element.into(), None).unwrap();
    let attr = DenseElementsAttr::from_i32(ty.into(), &[7]).unwrap();

    assert!(attr.is_splat());
    for index in 0..4 {
        assert_eq!(attr.get_i32(index).unwrap(), Some(7));
    }
}

proptest! {
    #[test]
    fn raw_boolean_storage_round_trips(values in prop::collection::vec(any::<bool>(), 1..32)) {
        let context = Context::new().unwrap();
        let element = IntegerType::new(&context, 1, Signedness::Signless).unwrap();
        let ty = RankedTensorType::new(&[values.len() as i64], element.into(), None).unwrap();
        let attr = DenseElementsAttr::from_bool(ty.into(), &values).unwrap();
        let raw = attr.raw_data().unwrap();
// SAFETY: `raw` was returned by MLIR and remains live for `the native query`.
        let rebuilt = unsafe { DenseElementsAttr::from_raw_buffer(ty.into(), &raw) }.unwrap();
        prop_assert_eq!(rebuilt, attr);
    }
}
