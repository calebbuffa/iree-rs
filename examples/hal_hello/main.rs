//! Rust port of IREE's pure-HAL buffer fill/copy sample.

use iree_samples::vm_support::SampleDevice;
use ireert::hal::{
    AccessScope, BufferParams, BufferUsage, CommandBuffer, CommandBufferMode, CommandCategory,
    ExecutionStage, MemoryAccess, MemoryBarrier, MemoryType, QueueAffinity, QueueSubmission,
    Semaphore, SemaphorePoint, WaitFlags,
};
use std::error::Error;

const ELEMENT_COUNT: usize = 16;
const BUFFER_SIZE: usize = ELEMENT_COUNT * std::mem::size_of::<u32>();
const FILL_PATTERN: u32 = 0xcafe_f00d;

fn main() -> Result<(), Box<dyn Error>> {
    let sample_device = SampleDevice::from_environment()?;
    let device = &sample_device.device;
    let allocator = device.allocator();

    let source = allocator.allocate_buffer(
        BufferParams {
            usage: BufferUsage::TRANSFER,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::DEVICE_LOCAL,
            ..Default::default()
        },
        BUFFER_SIZE,
    )?;
    let destination = allocator.allocate_buffer(
        BufferParams {
            usage: BufferUsage::TRANSFER_TARGET | BufferUsage::MAPPING,
            access: MemoryAccess::ALL,
            memory_type: MemoryType::HOST_LOCAL | MemoryType::DEVICE_VISIBLE,
            ..Default::default()
        },
        BUFFER_SIZE,
    )?;

    let mut command_buffer = CommandBuffer::create(
        device,
        CommandBufferMode::ONE_SHOT,
        CommandCategory::TRANSFER,
        QueueAffinity::ANY,
        0,
    )?;
    command_buffer.begin()?;
    command_buffer.fill_buffer(&source, 0, BUFFER_SIZE, &FILL_PATTERN.to_ne_bytes())?;
    command_buffer.execution_barrier(
        ExecutionStage::TRANSFER,
        ExecutionStage::TRANSFER,
        &[MemoryBarrier {
            source: AccessScope::TRANSFER_WRITE,
            target: AccessScope::TRANSFER_READ,
        }],
        &[],
    )?;
    command_buffer.copy_buffer(&source, 0, &destination, 0, BUFFER_SIZE)?;
    command_buffer.end()?;

    let completion = Semaphore::new(device, QueueAffinity::ANY, 0)?;
    device.queue_execute(
        &QueueSubmission::new(QueueAffinity::ANY).with_signals([SemaphorePoint {
            semaphore: &completion,
            value: 1,
        }]),
        &command_buffer,
        &[],
        Default::default(),
    )?;
    completion.wait(1, None, WaitFlags::default())?;

    let mut results = [0u32; ELEMENT_COUNT];
    // SAFETY: queue completion proves the copy initialized the entire host-visible destination.
    unsafe {
        destination.read_typed_unchecked(0, &mut results)?;
    }
    assert_eq!(results, [FILL_PATTERN; ELEMENT_COUNT]);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn fills_copies_and_reads_back() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
