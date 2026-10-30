//! Buffer fixtures shared by buffer and file CTS tests.

use ireert::hal::{
    Buffer, BufferParams, BufferUsage, Device, DeviceSize, MemoryAccess, MemoryType,
};

pub fn buffer(device: &Device, size: DeviceSize) -> ireert::Result<Buffer> {
    device.allocator().allocate_buffer(
        BufferParams {
            memory_type: MemoryType::DEVICE_LOCAL | MemoryType::HOST_VISIBLE,
            usage: BufferUsage::TRANSFER | BufferUsage::MAPPING | BufferUsage::DISPATCH_STORAGE,
            access: MemoryAccess::ALL,
            ..BufferParams::default()
        },
        size,
    )
}

pub fn read(buffer: &Buffer, offset: DeviceSize, length: usize) -> ireert::Result<Vec<u8>> {
    let mut data = vec![0; length];
    // SAFETY: tests initialize the source range before reading it.
    unsafe { buffer.read_unchecked(offset, &mut data)? };
    Ok(data)
}
