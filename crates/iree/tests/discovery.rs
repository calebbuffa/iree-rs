//! Native driver enumeration, selection, and retained ownership.
use iree::Instance;

#[cfg(any(feature = "local-sync", feature = "cpu"))]
#[test]
fn discover_select_and_retain_a_driver() {
    let instance = Instance::new().unwrap();
    assert!(
        instance
            .drivers()
            .unwrap()
            .iter()
            .any(|info| info.name == "local-sync")
    );
    let driver = instance.driver("local-sync").unwrap();
    let devices = driver.devices().unwrap();
    assert!(!devices.is_empty());
    let device = devices[0].builder().build().unwrap();
    let ordinal_device = driver.device_builder().ordinal(0).build().unwrap();
    drop(devices);
    drop(driver);
    drop(instance);
    let tensor = device.tensor(&[2], &[1_i32, 2]).unwrap();
    assert!(device.id().starts_with("local-sync"));
    assert_eq!(device.query_i64("hal.device.id", "local-*").unwrap(), 1);
    assert_eq!(
        device.query_i64("hal.device.id", "nonexistent-*").unwrap(),
        0
    );
    assert!(device.query_i64("iree.rs.unknown", "unknown").is_err());
    device.trim().unwrap();
    assert_eq!(tensor.read::<i32>().unwrap(), [1, 2]);
    assert_eq!(
        ordinal_device
            .tensor(&[], &[3_i32])
            .unwrap()
            .read::<i32>()
            .unwrap(),
        [3]
    );
}

#[test]
fn absent_driver_is_an_error() {
    let instance = Instance::new().unwrap();
    assert!(instance.driver("iree-rs-nonexistent-driver").is_err());
}
