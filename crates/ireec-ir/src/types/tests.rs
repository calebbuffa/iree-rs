//! Laws for MLIR type identity, construction, and parsing.
use super::*;
use crate::Context;
use proptest::prelude::*;

proptest! {
    #[test]
    fn integers_preserve_width_and_signedness(width in 1u32..256, sign in 0u8..3) {
        let context = Context::new().unwrap();
        let signedness = match sign {
            0 => Signedness::Signless,
            1 => Signedness::Signed,
            _ => Signedness::Unsigned,
        };
        let ty = IntegerType::new(&context, width, signedness).unwrap();
        prop_assert_eq!(ty.width(), width);
        prop_assert_eq!(ty.signedness(), signedness);
        prop_assert_eq!(Type::parse(&context, &ty.print()).unwrap(), Type::from(ty));
    }

    #[test]
    fn shaped_dimensions_round_trip(shape in prop::collection::vec(-1i64..32, 0..5)) {
        let context = Context::new().unwrap();
        let dynamic = ShapedType::dynamic_size().unwrap();
        let shape: Vec<_> = shape.into_iter()
            .map(|d| if d == -1 { dynamic } else { d })
            .collect();
        let element = FloatType::new(&context, FloatFormat::F32).unwrap();
        let ty = RankedTensorType::new(&shape, element.into(), None).unwrap();
        let shaped = ShapedType::try_from(Type::from(ty)).unwrap();
        prop_assert_eq!(shaped.rank(), Some(shape.len()));
        prop_assert_eq!(shaped.dimensions(), Some(shape));
        prop_assert_eq!(shaped.element_type(), Type::from(element));
        prop_assert_eq!(Type::parse(&context, &ty.print()).unwrap(), Type::from(ty));
    }

    #[test]
    fn vector_scalable_flags_round_trip(shape in prop::collection::vec(1i64..16, 1..5), flag in any::<bool>()) {
        let context = Context::new().unwrap();
        let element = IntegerType::new(&context, 32, Signedness::Signless).unwrap();
        let flags = vec![flag; shape.len()];
        let ty = VectorType::new(&shape, &flags, element.into()).unwrap();
        prop_assert_eq!(ty.scalable_dimensions(), flags);
        prop_assert_eq!(ty.is_scalable(), flag);
    }
}

#[test]
fn function_tuple_and_unranked_queries_preserve_structure() {
    let context = Context::new().unwrap();
    let integer: Type<'_> = IntegerType::new(&context, 32, Signedness::Signless)
        .unwrap()
        .into();
    let index: Type<'_> = IndexType::new(&context).unwrap().into();
    let tuple = TupleType::new(&context, &[integer, index]).unwrap();
    assert_eq!(tuple.elements(), vec![integer, index]);
    let function = FunctionType::new(&context, &[integer], &[index, integer]).unwrap();
    assert_eq!(function.inputs(), vec![integer]);
    assert_eq!(function.results(), vec![index, integer]);
    let unranked = UnrankedTensorType::new(integer).unwrap();
    assert_eq!(
        ShapedType::try_from(Type::from(unranked)).unwrap().rank(),
        None
    );
}

#[test]
fn invalid_shape_and_context_are_rejected() {
    let context = Context::new().unwrap();
    let other = Context::new().unwrap();
    let element: Type<'_> = FloatType::new(&context, FloatFormat::F32).unwrap().into();
    assert!(RankedTensorType::new(&[-2], element, None).is_err());
    assert!(VectorType::new(&[4], &[], element).is_err());
    assert!(TupleType::new(&other, &[element]).is_err());
}

#[test]
fn builtin_floating_formats_round_trip() {
    let context = Context::new().unwrap();
    for format in [
        FloatFormat::F16,
        FloatFormat::BF16,
        FloatFormat::F32,
        FloatFormat::F64,
        FloatFormat::TF32,
        FloatFormat::F8E5M2,
        FloatFormat::F8E4M3FN,
        FloatFormat::F8E4M3FNUZ,
    ] {
        let ty = FloatType::new(&context, format).unwrap();
        assert_eq!(ty.format(), Some(format));
        assert_eq!(Type::parse(&context, &ty.print()).unwrap(), Type::from(ty));
    }
}
