//! HAL bitfields and metadata value types.
//!
//! Every constant is taken from the bound C headers, never retyped, so the
//! values cannot drift from the runtime they are passed to.
use super::bitfield::{bitfield, scalar_value};
use super::queue::QueueAffinity;
use iree_native as n;

bitfield! {
    /// Memory placement and capabilities (`iree_hal_memory_type_t`).
    MemoryType: n::iree_hal_memory_type_t {
        /// Let the allocator choose the best placement.
        OPTIMAL = n::IREE_HAL_MEMORY_TYPE_OPTIMAL,
        /// Mappable by the host.
        HOST_VISIBLE = n::IREE_HAL_MEMORY_TYPE_HOST_VISIBLE,
        /// Host writes are visible to the device without flushing.
        HOST_COHERENT = n::IREE_HAL_MEMORY_TYPE_HOST_COHERENT,
        /// Cached on the host.
        HOST_CACHED = n::IREE_HAL_MEMORY_TYPE_HOST_CACHED,
        /// Resident in host memory.
        HOST_LOCAL = n::IREE_HAL_MEMORY_TYPE_HOST_LOCAL,
        /// The best host-resident placement.
        OPTIMAL_FOR_HOST = n::IREE_HAL_MEMORY_TYPE_OPTIMAL_FOR_HOST,
        /// Accessible by the device.
        DEVICE_VISIBLE = n::IREE_HAL_MEMORY_TYPE_DEVICE_VISIBLE,
        /// Resident in device memory.
        DEVICE_LOCAL = n::IREE_HAL_MEMORY_TYPE_DEVICE_LOCAL,
        /// The best device-resident placement.
        OPTIMAL_FOR_DEVICE = n::IREE_HAL_MEMORY_TYPE_OPTIMAL_FOR_DEVICE,
    }
}

bitfield! {
    /// Permitted memory access (`iree_hal_memory_access_t`).
    MemoryAccess: n::iree_hal_memory_access_t {
        /// Read access.
        READ = n::IREE_HAL_MEMORY_ACCESS_READ,
        /// Write access.
        WRITE = n::IREE_HAL_MEMORY_ACCESS_WRITE,
        /// Previous contents may be discarded.
        DISCARD = n::IREE_HAL_MEMORY_ACCESS_DISCARD,
        /// Write access that discards previous contents.
        DISCARD_WRITE = n::IREE_HAL_MEMORY_ACCESS_DISCARD_WRITE,
        /// The range may alias other ranges.
        MAY_ALIAS = n::IREE_HAL_MEMORY_ACCESS_MAY_ALIAS,
        /// Access may be unaligned.
        UNALIGNED = n::IREE_HAL_MEMORY_ACCESS_UNALIGNED,
        /// Any access is allowed.
        ANY = n::IREE_HAL_MEMORY_ACCESS_ANY,
        /// Read, write and discard.
        ALL = n::IREE_HAL_MEMORY_ACCESS_ALL,
    }
}

bitfield! {
    /// Intended buffer usage (`iree_hal_buffer_usage_t`).
    BufferUsage: n::iree_hal_buffer_usage_t {
        /// Source of transfer operations.
        TRANSFER_SOURCE = n::IREE_HAL_BUFFER_USAGE_TRANSFER_SOURCE,
        /// Target of transfer operations.
        TRANSFER_TARGET = n::IREE_HAL_BUFFER_USAGE_TRANSFER_TARGET,
        /// Source or target of transfers.
        TRANSFER = n::IREE_HAL_BUFFER_USAGE_TRANSFER,
        /// Indirect dispatch parameters.
        DISPATCH_INDIRECT_PARAMETERS = n::IREE_HAL_BUFFER_USAGE_DISPATCH_INDIRECT_PARAMETERS,
        /// Uniform reads from dispatches.
        DISPATCH_UNIFORM_READ = n::IREE_HAL_BUFFER_USAGE_DISPATCH_UNIFORM_READ,
        /// Storage reads from dispatches.
        DISPATCH_STORAGE_READ = n::IREE_HAL_BUFFER_USAGE_DISPATCH_STORAGE_READ,
        /// Storage writes from dispatches.
        DISPATCH_STORAGE_WRITE = n::IREE_HAL_BUFFER_USAGE_DISPATCH_STORAGE_WRITE,
        /// Storage reads and writes from dispatches.
        DISPATCH_STORAGE = n::IREE_HAL_BUFFER_USAGE_DISPATCH_STORAGE,
        /// Image reads from dispatches.
        DISPATCH_IMAGE_READ = n::IREE_HAL_BUFFER_USAGE_DISPATCH_IMAGE_READ,
        /// Image writes from dispatches.
        DISPATCH_IMAGE_WRITE = n::IREE_HAL_BUFFER_USAGE_DISPATCH_IMAGE_WRITE,
        /// Image reads and writes from dispatches.
        DISPATCH_IMAGE = n::IREE_HAL_BUFFER_USAGE_DISPATCH_IMAGE,
        /// Any dispatch usage.
        DISPATCH = n::IREE_HAL_BUFFER_USAGE_DISPATCH,
        /// Exportable to other APIs.
        SHARING_EXPORT = n::IREE_HAL_BUFFER_USAGE_SHARING_EXPORT,
        /// Replicable across devices.
        SHARING_REPLICATE = n::IREE_HAL_BUFFER_USAGE_SHARING_REPLICATE,
        /// Concurrently usable by multiple queues.
        SHARING_CONCURRENT = n::IREE_HAL_BUFFER_USAGE_SHARING_CONCURRENT,
        /// Contents never change after initialization.
        SHARING_IMMUTABLE = n::IREE_HAL_BUFFER_USAGE_SHARING_IMMUTABLE,
        /// Mappable for scoped host access.
        MAPPING_SCOPED = n::IREE_HAL_BUFFER_USAGE_MAPPING_SCOPED,
        /// Persistently mappable.
        MAPPING_PERSISTENT = n::IREE_HAL_BUFFER_USAGE_MAPPING_PERSISTENT,
        /// Mapping is requested but not required.
        MAPPING_OPTIONAL = n::IREE_HAL_BUFFER_USAGE_MAPPING_OPTIONAL,
        /// Mapped access is random.
        MAPPING_ACCESS_RANDOM = n::IREE_HAL_BUFFER_USAGE_MAPPING_ACCESS_RANDOM,
        /// Mapped access is sequential writes.
        MAPPING_ACCESS_SEQUENTIAL_WRITE = n::IREE_HAL_BUFFER_USAGE_MAPPING_ACCESS_SEQUENTIAL_WRITE,
        /// Scoped, randomly accessed mapping.
        MAPPING = n::IREE_HAL_BUFFER_USAGE_MAPPING,
        /// Transfers plus dispatch storage.
        DEFAULT = n::IREE_HAL_BUFFER_USAGE_DEFAULT,
    }
}

scalar_value! {
    /// A HAL element type (`iree_hal_element_type_t`); unknown values are preserved.
    ElementType: n::iree_hal_element_type_t {
        /// No element type.
        NONE = n::IREE_HAL_ELEMENT_TYPE_NONE,
        /// Opaque 8-bit elements.
        OPAQUE_8 = n::IREE_HAL_ELEMENT_TYPE_OPAQUE_8,
        /// Opaque 16-bit elements.
        OPAQUE_16 = n::IREE_HAL_ELEMENT_TYPE_OPAQUE_16,
        /// Opaque 32-bit elements.
        OPAQUE_32 = n::IREE_HAL_ELEMENT_TYPE_OPAQUE_32,
        /// Opaque 64-bit elements.
        OPAQUE_64 = n::IREE_HAL_ELEMENT_TYPE_OPAQUE_64,
        /// Boolean 8-bit elements.
        BOOL_8 = n::IREE_HAL_ELEMENT_TYPE_BOOL_8,
        /// Signless 4-bit integers.
        INT_4 = n::IREE_HAL_ELEMENT_TYPE_INT_4,
        /// Signed 4-bit integers.
        SINT_4 = n::IREE_HAL_ELEMENT_TYPE_SINT_4,
        /// Unsigned 4-bit integers.
        UINT_4 = n::IREE_HAL_ELEMENT_TYPE_UINT_4,
        /// Signless 8-bit integers.
        INT_8 = n::IREE_HAL_ELEMENT_TYPE_INT_8,
        /// Signed 8-bit integers.
        SINT_8 = n::IREE_HAL_ELEMENT_TYPE_SINT_8,
        /// Unsigned 8-bit integers.
        UINT_8 = n::IREE_HAL_ELEMENT_TYPE_UINT_8,
        /// Signless 16-bit integers.
        INT_16 = n::IREE_HAL_ELEMENT_TYPE_INT_16,
        /// Signed 16-bit integers.
        SINT_16 = n::IREE_HAL_ELEMENT_TYPE_SINT_16,
        /// Unsigned 16-bit integers.
        UINT_16 = n::IREE_HAL_ELEMENT_TYPE_UINT_16,
        /// Signless 32-bit integers.
        INT_32 = n::IREE_HAL_ELEMENT_TYPE_INT_32,
        /// Signed 32-bit integers.
        SINT_32 = n::IREE_HAL_ELEMENT_TYPE_SINT_32,
        /// Unsigned 32-bit integers.
        UINT_32 = n::IREE_HAL_ELEMENT_TYPE_UINT_32,
        /// Signless 64-bit integers.
        INT_64 = n::IREE_HAL_ELEMENT_TYPE_INT_64,
        /// Signed 64-bit integers.
        SINT_64 = n::IREE_HAL_ELEMENT_TYPE_SINT_64,
        /// Unsigned 64-bit integers.
        UINT_64 = n::IREE_HAL_ELEMENT_TYPE_UINT_64,
        /// IEEE-754 16-bit floats.
        FLOAT_16 = n::IREE_HAL_ELEMENT_TYPE_FLOAT_16,
        /// IEEE-754 32-bit floats.
        FLOAT_32 = n::IREE_HAL_ELEMENT_TYPE_FLOAT_32,
        /// IEEE-754 64-bit floats.
        FLOAT_64 = n::IREE_HAL_ELEMENT_TYPE_FLOAT_64,
        /// Brain floating point 16-bit floats.
        BFLOAT_16 = n::IREE_HAL_ELEMENT_TYPE_BFLOAT_16,
        /// Complex 64-bit floating-point elements.
        COMPLEX_FLOAT_64 = n::IREE_HAL_ELEMENT_TYPE_COMPLEX_FLOAT_64,
        /// Complex 128-bit floating-point elements.
        COMPLEX_FLOAT_128 = n::IREE_HAL_ELEMENT_TYPE_COMPLEX_FLOAT_128,
        /// 8-bit E5M2 floats.
        FLOAT_8_E5M2 = n::IREE_HAL_ELEMENT_TYPE_FLOAT_8_E5M2,
        /// 8-bit E4M3FN floats.
        FLOAT_8_E4M3_FN = n::IREE_HAL_ELEMENT_TYPE_FLOAT_8_E4M3_FN,
        /// 8-bit E5M2 FNUZ floats.
        FLOAT_8_E5M2_FNUZ = n::IREE_HAL_ELEMENT_TYPE_FLOAT_8_E5M2_FNUZ,
        /// 8-bit E4M3 FNUZ floats.
        FLOAT_8_E4M3_FNUZ = n::IREE_HAL_ELEMENT_TYPE_FLOAT_8_E4M3_FNUZ,
        /// 8-bit E8M0 FNU floats.
        FLOAT_8_E8M0_FNU = n::IREE_HAL_ELEMENT_TYPE_FLOAT_8_E8M0_FNU,
    }
}

scalar_value! {
        /// A HAL buffer encoding (`iree_hal_encoding_type_t`).
    EncodingType: n::iree_hal_encoding_type_t {
        /// Opaque or unspecified encoding.
        OPAQUE = n::IREE_HAL_ENCODING_TYPE_OPAQUE,
        /// Dense row-major encoding.
        DENSE_ROW_MAJOR = n::IREE_HAL_ENCODING_TYPE_DENSE_ROW_MAJOR,
    }
}

/// Allocation parameters for a HAL buffer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferParams {
    /// Intended operations on the buffer.
    pub usage: BufferUsage,
    /// Permitted access modes.
    pub access: MemoryAccess,
    /// Requested memory placement and capabilities.
    pub memory_type: MemoryType,
    /// Queues the buffer is used on; the default lets the allocator choose.
    pub queue_affinity: QueueAffinity,
    /// Minimum allocation alignment in bytes, or zero for the default.
    pub min_alignment: super::DeviceSize,
}

impl BufferParams {
    /// Applies IREE's canonical defaults and normalization rules in place.
    pub fn canonicalize(&mut self) {
        let mut raw = self.raw();
        // SAFETY: `raw` is a valid initialized parameter record.
        unsafe { n::iree_hal_buffer_params_canonicalize(&mut raw) };
        *self = Self::from_raw(raw);
    }

    /// Returns parameters with `usage` applied using IREE's canonical helper.
    pub fn with_usage(self, usage: BufferUsage) -> Self {
        // SAFETY: both values are plain C bitfields.
        Self::from_raw(unsafe { n::iree_hal_buffer_params_with_usage(self.raw(), usage.bits()) })
    }

    pub(crate) fn from_raw(raw: n::iree_hal_buffer_params_t) -> Self {
        Self {
            usage: BufferUsage::from_bits_retain(raw.usage),
            access: MemoryAccess::from_bits_retain(raw.access),
            memory_type: MemoryType::from_bits_retain(raw.type_),
            queue_affinity: QueueAffinity::from_bits_retain(raw.queue_affinity),
            min_alignment: raw.min_alignment,
        }
    }

    pub(crate) fn raw(self) -> n::iree_hal_buffer_params_t {
        n::iree_hal_buffer_params_t {
            usage: self.usage.bits(),
            access: self.access.bits(),
            type_: self.memory_type.bits(),
            queue_affinity: self.queue_affinity.bits(),
            min_alignment: self.min_alignment,
        }
    }
}

bitfield! {
    /// Executable-function behavior flags.
    ExecutableFunctionFlags: n::iree_hal_executable_function_flags_t {
        /// The function must execute workgroups sequentially.
        SEQUENTIAL = n::IREE_HAL_EXECUTABLE_FUNCTION_FLAG_SEQUENTIAL,
        /// The workgroup size is provided at dispatch time.
        WORKGROUP_SIZE_DYNAMIC = n::IREE_HAL_EXECUTABLE_FUNCTION_FLAG_WORKGROUP_SIZE_DYNAMIC,
    }
}

scalar_value! {
    /// Profile metric units.
    ProfileMetricUnit: n::iree_hal_profile_metric_unit_t {
        /// Unitless.
        NONE = n::IREE_HAL_PROFILE_METRIC_UNIT_NONE,
        /// Event count.
        COUNT = n::IREE_HAL_PROFILE_METRIC_UNIT_COUNT,
        /// Bytes.
        BYTES = n::IREE_HAL_PROFILE_METRIC_UNIT_BYTES,
        /// Bytes per second.
        BYTES_PER_SECOND = n::IREE_HAL_PROFILE_METRIC_UNIT_BYTES_PER_SECOND,
        /// Hertz.
        HERTZ = n::IREE_HAL_PROFILE_METRIC_UNIT_HERTZ,
        /// Microjoules.
        MICROJOULES = n::IREE_HAL_PROFILE_METRIC_UNIT_MICROJOULES,
        /// Microwatts.
        MICROWATTS = n::IREE_HAL_PROFILE_METRIC_UNIT_MICROWATTS,
        /// Thousandths of a degree Celsius.
        MILLIDEGREES_CELSIUS = n::IREE_HAL_PROFILE_METRIC_UNIT_MILLIDEGREES_CELSIUS,
        /// Thousandths of a percent.
        MILLIPERCENT = n::IREE_HAL_PROFILE_METRIC_UNIT_MILLIPERCENT,
        /// A bitfield value.
        BITFIELD = n::IREE_HAL_PROFILE_METRIC_UNIT_BITFIELD,
        /// An enumerated value.
        ENUM = n::IREE_HAL_PROFILE_METRIC_UNIT_ENUM,
    }
}
scalar_value! {
    /// Profile metric value representations.
    ProfileMetricValueKind: n::iree_hal_profile_metric_value_kind_t {
        /// Unspecified.
        NONE = n::IREE_HAL_PROFILE_METRIC_VALUE_KIND_NONE,
        /// Signed 64-bit integer.
        I64 = n::IREE_HAL_PROFILE_METRIC_VALUE_KIND_I64,
        /// Unsigned 64-bit integer.
        U64 = n::IREE_HAL_PROFILE_METRIC_VALUE_KIND_U64,
        /// 64-bit float.
        F64 = n::IREE_HAL_PROFILE_METRIC_VALUE_KIND_F64,
    }
}
scalar_value! {
    /// Profile metric sampling semantics.
    ProfileMetricSemantic: n::iree_hal_profile_metric_semantic_t {
        /// Unspecified.
        NONE = n::IREE_HAL_PROFILE_METRIC_SEMANTIC_NONE,
        /// Instantaneous sample.
        INSTANT = n::IREE_HAL_PROFILE_METRIC_SEMANTIC_INSTANT,
        /// Average over the sample interval.
        AVERAGE = n::IREE_HAL_PROFILE_METRIC_SEMANTIC_AVERAGE,
        /// Monotonic cumulative counter.
        CUMULATIVE = n::IREE_HAL_PROFILE_METRIC_SEMANTIC_CUMULATIVE,
        /// Change since the previous sample.
        DELTA = n::IREE_HAL_PROFILE_METRIC_SEMANTIC_DELTA,
        /// Discrete state.
        STATE = n::IREE_HAL_PROFILE_METRIC_SEMANTIC_STATE,
    }
}
scalar_value! {
    /// Profile metric visualization hints.
    ProfileMetricPlotHint: n::iree_hal_profile_metric_plot_hint_t {
        /// Unspecified.
        NONE = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_NONE,
        /// Plain number.
        NUMBER = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_NUMBER,
        /// Percentage.
        PERCENTAGE = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_PERCENTAGE,
        /// Memory size.
        MEMORY = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_MEMORY,
        /// Bandwidth.
        BANDWIDTH = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_BANDWIDTH,
        /// Frequency.
        FREQUENCY = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_FREQUENCY,
        /// Power.
        POWER = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_POWER,
        /// Energy.
        ENERGY = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_ENERGY,
        /// Temperature.
        TEMPERATURE = n::IREE_HAL_PROFILE_METRIC_PLOT_HINT_TEMPERATURE,
    }
}
scalar_value! {
    /// Profile statistics row categories.
    ProfileStatisticsRowType: n::iree_hal_profile_statistics_row_type_t {
        /// Unspecified.
        NONE = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_NONE,
        /// Dispatches aggregated by function.
        DISPATCH_FUNCTION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_DISPATCH_FUNCTION,
        /// Dispatches aggregated by command buffer.
        DISPATCH_COMMAND_BUFFER = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_DISPATCH_COMMAND_BUFFER,
        /// Dispatches aggregated by command operation.
        DISPATCH_COMMAND_OPERATION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_DISPATCH_COMMAND_OPERATION,
        /// Device-timed queue operations.
        QUEUE_DEVICE_OPERATION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_QUEUE_DEVICE_OPERATION,
        /// Host-timed queue operations.
        QUEUE_HOST_OPERATION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_QUEUE_HOST_OPERATION,
        /// Host execution aggregated by function.
        HOST_EXECUTION_FUNCTION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_HOST_EXECUTION_FUNCTION,
        /// Host execution aggregated by command buffer.
        HOST_EXECUTION_COMMAND_BUFFER = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_HOST_EXECUTION_COMMAND_BUFFER,
        /// Host execution aggregated by command operation.
        HOST_EXECUTION_COMMAND_OPERATION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_HOST_EXECUTION_COMMAND_OPERATION,
        /// Host execution aggregated by queue operation.
        HOST_EXECUTION_QUEUE_OPERATION = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_HOST_EXECUTION_QUEUE_OPERATION,
        /// Memory lifecycle events.
        MEMORY_LIFECYCLE = n::IREE_HAL_PROFILE_STATISTICS_ROW_TYPE_MEMORY_LIFECYCLE,
    }
}
scalar_value! {
    /// Profile statistics time domains.
    ProfileStatisticsTimeDomain: n::iree_hal_profile_statistics_time_domain_t {
        /// Unspecified.
        NONE = n::IREE_HAL_PROFILE_STATISTICS_TIME_DOMAIN_NONE,
        /// IREE host time in nanoseconds.
        IREE_HOST_TIME_NS = n::IREE_HAL_PROFILE_STATISTICS_TIME_DOMAIN_IREE_HOST_TIME_NS,
        /// Device-specific ticks.
        DEVICE_TICK = n::IREE_HAL_PROFILE_STATISTICS_TIME_DOMAIN_DEVICE_TICK,
    }
}
bitfield! {
    /// Profile statistics row flags.
    ProfileStatisticsRowFlags: n::iree_hal_profile_statistics_row_flags_t {
        /// Timing fields are valid.
        TIMING = n::IREE_HAL_PROFILE_STATISTICS_ROW_FLAG_TIMING,
        /// The operation count is valid.
        OPERATION_COUNT = n::IREE_HAL_PROFILE_STATISTICS_ROW_FLAG_OPERATION_COUNT,
        /// The payload byte count is valid.
        PAYLOAD_BYTES = n::IREE_HAL_PROFILE_STATISTICS_ROW_FLAG_PAYLOAD_BYTES,
        /// Tile totals are valid.
        TILE_TOTALS = n::IREE_HAL_PROFILE_STATISTICS_ROW_FLAG_TILE_TOTALS,
    }
}
scalar_value! {
    /// Profile-file record categories.
    ProfileRecordType: n::iree_hal_profile_file_record_type_t {
        /// Unspecified.
        NONE = n::IREE_HAL_PROFILE_FILE_RECORD_TYPE_NONE,
        /// Session begin marker.
        SESSION_BEGIN = n::IREE_HAL_PROFILE_FILE_RECORD_TYPE_SESSION_BEGIN,
        /// Data chunk.
        CHUNK = n::IREE_HAL_PROFILE_FILE_RECORD_TYPE_CHUNK,
        /// Session end marker.
        SESSION_END = n::IREE_HAL_PROFILE_FILE_RECORD_TYPE_SESSION_END,
    }
}
bitfield! {
    /// Profile chunk flags.
    ProfileChunkFlags: n::iree_hal_profile_chunk_flags_t {
        /// The producer dropped records from this chunk.
        TRUNCATED = n::IREE_HAL_PROFILE_CHUNK_FLAG_TRUNCATED,
    }
}
bitfield! {
    /// Profile counter-set selection flags; upstream defines none yet
    /// (`iree_hal_profile_counter_set_selection_flags_t`).
    CounterSetSelectionFlags: n::iree_hal_profile_counter_set_selection_flags_t {}
}

scalar_value! {
    /// Profile metric identifier.
    MetricId: u64 {}
}

bitfield! {
    /// Device-profiling session flags.
    ProfilingFlags: n::iree_hal_device_profiling_flags_t {
        /// Collect lightweight statistics.
        LIGHTWEIGHT_STATISTICS = n::IREE_HAL_DEVICE_PROFILING_FLAG_LIGHTWEIGHT_STATISTICS,
    }
}
bitfield! {
    /// Executable-cache behavior flags.
    ExecutableCachingMode: n::iree_hal_executable_caching_mode_t {
        /// Requests aliased executable bytes; [`super::ExecutableCache::prepare`]
        /// clears this bit because borrowed bytes cannot follow native owners.
        ALIAS_PROVIDED_DATA = n::IREE_HAL_EXECUTABLE_CACHING_MODE_ALIAS_PROVIDED_DATA,
        /// Allows persistent caching.
        ALLOW_PERSISTENT_CACHING = n::IREE_HAL_EXECUTABLE_CACHING_MODE_ALLOW_PERSISTENT_CACHING,
        /// Allows executable optimization.
        ALLOW_OPTIMIZATION = n::IREE_HAL_EXECUTABLE_CACHING_MODE_ALLOW_OPTIMIZATION,
        /// Enables executable debugging.
        ENABLE_DEBUGGING = n::IREE_HAL_EXECUTABLE_CACHING_MODE_ENABLE_DEBUGGING,
        /// Enables coverage instrumentation.
        ENABLE_COVERAGE = n::IREE_HAL_EXECUTABLE_CACHING_MODE_ENABLE_COVERAGE,
        /// Enables profiling instrumentation.
        ENABLE_PROFILING = n::IREE_HAL_EXECUTABLE_CACHING_MODE_ENABLE_PROFILING,
        /// Disables executable verification.
        DISABLE_VERIFICATION = n::IREE_HAL_EXECUTABLE_CACHING_MODE_DISABLE_VERIFICATION,
    }
}
bitfield! {
    /// Profile data families requested from a device.
    ProfileDataFamilies: n::iree_hal_device_profiling_data_families_t {
        /// Host-timestamped queue operation records.
        QUEUE_EVENTS = n::IREE_HAL_DEVICE_PROFILING_DATA_QUEUE_EVENTS,
        /// Host-side execution spans.
        HOST_EXECUTION_EVENTS = n::IREE_HAL_DEVICE_PROFILING_DATA_HOST_EXECUTION_EVENTS,
        /// Device-timestamped queue operation spans.
        DEVICE_QUEUE_EVENTS = n::IREE_HAL_DEVICE_PROFILING_DATA_DEVICE_QUEUE_EVENTS,
        /// Device-timestamped dispatch events.
        DISPATCH_EVENTS = n::IREE_HAL_DEVICE_PROFILING_DATA_DISPATCH_EVENTS,
        /// Operation-attributed counter samples.
        COUNTER_SAMPLES = n::IREE_HAL_DEVICE_PROFILING_DATA_COUNTER_SAMPLES,
        /// Executable metadata.
        EXECUTABLE_METADATA = n::IREE_HAL_DEVICE_PROFILING_DATA_EXECUTABLE_METADATA,
        /// Executable trace artifacts.
        EXECUTABLE_TRACES = n::IREE_HAL_DEVICE_PROFILING_DATA_EXECUTABLE_TRACES,
        /// Memory allocation lifecycle records.
        MEMORY_EVENTS = n::IREE_HAL_DEVICE_PROFILING_DATA_MEMORY_EVENTS,
        /// Periodic device metrics.
        DEVICE_METRICS = n::IREE_HAL_DEVICE_PROFILING_DATA_DEVICE_METRICS,
        /// Command-region events.
        COMMAND_REGION_EVENTS = n::IREE_HAL_DEVICE_PROFILING_DATA_COMMAND_REGION_EVENTS,
        /// Range-scoped counter samples.
        COUNTER_RANGES = n::IREE_HAL_DEVICE_PROFILING_DATA_COUNTER_RANGES,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn usage() -> impl Strategy<Value = BufferUsage> {
        any::<u32>().prop_map(BufferUsage::from_bits_retain)
    }

    proptest! {
        #[test]
        fn union_is_the_least_upper_bound(a in usage(), b in usage(), c in usage()) {
            let union = a | b;
            prop_assert!(union.contains(a));
            prop_assert!(union.contains(b));
            if c.contains(a) && c.contains(b) {
                prop_assert!(c.contains(union));
            }
        }

        #[test]
        fn intersection_is_contained_in_both(a in usage(), b in usage()) {
            let intersection = a & b;
            prop_assert!(a.contains(intersection));
            prop_assert!(b.contains(intersection));
        }

        #[test]
        fn union_assign_matches_union(a in usage(), b in usage()) {
            let mut assigned = a;
            assigned |= b;
            prop_assert_eq!(assigned, a | b);
        }

        #[test]
        fn params_survive_the_native_record(
            usage in usage(),
            access in any::<u16>(),
            memory_type in any::<u32>(),
            queue_affinity in any::<u64>(),
            min_alignment in any::<super::super::DeviceSize>(),
        ) {
            let params = BufferParams {
                usage,
                access: MemoryAccess::from_bits_retain(access),
                memory_type: MemoryType::from_bits_retain(memory_type),
                queue_affinity: QueueAffinity::from_bits_retain(queue_affinity),
                min_alignment,
            };
            prop_assert_eq!(BufferParams::from_raw(params.raw()), params);
        }

        #[test]
        fn canonicalize_is_idempotent(usage in usage(), access in any::<u16>(), memory_type in any::<u32>()) {
            let mut once = BufferParams {
                usage,
                access: MemoryAccess::from_bits_retain(access),
                memory_type: MemoryType::from_bits_retain(memory_type),
                ..BufferParams::default()
            };
            once.canonicalize();
            let mut twice = once;
            twice.canonicalize();
            prop_assert_eq!(twice, once);
        }
    }

    #[test]
    fn composite_constants_are_unions_of_their_parts() {
        assert_eq!(
            BufferUsage::TRANSFER,
            BufferUsage::TRANSFER_SOURCE | BufferUsage::TRANSFER_TARGET
        );
        assert_eq!(
            BufferUsage::DISPATCH_STORAGE,
            BufferUsage::DISPATCH_STORAGE_READ | BufferUsage::DISPATCH_STORAGE_WRITE
        );
        assert_eq!(
            MemoryAccess::DISCARD_WRITE,
            MemoryAccess::DISCARD | MemoryAccess::WRITE
        );
        assert!(MemoryType::DEVICE_LOCAL.contains(MemoryType::DEVICE_VISIBLE));
    }
}
