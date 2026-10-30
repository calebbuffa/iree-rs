//! HAL: drivers, devices, allocators, buffers, semaphores, command buffers, executables.

pub(crate) fn pod_cast_error(_: bytemuck::PodCastError) -> crate::Error {
    crate::Error::new(
        crate::Code::InvalidArgument,
        "buffer bytes do not match the element alignment and size",
    )
}

pub(crate) fn pod_element_size<T: bytemuck::Pod>() -> crate::Result<usize> {
    let size = core::mem::size_of::<T>();
    if size == 0 {
        return Err(crate::Error::new(
            crate::Code::InvalidArgument,
            "zero-sized buffer elements are unsupported",
        ));
    }
    Ok(size)
}

/// A byte size or offset in device memory (`iree_device_size_t`).
pub type DeviceSize = iree_native::iree_device_size_t;
/// One buffer view dimension (`iree_hal_dim_t`).
pub type Dim = iree_native::iree_hal_dim_t;

pub mod allocator;
pub mod backends;
pub(crate) mod bitfield;
pub mod buffer;
pub mod buffer_view;
pub mod channel;
pub mod command_buffer;
pub mod debug_label;
pub mod device;
pub mod driver;
pub mod driver_registry;
pub mod event;
pub mod executable;
pub mod external;
pub mod fence;
pub mod file;
pub mod flags;
pub mod profiling;
pub mod queue;
pub mod semaphore;
pub mod timepoint;
pub mod topology;
pub mod virtual_memory;

pub use allocator::{Allocator, AllocatorStatistics, BufferCompatibility, MemoryHeap};
pub use buffer::MappedRange;
pub use buffer::{Buffer, Overlap, Placement, PlacementFlags, PreservedAllocation};
pub use buffer_view::BufferView;
pub use channel::{
    Channel, ChannelParams, CollectiveElementType, CollectiveKind, CollectiveOp,
    CollectiveReduction,
};
pub use command_buffer::{
    AccessScope, BufferBarrier, BufferRef, CollectiveCommand, CommandBuffer, CommandBufferMode,
    CommandCategory, DispatchConfig, DispatchFlags, ExecutionStage, MemoryAdvice, MemoryBarrier,
    TransferCommand,
};
pub use debug_label::{LabelColor, LabelLocation};
pub use device::{Device, DeviceGroup, ExternalCaptureOptions, GroupedDevice, RetainedDevice};
pub use driver::{DeviceCreateParams, DeviceInfo, Driver};
pub use driver_registry::{DriverFactory, DriverInfo, DriverRegistry, FactoryRegistration};
pub use event::Event;
pub use executable::{
    Executable, ExecutableCache, ExecutableFunction, ExecutableFunctionInfo,
    ExecutableFunctionParameter, ExecutableParams,
};
pub use external::{ExportedExternalBuffer, ExternalBuffer, ExternalBufferType};
pub use fence::Fence;
pub use flags::{
    BufferParams, BufferUsage, CounterSetSelectionFlags, ElementType, EncodingType,
    ExecutableCachingMode, ExecutableFunctionFlags, MemoryAccess, MemoryType, MetricId,
    ProfileChunkFlags, ProfileDataFamilies, ProfileMetricPlotHint, ProfileMetricSemantic,
    ProfileMetricUnit, ProfileMetricValueKind, ProfileRecordType, ProfileStatisticsRowFlags,
    ProfileStatisticsRowType, ProfileStatisticsTimeDomain, ProfilingFlags,
};
pub use queue::{
    AllocaFlags, BufferBinding, DeallocaFlags, DeviceId, ExecuteFlags, FileRange, FileTransfer,
    FileTransferOptions, HostCallContext, HostCallFlags, QueueAffinity, QueueDispatch,
    QueueSubmission, SemaphorePoint, WaitFlags, WaitMode,
};
pub use semaphore::{Semaphore, SemaphoreCompatibility, SemaphoreFlags};

pub mod pool;
pub use pool::{
    FixedBlockPoolOptions, Pool, PoolCapabilities, PoolReservation, PoolStats, QueuePoolBackend,
    ReservationAcquire, TlsfPoolOptions,
};
pub use virtual_memory::{
    Granularity, MemoryProtection, PhysicalMemory, VirtualMapping, VirtualReservation,
};
