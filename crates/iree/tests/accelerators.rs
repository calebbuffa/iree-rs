//! Explicit device qualification; missing hardware or vendor runtimes are errors.
#[cfg(any(feature = "cuda", feature = "hip", feature = "metal"))]
fn tensor_round_trip(driver: &str) -> iree::Result<()> {
    let device = iree::Instance::new()?.device(driver)?;
    let tensor = device.tensor(&[4], &[1.0f32, 2.0, 3.0, 4.0])?;
    assert_eq!(tensor.read::<f32>()?, [1.0, 2.0, 3.0, 4.0]);
    device.trim()?;
    Ok(())
}

#[cfg(feature = "cuda")]
#[test]
#[ignore = "requires an NVIDIA GPU and CUDA driver"]
fn cuda_device_tensor_round_trip() -> iree::Result<()> {
    tensor_round_trip("cuda")
}

#[cfg(feature = "hip")]
#[test]
#[ignore = "requires a HIP GPU and installed HIP runtime"]
fn hip_device_tensor_round_trip() -> iree::Result<()> {
    tensor_round_trip("hip")
}

#[cfg(feature = "metal")]
#[test]
#[ignore = "requires an Apple Metal device"]
fn metal_device_tensor_round_trip() -> iree::Result<()> {
    tensor_round_trip("metal")
}
