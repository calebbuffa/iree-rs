//! Runtime-only shape, type, and ownership qualification.
#![cfg(any(feature = "cpu", feature = "local-sync", feature = "local-task"))]

use iree::Instance;

const DRIVERS: &[&str] = &[
    #[cfg(any(feature = "cpu", feature = "local-sync"))]
    "local-sync",
    #[cfg(any(feature = "cpu", feature = "local-task"))]
    "local-task",
];

#[test]
fn primitive_uploads_and_tensors_outlive_their_creators() -> iree::Result<()> {
    let device = Instance::new()?.device(DRIVERS[0])?;
    macro_rules! roundtrip {
        ($values:expr, $type:ty) => {{
            let values: &[$type] = &$values;
            let tensor = device.tensor(&[values.len()], values)?;
            assert_eq!(tensor.read::<$type>()?, values);
        }};
    }
    roundtrip!([i8::MIN, 0, i8::MAX], i8);
    roundtrip!([i16::MIN, 0, i16::MAX], i16);
    roundtrip!([i32::MIN, 0, i32::MAX], i32);
    roundtrip!([i64::MIN, 0, i64::MAX], i64);
    roundtrip!([0, u8::MAX], u8);
    roundtrip!([0, u16::MAX], u16);
    roundtrip!([0, u32::MAX], u32);
    roundtrip!([0, u64::MAX], u64);
    roundtrip!([-1.5, 0.0, 2.25], f32);
    roundtrip!([-1.5, 0.0, 2.25], f64);
    let tensor = device.tensor(&[2], &[1.5f32, -3.0])?;
    let clone = tensor.clone();
    drop(device);
    drop(tensor);
    assert_eq!(clone.read::<f32>()?, [1.5, -3.0]);
    Ok(())
}

#[test]
fn validates_shapes_and_supports_scalar_and_empty_tensors() -> iree::Result<()> {
    let instance = Instance::new()?;
    assert!(instance.device("not-a-registered-driver").is_err());
    for &driver in DRIVERS {
        let device = instance.device(driver)?;
        assert!(device.tensor(&[2], &[1.0f32]).is_err());
        assert!(device.tensor::<f32>(&[usize::MAX, 2], &[]).is_err());
        let scalar = device.tensor(&[], &[42i32])?;
        assert_eq!(scalar.shape(), Vec::<usize>::new());
        assert_eq!(scalar.read::<i32>()?, [42]);
        assert!(scalar.read::<f32>().is_err());
        let empty = device.tensor::<f32>(&[0, 4], &[])?;
        assert_eq!(empty.shape(), [0, 4]);
        assert_eq!(empty.read::<f32>()?, Vec::<f32>::new());
    }
    Ok(())
}
