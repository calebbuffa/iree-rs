//! Small native-link and execution check for the default CPU configuration.

#![cfg(feature = "cpu")]

#[path = "support/cts.rs"]
mod cts;

use ireert::{
    hal::{BufferParams, BufferUsage, MemoryAccess, MemoryType},
    vm::{Buffer, BufferAccess, Instance},
};
use std::time::Duration;

#[test]
fn cpu_runtime_links_and_round_trips_buffers() -> ireert::Result<()> {
    let expected = *b"IREE Rust smoke!";
    let instance = Instance::new()?;
    let mut vm_buffer = Buffer::new(&instance, BufferAccess::MUTABLE, expected.len(), 1)?;
    vm_buffer.write(0, &expected)?;
    let mut actual = [0; 16];
    vm_buffer.read(0, &mut actual)?;
    assert_eq!(actual, expected);

    let mut tested = Vec::new();
    for (name, _, device) in cts::devices()? {
        let buffer = device.allocator().allocate_buffer(
            BufferParams {
                memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
                usage: BufferUsage::TRANSFER,
                access: MemoryAccess::ALL,
                ..BufferParams::default()
            },
            expected.len(),
        )?;
        let timeout = Some(Duration::from_secs(10));
        device.transfer_h2d(&expected, &buffer, 0, timeout)?;
        actual.fill(0);
        // SAFETY: the completed host-to-device transfer initialized the entire buffer.
        unsafe { device.transfer_d2h(&buffer, 0, &mut actual, timeout)? };
        assert_eq!(actual, expected, "{name} buffer round trip");
        eprintln!("{name}: native device creation and buffer round trip passed");
        tested.push(name);
    }
    for required in ["local-sync", "local-task"] {
        assert!(
            tested.iter().any(|name| name == required),
            "required CPU backend {required} was not exercised"
        );
    }
    Ok(())
}
