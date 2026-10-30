use super::*;
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::core::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    ///# Safety
    ///The trailing allocation must contain len aligned, initialized T elements, with a total size no greater than isize::MAX. They must remain valid for the returned slice's lifetime, without conflicting access (and with exclusive access for a mutable slice).
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        // SAFETY: The caller guarantees the trailing allocation contains len initialized, aligned elements and that the returned slice obeys Rust's lifetime and aliasing requirements.
        unsafe { ::core::slice::from_raw_parts(self.as_ptr(), len) }
    }
    #[inline]
    ///# Safety
    ///The trailing allocation must contain len aligned, initialized T elements, with a total size no greater than isize::MAX. They must remain valid for the returned slice's lifetime, without conflicting access (and with exclusive access for a mutable slice).
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        // SAFETY: The caller guarantees the trailing allocation contains len initialized, aligned elements and that the returned slice obeys Rust's lifetime and aliasing requirements.
        unsafe { ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}
impl<T> ::core::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const HSA_VERSION_1_0: u32 = 1;
pub const HSA_AMD_INTERFACE_VERSION_MAJOR: u32 = 1;
pub const HSA_AMD_INTERFACE_VERSION_MINOR: u32 = 14;
pub const IREE_HAL_AMDGPU_LIBHSA_STATIC: u32 = 0;
pub const IREE_HAL_AMDGPU_LIBHSA_TRACE_CATEGORY_ALWAYS: u32 = 1;
pub const IREE_HAL_AMDGPU_LIBHSA_TRACE_CATEGORY_SIGNALS: u32 = 2;
pub const IREE_HAL_AMDGPU_LIBHSA_TRACE_CATEGORY_QUEUES: u32 = 4;
pub const IREE_HAL_AMDGPU_LIBHSA_TRACE_CATEGORY_ALL: u32 = 7;
pub const IREE_HAL_AMDGPU_LIBHSA_TRACING_MODE: u32 = 0;
pub const IREE_HAL_AMDGPU_MAX_CPU_AGENT: u32 = 64;
pub const IREE_HAL_AMDGPU_MAX_GPU_AGENT: u32 = 64;
pub const IREE_HAL_AMDGPU_DEFAULT_GPU_AGENT_QUEUE_COUNT: u32 = 2;
/// The function has been executed successfully.
pub const HSA_STATUS_SUCCESS: hsa_status_t = 0;
/** A traversal over a list of elements has been interrupted by the
 application before completing.*/
pub const HSA_STATUS_INFO_BREAK: hsa_status_t = 1;
/// A generic error has occurred.
pub const HSA_STATUS_ERROR: hsa_status_t = 4096;
/** One of the actual arguments does not meet a precondition stated in the
 documentation of the corresponding formal argument.*/
pub const HSA_STATUS_ERROR_INVALID_ARGUMENT: hsa_status_t = 4097;
/// The requested queue creation is not valid.
pub const HSA_STATUS_ERROR_INVALID_QUEUE_CREATION: hsa_status_t = 4098;
/// The requested allocation is not valid.
pub const HSA_STATUS_ERROR_INVALID_ALLOCATION: hsa_status_t = 4099;
/// The agent is invalid.
pub const HSA_STATUS_ERROR_INVALID_AGENT: hsa_status_t = 4100;
/// The memory region is invalid.
pub const HSA_STATUS_ERROR_INVALID_REGION: hsa_status_t = 4101;
/// The signal is invalid.
pub const HSA_STATUS_ERROR_INVALID_SIGNAL: hsa_status_t = 4102;
/// The queue is invalid.
pub const HSA_STATUS_ERROR_INVALID_QUEUE: hsa_status_t = 4103;
/** The HSA runtime failed to allocate the necessary resources. This error
 may also occur when the HSA runtime needs to spawn threads or create
 internal OS-specific events.*/
pub const HSA_STATUS_ERROR_OUT_OF_RESOURCES: hsa_status_t = 4104;
/// The AQL packet is malformed.
pub const HSA_STATUS_ERROR_INVALID_PACKET_FORMAT: hsa_status_t = 4105;
/// An error has been detected while releasing a resource.
pub const HSA_STATUS_ERROR_RESOURCE_FREE: hsa_status_t = 4106;
/** An API other than ::hsa_init has been invoked while the reference count
 of the HSA runtime is 0.*/
pub const HSA_STATUS_ERROR_NOT_INITIALIZED: hsa_status_t = 4107;
/// The maximum reference count for the object has been reached.
pub const HSA_STATUS_ERROR_REFCOUNT_OVERFLOW: hsa_status_t = 4108;
/// The arguments passed to a functions are not compatible.
pub const HSA_STATUS_ERROR_INCOMPATIBLE_ARGUMENTS: hsa_status_t = 4109;
/// The index is invalid.
pub const HSA_STATUS_ERROR_INVALID_INDEX: hsa_status_t = 4110;
/// The instruction set architecture is invalid.
pub const HSA_STATUS_ERROR_INVALID_ISA: hsa_status_t = 4111;
/// The instruction set architecture name is invalid.
pub const HSA_STATUS_ERROR_INVALID_ISA_NAME: hsa_status_t = 4119;
/// The code object is invalid.
pub const HSA_STATUS_ERROR_INVALID_CODE_OBJECT: hsa_status_t = 4112;
/// The executable is invalid.
pub const HSA_STATUS_ERROR_INVALID_EXECUTABLE: hsa_status_t = 4113;
/// The executable is frozen.
pub const HSA_STATUS_ERROR_FROZEN_EXECUTABLE: hsa_status_t = 4114;
/// There is no symbol with the given name.
pub const HSA_STATUS_ERROR_INVALID_SYMBOL_NAME: hsa_status_t = 4115;
/// The variable is already defined.
pub const HSA_STATUS_ERROR_VARIABLE_ALREADY_DEFINED: hsa_status_t = 4116;
/// The variable is undefined.
pub const HSA_STATUS_ERROR_VARIABLE_UNDEFINED: hsa_status_t = 4117;
/// An HSAIL operation resulted in a hardware exception.
pub const HSA_STATUS_ERROR_EXCEPTION: hsa_status_t = 4118;
/// The code object symbol is invalid.
pub const HSA_STATUS_ERROR_INVALID_CODE_SYMBOL: hsa_status_t = 4120;
/// The executable symbol is invalid.
pub const HSA_STATUS_ERROR_INVALID_EXECUTABLE_SYMBOL: hsa_status_t = 4121;
/// The file descriptor is invalid.
pub const HSA_STATUS_ERROR_INVALID_FILE: hsa_status_t = 4128;
/// The code object reader is invalid.
pub const HSA_STATUS_ERROR_INVALID_CODE_OBJECT_READER: hsa_status_t = 4129;
/// The cache is invalid.
pub const HSA_STATUS_ERROR_INVALID_CACHE: hsa_status_t = 4130;
/// The wavefront is invalid.
pub const HSA_STATUS_ERROR_INVALID_WAVEFRONT: hsa_status_t = 4131;
/// The signal group is invalid.
pub const HSA_STATUS_ERROR_INVALID_SIGNAL_GROUP: hsa_status_t = 4132;
/// The HSA runtime is not in the configuration state.
pub const HSA_STATUS_ERROR_INVALID_RUNTIME_STATE: hsa_status_t = 4133;
/// The queue received an error that may require process termination.
pub const HSA_STATUS_ERROR_FATAL: hsa_status_t = 4134;
/// @brief Status codes.
pub type hsa_status_t = ::core::ffi::c_uint;
/// @brief Three-dimensional coordinate.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_dim3_s {
    /// X dimension.
    pub x: u32,
    /// Y dimension.
    pub y: u32,
    /// Z dimension.
    pub z: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_dim3_s"][::core::mem::size_of::<hsa_dim3_s>() - 12usize];
    ["Alignment of hsa_dim3_s"][::core::mem::align_of::<hsa_dim3_s>() - 4usize];
    ["Offset of field: hsa_dim3_s::x"][::core::mem::offset_of!(hsa_dim3_s, x) - 0usize];
    ["Offset of field: hsa_dim3_s::y"][::core::mem::offset_of!(hsa_dim3_s, y) - 4usize];
    ["Offset of field: hsa_dim3_s::z"][::core::mem::offset_of!(hsa_dim3_s, z) - 8usize];
};
/// @brief Three-dimensional coordinate.
pub type hsa_dim3_t = hsa_dim3_s;
/// Used to remove existing access
pub const HSA_ACCESS_PERMISSION_NONE: hsa_access_permission_t = 0;
/// Read-only access.
pub const HSA_ACCESS_PERMISSION_RO: hsa_access_permission_t = 1;
/// Write-only access.
pub const HSA_ACCESS_PERMISSION_WO: hsa_access_permission_t = 2;
/// Read and write access.
pub const HSA_ACCESS_PERMISSION_RW: hsa_access_permission_t = 3;
/// @brief Access permissions.
pub type hsa_access_permission_t = ::core::ffi::c_uint;
/// @brief POSIX file descriptor.
pub type hsa_file_t = ::core::ffi::c_int;
/// The least significant byte is stored in the smallest address.
pub const HSA_ENDIANNESS_LITTLE: hsa_endianness_t = 0;
/// The most significant byte is stored in the smallest address.
pub const HSA_ENDIANNESS_BIG: hsa_endianness_t = 1;
/** @brief Endianness. A convention used to interpret the bytes making up a data
 word.*/
pub type hsa_endianness_t = ::core::ffi::c_uint;
/// Small machine model. Addresses use 32 bits.
pub const HSA_MACHINE_MODEL_SMALL: hsa_machine_model_t = 0;
/// Large machine model. Addresses use 64 bits.
pub const HSA_MACHINE_MODEL_LARGE: hsa_machine_model_t = 1;
/** @brief Machine model. A machine model determines the size of certain data
 types in HSA runtime and an agent.*/
pub type hsa_machine_model_t = ::core::ffi::c_uint;
/// Base profile.
pub const HSA_PROFILE_BASE: hsa_profile_t = 0;
/// Full profile.
pub const HSA_PROFILE_FULL: hsa_profile_t = 1;
/** @brief Profile. A profile indicates a particular level of feature
 support. For example, in the base profile the application must use the HSA
 runtime allocator to reserve shared virtual memory, while in the full profile
 any host pointer can be shared across all the agents.*/
pub type hsa_profile_t = ::core::ffi::c_uint;
/** Major version of the HSA runtime specification supported by the
 implementation. The type of this attribute is uint16_t.*/
pub const HSA_SYSTEM_INFO_VERSION_MAJOR: hsa_system_info_t = 0;
/** Minor version of the HSA runtime specification supported by the
 implementation. The type of this attribute is uint16_t.*/
pub const HSA_SYSTEM_INFO_VERSION_MINOR: hsa_system_info_t = 1;
/** Current timestamp. The value of this attribute monotonically increases at a
 constant rate. The type of this attribute is uint64_t.*/
pub const HSA_SYSTEM_INFO_TIMESTAMP: hsa_system_info_t = 2;
/** Timestamp value increase rate, in Hz. The timestamp (clock) frequency is
 in the range 1-400MHz. The type of this attribute is uint64_t.*/
pub const HSA_SYSTEM_INFO_TIMESTAMP_FREQUENCY: hsa_system_info_t = 3;
/** Maximum duration of a signal wait operation. Expressed as a count based on
 the timestamp frequency. The type of this attribute is uint64_t.*/
pub const HSA_SYSTEM_INFO_SIGNAL_MAX_WAIT: hsa_system_info_t = 4;
/// Endianness of the system. The type of this attribute is ::hsa_endianness_t.
pub const HSA_SYSTEM_INFO_ENDIANNESS: hsa_system_info_t = 5;
/** Machine model supported by the HSA runtime. The type of this attribute is
 ::hsa_machine_model_t.*/
pub const HSA_SYSTEM_INFO_MACHINE_MODEL: hsa_system_info_t = 6;
/** Bit-mask indicating which extensions are supported by the
 implementation. An extension with an ID of @p i is supported if the bit at
 position @p i is set. The type of this attribute is uint8_t[128].*/
pub const HSA_SYSTEM_INFO_EXTENSIONS: hsa_system_info_t = 7;
/// String containing the ROCr build identifier.
pub const HSA_AMD_SYSTEM_INFO_BUILD_VERSION: hsa_system_info_t = 512;
/** Returns true if hsa_amd_svm_* APIs are supported by the driver.  The type of
 this attribute is bool.*/
pub const HSA_AMD_SYSTEM_INFO_SVM_SUPPORTED: hsa_system_info_t = 513;
/** Returns true if all Agents have access to system allocated memory (such as
 that allocated by mmap, malloc, or new) by default.
 If false then system allocated memory may only be made SVM accessible to
 an Agent by declaration of accessibility with hsa_amd_svm_set_attributes.
 The type of this attribute is bool.*/
pub const HSA_AMD_SYSTEM_INFO_SVM_ACCESSIBLE_BY_DEFAULT: hsa_system_info_t = 514;
/** Returns true if mwaitx is enabled on this system
 The type of this attribute is bool.*/
pub const HSA_AMD_SYSTEM_INFO_MWAITX_ENABLED: hsa_system_info_t = 515;
/** Returns true if DMABUF APIs are supported by the driver.  The type of
 this attribute is bool.*/
pub const HSA_AMD_SYSTEM_INFO_DMABUF_SUPPORTED: hsa_system_info_t = 516;
/** Returns true if Virtual Memory APIs are supported by the driver.  The type of
 this attribute is bool.*/
pub const HSA_AMD_SYSTEM_INFO_VIRTUAL_MEM_API_SUPPORTED: hsa_system_info_t = 517;
/** Returns true if XNACK is enabled on this system.  The type of
 this attribute is bool.*/
pub const HSA_AMD_SYSTEM_INFO_XNACK_ENABLED: hsa_system_info_t = 518;
/** Major version of the HSA runtime extension specification supported by the
 implementation. The type of this attribute is uint16_t.*/
pub const HSA_AMD_SYSTEM_INFO_EXT_VERSION_MAJOR: hsa_system_info_t = 519;
/** Minor version of the HSA runtime extension specification supported by the
 implementation. The type of this attribute is uint16_t.*/
pub const HSA_AMD_SYSTEM_INFO_EXT_VERSION_MINOR: hsa_system_info_t = 520;
/// @brief System attributes.
pub type hsa_system_info_t = ::core::ffi::c_uint;
/// Finalizer extension.
pub const HSA_EXTENSION_FINALIZER: hsa_extension_t = 0;
/// Images extension.
pub const HSA_EXTENSION_IMAGES: hsa_extension_t = 1;
/// Performance counter extension.
pub const HSA_EXTENSION_PERFORMANCE_COUNTERS: hsa_extension_t = 2;
/// Profiling events extension.
pub const HSA_EXTENSION_PROFILING_EVENTS: hsa_extension_t = 3;
/// Extension count.
pub const HSA_EXTENSION_STD_LAST: hsa_extension_t = 3;
/// First AMD extension number.
pub const HSA_AMD_FIRST_EXTENSION: hsa_extension_t = 512;
/// Profiler extension.
pub const HSA_EXTENSION_AMD_PROFILER: hsa_extension_t = 512;
/// Loader extension.
pub const HSA_EXTENSION_AMD_LOADER: hsa_extension_t = 513;
/// AqlProfile extension.
pub const HSA_EXTENSION_AMD_AQLPROFILE: hsa_extension_t = 514;
/// PC Sampling extension.
pub const HSA_EXTENSION_AMD_PC_SAMPLING: hsa_extension_t = 515;
/// Last AMD extension.
pub const HSA_AMD_LAST_EXTENSION: hsa_extension_t = 515;
/// @brief HSA extensions.
pub type hsa_extension_t = ::core::ffi::c_uint;
/** @brief Struct containing an opaque handle to an agent, a device that participates in
 the HSA memory model. An agent can submit AQL packets for execution, and
 may also accept AQL packets for execution (agent dispatch packets or kernel
 dispatch packets launching HSAIL-derived binaries).*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_agent_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_agent_s"][::core::mem::size_of::<hsa_agent_s>() - 8usize];
    ["Alignment of hsa_agent_s"][::core::mem::align_of::<hsa_agent_s>() - 8usize];
    [
        "Offset of field: hsa_agent_s::handle",
    ][::core::mem::offset_of!(hsa_agent_s, handle) - 0usize];
};
/** @brief Struct containing an opaque handle to an agent, a device that participates in
 the HSA memory model. An agent can submit AQL packets for execution, and
 may also accept AQL packets for execution (agent dispatch packets or kernel
 dispatch packets launching HSAIL-derived binaries).*/
pub type hsa_agent_t = hsa_agent_s;
/** The agent supports AQL packets of kernel dispatch type. If this
 feature is enabled, the agent is also a kernel agent.*/
pub const HSA_AGENT_FEATURE_KERNEL_DISPATCH: hsa_agent_feature_t = 1;
/// The agent supports AQL packets of agent dispatch type.
pub const HSA_AGENT_FEATURE_AGENT_DISPATCH: hsa_agent_feature_t = 2;
/// @brief Agent features.
pub type hsa_agent_feature_t = ::core::ffi::c_uint;
/// CPU device.
pub const HSA_DEVICE_TYPE_CPU: hsa_device_type_t = 0;
/// GPU device.
pub const HSA_DEVICE_TYPE_GPU: hsa_device_type_t = 1;
/// DSP device.
pub const HSA_DEVICE_TYPE_DSP: hsa_device_type_t = 2;
/// AI Engine (AIE) device.
pub const HSA_DEVICE_TYPE_AIE: hsa_device_type_t = 3;
/// @brief Hardware device type.
pub type hsa_device_type_t = ::core::ffi::c_uint;
/// Use a default floating-point rounding mode specified elsewhere.
pub const HSA_DEFAULT_FLOAT_ROUNDING_MODE_DEFAULT: hsa_default_float_rounding_mode_t = 0;
/** Operations that specify the default floating-point mode are rounded to zero
 by default.*/
pub const HSA_DEFAULT_FLOAT_ROUNDING_MODE_ZERO: hsa_default_float_rounding_mode_t = 1;
/** Operations that specify the default floating-point mode are rounded to the
 nearest representable number and that ties should be broken by selecting
 the value with an even least significant bit.*/
pub const HSA_DEFAULT_FLOAT_ROUNDING_MODE_NEAR: hsa_default_float_rounding_mode_t = 2;
/// @brief Default floating-point rounding mode.
pub type hsa_default_float_rounding_mode_t = ::core::ffi::c_uint;
/** Agent name. The type of this attribute is a NUL-terminated char[64]. The
 name must be at most 63 characters long (not including the NUL terminator)
 and all array elements not used for the name must be NUL.*/
pub const HSA_AGENT_INFO_NAME: hsa_agent_info_t = 0;
/** Name of vendor. The type of this attribute is a NUL-terminated char[64].
 The name must be at most 63 characters long (not including the NUL
 terminator) and all array elements not used for the name must be NUL.*/
pub const HSA_AGENT_INFO_VENDOR_NAME: hsa_agent_info_t = 1;
/// Agent capability. The type of this attribute is ::hsa_agent_feature_t.
pub const HSA_AGENT_INFO_FEATURE: hsa_agent_info_t = 2;
/** @deprecated Query ::HSA_ISA_INFO_MACHINE_MODELS for a given intruction set
 architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Machine model supported by the agent. The type of this attribute is
 ::hsa_machine_model_t.*/
pub const HSA_AGENT_INFO_MACHINE_MODEL: hsa_agent_info_t = 3;
/** @deprecated Query ::HSA_ISA_INFO_PROFILES for a given intruction set
 architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Profile supported by the agent. The type of this attribute is
 ::hsa_profile_t.*/
pub const HSA_AGENT_INFO_PROFILE: hsa_agent_info_t = 4;
/** @deprecated Query ::HSA_ISA_INFO_DEFAULT_FLOAT_ROUNDING_MODES for a given
 intruction set architecture supported by the agent instead.  If more than
 one ISA is supported by the agent, the returned value corresponds to the
 first ISA enumerated by ::hsa_agent_iterate_isas.

 Default floating-point rounding mode. The type of this attribute is
 ::hsa_default_float_rounding_mode_t, but the value
 ::HSA_DEFAULT_FLOAT_ROUNDING_MODE_DEFAULT is not allowed.*/
pub const HSA_AGENT_INFO_DEFAULT_FLOAT_ROUNDING_MODE: hsa_agent_info_t = 5;
/** @deprecated Query ::HSA_ISA_INFO_BASE_PROFILE_DEFAULT_FLOAT_ROUNDING_MODES
 for a given intruction set architecture supported by the agent instead.  If
 more than one ISA is supported by the agent, the returned value corresponds
 to the first ISA enumerated by ::hsa_agent_iterate_isas.

 A bit-mask of ::hsa_default_float_rounding_mode_t values, representing the
 default floating-point rounding modes supported by the agent in the Base
 profile. The type of this attribute is uint32_t. The default floating-point
 rounding mode (::HSA_AGENT_INFO_DEFAULT_FLOAT_ROUNDING_MODE) bit must not
 be set.*/
pub const HSA_AGENT_INFO_BASE_PROFILE_DEFAULT_FLOAT_ROUNDING_MODES: hsa_agent_info_t = 23;
/** @deprecated Query ::HSA_ISA_INFO_FAST_F16_OPERATION for a given intruction
 set architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Flag indicating that the f16 HSAIL operation is at least as fast as the
 f32 operation in the current agent. The value of this attribute is
 undefined if the agent is not a kernel agent. The type of this
 attribute is bool.*/
pub const HSA_AGENT_INFO_FAST_F16_OPERATION: hsa_agent_info_t = 24;
/** @deprecated Query ::HSA_WAVEFRONT_INFO_SIZE for a given wavefront and
 intruction set architecture supported by the agent instead.  If more than
 one ISA is supported by the agent, the returned value corresponds to the
 first ISA enumerated by ::hsa_agent_iterate_isas and the first wavefront
 enumerated by ::hsa_isa_iterate_wavefronts for that ISA.

 Number of work-items in a wavefront. Must be a power of 2 in the range
 [1,256]. The value of this attribute is undefined if the agent is not
 a kernel agent. The type of this attribute is uint32_t.*/
pub const HSA_AGENT_INFO_WAVEFRONT_SIZE: hsa_agent_info_t = 6;
/** @deprecated Query ::HSA_ISA_INFO_WORKGROUP_MAX_DIM for a given intruction
 set architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Maximum number of work-items of each dimension of a work-group.  Each
 maximum must be greater than 0. No maximum can exceed the value of
 ::HSA_AGENT_INFO_WORKGROUP_MAX_SIZE. The value of this attribute is
 undefined if the agent is not a kernel agent. The type of this
 attribute is uint16_t[3].*/
pub const HSA_AGENT_INFO_WORKGROUP_MAX_DIM: hsa_agent_info_t = 7;
/** @deprecated Query ::HSA_ISA_INFO_WORKGROUP_MAX_SIZE for a given intruction
 set architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Maximum total number of work-items in a work-group. The value of this
 attribute is undefined if the agent is not a kernel agent. The type
 of this attribute is uint32_t.*/
pub const HSA_AGENT_INFO_WORKGROUP_MAX_SIZE: hsa_agent_info_t = 8;
/** @deprecated Query ::HSA_ISA_INFO_GRID_MAX_DIM for a given intruction set
 architecture supported by the agent instead.

 Maximum number of work-items of each dimension of a grid. Each maximum must
 be greater than 0, and must not be smaller than the corresponding value in
 ::HSA_AGENT_INFO_WORKGROUP_MAX_DIM. No maximum can exceed the value of
 ::HSA_AGENT_INFO_GRID_MAX_SIZE. The value of this attribute is undefined
 if the agent is not a kernel agent. The type of this attribute is
 ::hsa_dim3_t.*/
pub const HSA_AGENT_INFO_GRID_MAX_DIM: hsa_agent_info_t = 9;
/** @deprecated Query ::HSA_ISA_INFO_GRID_MAX_SIZE for a given intruction set
 architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Maximum total number of work-items in a grid. The value of this attribute
 is undefined if the agent is not a kernel agent. The type of this
 attribute is uint32_t.*/
pub const HSA_AGENT_INFO_GRID_MAX_SIZE: hsa_agent_info_t = 10;
/** @deprecated Query ::HSA_ISA_INFO_FBARRIER_MAX_SIZE for a given intruction
 set architecture supported by the agent instead.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Maximum number of fbarriers per work-group. Must be at least 32. The value
 of this attribute is undefined if the agent is not a kernel agent. The
 type of this attribute is uint32_t.*/
pub const HSA_AGENT_INFO_FBARRIER_MAX_SIZE: hsa_agent_info_t = 11;
/** @deprecated The maximum number of queues is not statically determined.

 Maximum number of queues that can be active (created but not destroyed) at
 one time in the agent. The type of this attribute is uint32_t.*/
pub const HSA_AGENT_INFO_QUEUES_MAX: hsa_agent_info_t = 12;
/** Minimum number of packets that a queue created in the agent
 can hold. Must be a power of 2 greater than 0. Must not exceed
 the value of ::HSA_AGENT_INFO_QUEUE_MAX_SIZE. The type of this
 attribute is uint32_t.*/
pub const HSA_AGENT_INFO_QUEUE_MIN_SIZE: hsa_agent_info_t = 13;
/** Maximum number of packets that a queue created in the agent can
 hold. Must be a power of 2 greater than 0. The type of this attribute
 is uint32_t.*/
pub const HSA_AGENT_INFO_QUEUE_MAX_SIZE: hsa_agent_info_t = 14;
/** Type of a queue created in the agent. The type of this attribute is
 ::hsa_queue_type32_t.*/
pub const HSA_AGENT_INFO_QUEUE_TYPE: hsa_agent_info_t = 15;
/** @deprecated NUMA information is not exposed anywhere else in the API.

 Identifier of the NUMA node associated with the agent. The type of this
 attribute is uint32_t.*/
pub const HSA_AGENT_INFO_NODE: hsa_agent_info_t = 16;
/** Type of hardware device associated with the agent. The type of this
 attribute is ::hsa_device_type_t.*/
pub const HSA_AGENT_INFO_DEVICE: hsa_agent_info_t = 17;
/** @deprecated Query ::hsa_agent_iterate_caches to retrieve information about
 the caches present in a given agent.

 Array of data cache sizes (L1..L4). Each size is expressed in bytes. A size
 of 0 for a particular level indicates that there is no cache information
 for that level. The type of this attribute is uint32_t[4].*/
pub const HSA_AGENT_INFO_CACHE_SIZE: hsa_agent_info_t = 18;
/** @deprecated An agent may support multiple instruction set
 architectures. See ::hsa_agent_iterate_isas.  If more than one ISA is
 supported by the agent, the returned value corresponds to the first ISA
 enumerated by ::hsa_agent_iterate_isas.

 Instruction set architecture of the agent. The type of this attribute
 is ::hsa_isa_t.*/
pub const HSA_AGENT_INFO_ISA: hsa_agent_info_t = 19;
/** Bit-mask indicating which extensions are supported by the agent. An
 extension with an ID of @p i is supported if the bit at position @p i is
 set. The type of this attribute is uint8_t[128].*/
pub const HSA_AGENT_INFO_EXTENSIONS: hsa_agent_info_t = 20;
/** Major version of the HSA runtime specification supported by the
 agent. The type of this attribute is uint16_t.*/
pub const HSA_AGENT_INFO_VERSION_MAJOR: hsa_agent_info_t = 21;
/** Minor version of the HSA runtime specification supported by the
 agent. The type of this attribute is uint16_t.*/
pub const HSA_AGENT_INFO_VERSION_MINOR: hsa_agent_info_t = 22;
/** This enum does not have a fixed underlying type, thus in C++ post D2338:
 If the enumeration type does not have a fixed underlying type, the value is
 unchanged if the original value is within the range of the enumeration
 values (9.7.1 [dcl.enum]), and otherwise, the behavior is
 undefined.
 Thus increase the range of this enum to encompass vendor extensions.*/
pub const HSA_AGENT_INFO_LAST: hsa_agent_info_t = 2147483647;
/// @brief Agent attributes.
pub type hsa_agent_info_t = ::core::ffi::c_uint;
/// If a hardware exception is detected, a work-item signals an exception.
pub const HSA_EXCEPTION_POLICY_BREAK: hsa_exception_policy_t = 1;
/// If a hardware exception is detected, a hardware status bit is set.
pub const HSA_EXCEPTION_POLICY_DETECT: hsa_exception_policy_t = 2;
/// @brief Exception policies applied in the presence of hardware exceptions.
pub type hsa_exception_policy_t = ::core::ffi::c_uint;
/// @brief Cache handle.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_cache_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_cache_s"][::core::mem::size_of::<hsa_cache_s>() - 8usize];
    ["Alignment of hsa_cache_s"][::core::mem::align_of::<hsa_cache_s>() - 8usize];
    [
        "Offset of field: hsa_cache_s::handle",
    ][::core::mem::offset_of!(hsa_cache_s, handle) - 0usize];
};
/// @brief Cache handle.
pub type hsa_cache_t = hsa_cache_s;
/** The length of the cache name in bytes, not including the NUL terminator.
 The type of this attribute is uint32_t.*/
pub const HSA_CACHE_INFO_NAME_LENGTH: hsa_cache_info_t = 0;
/** Human-readable description.  The type of this attribute is a NUL-terminated
 character array with the length equal to the value of
 ::HSA_CACHE_INFO_NAME_LENGTH attribute.*/
pub const HSA_CACHE_INFO_NAME: hsa_cache_info_t = 1;
/** Cache level. A L1 cache must return a value of 1, a L2 must return a value
 of 2, and so on.  The type of this attribute is uint8_t.*/
pub const HSA_CACHE_INFO_LEVEL: hsa_cache_info_t = 2;
/** Cache size, in bytes. A value of 0 indicates that there is no size
 information available. The type of this attribute is uint32_t.*/
pub const HSA_CACHE_INFO_SIZE: hsa_cache_info_t = 3;
/// @brief Cache attributes.
pub type hsa_cache_info_t = ::core::ffi::c_uint;
/// @brief Signal handle.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_signal_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal. The value 0 is reserved.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_signal_s"][::core::mem::size_of::<hsa_signal_s>() - 8usize];
    ["Alignment of hsa_signal_s"][::core::mem::align_of::<hsa_signal_s>() - 8usize];
    [
        "Offset of field: hsa_signal_s::handle",
    ][::core::mem::offset_of!(hsa_signal_s, handle) - 0usize];
};
/// @brief Signal handle.
pub type hsa_signal_t = hsa_signal_s;
pub type hsa_signal_value_t = i64;
/// The two operands are equal.
pub const HSA_SIGNAL_CONDITION_EQ: hsa_signal_condition_t = 0;
/// The two operands are not equal.
pub const HSA_SIGNAL_CONDITION_NE: hsa_signal_condition_t = 1;
/// The first operand is less than the second operand.
pub const HSA_SIGNAL_CONDITION_LT: hsa_signal_condition_t = 2;
/// The first operand is greater than or equal to the second operand.
pub const HSA_SIGNAL_CONDITION_GTE: hsa_signal_condition_t = 3;
/// @brief Wait condition operator.
pub type hsa_signal_condition_t = ::core::ffi::c_uint;
/// The application thread may be rescheduled while waiting on the signal.
pub const HSA_WAIT_STATE_BLOCKED: hsa_wait_state_t = 0;
/// The application thread stays active while waiting on a signal.
pub const HSA_WAIT_STATE_ACTIVE: hsa_wait_state_t = 1;
/// @brief State of the application thread during a signal wait.
pub type hsa_wait_state_t = ::core::ffi::c_uint;
/// @brief Group of signals.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_signal_group_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_signal_group_s",
    ][::core::mem::size_of::<hsa_signal_group_s>() - 8usize];
    [
        "Alignment of hsa_signal_group_s",
    ][::core::mem::align_of::<hsa_signal_group_s>() - 8usize];
    [
        "Offset of field: hsa_signal_group_s::handle",
    ][::core::mem::offset_of!(hsa_signal_group_s, handle) - 0usize];
};
/// @brief Group of signals.
pub type hsa_signal_group_t = hsa_signal_group_s;
/** @brief A memory region represents a block of virtual memory with certain
 properties. For example, the HSA runtime represents fine-grained memory in
 the global segment using a region. A region might be associated with more
 than one agent.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_region_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_region_s"][::core::mem::size_of::<hsa_region_s>() - 8usize];
    ["Alignment of hsa_region_s"][::core::mem::align_of::<hsa_region_s>() - 8usize];
    [
        "Offset of field: hsa_region_s::handle",
    ][::core::mem::offset_of!(hsa_region_s, handle) - 0usize];
};
/** @brief A memory region represents a block of virtual memory with certain
 properties. For example, the HSA runtime represents fine-grained memory in
 the global segment using a region. A region might be associated with more
 than one agent.*/
pub type hsa_region_t = hsa_region_s;
/** Queue supports multiple producers. Use of multiproducer queue mechanics is
 required.*/
pub const HSA_QUEUE_TYPE_MULTI: hsa_queue_type_t = 0;
/** Queue only supports a single producer. In some scenarios, the application
 may want to limit the submission of AQL packets to a single agent. Queues
 that support a single producer may be more efficient than queues supporting
 multiple producers. Use of multiproducer queue mechanics is not supported.*/
pub const HSA_QUEUE_TYPE_SINGLE: hsa_queue_type_t = 1;
/** Queue supports multiple producers and cooperative dispatches. Cooperative
 dispatches are able to use GWS synchronization. Queues of this type may be
 limited in number. The runtime may return the same queue to serve multiple
 ::hsa_queue_create calls when this type is given. Callers must inspect the
 returned queue to discover queue size. Queues of this type are reference
 counted and require a matching number of ::hsa_queue_destroy calls to
 release. Use of multiproducer queue mechanics is required. See
 ::HSA_AMD_AGENT_INFO_COOPERATIVE_QUEUES to query agent support for this
 type.*/
pub const HSA_QUEUE_TYPE_COOPERATIVE: hsa_queue_type_t = 2;
/** @brief Queue type. Intended to be used for dynamic queue protocol
 determination.*/
pub type hsa_queue_type_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_queue_type_t constants.
pub type hsa_queue_type32_t = u32;
/// Queue supports kernel dispatch packets.
pub const HSA_QUEUE_FEATURE_KERNEL_DISPATCH: hsa_queue_feature_t = 1;
/// Queue supports agent dispatch packets.
pub const HSA_QUEUE_FEATURE_AGENT_DISPATCH: hsa_queue_feature_t = 2;
/// @brief Queue features.
pub type hsa_queue_feature_t = ::core::ffi::c_uint;
/** @brief User mode queue.

 @details The queue structure is read-only and allocated by the HSA runtime,
 but agents can directly modify the contents of the buffer pointed by @a
 base_address, or use HSA runtime APIs to access the doorbell signal.
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_queue_s {
    /// Queue type.
    pub type_: hsa_queue_type32_t,
    /** Queue features mask. This is a bit-field of ::hsa_queue_feature_t
 values. Applications should ignore any unknown set bits.*/
    pub features: u32,
    pub base_address: *mut ::core::ffi::c_void,
    /** Signal object used by the application to indicate the ID of a packet that
 is ready to be processed. The HSA runtime manages the doorbell signal. If
 the application tries to replace or destroy this signal, the behavior is
 undefined.

 If @a type is ::HSA_QUEUE_TYPE_SINGLE, the doorbell signal value must be
 updated in a monotonically increasing fashion. If @a type is
 ::HSA_QUEUE_TYPE_MULTI, the doorbell signal value can be updated with any
 value.*/
    pub doorbell_signal: hsa_signal_t,
    /// Maximum number of packets the queue can hold. Must be a power of 2.
    pub size: u32,
    /// Reserved. Must be 0.
    pub reserved1: u32,
    /// Queue identifier, which is unique over the lifetime of the application.
    pub id: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_queue_s"][::core::mem::size_of::<hsa_queue_s>() - 40usize];
    ["Alignment of hsa_queue_s"][::core::mem::align_of::<hsa_queue_s>() - 8usize];
    [
        "Offset of field: hsa_queue_s::type_",
    ][::core::mem::offset_of!(hsa_queue_s, type_) - 0usize];
    [
        "Offset of field: hsa_queue_s::features",
    ][::core::mem::offset_of!(hsa_queue_s, features) - 4usize];
    [
        "Offset of field: hsa_queue_s::base_address",
    ][::core::mem::offset_of!(hsa_queue_s, base_address) - 8usize];
    [
        "Offset of field: hsa_queue_s::doorbell_signal",
    ][::core::mem::offset_of!(hsa_queue_s, doorbell_signal) - 16usize];
    [
        "Offset of field: hsa_queue_s::size",
    ][::core::mem::offset_of!(hsa_queue_s, size) - 24usize];
    [
        "Offset of field: hsa_queue_s::reserved1",
    ][::core::mem::offset_of!(hsa_queue_s, reserved1) - 28usize];
    [
        "Offset of field: hsa_queue_s::id",
    ][::core::mem::offset_of!(hsa_queue_s, id) - 32usize];
};
impl Default for hsa_queue_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** @brief User mode queue.

 @details The queue structure is read-only and allocated by the HSA runtime,
 but agents can directly modify the contents of the buffer pointed by @a
 base_address, or use HSA runtime APIs to access the doorbell signal.
*/
pub type hsa_queue_t = hsa_queue_s;
/// Vendor-specific packet.
pub const HSA_PACKET_TYPE_VENDOR_SPECIFIC: hsa_packet_type_t = 0;
/** The packet has been processed in the past, but has not been reassigned to
 the packet processor. A packet processor must not process a packet of this
 type. All queues support this packet type.*/
pub const HSA_PACKET_TYPE_INVALID: hsa_packet_type_t = 1;
/** Packet used by agents for dispatching jobs to kernel agents. Not all
 queues support packets of this type (see ::hsa_queue_feature_t).*/
pub const HSA_PACKET_TYPE_KERNEL_DISPATCH: hsa_packet_type_t = 2;
/** Packet used by agents to delay processing of subsequent packets, and to
 express complex dependencies between multiple packets. All queues support
 this packet type.*/
pub const HSA_PACKET_TYPE_BARRIER_AND: hsa_packet_type_t = 3;
/** Packet used by agents for dispatching jobs to agents.  Not all
 queues support packets of this type (see ::hsa_queue_feature_t).*/
pub const HSA_PACKET_TYPE_AGENT_DISPATCH: hsa_packet_type_t = 4;
/** Packet used by agents to delay processing of subsequent packets, and to
 express complex dependencies between multiple packets. All queues support
 this packet type.*/
pub const HSA_PACKET_TYPE_BARRIER_OR: hsa_packet_type_t = 5;
/// @brief Packet type.
pub type hsa_packet_type_t = ::core::ffi::c_uint;
/** No scope (no fence is applied). The packet relies on external fences to
 ensure visibility of memory updates.*/
pub const HSA_FENCE_SCOPE_NONE: hsa_fence_scope_t = 0;
/// The fence is applied with agent scope for the global segment.
pub const HSA_FENCE_SCOPE_AGENT: hsa_fence_scope_t = 1;
/** The fence is applied across both agent and system scope for the global
 segment.*/
pub const HSA_FENCE_SCOPE_SYSTEM: hsa_fence_scope_t = 2;
/// @brief Scope of the memory fence operation associated with a packet.
pub type hsa_fence_scope_t = ::core::ffi::c_uint;
/** Packet type. The value of this sub-field must be one of
 ::hsa_packet_type_t. If the type is ::HSA_PACKET_TYPE_VENDOR_SPECIFIC, the
 packet layout is vendor-specific.*/
pub const HSA_PACKET_HEADER_TYPE: hsa_packet_header_t = 0;
/** Barrier bit. If the barrier bit is set, the processing of the current
 packet only launches when all preceding packets (within the same queue) are
 complete.*/
pub const HSA_PACKET_HEADER_BARRIER: hsa_packet_header_t = 8;
/** Acquire fence scope. The value of this sub-field determines the scope and
 type of the memory fence operation applied before the packet enters the
 active phase. An acquire fence ensures that any subsequent global segment
 or image loads by any unit of execution that belongs to a dispatch that has
 not yet entered the active phase on any queue of the same kernel agent,
 sees any data previously released at the scopes specified by the acquire
 fence. The value of this sub-field must be one of ::hsa_fence_scope_t.*/
pub const HSA_PACKET_HEADER_SCACQUIRE_FENCE_SCOPE: hsa_packet_header_t = 9;
/// @deprecated Renamed as ::HSA_PACKET_HEADER_SCACQUIRE_FENCE_SCOPE.
pub const HSA_PACKET_HEADER_ACQUIRE_FENCE_SCOPE: hsa_packet_header_t = 9;
/** Release fence scope, The value of this sub-field determines the scope and
 type of the memory fence operation applied after kernel completion but
 before the packet is completed. A release fence makes any global segment or
 image data that was stored by any unit of execution that belonged to a
 dispatch that has completed the active phase on any queue of the same
 kernel agent visible in all the scopes specified by the release fence. The
 value of this sub-field must be one of ::hsa_fence_scope_t.*/
pub const HSA_PACKET_HEADER_SCRELEASE_FENCE_SCOPE: hsa_packet_header_t = 11;
/// @deprecated Renamed as ::HSA_PACKET_HEADER_SCRELEASE_FENCE_SCOPE.
pub const HSA_PACKET_HEADER_RELEASE_FENCE_SCOPE: hsa_packet_header_t = 11;
/** @brief Sub-fields of the @a header field that is present in any AQL
 packet. The offset (with respect to the address of @a header) of a sub-field
 is identical to its enumeration constant. The width of each sub-field is
 determined by the corresponding value in ::hsa_packet_header_width_t. The
 offset and the width are expressed in bits.*/
pub type hsa_packet_header_t = ::core::ffi::c_uint;
pub const HSA_PACKET_HEADER_WIDTH_TYPE: hsa_packet_header_width_t = 8;
pub const HSA_PACKET_HEADER_WIDTH_BARRIER: hsa_packet_header_width_t = 1;
pub const HSA_PACKET_HEADER_WIDTH_SCACQUIRE_FENCE_SCOPE: hsa_packet_header_width_t = 2;
/// @deprecated Use HSA_PACKET_HEADER_WIDTH_SCACQUIRE_FENCE_SCOPE.
pub const HSA_PACKET_HEADER_WIDTH_ACQUIRE_FENCE_SCOPE: hsa_packet_header_width_t = 2;
/// @deprecated Use HSA_PACKET_HEADER_WIDTH_SCACQUIRE_FENCE_SCOPE.
pub const HSA_PACKET_HEADER_WIDTH_SCRELEASE_FENCE_SCOPE: hsa_packet_header_width_t = 2;
/// @deprecated Use HSA_PACKET_HEADER_WIDTH_SCRELEASE_FENCE_SCOPE.
pub const HSA_PACKET_HEADER_WIDTH_RELEASE_FENCE_SCOPE: hsa_packet_header_width_t = 2;
/// @brief Width (in bits) of the sub-fields in ::hsa_packet_header_t.
pub type hsa_packet_header_width_t = ::core::ffi::c_uint;
/** Number of dimensions of the grid. Valid values are 1, 2, or 3.
*/
pub const HSA_KERNEL_DISPATCH_PACKET_SETUP_DIMENSIONS: hsa_kernel_dispatch_packet_setup_t = 0;
/** @brief Sub-fields of the kernel dispatch packet @a setup field. The offset
 (with respect to the address of @a setup) of a sub-field is identical to its
 enumeration constant. The width of each sub-field is determined by the
 corresponding value in ::hsa_kernel_dispatch_packet_setup_width_t. The
 offset and the width are expressed in bits.*/
pub type hsa_kernel_dispatch_packet_setup_t = ::core::ffi::c_uint;
pub const HSA_KERNEL_DISPATCH_PACKET_SETUP_WIDTH_DIMENSIONS: hsa_kernel_dispatch_packet_setup_width_t = 2;
/** @brief Width (in bits) of the sub-fields in
 ::hsa_kernel_dispatch_packet_setup_t.*/
pub type hsa_kernel_dispatch_packet_setup_width_t = ::core::ffi::c_uint;
/// @brief AQL kernel dispatch packet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsa_kernel_dispatch_packet_s {
    pub __bindgen_anon_1: hsa_kernel_dispatch_packet_s__bindgen_ty_1,
    /// X dimension of work-group, in work-items. Must be greater than 0.
    pub workgroup_size_x: u16,
    /** Y dimension of work-group, in work-items. Must be greater than
 0. If the grid has 1 dimension, the only valid value is 1.*/
    pub workgroup_size_y: u16,
    /** Z dimension of work-group, in work-items. Must be greater than
 0. If the grid has 1 or 2 dimensions, the only valid value is 1.*/
    pub workgroup_size_z: u16,
    /// Reserved. Must be 0.
    pub reserved0: u16,
    /** X dimension of grid, in work-items. Must be greater than 0. Must
 not be smaller than @a workgroup_size_x.*/
    pub grid_size_x: u32,
    /** Y dimension of grid, in work-items. Must be greater than 0. If the grid has
 1 dimension, the only valid value is 1. Must not be smaller than @a
 workgroup_size_y.*/
    pub grid_size_y: u32,
    /** Z dimension of grid, in work-items. Must be greater than 0. If the grid has
 1 or 2 dimensions, the only valid value is 1. Must not be smaller than @a
 workgroup_size_z.*/
    pub grid_size_z: u32,
    /// Size in bytes of private memory allocation request (per work-item).
    pub private_segment_size: u32,
    /** Size in bytes of group memory allocation request (per work-group). Must not
 be less than the sum of the group memory used by the kernel (and the
 functions it calls directly or indirectly) and the dynamically allocated
 group segment variables.*/
    pub group_segment_size: u32,
    /** Opaque handle to a code object that includes an implementation-defined
 executable code for the kernel.*/
    pub kernel_object: u64,
    pub kernarg_address: *mut ::core::ffi::c_void,
    /// Reserved. Must be 0.
    pub reserved2: u64,
    /** Signal used to indicate completion of the job. The application can use the
 special signal handle 0 to indicate that no signal is used.*/
    pub completion_signal: hsa_signal_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hsa_kernel_dispatch_packet_s__bindgen_ty_1 {
    pub __bindgen_anon_1: hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1,
    pub full_header: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1 {
    /** Packet header. Used to configure multiple packet parameters such as the
 packet type. The parameters are described by ::hsa_packet_header_t.*/
    pub header: u16,
    /** Dispatch setup parameters. Used to configure kernel dispatch parameters
 such as the number of dimensions in the grid. The parameters are described
 by ::hsa_kernel_dispatch_packet_setup_t.*/
    pub setup: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1",
    ][::core::mem::size_of::<hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1>()
        - 4usize];
    [
        "Alignment of hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1",
    ][::core::mem::align_of::<hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1>()
        - 2usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1::header",
    ][::core::mem::offset_of!(
        hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1, header
    ) - 0usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1::setup",
    ][::core::mem::offset_of!(
        hsa_kernel_dispatch_packet_s__bindgen_ty_1__bindgen_ty_1, setup
    ) - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_kernel_dispatch_packet_s__bindgen_ty_1",
    ][::core::mem::size_of::<hsa_kernel_dispatch_packet_s__bindgen_ty_1>() - 4usize];
    [
        "Alignment of hsa_kernel_dispatch_packet_s__bindgen_ty_1",
    ][::core::mem::align_of::<hsa_kernel_dispatch_packet_s__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s__bindgen_ty_1::full_header",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s__bindgen_ty_1, full_header)
        - 0usize];
};
impl Default for hsa_kernel_dispatch_packet_s__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_kernel_dispatch_packet_s",
    ][::core::mem::size_of::<hsa_kernel_dispatch_packet_s>() - 64usize];
    [
        "Alignment of hsa_kernel_dispatch_packet_s",
    ][::core::mem::align_of::<hsa_kernel_dispatch_packet_s>() - 8usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::workgroup_size_x",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, workgroup_size_x) - 4usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::workgroup_size_y",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, workgroup_size_y) - 6usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::workgroup_size_z",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, workgroup_size_z) - 8usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::reserved0",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, reserved0) - 10usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::grid_size_x",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, grid_size_x) - 12usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::grid_size_y",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, grid_size_y) - 16usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::grid_size_z",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, grid_size_z) - 20usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::private_segment_size",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, private_segment_size)
        - 24usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::group_segment_size",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, group_segment_size)
        - 28usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::kernel_object",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, kernel_object) - 32usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::kernarg_address",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, kernarg_address) - 40usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::reserved2",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, reserved2) - 48usize];
    [
        "Offset of field: hsa_kernel_dispatch_packet_s::completion_signal",
    ][::core::mem::offset_of!(hsa_kernel_dispatch_packet_s, completion_signal)
        - 56usize];
};
impl Default for hsa_kernel_dispatch_packet_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// @brief AQL kernel dispatch packet
pub type hsa_kernel_dispatch_packet_t = hsa_kernel_dispatch_packet_s;
/// @brief Agent dispatch packet.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_agent_dispatch_packet_s {
    /** Packet header. Used to configure multiple packet parameters such as the
 packet type. The parameters are described by ::hsa_packet_header_t.*/
    pub header: u16,
    /// Application-defined function to be performed by the destination agent.
    pub type_: u16,
    /// Reserved. Must be 0.
    pub reserved0: u32,
    pub return_address: *mut ::core::ffi::c_void,
    /// Function arguments.
    pub arg: [u64; 4usize],
    /// Reserved. Must be 0.
    pub reserved2: u64,
    /** Signal used to indicate completion of the job. The application can use the
 special signal handle 0 to indicate that no signal is used.*/
    pub completion_signal: hsa_signal_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_agent_dispatch_packet_s",
    ][::core::mem::size_of::<hsa_agent_dispatch_packet_s>() - 64usize];
    [
        "Alignment of hsa_agent_dispatch_packet_s",
    ][::core::mem::align_of::<hsa_agent_dispatch_packet_s>() - 8usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::header",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, header) - 0usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::type_",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, type_) - 2usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::reserved0",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, reserved0) - 4usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::return_address",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, return_address) - 8usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::arg",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, arg) - 16usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::reserved2",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, reserved2) - 48usize];
    [
        "Offset of field: hsa_agent_dispatch_packet_s::completion_signal",
    ][::core::mem::offset_of!(hsa_agent_dispatch_packet_s, completion_signal) - 56usize];
};
impl Default for hsa_agent_dispatch_packet_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// @brief Agent dispatch packet.
pub type hsa_agent_dispatch_packet_t = hsa_agent_dispatch_packet_s;
/// @brief Barrier-AND packet.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_barrier_and_packet_s {
    /** Packet header. Used to configure multiple packet parameters such as the
 packet type. The parameters are described by ::hsa_packet_header_t.*/
    pub header: u16,
    /// Reserved. Must be 0.
    pub reserved0: u16,
    /// Reserved. Must be 0.
    pub reserved1: u32,
    /** Array of dependent signal objects. Signals with a handle value of 0 are
 allowed and are interpreted by the packet processor as satisfied
 dependencies.*/
    pub dep_signal: [hsa_signal_t; 5usize],
    /// Reserved. Must be 0.
    pub reserved2: u64,
    /** Signal used to indicate completion of the job. The application can use the
 special signal handle 0 to indicate that no signal is used.*/
    pub completion_signal: hsa_signal_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_barrier_and_packet_s",
    ][::core::mem::size_of::<hsa_barrier_and_packet_s>() - 64usize];
    [
        "Alignment of hsa_barrier_and_packet_s",
    ][::core::mem::align_of::<hsa_barrier_and_packet_s>() - 8usize];
    [
        "Offset of field: hsa_barrier_and_packet_s::header",
    ][::core::mem::offset_of!(hsa_barrier_and_packet_s, header) - 0usize];
    [
        "Offset of field: hsa_barrier_and_packet_s::reserved0",
    ][::core::mem::offset_of!(hsa_barrier_and_packet_s, reserved0) - 2usize];
    [
        "Offset of field: hsa_barrier_and_packet_s::reserved1",
    ][::core::mem::offset_of!(hsa_barrier_and_packet_s, reserved1) - 4usize];
    [
        "Offset of field: hsa_barrier_and_packet_s::dep_signal",
    ][::core::mem::offset_of!(hsa_barrier_and_packet_s, dep_signal) - 8usize];
    [
        "Offset of field: hsa_barrier_and_packet_s::reserved2",
    ][::core::mem::offset_of!(hsa_barrier_and_packet_s, reserved2) - 48usize];
    [
        "Offset of field: hsa_barrier_and_packet_s::completion_signal",
    ][::core::mem::offset_of!(hsa_barrier_and_packet_s, completion_signal) - 56usize];
};
/// @brief Barrier-AND packet.
pub type hsa_barrier_and_packet_t = hsa_barrier_and_packet_s;
/// @brief Barrier-OR packet.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_barrier_or_packet_s {
    /** Packet header. Used to configure multiple packet parameters such as the
 packet type. The parameters are described by ::hsa_packet_header_t.*/
    pub header: u16,
    /// Reserved. Must be 0.
    pub reserved0: u16,
    /// Reserved. Must be 0.
    pub reserved1: u32,
    /** Array of dependent signal objects. Signals with a handle value of 0 are
 allowed and are interpreted by the packet processor as dependencies not
 satisfied.*/
    pub dep_signal: [hsa_signal_t; 5usize],
    /// Reserved. Must be 0.
    pub reserved2: u64,
    /** Signal used to indicate completion of the job. The application can use the
 special signal handle 0 to indicate that no signal is used.*/
    pub completion_signal: hsa_signal_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_barrier_or_packet_s",
    ][::core::mem::size_of::<hsa_barrier_or_packet_s>() - 64usize];
    [
        "Alignment of hsa_barrier_or_packet_s",
    ][::core::mem::align_of::<hsa_barrier_or_packet_s>() - 8usize];
    [
        "Offset of field: hsa_barrier_or_packet_s::header",
    ][::core::mem::offset_of!(hsa_barrier_or_packet_s, header) - 0usize];
    [
        "Offset of field: hsa_barrier_or_packet_s::reserved0",
    ][::core::mem::offset_of!(hsa_barrier_or_packet_s, reserved0) - 2usize];
    [
        "Offset of field: hsa_barrier_or_packet_s::reserved1",
    ][::core::mem::offset_of!(hsa_barrier_or_packet_s, reserved1) - 4usize];
    [
        "Offset of field: hsa_barrier_or_packet_s::dep_signal",
    ][::core::mem::offset_of!(hsa_barrier_or_packet_s, dep_signal) - 8usize];
    [
        "Offset of field: hsa_barrier_or_packet_s::reserved2",
    ][::core::mem::offset_of!(hsa_barrier_or_packet_s, reserved2) - 48usize];
    [
        "Offset of field: hsa_barrier_or_packet_s::completion_signal",
    ][::core::mem::offset_of!(hsa_barrier_or_packet_s, completion_signal) - 56usize];
};
/// @brief Barrier-OR packet.
pub type hsa_barrier_or_packet_t = hsa_barrier_or_packet_s;
/// Global segment. Used to hold data that is shared by all agents.
pub const HSA_REGION_SEGMENT_GLOBAL: hsa_region_segment_t = 0;
/** Read-only segment. Used to hold data that remains constant during the
 execution of a kernel.*/
pub const HSA_REGION_SEGMENT_READONLY: hsa_region_segment_t = 1;
/// Private segment. Used to hold data that is local to a single work-item.
pub const HSA_REGION_SEGMENT_PRIVATE: hsa_region_segment_t = 2;
/** Group segment. Used to hold data that is shared by the work-items of a
 work-group.*/
pub const HSA_REGION_SEGMENT_GROUP: hsa_region_segment_t = 3;
/// Kernarg segment. Used to store kernel arguments.
pub const HSA_REGION_SEGMENT_KERNARG: hsa_region_segment_t = 4;
/// @brief Memory segments associated with a region.
pub type hsa_region_segment_t = ::core::ffi::c_uint;
/** The application can use memory in the region to store kernel arguments, and
 provide the values for the kernarg segment of a kernel dispatch. If this
 flag is set, then ::HSA_REGION_GLOBAL_FLAG_FINE_GRAINED must be set.*/
pub const HSA_REGION_GLOBAL_FLAG_KERNARG: hsa_region_global_flag_t = 1;
/** Updates to memory in this region are immediately visible to all the
 agents under the terms of the HSA memory model. If this
 flag is set, then ::HSA_REGION_GLOBAL_FLAG_COARSE_GRAINED must not be set.*/
pub const HSA_REGION_GLOBAL_FLAG_FINE_GRAINED: hsa_region_global_flag_t = 2;
/** Updates to memory in this region can be performed by a single agent at
 a time. If a different agent in the system is allowed to access the
 region, the application must explicitely invoke ::hsa_memory_assign_agent
 in order to transfer ownership to that agent for a particular buffer.*/
pub const HSA_REGION_GLOBAL_FLAG_COARSE_GRAINED: hsa_region_global_flag_t = 4;
/** Updates to memory in this region have extended scope, where the device-scope atomics
 to this memory type act as system-scope with respect to all variables located in
 memory regions of this type.
 Note: On non-compliant systems, the application may still be responsible for performing
 device-specific actions necessary to achieve system-scope coherence.*/
pub const HSA_REGION_GLOBAL_FLAG_EXTENDED_SCOPE_FINE_GRAINED: hsa_region_global_flag_t = 8;
/// @brief Global region flags.
pub type hsa_region_global_flag_t = ::core::ffi::c_uint;
/** Segment where memory in the region can be used. The type of this
 attribute is ::hsa_region_segment_t.*/
pub const HSA_REGION_INFO_SEGMENT: hsa_region_info_t = 0;
/** Flag mask. The value of this attribute is undefined if the value of
 ::HSA_REGION_INFO_SEGMENT is not ::HSA_REGION_SEGMENT_GLOBAL. The type of
 this attribute is uint32_t, a bit-field of ::hsa_region_global_flag_t
 values.*/
pub const HSA_REGION_INFO_GLOBAL_FLAGS: hsa_region_info_t = 1;
/// Size of this region, in bytes. The type of this attribute is size_t.
pub const HSA_REGION_INFO_SIZE: hsa_region_info_t = 2;
/** Maximum allocation size in this region, in bytes. Must not exceed the value
 of ::HSA_REGION_INFO_SIZE. The type of this attribute is size_t.

 If the region is in the global or readonly segments, this is the maximum
 size that the application can pass to ::hsa_memory_allocate.

 If the region is in the group segment, this is the maximum size (per
 work-group) that can be requested for a given kernel dispatch. If the
 region is in the private segment, this is the maximum size (per work-item)
 that can be requested for a specific kernel dispatch, and must be at least
 256 bytes.*/
pub const HSA_REGION_INFO_ALLOC_MAX_SIZE: hsa_region_info_t = 4;
/** Maximum size (per work-group) of private memory that can be requested for a
 specific kernel dispatch. Must be at least 65536 bytes. The type of this
 attribute is uint32_t. The value of this attribute is undefined if the
 region is not in the private segment.*/
pub const HSA_REGION_INFO_ALLOC_MAX_PRIVATE_WORKGROUP_SIZE: hsa_region_info_t = 8;
/** Indicates whether memory in this region can be allocated using
 ::hsa_memory_allocate. The type of this attribute is bool.

 The value of this flag is always false for regions in the group and private
 segments.*/
pub const HSA_REGION_INFO_RUNTIME_ALLOC_ALLOWED: hsa_region_info_t = 5;
/** Allocation granularity of buffers allocated by ::hsa_memory_allocate in
 this region. The size of a buffer allocated in this region is a multiple of
 the value of this attribute. The value of this attribute is only defined if
 ::HSA_REGION_INFO_RUNTIME_ALLOC_ALLOWED is true for this region. The type
 of this attribute is size_t.*/
pub const HSA_REGION_INFO_RUNTIME_ALLOC_GRANULE: hsa_region_info_t = 6;
/** Alignment of buffers allocated by ::hsa_memory_allocate in this region. The
 value of this attribute is only defined if
 ::HSA_REGION_INFO_RUNTIME_ALLOC_ALLOWED is true for this region, and must be
 a power of 2. The type of this attribute is size_t.*/
pub const HSA_REGION_INFO_RUNTIME_ALLOC_ALIGNMENT: hsa_region_info_t = 7;
pub type hsa_region_info_t = ::core::ffi::c_uint;
/// @brief Instruction set architecture.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_isa_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_isa_s"][::core::mem::size_of::<hsa_isa_s>() - 8usize];
    ["Alignment of hsa_isa_s"][::core::mem::align_of::<hsa_isa_s>() - 8usize];
    [
        "Offset of field: hsa_isa_s::handle",
    ][::core::mem::offset_of!(hsa_isa_s, handle) - 0usize];
};
/// @brief Instruction set architecture.
pub type hsa_isa_t = hsa_isa_s;
/** The length of the ISA name in bytes, not including the NUL terminator. The
 type of this attribute is uint32_t.*/
pub const HSA_ISA_INFO_NAME_LENGTH: hsa_isa_info_t = 0;
/** Human-readable description.  The type of this attribute is character array
 with the length equal to the value of ::HSA_ISA_INFO_NAME_LENGTH attribute.*/
pub const HSA_ISA_INFO_NAME: hsa_isa_info_t = 1;
/** @deprecated

 Number of call conventions supported by the instruction set architecture.
 Must be greater than zero. The type of this attribute is uint32_t.*/
pub const HSA_ISA_INFO_CALL_CONVENTION_COUNT: hsa_isa_info_t = 2;
/** @deprecated

 Number of work-items in a wavefront for a given call convention. Must be a
 power of 2 in the range [1,256]. The type of this attribute is uint32_t.*/
pub const HSA_ISA_INFO_CALL_CONVENTION_INFO_WAVEFRONT_SIZE: hsa_isa_info_t = 3;
/** @deprecated

 Number of wavefronts per compute unit for a given call convention. In
 practice, other factors (for example, the amount of group memory used by a
 work-group) may further limit the number of wavefronts per compute
 unit. The type of this attribute is uint32_t.*/
pub const HSA_ISA_INFO_CALL_CONVENTION_INFO_WAVEFRONTS_PER_COMPUTE_UNIT: hsa_isa_info_t = 4;
/** Machine models supported by the instruction set architecture. The type of
 this attribute is a bool[2]. If the ISA supports the small machine model,
 the element at index ::HSA_MACHINE_MODEL_SMALL is true. If the ISA supports
 the large model, the element at index ::HSA_MACHINE_MODEL_LARGE is true.*/
pub const HSA_ISA_INFO_MACHINE_MODELS: hsa_isa_info_t = 5;
/** Profiles supported by the instruction set architecture. The type of this
 attribute is a bool[2]. If the ISA supports the base profile, the element
 at index ::HSA_PROFILE_BASE is true. If the ISA supports the full profile,
 the element at index ::HSA_PROFILE_FULL is true.*/
pub const HSA_ISA_INFO_PROFILES: hsa_isa_info_t = 6;
/** Default floating-point rounding modes supported by the instruction set
 architecture. The type of this attribute is a bool[3]. The value at a given
 index is true if the corresponding rounding mode in
 ::hsa_default_float_rounding_mode_t is supported. At least one default mode
 has to be supported.

 If the default mode is supported, then
 ::HSA_ISA_INFO_BASE_PROFILE_DEFAULT_FLOAT_ROUNDING_MODES must report that
 both the zero and the near roundings modes are supported.*/
pub const HSA_ISA_INFO_DEFAULT_FLOAT_ROUNDING_MODES: hsa_isa_info_t = 7;
/** Default floating-point rounding modes supported by the instruction set
 architecture in the Base profile. The type of this attribute is a
 bool[3]. The value at a given index is true if the corresponding rounding
 mode in ::hsa_default_float_rounding_mode_t is supported. The value at
 index HSA_DEFAULT_FLOAT_ROUNDING_MODE_DEFAULT must be false.  At least one
 of the values at indexes ::HSA_DEFAULT_FLOAT_ROUNDING_MODE_ZERO or
 HSA_DEFAULT_FLOAT_ROUNDING_MODE_NEAR must be true.*/
pub const HSA_ISA_INFO_BASE_PROFILE_DEFAULT_FLOAT_ROUNDING_MODES: hsa_isa_info_t = 8;
/** Flag indicating that the f16 HSAIL operation is at least as fast as the
 f32 operation in the instruction set architecture. The type of this
 attribute is bool.*/
pub const HSA_ISA_INFO_FAST_F16_OPERATION: hsa_isa_info_t = 9;
/** Maximum number of work-items of each dimension of a work-group.  Each
 maximum must be greater than 0. No maximum can exceed the value of
 ::HSA_ISA_INFO_WORKGROUP_MAX_SIZE. The type of this attribute is
 uint16_t[3].*/
pub const HSA_ISA_INFO_WORKGROUP_MAX_DIM: hsa_isa_info_t = 12;
/** Maximum total number of work-items in a work-group. The type
 of this attribute is uint32_t.*/
pub const HSA_ISA_INFO_WORKGROUP_MAX_SIZE: hsa_isa_info_t = 13;
/** Maximum number of work-items of each dimension of a grid. Each maximum must
 be greater than 0, and must not be smaller than the corresponding value in
 ::HSA_ISA_INFO_WORKGROUP_MAX_DIM. No maximum can exceed the value of
 ::HSA_ISA_INFO_GRID_MAX_SIZE. The type of this attribute is
 ::hsa_dim3_t.*/
pub const HSA_ISA_INFO_GRID_MAX_DIM: hsa_isa_info_t = 14;
/** Maximum total number of work-items in a grid. The type of this
 attribute is uint64_t.*/
pub const HSA_ISA_INFO_GRID_MAX_SIZE: hsa_isa_info_t = 16;
/** Maximum number of fbarriers per work-group. Must be at least 32. The
 type of this attribute is uint32_t.*/
pub const HSA_ISA_INFO_FBARRIER_MAX_SIZE: hsa_isa_info_t = 17;
/// @brief Instruction set architecture attributes.
pub type hsa_isa_info_t = ::core::ffi::c_uint;
/// 16-bit floating-point type.
pub const HSA_FP_TYPE_16: hsa_fp_type_t = 1;
/// 32-bit floating-point type.
pub const HSA_FP_TYPE_32: hsa_fp_type_t = 2;
/// 64-bit floating-point type.
pub const HSA_FP_TYPE_64: hsa_fp_type_t = 4;
/// @brief Floating-point types.
pub type hsa_fp_type_t = ::core::ffi::c_uint;
/// Flush to zero.
pub const HSA_FLUSH_MODE_FTZ: hsa_flush_mode_t = 1;
/// Do not flush to zero.
pub const HSA_FLUSH_MODE_NON_FTZ: hsa_flush_mode_t = 2;
/// @brief Flush to zero modes.
pub type hsa_flush_mode_t = ::core::ffi::c_uint;
/// Single round method.
pub const HSA_ROUND_METHOD_SINGLE: hsa_round_method_t = 1;
/// Double round method.
pub const HSA_ROUND_METHOD_DOUBLE: hsa_round_method_t = 2;
/// @brief Round methods.
pub type hsa_round_method_t = ::core::ffi::c_uint;
/// @brief Wavefront handle
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_wavefront_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_wavefront_s"][::core::mem::size_of::<hsa_wavefront_s>() - 8usize];
    [
        "Alignment of hsa_wavefront_s",
    ][::core::mem::align_of::<hsa_wavefront_s>() - 8usize];
    [
        "Offset of field: hsa_wavefront_s::handle",
    ][::core::mem::offset_of!(hsa_wavefront_s, handle) - 0usize];
};
/// @brief Wavefront handle
pub type hsa_wavefront_t = hsa_wavefront_s;
/** Number of work-items in the wavefront. Must be a power of 2 in the range
 [1,256]. The type of this attribute is uint32_t.*/
pub const HSA_WAVEFRONT_INFO_SIZE: hsa_wavefront_info_t = 0;
/// @brief Wavefront attributes.
pub type hsa_wavefront_info_t = ::core::ffi::c_uint;
/** @brief Code object reader handle. A code object reader is used to
 load a code object from file (when created using
 ::hsa_code_object_reader_create_from_file), or from memory (if created using
 ::hsa_code_object_reader_create_from_memory).*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_code_object_reader_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_code_object_reader_s",
    ][::core::mem::size_of::<hsa_code_object_reader_s>() - 8usize];
    [
        "Alignment of hsa_code_object_reader_s",
    ][::core::mem::align_of::<hsa_code_object_reader_s>() - 8usize];
    [
        "Offset of field: hsa_code_object_reader_s::handle",
    ][::core::mem::offset_of!(hsa_code_object_reader_s, handle) - 0usize];
};
/** @brief Code object reader handle. A code object reader is used to
 load a code object from file (when created using
 ::hsa_code_object_reader_create_from_file), or from memory (if created using
 ::hsa_code_object_reader_create_from_memory).*/
pub type hsa_code_object_reader_t = hsa_code_object_reader_s;
/** @brief Struct containing an opaque handle to an executable, which contains
 ISA for finalized kernels and indirect functions together with the allocated
 global or readonly segment variables they reference.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_executable_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_executable_s"][::core::mem::size_of::<hsa_executable_s>() - 8usize];
    [
        "Alignment of hsa_executable_s",
    ][::core::mem::align_of::<hsa_executable_s>() - 8usize];
    [
        "Offset of field: hsa_executable_s::handle",
    ][::core::mem::offset_of!(hsa_executable_s, handle) - 0usize];
};
/** @brief Struct containing an opaque handle to an executable, which contains
 ISA for finalized kernels and indirect functions together with the allocated
 global or readonly segment variables they reference.*/
pub type hsa_executable_t = hsa_executable_s;
/** Executable state, which allows the user to load code objects and define
 external variables. Variable addresses, kernel code handles, and
 indirect function code handles are not available in query operations until
 the executable is frozen (zero always returned).*/
pub const HSA_EXECUTABLE_STATE_UNFROZEN: hsa_executable_state_t = 0;
/** Executable state, which allows the user to query variable addresses,
 kernel code handles, and indirect function code handles using query
 operations. Loading new code objects, as well as defining external
 variables, is not allowed in this state.*/
pub const HSA_EXECUTABLE_STATE_FROZEN: hsa_executable_state_t = 1;
/// @brief Executable state.
pub type hsa_executable_state_t = ::core::ffi::c_uint;
/// @brief Loaded code object handle.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_loaded_code_object_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_loaded_code_object_s",
    ][::core::mem::size_of::<hsa_loaded_code_object_s>() - 8usize];
    [
        "Alignment of hsa_loaded_code_object_s",
    ][::core::mem::align_of::<hsa_loaded_code_object_s>() - 8usize];
    [
        "Offset of field: hsa_loaded_code_object_s::handle",
    ][::core::mem::offset_of!(hsa_loaded_code_object_s, handle) - 0usize];
};
/// @brief Loaded code object handle.
pub type hsa_loaded_code_object_t = hsa_loaded_code_object_s;
/** Profile this executable is created for. The type of this attribute is
 ::hsa_profile_t.*/
pub const HSA_EXECUTABLE_INFO_PROFILE: hsa_executable_info_t = 1;
/// Executable state. The type of this attribute is ::hsa_executable_state_t.
pub const HSA_EXECUTABLE_INFO_STATE: hsa_executable_info_t = 2;
/** Default floating-point rounding mode specified when executable was created.
 The type of this attribute is ::hsa_default_float_rounding_mode_t.*/
pub const HSA_EXECUTABLE_INFO_DEFAULT_FLOAT_ROUNDING_MODE: hsa_executable_info_t = 3;
/// @brief Executable attributes.
pub type hsa_executable_info_t = ::core::ffi::c_uint;
/** @brief Executable symbol handle.

 The lifetime of an executable object symbol matches that of the executable
 associated with it. An operation on a symbol whose associated executable has
 been destroyed results in undefined behavior.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_executable_symbol_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_executable_symbol_s",
    ][::core::mem::size_of::<hsa_executable_symbol_s>() - 8usize];
    [
        "Alignment of hsa_executable_symbol_s",
    ][::core::mem::align_of::<hsa_executable_symbol_s>() - 8usize];
    [
        "Offset of field: hsa_executable_symbol_s::handle",
    ][::core::mem::offset_of!(hsa_executable_symbol_s, handle) - 0usize];
};
/** @brief Executable symbol handle.

 The lifetime of an executable object symbol matches that of the executable
 associated with it. An operation on a symbol whose associated executable has
 been destroyed results in undefined behavior.*/
pub type hsa_executable_symbol_t = hsa_executable_symbol_s;
/// Variable.
pub const HSA_SYMBOL_KIND_VARIABLE: hsa_symbol_kind_t = 0;
/// Kernel.
pub const HSA_SYMBOL_KIND_KERNEL: hsa_symbol_kind_t = 1;
/// Indirect function.
pub const HSA_SYMBOL_KIND_INDIRECT_FUNCTION: hsa_symbol_kind_t = 2;
/// @brief Symbol type.
pub type hsa_symbol_kind_t = ::core::ffi::c_uint;
/// Module linkage.
pub const HSA_SYMBOL_LINKAGE_MODULE: hsa_symbol_linkage_t = 0;
/// Program linkage.
pub const HSA_SYMBOL_LINKAGE_PROGRAM: hsa_symbol_linkage_t = 1;
/// @brief Linkage type of a symbol.
pub type hsa_symbol_linkage_t = ::core::ffi::c_uint;
/// Agent allocation.
pub const HSA_VARIABLE_ALLOCATION_AGENT: hsa_variable_allocation_t = 0;
/// Program allocation.
pub const HSA_VARIABLE_ALLOCATION_PROGRAM: hsa_variable_allocation_t = 1;
/// @brief Allocation type of a variable.
pub type hsa_variable_allocation_t = ::core::ffi::c_uint;
/// Global memory segment.
pub const HSA_VARIABLE_SEGMENT_GLOBAL: hsa_variable_segment_t = 0;
/// Readonly memory segment.
pub const HSA_VARIABLE_SEGMENT_READONLY: hsa_variable_segment_t = 1;
/// @brief Memory segment associated with a variable.
pub type hsa_variable_segment_t = ::core::ffi::c_uint;
/// The kind of the symbol. The type of this attribute is ::hsa_symbol_kind_t.
pub const HSA_EXECUTABLE_SYMBOL_INFO_TYPE: hsa_executable_symbol_info_t = 0;
/** The length of the symbol name in bytes, not including the NUL terminator.
 The type of this attribute is uint32_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_NAME_LENGTH: hsa_executable_symbol_info_t = 1;
/** The name of the symbol. The type of this attribute is character array with
 the length equal to the value of ::HSA_EXECUTABLE_SYMBOL_INFO_NAME_LENGTH
 attribute.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_NAME: hsa_executable_symbol_info_t = 2;
/** @deprecated

 The length of the module name in bytes (not including the NUL terminator)
 to which this symbol belongs if this symbol has module linkage, otherwise 0
 is returned. The type of this attribute is uint32_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_MODULE_NAME_LENGTH: hsa_executable_symbol_info_t = 3;
/** @deprecated

 The module name to which this symbol belongs if this symbol has module
 linkage, otherwise an empty string is returned. The type of this attribute
 is character array with the length equal to the value of
 ::HSA_EXECUTABLE_SYMBOL_INFO_MODULE_NAME_LENGTH attribute.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_MODULE_NAME: hsa_executable_symbol_info_t = 4;
/** @deprecated

 Agent associated with this symbol. If the symbol is a variable, the
 value of this attribute is only defined if
 ::HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_ALLOCATION is
 ::HSA_VARIABLE_ALLOCATION_AGENT. The type of this attribute is hsa_agent_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_AGENT: hsa_executable_symbol_info_t = 20;
/** The address of the variable. The value of this attribute is undefined if
 the symbol is not a variable. The type of this attribute is uint64_t.

 If executable's state is ::HSA_EXECUTABLE_STATE_UNFROZEN, then 0 is
 returned.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_ADDRESS: hsa_executable_symbol_info_t = 21;
/** The linkage kind of the symbol. The type of this attribute is
 ::hsa_symbol_linkage_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_LINKAGE: hsa_executable_symbol_info_t = 5;
/** Indicates whether the symbol corresponds to a definition. The type of this
 attribute is bool.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_IS_DEFINITION: hsa_executable_symbol_info_t = 17;
/** @deprecated

 The allocation kind of the variable. The value of this attribute is
 undefined if the symbol is not a variable.  The type of this attribute is
 ::hsa_variable_allocation_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_ALLOCATION: hsa_executable_symbol_info_t = 6;
/** @deprecated

 The segment kind of the variable. The value of this attribute is undefined
 if the symbol is not a variable. The type of this attribute is
 ::hsa_variable_segment_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_SEGMENT: hsa_executable_symbol_info_t = 7;
/** @deprecated

 Alignment of the symbol in memory. The value of this attribute is undefined
 if the symbol is not a variable. The type of this attribute is uint32_t.

 The current alignment of the variable in memory may be greater than the
 value specified in the source program variable declaration.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_ALIGNMENT: hsa_executable_symbol_info_t = 8;
/** @deprecated

 Size of the variable. The value of this attribute is undefined if
 the symbol is not a variable. The type of this attribute is uint32_t.

 A value of 0 is returned if the variable is an external variable and has an
 unknown dimension.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_SIZE: hsa_executable_symbol_info_t = 9;
/** @deprecated

 Indicates whether the variable is constant. The value of this attribute is
 undefined if the symbol is not a variable. The type of this attribute is
 bool.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_VARIABLE_IS_CONST: hsa_executable_symbol_info_t = 10;
/** Kernel object handle, used in the kernel dispatch packet. The value of this
 attribute is undefined if the symbol is not a kernel. The type of this
 attribute is uint64_t.

 If the state of the executable is ::HSA_EXECUTABLE_STATE_UNFROZEN, then 0
 is returned.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_OBJECT: hsa_executable_symbol_info_t = 22;
/** Size of kernarg segment memory that is required to hold the values of the
 kernel arguments, in bytes. Must be a multiple of 16. The value of this
 attribute is undefined if the symbol is not a kernel. The type of this
 attribute is uint32_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_KERNARG_SEGMENT_SIZE: hsa_executable_symbol_info_t = 11;
/** Alignment (in bytes) of the buffer used to pass arguments to the kernel,
 which is the maximum of 16 and the maximum alignment of any of the kernel
 arguments. The value of this attribute is undefined if the symbol is not a
 kernel. The type of this attribute is uint32_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_KERNARG_SEGMENT_ALIGNMENT: hsa_executable_symbol_info_t = 12;
/** Size of static group segment memory required by the kernel (per
 work-group), in bytes. The value of this attribute is undefined
 if the symbol is not a kernel. The type of this attribute is uint32_t.

 The reported amount does not include any dynamically allocated group
 segment memory that may be requested by the application when a kernel is
 dispatched.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_GROUP_SEGMENT_SIZE: hsa_executable_symbol_info_t = 13;
/** Size of static private, spill, and arg segment memory required by
 this kernel (per work-item), in bytes. The value of this attribute is
 undefined if the symbol is not a kernel. The type of this attribute is
 uint32_t.

 If the value of ::HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_DYNAMIC_CALLSTACK is
 true, the kernel may use more private memory than the reported value, and
 the application must add the dynamic call stack usage to @a
 private_segment_size when populating a kernel dispatch packet.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_PRIVATE_SEGMENT_SIZE: hsa_executable_symbol_info_t = 14;
/** Dynamic callstack flag. The value of this attribute is undefined if the
 symbol is not a kernel. The type of this attribute is bool.

 If this flag is set (the value is true), the kernel uses a dynamically
 sized call stack. This can happen if recursive calls, calls to indirect
 functions, or the HSAIL alloca instruction are present in the kernel.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_DYNAMIC_CALLSTACK: hsa_executable_symbol_info_t = 15;
/** @deprecated

 Call convention of the kernel. The value of this attribute is undefined if
 the symbol is not a kernel. The type of this attribute is uint32_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_KERNEL_CALL_CONVENTION: hsa_executable_symbol_info_t = 18;
/** Indirect function object handle. The value of this attribute is undefined
 if the symbol is not an indirect function, or the associated agent does
 not support the Full Profile. The type of this attribute depends on the
 machine model: the type is uint32_t for small machine model, and uint64_t
 for large model.

 If the state of the executable is ::HSA_EXECUTABLE_STATE_UNFROZEN, then 0
 is returned.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_INDIRECT_FUNCTION_OBJECT: hsa_executable_symbol_info_t = 23;
/** @deprecated

 Call convention of the indirect function. The value of this attribute is
 undefined if the symbol is not an indirect function, or the associated
 agent does not support the Full Profile. The type of this attribute is
 uint32_t.*/
pub const HSA_EXECUTABLE_SYMBOL_INFO_INDIRECT_FUNCTION_CALL_CONVENTION: hsa_executable_symbol_info_t = 16;
/// @brief Executable symbol attributes.
pub type hsa_executable_symbol_info_t = ::core::ffi::c_uint;
/** @deprecated

 @brief Struct containing an opaque handle to a code object, which contains
 ISA for finalized kernels and indirect functions together with information
 about the global or readonly segment variables they reference.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_code_object_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_code_object_s"][::core::mem::size_of::<hsa_code_object_s>() - 8usize];
    [
        "Alignment of hsa_code_object_s",
    ][::core::mem::align_of::<hsa_code_object_s>() - 8usize];
    [
        "Offset of field: hsa_code_object_s::handle",
    ][::core::mem::offset_of!(hsa_code_object_s, handle) - 0usize];
};
/** @deprecated

 @brief Struct containing an opaque handle to a code object, which contains
 ISA for finalized kernels and indirect functions together with information
 about the global or readonly segment variables they reference.*/
pub type hsa_code_object_t = hsa_code_object_s;
/** @deprecated

 @brief Application data handle that is passed to the serialization
 and deserialization functions.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_callback_data_s {
    /// Opaque handle.
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_callback_data_s",
    ][::core::mem::size_of::<hsa_callback_data_s>() - 8usize];
    [
        "Alignment of hsa_callback_data_s",
    ][::core::mem::align_of::<hsa_callback_data_s>() - 8usize];
    [
        "Offset of field: hsa_callback_data_s::handle",
    ][::core::mem::offset_of!(hsa_callback_data_s, handle) - 0usize];
};
/** @deprecated

 @brief Application data handle that is passed to the serialization
 and deserialization functions.*/
pub type hsa_callback_data_t = hsa_callback_data_s;
/** Produces code object that contains ISA for all kernels and indirect
 functions in HSA source.*/
pub const HSA_CODE_OBJECT_TYPE_PROGRAM: hsa_code_object_type_t = 0;
/** @deprecated

 @brief Code object type.*/
pub type hsa_code_object_type_t = ::core::ffi::c_uint;
/** The version of the code object. The type of this attribute is a
 NUL-terminated char[64]. The name must be at most 63 characters long (not
 including the NUL terminator) and all array elements not used for the name
 must be NUL.*/
pub const HSA_CODE_OBJECT_INFO_VERSION: hsa_code_object_info_t = 0;
/** Type of code object. The type of this attribute is
 ::hsa_code_object_type_t.*/
pub const HSA_CODE_OBJECT_INFO_TYPE: hsa_code_object_info_t = 1;
/** Instruction set architecture this code object is produced for. The type of
 this attribute is ::hsa_isa_t.*/
pub const HSA_CODE_OBJECT_INFO_ISA: hsa_code_object_info_t = 2;
/** Machine model this code object is produced for. The type of this attribute
 is ::hsa_machine_model_t.*/
pub const HSA_CODE_OBJECT_INFO_MACHINE_MODEL: hsa_code_object_info_t = 3;
/** Profile this code object is produced for. The type of this attribute is
 ::hsa_profile_t.*/
pub const HSA_CODE_OBJECT_INFO_PROFILE: hsa_code_object_info_t = 4;
/** Default floating-point rounding mode used when the code object is
 produced. The type of this attribute is
 ::hsa_default_float_rounding_mode_t.*/
pub const HSA_CODE_OBJECT_INFO_DEFAULT_FLOAT_ROUNDING_MODE: hsa_code_object_info_t = 5;
/** @deprecated

 @brief Code object attributes.*/
pub type hsa_code_object_info_t = ::core::ffi::c_uint;
/** @deprecated

 @brief Code object symbol handle.

 The lifetime of a code object symbol matches that of the code object
 associated with it. An operation on a symbol whose associated code object has
 been destroyed results in undefined behavior.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_code_symbol_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_code_symbol_s"][::core::mem::size_of::<hsa_code_symbol_s>() - 8usize];
    [
        "Alignment of hsa_code_symbol_s",
    ][::core::mem::align_of::<hsa_code_symbol_s>() - 8usize];
    [
        "Offset of field: hsa_code_symbol_s::handle",
    ][::core::mem::offset_of!(hsa_code_symbol_s, handle) - 0usize];
};
/** @deprecated

 @brief Code object symbol handle.

 The lifetime of a code object symbol matches that of the code object
 associated with it. An operation on a symbol whose associated code object has
 been destroyed results in undefined behavior.*/
pub type hsa_code_symbol_t = hsa_code_symbol_s;
/// The type of the symbol. The type of this attribute is ::hsa_symbol_kind_t.
pub const HSA_CODE_SYMBOL_INFO_TYPE: hsa_code_symbol_info_t = 0;
/** The length of the symbol name in bytes, not including the NUL terminator.
 The type of this attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_NAME_LENGTH: hsa_code_symbol_info_t = 1;
/** The name of the symbol. The type of this attribute is character array with
 the length equal to the value of ::HSA_CODE_SYMBOL_INFO_NAME_LENGTH
 attribute.*/
pub const HSA_CODE_SYMBOL_INFO_NAME: hsa_code_symbol_info_t = 2;
/** The length of the module name in bytes (not including the NUL terminator)
 to which this symbol belongs if this symbol has module linkage, otherwise 0
 is returned. The type of this attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_MODULE_NAME_LENGTH: hsa_code_symbol_info_t = 3;
/** The module name to which this symbol belongs if this symbol has module
 linkage, otherwise an empty string is returned. The type of this attribute
 is character array with the length equal to the value of
 ::HSA_CODE_SYMBOL_INFO_MODULE_NAME_LENGTH attribute.*/
pub const HSA_CODE_SYMBOL_INFO_MODULE_NAME: hsa_code_symbol_info_t = 4;
/** The linkage kind of the symbol. The type of this attribute is
 ::hsa_symbol_linkage_t.*/
pub const HSA_CODE_SYMBOL_INFO_LINKAGE: hsa_code_symbol_info_t = 5;
/** Indicates whether the symbol corresponds to a definition. The type of this
 attribute is bool.*/
pub const HSA_CODE_SYMBOL_INFO_IS_DEFINITION: hsa_code_symbol_info_t = 17;
/** The allocation kind of the variable. The value of this attribute is
 undefined if the symbol is not a variable. The type of this attribute is
 ::hsa_variable_allocation_t.*/
pub const HSA_CODE_SYMBOL_INFO_VARIABLE_ALLOCATION: hsa_code_symbol_info_t = 6;
/** The segment kind of the variable. The value of this attribute is
 undefined if the symbol is not a variable. The type of this attribute is
 ::hsa_variable_segment_t.*/
pub const HSA_CODE_SYMBOL_INFO_VARIABLE_SEGMENT: hsa_code_symbol_info_t = 7;
/** Alignment of the symbol in memory. The value of this attribute is undefined
 if the symbol is not a variable. The type of this attribute is uint32_t.

 The current alignment of the variable in memory may be greater than the
 value specified in the source program variable declaration.*/
pub const HSA_CODE_SYMBOL_INFO_VARIABLE_ALIGNMENT: hsa_code_symbol_info_t = 8;
/** Size of the variable. The value of this attribute is undefined if the
 symbol is not a variable. The type of this attribute is uint32_t.

 A size of 0 is returned if the variable is an external variable and has an
 unknown dimension.*/
pub const HSA_CODE_SYMBOL_INFO_VARIABLE_SIZE: hsa_code_symbol_info_t = 9;
/** Indicates whether the variable is constant. The value of this attribute is
 undefined if the symbol is not a variable. The type of this attribute is
 bool.*/
pub const HSA_CODE_SYMBOL_INFO_VARIABLE_IS_CONST: hsa_code_symbol_info_t = 10;
/** Size of kernarg segment memory that is required to hold the values of the
 kernel arguments, in bytes. Must be a multiple of 16. The value of this
 attribute is undefined if the symbol is not a kernel. The type of this
 attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_KERNARG_SEGMENT_SIZE: hsa_code_symbol_info_t = 11;
/** Alignment (in bytes) of the buffer used to pass arguments to the kernel,
 which is the maximum of 16 and the maximum alignment of any of the kernel
 arguments. The value of this attribute is undefined if the symbol is not a
 kernel. The type of this attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_KERNARG_SEGMENT_ALIGNMENT: hsa_code_symbol_info_t = 12;
/** Size of static group segment memory required by the kernel (per
 work-group), in bytes. The value of this attribute is undefined
 if the symbol is not a kernel. The type of this attribute is uint32_t.

 The reported amount does not include any dynamically allocated group
 segment memory that may be requested by the application when a kernel is
 dispatched.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_GROUP_SEGMENT_SIZE: hsa_code_symbol_info_t = 13;
/** Size of static private, spill, and arg segment memory required by
 this kernel (per work-item), in bytes. The value of this attribute is
 undefined if the symbol is not a kernel. The type of this attribute is
 uint32_t.

 If the value of ::HSA_CODE_SYMBOL_INFO_KERNEL_DYNAMIC_CALLSTACK is true,
 the kernel may use more private memory than the reported value, and the
 application must add the dynamic call stack usage to @a
 private_segment_size when populating a kernel dispatch packet.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_PRIVATE_SEGMENT_SIZE: hsa_code_symbol_info_t = 14;
/** Dynamic callstack flag. The value of this attribute is undefined if the
 symbol is not a kernel. The type of this attribute is bool.

 If this flag is set (the value is true), the kernel uses a dynamically
 sized call stack. This can happen if recursive calls, calls to indirect
 functions, or the HSAIL alloca instruction are present in the kernel.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_DYNAMIC_CALLSTACK: hsa_code_symbol_info_t = 15;
/** Call convention of the kernel. The value of this attribute is undefined if
 the symbol is not a kernel. The type of this attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_CALL_CONVENTION: hsa_code_symbol_info_t = 18;
/** Call convention of the indirect function. The value of this attribute is
 undefined if the symbol is not an indirect function. The type of this
 attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_INDIRECT_FUNCTION_CALL_CONVENTION: hsa_code_symbol_info_t = 16;
/** Wavefront size used by the kernel. The value of this attribute is either
 32 or 64. The type of this attribute is uint32_t.*/
pub const HSA_CODE_SYMBOL_INFO_KERNEL_WAVEFRONT_SIZE: hsa_code_symbol_info_t = 19;
/** @deprecated

 @brief Code object symbol attributes.*/
pub type hsa_code_symbol_info_t = ::core::ffi::c_uint;
/// Image format is not supported.
pub const HSA_EXT_STATUS_ERROR_IMAGE_FORMAT_UNSUPPORTED: _bindgen_ty_18 = 12288;
/// Image size is not supported.
pub const HSA_EXT_STATUS_ERROR_IMAGE_SIZE_UNSUPPORTED: _bindgen_ty_18 = 12289;
/// Image pitch is not supported or invalid.
pub const HSA_EXT_STATUS_ERROR_IMAGE_PITCH_UNSUPPORTED: _bindgen_ty_18 = 12290;
/// Sampler descriptor is not supported or invalid.
pub const HSA_EXT_STATUS_ERROR_SAMPLER_DESCRIPTOR_UNSUPPORTED: _bindgen_ty_18 = 12291;
/** @brief Enumeration constants added to ::hsa_status_t by this extension.

 @remark Additions to hsa_status_t*/
pub type _bindgen_ty_18 = ::core::ffi::c_uint;
/** Maximum number of elements in 1D images. Must be at least 16384. The type
 of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_IMAGE_1D_MAX_ELEMENTS: _bindgen_ty_19 = 12288;
/** Maximum number of elements in 1DA images. Must be at least 16384. The type
 of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_IMAGE_1DA_MAX_ELEMENTS: _bindgen_ty_19 = 12289;
/** Maximum number of elements in 1DB images. Must be at least 65536. The type
 of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_IMAGE_1DB_MAX_ELEMENTS: _bindgen_ty_19 = 12290;
/** Maximum dimensions (width, height) of 2D images, in image elements. The X
 and Y maximums must be at least 16384. The type of this attribute is
 size_t[2].*/
pub const HSA_EXT_AGENT_INFO_IMAGE_2D_MAX_ELEMENTS: _bindgen_ty_19 = 12291;
/** Maximum dimensions (width, height) of 2DA images, in image elements. The X
 and Y maximums must be at least 16384. The type of this attribute is
 size_t[2].*/
pub const HSA_EXT_AGENT_INFO_IMAGE_2DA_MAX_ELEMENTS: _bindgen_ty_19 = 12292;
/** Maximum dimensions (width, height) of 2DDEPTH images, in image
 elements. The X and Y maximums must be at least 16384. The type of this
 attribute is size_t[2].*/
pub const HSA_EXT_AGENT_INFO_IMAGE_2DDEPTH_MAX_ELEMENTS: _bindgen_ty_19 = 12293;
/** Maximum dimensions (width, height) of 2DADEPTH images, in image
 elements. The X and Y maximums must be at least 16384. The type of this
 attribute is size_t[2].*/
pub const HSA_EXT_AGENT_INFO_IMAGE_2DADEPTH_MAX_ELEMENTS: _bindgen_ty_19 = 12294;
/** Maximum dimensions (width, height, depth) of 3D images, in image
 elements. The maximum along any dimension must be at least 2048. The type
 of this attribute is size_t[3].*/
pub const HSA_EXT_AGENT_INFO_IMAGE_3D_MAX_ELEMENTS: _bindgen_ty_19 = 12295;
/** Maximum number of image layers in a image array. Must be at least 2048. The
 type of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_IMAGE_ARRAY_MAX_LAYERS: _bindgen_ty_19 = 12296;
/** Maximum number of read-only image handles that can be created for an agent at any one
 time. Must be at least 128. The type of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_MAX_IMAGE_RD_HANDLES: _bindgen_ty_19 = 12297;
/** Maximum number of write-only and read-write image handles (combined) that
 can be created for an agent at any one time. Must be at least 64. The type of this
 attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_MAX_IMAGE_RORW_HANDLES: _bindgen_ty_19 = 12298;
/** Maximum number of sampler handlers that can be created for an agent at any one
 time. Must be at least 16. The type of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_MAX_SAMPLER_HANDLERS: _bindgen_ty_19 = 12299;
/** Image pitch alignment. The agent only supports linear image data
 layouts with a row pitch that is a multiple of this value. Must be
 a power of 2. The type of this attribute is size_t.*/
pub const HSA_EXT_AGENT_INFO_IMAGE_LINEAR_ROW_PITCH_ALIGNMENT: _bindgen_ty_19 = 12300;
/** @brief Enumeration constants added to ::hsa_agent_info_t by this
 extension.

 @remark Additions to hsa_agent_info_t*/
pub type _bindgen_ty_19 = ::core::ffi::c_uint;
/** @brief Image handle, populated by ::hsa_ext_image_create or
 ::hsa_ext_image_create_with_layout. Image
 handles are only unique within an agent, not across agents.
*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_image_s {
    /**  Opaque handle. For a given agent, two handles reference the same object of
  the enclosing type if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_ext_image_s"][::core::mem::size_of::<hsa_ext_image_s>() - 8usize];
    [
        "Alignment of hsa_ext_image_s",
    ][::core::mem::align_of::<hsa_ext_image_s>() - 8usize];
    [
        "Offset of field: hsa_ext_image_s::handle",
    ][::core::mem::offset_of!(hsa_ext_image_s, handle) - 0usize];
};
/** @brief Image handle, populated by ::hsa_ext_image_create or
 ::hsa_ext_image_create_with_layout. Image
 handles are only unique within an agent, not across agents.
*/
pub type hsa_ext_image_t = hsa_ext_image_s;
/// One-dimensional image addressed by width coordinate.
pub const HSA_EXT_IMAGE_GEOMETRY_1D: hsa_ext_image_geometry_t = 0;
/// Two-dimensional image addressed by width and height coordinates.
pub const HSA_EXT_IMAGE_GEOMETRY_2D: hsa_ext_image_geometry_t = 1;
/// Three-dimensional image addressed by width, height, and depth coordinates.
pub const HSA_EXT_IMAGE_GEOMETRY_3D: hsa_ext_image_geometry_t = 2;
/** Array of one-dimensional images with the same size and format. 1D arrays
 are addressed by width and index coordinate.*/
pub const HSA_EXT_IMAGE_GEOMETRY_1DA: hsa_ext_image_geometry_t = 3;
/** Array of two-dimensional images with the same size and format. 2D arrays
 are addressed by width,  height, and index coordinates.*/
pub const HSA_EXT_IMAGE_GEOMETRY_2DA: hsa_ext_image_geometry_t = 4;
/** One-dimensional image addressed by width coordinate. It has
 specific restrictions compared to ::HSA_EXT_IMAGE_GEOMETRY_1D. An
 image with an opaque image data layout will always use a linear
 image data layout, and one with an explicit image data layout
 must specify ::HSA_EXT_IMAGE_DATA_LAYOUT_LINEAR.*/
pub const HSA_EXT_IMAGE_GEOMETRY_1DB: hsa_ext_image_geometry_t = 5;
/// Two-dimensional depth image addressed by width and height coordinates.
pub const HSA_EXT_IMAGE_GEOMETRY_2DDEPTH: hsa_ext_image_geometry_t = 6;
/** Array of two-dimensional depth images with the same size and format. 2D
 arrays are addressed by width, height, and index coordinates.*/
pub const HSA_EXT_IMAGE_GEOMETRY_2DADEPTH: hsa_ext_image_geometry_t = 7;
/** @brief Geometry associated with the image. This specifies the
 number of image dimensions and whether the image is an image
 array. See the <em>Image Geometry</em> section in the <em>HSA
 Programming Reference Manual</em> for definitions on each
 geometry. The enumeration values match the BRIG type @p
 hsa_ext_brig_image_geometry_t.*/
pub type hsa_ext_image_geometry_t = ::core::ffi::c_uint;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_SNORM_INT8: hsa_ext_image_channel_type_t = 0;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_SNORM_INT16: hsa_ext_image_channel_type_t = 1;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNORM_INT8: hsa_ext_image_channel_type_t = 2;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNORM_INT16: hsa_ext_image_channel_type_t = 3;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNORM_INT24: hsa_ext_image_channel_type_t = 4;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNORM_SHORT_555: hsa_ext_image_channel_type_t = 5;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNORM_SHORT_565: hsa_ext_image_channel_type_t = 6;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNORM_SHORT_101010: hsa_ext_image_channel_type_t = 7;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_SIGNED_INT8: hsa_ext_image_channel_type_t = 8;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_SIGNED_INT16: hsa_ext_image_channel_type_t = 9;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_SIGNED_INT32: hsa_ext_image_channel_type_t = 10;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNSIGNED_INT8: hsa_ext_image_channel_type_t = 11;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNSIGNED_INT16: hsa_ext_image_channel_type_t = 12;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_UNSIGNED_INT32: hsa_ext_image_channel_type_t = 13;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_HALF_FLOAT: hsa_ext_image_channel_type_t = 14;
pub const HSA_EXT_IMAGE_CHANNEL_TYPE_FLOAT: hsa_ext_image_channel_type_t = 15;
/** @brief Channel type associated with the elements of an image. See
 the <em>Channel Type</em> section in the <em>HSA Programming Reference
 Manual</em> for definitions on each channel type. The
 enumeration values and definition match the BRIG type @p
 hsa_ext_brig_image_channel_type_t.*/
pub type hsa_ext_image_channel_type_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_ext_image_channel_type_t constants.
pub type hsa_ext_image_channel_type32_t = u32;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_A: hsa_ext_image_channel_order_t = 0;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_R: hsa_ext_image_channel_order_t = 1;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RX: hsa_ext_image_channel_order_t = 2;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RG: hsa_ext_image_channel_order_t = 3;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RGX: hsa_ext_image_channel_order_t = 4;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RA: hsa_ext_image_channel_order_t = 5;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RGB: hsa_ext_image_channel_order_t = 6;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RGBX: hsa_ext_image_channel_order_t = 7;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_RGBA: hsa_ext_image_channel_order_t = 8;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_BGRA: hsa_ext_image_channel_order_t = 9;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_ARGB: hsa_ext_image_channel_order_t = 10;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_ABGR: hsa_ext_image_channel_order_t = 11;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_SRGB: hsa_ext_image_channel_order_t = 12;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_SRGBX: hsa_ext_image_channel_order_t = 13;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_SRGBA: hsa_ext_image_channel_order_t = 14;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_SBGRA: hsa_ext_image_channel_order_t = 15;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_INTENSITY: hsa_ext_image_channel_order_t = 16;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_LUMINANCE: hsa_ext_image_channel_order_t = 17;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_DEPTH: hsa_ext_image_channel_order_t = 18;
pub const HSA_EXT_IMAGE_CHANNEL_ORDER_DEPTH_STENCIL: hsa_ext_image_channel_order_t = 19;
/** @brief Channel order associated with the elements of an image. See
 the <em>Channel Order</em> section in the <em>HSA Programming Reference
 Manual</em> for definitions on each channel order. The
 enumeration values match the BRIG type @p
 hsa_ext_brig_image_channel_order_t.*/
pub type hsa_ext_image_channel_order_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_ext_image_channel_order_t constants.
pub type hsa_ext_image_channel_order32_t = u32;
/// @brief Image format.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_image_format_s {
    /// Channel type.
    pub channel_type: hsa_ext_image_channel_type32_t,
    /// Channel order.
    pub channel_order: hsa_ext_image_channel_order32_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_image_format_s",
    ][::core::mem::size_of::<hsa_ext_image_format_s>() - 8usize];
    [
        "Alignment of hsa_ext_image_format_s",
    ][::core::mem::align_of::<hsa_ext_image_format_s>() - 4usize];
    [
        "Offset of field: hsa_ext_image_format_s::channel_type",
    ][::core::mem::offset_of!(hsa_ext_image_format_s, channel_type) - 0usize];
    [
        "Offset of field: hsa_ext_image_format_s::channel_order",
    ][::core::mem::offset_of!(hsa_ext_image_format_s, channel_order) - 4usize];
};
/// @brief Image format.
pub type hsa_ext_image_format_t = hsa_ext_image_format_s;
/// @brief Implementation independent image descriptor.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_ext_image_descriptor_s {
    /// Image geometry.
    pub geometry: hsa_ext_image_geometry_t,
    /// Width of the image, in components.
    pub width: usize,
    /** Height of the image, in components. Only used if the geometry is
 ::HSA_EXT_IMAGE_GEOMETRY_2D, ::HSA_EXT_IMAGE_GEOMETRY_3D,
 HSA_EXT_IMAGE_GEOMETRY_2DA, HSA_EXT_IMAGE_GEOMETRY_2DDEPTH, or
 HSA_EXT_IMAGE_GEOMETRY_2DADEPTH, otherwise must be 0.*/
    pub height: usize,
    /** Depth of the image, in components. Only used if the geometry is
 ::HSA_EXT_IMAGE_GEOMETRY_3D, otherwise must be 0.*/
    pub depth: usize,
    /** Number of image layers in the image array. Only used if the geometry is
 ::HSA_EXT_IMAGE_GEOMETRY_1DA, ::HSA_EXT_IMAGE_GEOMETRY_2DA, or
 HSA_EXT_IMAGE_GEOMETRY_2DADEPTH, otherwise must be 0.*/
    pub array_size: usize,
    /// Image format.
    pub format: hsa_ext_image_format_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_image_descriptor_s",
    ][::core::mem::size_of::<hsa_ext_image_descriptor_s>() - 48usize];
    [
        "Alignment of hsa_ext_image_descriptor_s",
    ][::core::mem::align_of::<hsa_ext_image_descriptor_s>() - 8usize];
    [
        "Offset of field: hsa_ext_image_descriptor_s::geometry",
    ][::core::mem::offset_of!(hsa_ext_image_descriptor_s, geometry) - 0usize];
    [
        "Offset of field: hsa_ext_image_descriptor_s::width",
    ][::core::mem::offset_of!(hsa_ext_image_descriptor_s, width) - 8usize];
    [
        "Offset of field: hsa_ext_image_descriptor_s::height",
    ][::core::mem::offset_of!(hsa_ext_image_descriptor_s, height) - 16usize];
    [
        "Offset of field: hsa_ext_image_descriptor_s::depth",
    ][::core::mem::offset_of!(hsa_ext_image_descriptor_s, depth) - 24usize];
    [
        "Offset of field: hsa_ext_image_descriptor_s::array_size",
    ][::core::mem::offset_of!(hsa_ext_image_descriptor_s, array_size) - 32usize];
    [
        "Offset of field: hsa_ext_image_descriptor_s::format",
    ][::core::mem::offset_of!(hsa_ext_image_descriptor_s, format) - 40usize];
};
impl Default for hsa_ext_image_descriptor_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// @brief Implementation independent image descriptor.
pub type hsa_ext_image_descriptor_t = hsa_ext_image_descriptor_s;
/** Images of this geometry, format, and layout are not supported by
 the agent.*/
pub const HSA_EXT_IMAGE_CAPABILITY_NOT_SUPPORTED: hsa_ext_image_capability_t = 0;
/** Read-only images of this geometry, format, and layout are
 supported by the agent.*/
pub const HSA_EXT_IMAGE_CAPABILITY_READ_ONLY: hsa_ext_image_capability_t = 1;
/** Write-only images of this geometry, format, and layout are
 supported by the agent.*/
pub const HSA_EXT_IMAGE_CAPABILITY_WRITE_ONLY: hsa_ext_image_capability_t = 2;
/** Read-write images of this geometry, format, and layout are
 supported by the agent.*/
pub const HSA_EXT_IMAGE_CAPABILITY_READ_WRITE: hsa_ext_image_capability_t = 4;
/** @deprecated Images of this geometry, format, and layout can be accessed from
 read-modify-write atomic operations in the agent.*/
pub const HSA_EXT_IMAGE_CAPABILITY_READ_MODIFY_WRITE: hsa_ext_image_capability_t = 8;
/** Images of this geometry, format, and layout are guaranteed to
 have a consistent data layout regardless of how they are
 accessed by the associated agent.*/
pub const HSA_EXT_IMAGE_CAPABILITY_ACCESS_INVARIANT_DATA_LAYOUT: hsa_ext_image_capability_t = 16;
/// @brief Image capability.
pub type hsa_ext_image_capability_t = ::core::ffi::c_uint;
/** An implementation specific opaque image data layout which can
 vary depending on the agent, geometry, image format, image size,
 and access permissions.*/
pub const HSA_EXT_IMAGE_DATA_LAYOUT_OPAQUE: hsa_ext_image_data_layout_t = 0;
/** The image data layout is specified by the following rules in
 ascending byte address order. For a 3D image, 2DA image array,
 or 1DA image array, the image data is stored as a linear sequence
 of adjacent 2D image slices, 2D images, or 1D images
 respectively, spaced according to the slice pitch. Each 2D image
 is stored as a linear sequence of adjacent image rows, spaced
 according to the row pitch. Each 1D or 1DB image is stored as a
 single image row. Each image row is stored as a linear sequence
 of image elements. Each image element is stored as a linear
 sequence of image components specified by the left to right
 channel order definition. Each image component is stored using
 the memory type specified by the channel type.

 The 1DB image geometry always uses the linear image data layout.*/
pub const HSA_EXT_IMAGE_DATA_LAYOUT_LINEAR: hsa_ext_image_data_layout_t = 1;
/** @brief Image data layout.

 @details An image data layout denotes such aspects of image data
 layout as tiling and organization of channels in memory. Some image
 data layouts may only apply to specific image geometries, formats,
 and access permissions. Different agents may support different
 image layout identifiers, including vendor specific layouts. Note
 that an agent may not support the same image data layout for
 different access permissions to images with the same image
 geometry, size, and format. If multiple agents support the same
 image data layout then it is possible to use separate image handles
 for each agent that references the same image data.*/
pub type hsa_ext_image_data_layout_t = ::core::ffi::c_uint;
/** @brief Agent specific image size and alignment requirements, populated by
 ::hsa_ext_image_data_get_info and ::hsa_ext_image_data_get_info_with_layout.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_image_data_info_s {
    /// Image data size, in bytes.
    pub size: usize,
    /// Image data alignment, in bytes. Must always be a power of 2.
    pub alignment: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_image_data_info_s",
    ][::core::mem::size_of::<hsa_ext_image_data_info_s>() - 16usize];
    [
        "Alignment of hsa_ext_image_data_info_s",
    ][::core::mem::align_of::<hsa_ext_image_data_info_s>() - 8usize];
    [
        "Offset of field: hsa_ext_image_data_info_s::size",
    ][::core::mem::offset_of!(hsa_ext_image_data_info_s, size) - 0usize];
    [
        "Offset of field: hsa_ext_image_data_info_s::alignment",
    ][::core::mem::offset_of!(hsa_ext_image_data_info_s, alignment) - 8usize];
};
/** @brief Agent specific image size and alignment requirements, populated by
 ::hsa_ext_image_data_get_info and ::hsa_ext_image_data_get_info_with_layout.*/
pub type hsa_ext_image_data_info_t = hsa_ext_image_data_info_s;
/// @brief Image region.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_image_region_s {
    /// Offset within an image (in coordinates).
    pub offset: hsa_dim3_t,
    /** Dimension size of the image range (in coordinates). The x, y, and z dimensions
 correspond to width, height, and depth or index respectively.*/
    pub range: hsa_dim3_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_image_region_s",
    ][::core::mem::size_of::<hsa_ext_image_region_s>() - 24usize];
    [
        "Alignment of hsa_ext_image_region_s",
    ][::core::mem::align_of::<hsa_ext_image_region_s>() - 4usize];
    [
        "Offset of field: hsa_ext_image_region_s::offset",
    ][::core::mem::offset_of!(hsa_ext_image_region_s, offset) - 0usize];
    [
        "Offset of field: hsa_ext_image_region_s::range",
    ][::core::mem::offset_of!(hsa_ext_image_region_s, range) - 12usize];
};
/// @brief Image region.
pub type hsa_ext_image_region_t = hsa_ext_image_region_s;
/** @brief Sampler handle. Samplers are populated by
 ::hsa_ext_sampler_create or ::hsa_ext_sampler_create_v2. Sampler handles are only unique
  within an agent, not across agents.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_sampler_s {
    /**  Opaque handle. For a given agent, two handles reference the same object of
  the enclosing type if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_ext_sampler_s"][::core::mem::size_of::<hsa_ext_sampler_s>() - 8usize];
    [
        "Alignment of hsa_ext_sampler_s",
    ][::core::mem::align_of::<hsa_ext_sampler_s>() - 8usize];
    [
        "Offset of field: hsa_ext_sampler_s::handle",
    ][::core::mem::offset_of!(hsa_ext_sampler_s, handle) - 0usize];
};
/** @brief Sampler handle. Samplers are populated by
 ::hsa_ext_sampler_create or ::hsa_ext_sampler_create_v2. Sampler handles are only unique
  within an agent, not across agents.*/
pub type hsa_ext_sampler_t = hsa_ext_sampler_s;
/// Out-of-range coordinates are not handled.
pub const HSA_EXT_SAMPLER_ADDRESSING_MODE_UNDEFINED: hsa_ext_sampler_addressing_mode_t = 0;
/// Clamp out-of-range coordinates to the image edge.
pub const HSA_EXT_SAMPLER_ADDRESSING_MODE_CLAMP_TO_EDGE: hsa_ext_sampler_addressing_mode_t = 1;
/// Clamp out-of-range coordinates to the image border color.
pub const HSA_EXT_SAMPLER_ADDRESSING_MODE_CLAMP_TO_BORDER: hsa_ext_sampler_addressing_mode_t = 2;
/** Wrap out-of-range coordinates back into the valid coordinate
 range so the image appears as repeated tiles.*/
pub const HSA_EXT_SAMPLER_ADDRESSING_MODE_REPEAT: hsa_ext_sampler_addressing_mode_t = 3;
/** Mirror out-of-range coordinates back into the valid coordinate
 range so the image appears as repeated tiles with every other
 tile a reflection.*/
pub const HSA_EXT_SAMPLER_ADDRESSING_MODE_MIRRORED_REPEAT: hsa_ext_sampler_addressing_mode_t = 4;
/** @brief Sampler address modes. The sampler address mode describes
 the processing of out-of-range image coordinates. See the
 <em>Addressing Mode</em> section in the <em>HSA Programming Reference
 Manual</em> for definitions on each address mode. The values
 match the BRIG type @p hsa_ext_brig_sampler_addressing_t.*/
pub type hsa_ext_sampler_addressing_mode_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_ext_sampler_addressing_mode_t constants.
pub type hsa_ext_sampler_addressing_mode32_t = u32;
/// Coordinates are used to directly address an image element.
pub const HSA_EXT_SAMPLER_COORDINATE_MODE_UNNORMALIZED: hsa_ext_sampler_coordinate_mode_t = 0;
/** Coordinates are scaled by the image dimension size before being
 used to address an image element.*/
pub const HSA_EXT_SAMPLER_COORDINATE_MODE_NORMALIZED: hsa_ext_sampler_coordinate_mode_t = 1;
/** @brief Sampler coordinate normalization modes. See the
 <em>Coordinate Normalization Mode</em> section in the <em>HSA
 Programming Reference Manual</em> for definitions on each
 coordinate normalization mode. The values match the BRIG type @p
 hsa_ext_brig_sampler_coord_normalization_t.*/
pub type hsa_ext_sampler_coordinate_mode_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_ext_sampler_coordinate_mode_t constants.
pub type hsa_ext_sampler_coordinate_mode32_t = u32;
/** Filter to the image element nearest (in Manhattan distance) to the
 specified coordinate.*/
pub const HSA_EXT_SAMPLER_FILTER_MODE_NEAREST: hsa_ext_sampler_filter_mode_t = 0;
/** Filter to the image element calculated by combining the elements in a 2x2
 square block or 2x2x2 cube block around the specified coordinate. The
 elements are combined using linear interpolation.*/
pub const HSA_EXT_SAMPLER_FILTER_MODE_LINEAR: hsa_ext_sampler_filter_mode_t = 1;
/** @brief Sampler filter modes. See the <em>Filter Mode</em> section
 in the <em>HSA Programming Reference Manual</em> for definitions
 on each address mode. The enumeration values match the BRIG type @p
 hsa_ext_brig_sampler_filter_t.*/
pub type hsa_ext_sampler_filter_mode_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_ext_sampler_filter_mode_t constants.
pub type hsa_ext_sampler_filter_mode32_t = u32;
/// @brief Implementation independent sampler descriptor.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_sampler_descriptor_s {
    /// Sampler coordinate mode describes the normalization of image coordinates.
    pub coordinate_mode: hsa_ext_sampler_coordinate_mode32_t,
    /// Sampler filter type describes the type of sampling performed.
    pub filter_mode: hsa_ext_sampler_filter_mode32_t,
    /** Sampler address mode describes the processing of out-of-range image
 coordinates.*/
    pub address_mode: hsa_ext_sampler_addressing_mode32_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_sampler_descriptor_s",
    ][::core::mem::size_of::<hsa_ext_sampler_descriptor_s>() - 12usize];
    [
        "Alignment of hsa_ext_sampler_descriptor_s",
    ][::core::mem::align_of::<hsa_ext_sampler_descriptor_s>() - 4usize];
    [
        "Offset of field: hsa_ext_sampler_descriptor_s::coordinate_mode",
    ][::core::mem::offset_of!(hsa_ext_sampler_descriptor_s, coordinate_mode) - 0usize];
    [
        "Offset of field: hsa_ext_sampler_descriptor_s::filter_mode",
    ][::core::mem::offset_of!(hsa_ext_sampler_descriptor_s, filter_mode) - 4usize];
    [
        "Offset of field: hsa_ext_sampler_descriptor_s::address_mode",
    ][::core::mem::offset_of!(hsa_ext_sampler_descriptor_s, address_mode) - 8usize];
};
/// @brief Implementation independent sampler descriptor.
pub type hsa_ext_sampler_descriptor_t = hsa_ext_sampler_descriptor_s;
/** @brief Implementation independent sampler descriptor v2 which supports
  different address modes in X, Y and Z axises.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_sampler_descriptor_v2_s {
    /// Sampler coordinate mode describes the normalization of image coordinates.
    pub coordinate_mode: hsa_ext_sampler_coordinate_mode32_t,
    /// Sampler filter type describes the type of sampling performed.
    pub filter_mode: hsa_ext_sampler_filter_mode32_t,
    /** Sampler address mode describes the processing of out-of-range image
 coordinates.*/
    pub address_modes: [hsa_ext_sampler_addressing_mode32_t; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_sampler_descriptor_v2_s",
    ][::core::mem::size_of::<hsa_ext_sampler_descriptor_v2_s>() - 20usize];
    [
        "Alignment of hsa_ext_sampler_descriptor_v2_s",
    ][::core::mem::align_of::<hsa_ext_sampler_descriptor_v2_s>() - 4usize];
    [
        "Offset of field: hsa_ext_sampler_descriptor_v2_s::coordinate_mode",
    ][::core::mem::offset_of!(hsa_ext_sampler_descriptor_v2_s, coordinate_mode)
        - 0usize];
    [
        "Offset of field: hsa_ext_sampler_descriptor_v2_s::filter_mode",
    ][::core::mem::offset_of!(hsa_ext_sampler_descriptor_v2_s, filter_mode) - 4usize];
    [
        "Offset of field: hsa_ext_sampler_descriptor_v2_s::address_modes",
    ][::core::mem::offset_of!(hsa_ext_sampler_descriptor_v2_s, address_modes) - 8usize];
};
/** @brief Implementation independent sampler descriptor v2 which supports
  different address modes in X, Y and Z axises.*/
pub type hsa_ext_sampler_descriptor_v2_t = hsa_ext_sampler_descriptor_v2_s;
/// @brief The function pointer table for the images v1.00 extension. Can be returned by ::hsa_system_get_extension_table or ::hsa_system_get_major_extension_table.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_images_1_00_pfn_s {
    pub hsa_ext_image_get_capability: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            geometry: hsa_ext_image_geometry_t,
            image_format: *const hsa_ext_image_format_t,
            capability_mask: *mut u32,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_data_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image_descriptor: *const hsa_ext_image_descriptor_t,
            access_permission: hsa_access_permission_t,
            image_data_info: *mut hsa_ext_image_data_info_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_create: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image_descriptor: *const hsa_ext_image_descriptor_t,
            image_data: *const ::core::ffi::c_void,
            access_permission: hsa_access_permission_t,
            image: *mut hsa_ext_image_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_destroy: ::core::option::Option<
        unsafe extern "C" fn(agent: hsa_agent_t, image: hsa_ext_image_t) -> hsa_status_t,
    >,
    pub hsa_ext_image_copy: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            src_image: hsa_ext_image_t,
            src_offset: *const hsa_dim3_t,
            dst_image: hsa_ext_image_t,
            dst_offset: *const hsa_dim3_t,
            range: *const hsa_dim3_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_import: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            src_memory: *const ::core::ffi::c_void,
            src_row_pitch: usize,
            src_slice_pitch: usize,
            dst_image: hsa_ext_image_t,
            image_region: *const hsa_ext_image_region_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_export: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            src_image: hsa_ext_image_t,
            dst_memory: *mut ::core::ffi::c_void,
            dst_row_pitch: usize,
            dst_slice_pitch: usize,
            image_region: *const hsa_ext_image_region_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_clear: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image: hsa_ext_image_t,
            data: *const ::core::ffi::c_void,
            image_region: *const hsa_ext_image_region_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_sampler_create: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            sampler_descriptor: *const hsa_ext_sampler_descriptor_t,
            sampler: *mut hsa_ext_sampler_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_sampler_destroy: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            sampler: hsa_ext_sampler_t,
        ) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_images_1_00_pfn_s",
    ][::core::mem::size_of::<hsa_ext_images_1_00_pfn_s>() - 80usize];
    [
        "Alignment of hsa_ext_images_1_00_pfn_s",
    ][::core::mem::align_of::<hsa_ext_images_1_00_pfn_s>() - 8usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_get_capability",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_get_capability)
        - 0usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_data_get_info",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_data_get_info)
        - 8usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_create",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_create)
        - 16usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_destroy",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_destroy)
        - 24usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_copy",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_copy) - 32usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_import",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_import)
        - 40usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_export",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_export)
        - 48usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_image_clear",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_image_clear) - 56usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_sampler_create",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_sampler_create)
        - 64usize];
    [
        "Offset of field: hsa_ext_images_1_00_pfn_s::hsa_ext_sampler_destroy",
    ][::core::mem::offset_of!(hsa_ext_images_1_00_pfn_s, hsa_ext_sampler_destroy)
        - 72usize];
};
/// @brief The function pointer table for the images v1.00 extension. Can be returned by ::hsa_system_get_extension_table or ::hsa_system_get_major_extension_table.
pub type hsa_ext_images_1_00_pfn_t = hsa_ext_images_1_00_pfn_s;
/// @brief The function pointer table for the images v1 extension. Can be returned by ::hsa_system_get_extension_table or ::hsa_system_get_major_extension_table.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ext_images_1_pfn_s {
    pub hsa_ext_image_get_capability: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            geometry: hsa_ext_image_geometry_t,
            image_format: *const hsa_ext_image_format_t,
            capability_mask: *mut u32,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_data_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image_descriptor: *const hsa_ext_image_descriptor_t,
            access_permission: hsa_access_permission_t,
            image_data_info: *mut hsa_ext_image_data_info_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_create: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image_descriptor: *const hsa_ext_image_descriptor_t,
            image_data: *const ::core::ffi::c_void,
            access_permission: hsa_access_permission_t,
            image: *mut hsa_ext_image_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_destroy: ::core::option::Option<
        unsafe extern "C" fn(agent: hsa_agent_t, image: hsa_ext_image_t) -> hsa_status_t,
    >,
    pub hsa_ext_image_copy: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            src_image: hsa_ext_image_t,
            src_offset: *const hsa_dim3_t,
            dst_image: hsa_ext_image_t,
            dst_offset: *const hsa_dim3_t,
            range: *const hsa_dim3_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_import: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            src_memory: *const ::core::ffi::c_void,
            src_row_pitch: usize,
            src_slice_pitch: usize,
            dst_image: hsa_ext_image_t,
            image_region: *const hsa_ext_image_region_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_export: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            src_image: hsa_ext_image_t,
            dst_memory: *mut ::core::ffi::c_void,
            dst_row_pitch: usize,
            dst_slice_pitch: usize,
            image_region: *const hsa_ext_image_region_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_clear: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image: hsa_ext_image_t,
            data: *const ::core::ffi::c_void,
            image_region: *const hsa_ext_image_region_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_sampler_create: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            sampler_descriptor: *const hsa_ext_sampler_descriptor_t,
            sampler: *mut hsa_ext_sampler_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_sampler_destroy: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            sampler: hsa_ext_sampler_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_get_capability_with_layout: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            geometry: hsa_ext_image_geometry_t,
            image_format: *const hsa_ext_image_format_t,
            image_data_layout: hsa_ext_image_data_layout_t,
            capability_mask: *mut u32,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_data_get_info_with_layout: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image_descriptor: *const hsa_ext_image_descriptor_t,
            access_permission: hsa_access_permission_t,
            image_data_layout: hsa_ext_image_data_layout_t,
            image_data_row_pitch: usize,
            image_data_slice_pitch: usize,
            image_data_info: *mut hsa_ext_image_data_info_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_image_create_with_layout: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            image_descriptor: *const hsa_ext_image_descriptor_t,
            image_data: *const ::core::ffi::c_void,
            access_permission: hsa_access_permission_t,
            image_data_layout: hsa_ext_image_data_layout_t,
            image_data_row_pitch: usize,
            image_data_slice_pitch: usize,
            image: *mut hsa_ext_image_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ext_sampler_create_v2: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            sampler_descriptor: *const hsa_ext_sampler_descriptor_v2_t,
            sampler: *mut hsa_ext_sampler_t,
        ) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ext_images_1_pfn_s",
    ][::core::mem::size_of::<hsa_ext_images_1_pfn_s>() - 112usize];
    [
        "Alignment of hsa_ext_images_1_pfn_s",
    ][::core::mem::align_of::<hsa_ext_images_1_pfn_s>() - 8usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_get_capability",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_get_capability)
        - 0usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_data_get_info",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_data_get_info)
        - 8usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_create",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_create) - 16usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_destroy",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_destroy) - 24usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_copy",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_copy) - 32usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_import",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_import) - 40usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_export",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_export) - 48usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_clear",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_clear) - 56usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_sampler_create",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_sampler_create) - 64usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_sampler_destroy",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_sampler_destroy)
        - 72usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_get_capability_with_layout",
    ][::core::mem::offset_of!(
        hsa_ext_images_1_pfn_s, hsa_ext_image_get_capability_with_layout
    ) - 80usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_data_get_info_with_layout",
    ][::core::mem::offset_of!(
        hsa_ext_images_1_pfn_s, hsa_ext_image_data_get_info_with_layout
    ) - 88usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_image_create_with_layout",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_image_create_with_layout)
        - 96usize];
    [
        "Offset of field: hsa_ext_images_1_pfn_s::hsa_ext_sampler_create_v2",
    ][::core::mem::offset_of!(hsa_ext_images_1_pfn_s, hsa_ext_sampler_create_v2)
        - 104usize];
};
/// @brief The function pointer table for the images v1 extension. Can be returned by ::hsa_system_get_extension_table or ::hsa_system_get_major_extension_table.
pub type hsa_ext_images_1_pfn_t = hsa_ext_images_1_pfn_s;
pub const HSA_VEN_AMD_PCS_METHOD_HOSTTRAP_V1: hsa_ven_amd_pcs_method_kind_t = 0;
pub const HSA_VEN_AMD_PCS_METHOD_STOCHASTIC_V1: hsa_ven_amd_pcs_method_kind_t = 1;
/// @brief PC Sampling method kinds
pub type hsa_ven_amd_pcs_method_kind_t = ::core::ffi::c_uint;
pub const HSA_VEN_AMD_PCS_INTERVAL_UNITS_MICRO_SECONDS: hsa_ven_amd_pcs_units_t = 0;
pub const HSA_VEN_AMD_PCS_INTERVAL_UNITS_CLOCK_CYCLES: hsa_ven_amd_pcs_units_t = 1;
pub const HSA_VEN_AMD_PCS_INTERVAL_UNITS_INSTRUCTIONS: hsa_ven_amd_pcs_units_t = 2;
/// @brief PC Sampling interval unit type
pub type hsa_ven_amd_pcs_units_t = ::core::ffi::c_uint;
/** @brief HSA callback function to perform the copy onto a destination buffer

 If data_size is 0, HSA will stop current copy operation and keep remaining data in internal
 buffers. Remaining contents of HSA internal buffers will be included in next
 hsa_ven_amd_pcs_data_ready_callback_t. HSA internal buffers can also be drained by calling
 hsa_ven_amd_pcs_flush.

 @param[in] hsa_callback_data private data to pass back to HSA. Provided in
 hsa_ven_amd_pcs_data_ready_callback_t

 @param[in] data_size size of destination buffer in bytes.
 @param[in] destination destination buffer
 @retval    TBD: but could be used to indicate that there is no more data to be read.
 Or indicate an error and abort of current copy operations*/
pub type hsa_ven_amd_pcs_data_copy_callback_t = ::core::option::Option<
    unsafe extern "C" fn(
        hsa_callback_data: *mut ::core::ffi::c_void,
        data_size: usize,
        destination: *mut ::core::ffi::c_void,
    ) -> hsa_status_t,
>;
/** @brief HSA callback function to to indicate that there is data ready to be copied

 When the client receives this callback, the client should call back @p data_copy_callback for HSA
 to perform the copy operation into an available buffer. @p data_copy_callback can be called back
 multiple times with smaller @p data_size to split the copy operation.

 This callback must not call ::hsa_ven_amd_pcs_flush.

 @param[in] client_callback_data client private data passed in via
 hsa_ven_amd_pcs_create/hsa_ven_amd_pcs_create_from_id
 @param[in] data_size size of data available to be copied
 @param[in] lost_sample_count number of lost samples since last call to
 hsa_ven_amd_pcs_data_ready_callback_t.
 @param[in] data_copy_callback callback function for HSA to perform the actual copy
 @param[in] hsa_callback_data private data to pass back to HSA*/
pub type hsa_ven_amd_pcs_data_ready_callback_t = ::core::option::Option<
    unsafe extern "C" fn(
        client_callback_data: *mut ::core::ffi::c_void,
        data_size: usize,
        lost_sample_count: usize,
        data_copy_callback: hsa_ven_amd_pcs_data_copy_callback_t,
        hsa_callback_data: *mut ::core::ffi::c_void,
    ),
>;
/** @brief Opaque handle representing a sampling session.
 Two sessions having same handle value represent the same session*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ven_amd_pcs_t {
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_ven_amd_pcs_t"][::core::mem::size_of::<hsa_ven_amd_pcs_t>() - 8usize];
    [
        "Alignment of hsa_ven_amd_pcs_t",
    ][::core::mem::align_of::<hsa_ven_amd_pcs_t>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_pcs_t::handle",
    ][::core::mem::offset_of!(hsa_ven_amd_pcs_t, handle) - 0usize];
};
pub const HSA_VEN_AMD_PCS_CONFIGURATION_FLAGS_INTERVAL_POWER_OF_2: hsa_ven_amd_pcs_configuration_flags_t = 1;
/// @brief PC Sampling configuration flag options
pub type hsa_ven_amd_pcs_configuration_flags_t = ::core::ffi::c_uint;
/** @brief PC Sampling method information
 Used to provide client with list of supported PC Sampling methods*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_ven_amd_pcs_configuration_t {
    pub method: hsa_ven_amd_pcs_method_kind_t,
    pub units: hsa_ven_amd_pcs_units_t,
    pub min_interval: usize,
    pub max_interval: usize,
    pub flags: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_pcs_configuration_t",
    ][::core::mem::size_of::<hsa_ven_amd_pcs_configuration_t>() - 32usize];
    [
        "Alignment of hsa_ven_amd_pcs_configuration_t",
    ][::core::mem::align_of::<hsa_ven_amd_pcs_configuration_t>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_pcs_configuration_t::method",
    ][::core::mem::offset_of!(hsa_ven_amd_pcs_configuration_t, method) - 0usize];
    [
        "Offset of field: hsa_ven_amd_pcs_configuration_t::units",
    ][::core::mem::offset_of!(hsa_ven_amd_pcs_configuration_t, units) - 4usize];
    [
        "Offset of field: hsa_ven_amd_pcs_configuration_t::min_interval",
    ][::core::mem::offset_of!(hsa_ven_amd_pcs_configuration_t, min_interval) - 8usize];
    [
        "Offset of field: hsa_ven_amd_pcs_configuration_t::max_interval",
    ][::core::mem::offset_of!(hsa_ven_amd_pcs_configuration_t, max_interval) - 16usize];
    [
        "Offset of field: hsa_ven_amd_pcs_configuration_t::flags",
    ][::core::mem::offset_of!(hsa_ven_amd_pcs_configuration_t, flags) - 24usize];
};
impl Default for hsa_ven_amd_pcs_configuration_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** @brief Callback function to iterate through list of supported PC Sampling configurations

 @param[in] configuration one entry for supported PC Sampling method and configuration options
 @param[in] callback_data client private callback data that was passed in when calling
 hsa_ven_amd_pcs_iterate_configuration*/
pub type hsa_ven_amd_pcs_iterate_configuration_callback_t = ::core::option::Option<
    unsafe extern "C" fn(
        configuration: *const hsa_ven_amd_pcs_configuration_t,
        callback_data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t,
>;
/** @brief The function pointer table for the PC Sampling v1.00 extension. Can be returned by
 ::hsa_system_get_extension_table or ::hsa_system_get_major_extension_table.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ven_amd_pc_sampling_1_00_pfn_t {
    pub hsa_ven_amd_pcs_iterate_configuration: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            configuration_callback: hsa_ven_amd_pcs_iterate_configuration_callback_t,
            callback_data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_pcs_create: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            method: hsa_ven_amd_pcs_method_kind_t,
            units: hsa_ven_amd_pcs_units_t,
            interval: usize,
            latency: usize,
            buffer_size: usize,
            data_ready_callback: hsa_ven_amd_pcs_data_ready_callback_t,
            client_callback_data: *mut ::core::ffi::c_void,
            pc_sampling: *mut hsa_ven_amd_pcs_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_pcs_create_from_id: ::core::option::Option<
        unsafe extern "C" fn(
            pcs_id: u32,
            agent: hsa_agent_t,
            method: hsa_ven_amd_pcs_method_kind_t,
            units: hsa_ven_amd_pcs_units_t,
            interval: usize,
            latency: usize,
            buffer_size: usize,
            data_ready_callback: hsa_ven_amd_pcs_data_ready_callback_t,
            client_callback_data: *mut ::core::ffi::c_void,
            pc_sampling: *mut hsa_ven_amd_pcs_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_pcs_destroy: ::core::option::Option<
        unsafe extern "C" fn(pc_sampling: hsa_ven_amd_pcs_t) -> hsa_status_t,
    >,
    pub hsa_ven_amd_pcs_start: ::core::option::Option<
        unsafe extern "C" fn(pc_sampling: hsa_ven_amd_pcs_t) -> hsa_status_t,
    >,
    pub hsa_ven_amd_pcs_stop: ::core::option::Option<
        unsafe extern "C" fn(pc_sampling: hsa_ven_amd_pcs_t) -> hsa_status_t,
    >,
    pub hsa_ven_amd_pcs_flush: ::core::option::Option<
        unsafe extern "C" fn(pc_sampling: hsa_ven_amd_pcs_t) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_pc_sampling_1_00_pfn_t",
    ][::core::mem::size_of::<hsa_ven_amd_pc_sampling_1_00_pfn_t>() - 56usize];
    [
        "Alignment of hsa_ven_amd_pc_sampling_1_00_pfn_t",
    ][::core::mem::align_of::<hsa_ven_amd_pc_sampling_1_00_pfn_t>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_iterate_configuration",
    ][::core::mem::offset_of!(
        hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_iterate_configuration
    ) - 0usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_create",
    ][::core::mem::offset_of!(hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_create)
        - 8usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_create_from_id",
    ][::core::mem::offset_of!(
        hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_create_from_id
    ) - 16usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_destroy",
    ][::core::mem::offset_of!(
        hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_destroy
    ) - 24usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_start",
    ][::core::mem::offset_of!(hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_start)
        - 32usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_stop",
    ][::core::mem::offset_of!(hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_stop)
        - 40usize];
    [
        "Offset of field: hsa_ven_amd_pc_sampling_1_00_pfn_t::hsa_ven_amd_pcs_flush",
    ][::core::mem::offset_of!(hsa_ven_amd_pc_sampling_1_00_pfn_t, hsa_ven_amd_pcs_flush)
        - 48usize];
};
/// @brief A fixed-size type used to represent ::hsa_signal_condition_t constants.
pub type hsa_signal_condition32_t = u32;
/** Packet used by agents to delay processing of subsequent packets until a
 configurable condition is satisfied by an HSA signal.  Only kernel dispatch
 queues created from AMD GPU Agents support this packet.*/
pub const HSA_AMD_PACKET_TYPE_BARRIER_VALUE: hsa_amd_packet_type_t = 2;
/** Packet used to send commands to an AIE agent's embedded runtime (ERT). The
 ERT is responsible for, among other things, handling dispatches. Only
 queues created on AIE agents support this packet.*/
pub const HSA_AMD_PACKET_TYPE_AIE_ERT: hsa_amd_packet_type_t = 3;
/// @brief AMD vendor specific packet type.
pub type hsa_amd_packet_type_t = ::core::ffi::c_uint;
/// @brief A fixed-size type used to represent ::hsa_amd_packet_type_t constants.
pub type hsa_amd_packet_type8_t = u8;
/// @brief AMD vendor specific AQL packet header
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_packet_header_s {
    /** Packet header. Used to configure multiple packet parameters such as the
 packet type. The parameters are described by ::hsa_packet_header_t.*/
    pub header: u16,
    /// Format of the vendor specific packet.
    pub AmdFormat: hsa_amd_packet_type8_t,
    /// Reserved. Must be 0.
    pub reserved: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_packet_header_s",
    ][::core::mem::size_of::<hsa_amd_packet_header_s>() - 4usize];
    [
        "Alignment of hsa_amd_packet_header_s",
    ][::core::mem::align_of::<hsa_amd_packet_header_s>() - 2usize];
    [
        "Offset of field: hsa_amd_packet_header_s::header",
    ][::core::mem::offset_of!(hsa_amd_packet_header_s, header) - 0usize];
    [
        "Offset of field: hsa_amd_packet_header_s::AmdFormat",
    ][::core::mem::offset_of!(hsa_amd_packet_header_s, AmdFormat) - 2usize];
    [
        "Offset of field: hsa_amd_packet_header_s::reserved",
    ][::core::mem::offset_of!(hsa_amd_packet_header_s, reserved) - 3usize];
};
/// @brief AMD vendor specific AQL packet header
pub type hsa_amd_vendor_packet_header_t = hsa_amd_packet_header_s;
/** @brief AMD barrier value packet.  Halts packet processing and waits for
 (signal_value & ::mask) ::cond ::value to be satisfied, where signal_value
 is the value of the signal ::signal.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_barrier_value_packet_s {
    /// AMD vendor specific packet header.
    pub header: hsa_amd_vendor_packet_header_t,
    /// Reserved. Must be 0.
    pub reserved0: u32,
    /** Dependent signal object. A signal with a handle value of 0 is
 allowed and is interpreted by the packet processor a satisfied
 dependency.*/
    pub signal: hsa_signal_t,
    /// Value to compare against.
    pub value: hsa_signal_value_t,
    /// Bit mask to be combined by bitwise AND with ::signal's value.
    pub mask: hsa_signal_value_t,
    /// Comparison operation.  See ::hsa_signal_condition_t.
    pub cond: hsa_signal_condition32_t,
    /// Reserved. Must be 0.
    pub reserved1: u32,
    /// Reserved. Must be 0.
    pub reserved2: u64,
    /// Reserved. Must be 0.
    pub reserved3: u64,
    /** Signal used to indicate completion of the job. The application can use the
 special signal handle 0 to indicate that no signal is used.*/
    pub completion_signal: hsa_signal_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_barrier_value_packet_s",
    ][::core::mem::size_of::<hsa_amd_barrier_value_packet_s>() - 64usize];
    [
        "Alignment of hsa_amd_barrier_value_packet_s",
    ][::core::mem::align_of::<hsa_amd_barrier_value_packet_s>() - 8usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::header",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, header) - 0usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::reserved0",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, reserved0) - 4usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::signal",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, signal) - 8usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::value",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, value) - 16usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::mask",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, mask) - 24usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::cond",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, cond) - 32usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::reserved1",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, reserved1) - 36usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::reserved2",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, reserved2) - 40usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::reserved3",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, reserved3) - 48usize];
    [
        "Offset of field: hsa_amd_barrier_value_packet_s::completion_signal",
    ][::core::mem::offset_of!(hsa_amd_barrier_value_packet_s, completion_signal)
        - 56usize];
};
/** @brief AMD barrier value packet.  Halts packet processing and waits for
 (signal_value & ::mask) ::cond ::value to be satisfied, where signal_value
 is the value of the signal ::signal.*/
pub type hsa_amd_barrier_value_packet_t = hsa_amd_barrier_value_packet_s;
/// Set by the host before submitting a command to the scheduler.
pub const HSA_AMD_AIE_ERT_STATE_NEW: hsa_amd_aie_ert_state = 1;
/// Internal scheduler state.
pub const HSA_AMD_AIE_ERT_STATE_QUEUED: hsa_amd_aie_ert_state = 2;
/// Internal scheduler state.
pub const HSA_AMD_AIE_ERT_STATE_RUNNING: hsa_amd_aie_ert_state = 3;
/// Set by the scheduler when a command completes.
pub const HSA_AMD_AIE_ERT_STATE_COMPLETED: hsa_amd_aie_ert_state = 4;
/// Set by the scheduler if a command failed.
pub const HSA_AMD_AIE_ERT_STATE_ERROR: hsa_amd_aie_ert_state = 5;
/// Set by the scheduler if a command aborted.
pub const HSA_AMD_AIE_ERT_STATE_ABORT: hsa_amd_aie_ert_state = 6;
/// Internal scheduler state.
pub const HSA_AMD_AIE_ERT_STATE_SUBMITTED: hsa_amd_aie_ert_state = 7;
/// Set by the scheduler on a timeout and reset.
pub const HSA_AMD_AIE_ERT_STATE_TIMEOUT: hsa_amd_aie_ert_state = 8;
/// Set by the scheduler on a timeout and fail to reset.
pub const HSA_AMD_AIE_ERT_STATE_NORESPONSE: hsa_amd_aie_ert_state = 9;
/// Set by the scheduler on a timeout and fail to reset.
pub const HSA_AMD_AIE_ERT_STATE_SKERROR: hsa_amd_aie_ert_state = 10;
/// Set by the scheduler on a timeout and fail to reset.
pub const HSA_AMD_AIE_ERT_STATE_SKCRASHED: hsa_amd_aie_ert_state = 11;
/// Set by the scheduler on a timeout and fail to reset.
pub const HSA_AMD_AIE_ERT_STATE_MAX: hsa_amd_aie_ert_state = 12;
/// State of an AIE ERT command.
pub type hsa_amd_aie_ert_state = ::core::ffi::c_uint;
/// Start a workgroup on a compute unit (CU).
pub const HSA_AMD_AIE_ERT_START_CU: hsa_amd_aie_ert_cmd_opcode_t = 0;
/// Currently aliased to HSA_AMD_AIE_ERT_START_CU.
pub const HSA_AMD_AIE_ERT_START_KERNEL: hsa_amd_aie_ert_cmd_opcode_t = 0;
/// Configure command scheduler.
pub const HSA_AMD_AIE_ERT_CONFIGURE: hsa_amd_aie_ert_cmd_opcode_t = 2;
/// Configure command scheduler.
pub const HSA_AMD_AIE_ERT_EXIT: hsa_amd_aie_ert_cmd_opcode_t = 3;
/// Configure command scheduler.
pub const HSA_AMD_AIE_ERT_ABORT: hsa_amd_aie_ert_cmd_opcode_t = 4;
/// Execute a specified CU after writing.
pub const HSA_AMD_AIE_ERT_EXEC_WRITE: hsa_amd_aie_ert_cmd_opcode_t = 5;
/// Get stats about a CU's execution.
pub const HSA_AMD_AIE_ERT_CU_STAT: hsa_amd_aie_ert_cmd_opcode_t = 6;
/// Start KDMA CU or P2P.
pub const HSA_AMD_AIE_ERT_START_COPYBO: hsa_amd_aie_ert_cmd_opcode_t = 7;
/// Configure a soft kernel.
pub const HSA_AMD_AIE_ERT_SK_CONFIG: hsa_amd_aie_ert_cmd_opcode_t = 8;
/// Start a soft kernel.
pub const HSA_AMD_AIE_ERT_SK_START: hsa_amd_aie_ert_cmd_opcode_t = 9;
/// Unconfigure a soft kernel.
pub const HSA_AMD_AIE_ERT_SK_UNCONFIG: hsa_amd_aie_ert_cmd_opcode_t = 10;
/// Initialize a CU.
pub const HSA_AMD_AIE_ERT_INIT_CU: hsa_amd_aie_ert_cmd_opcode_t = 11;
/// Initialize a CU.
pub const HSA_AMD_AIE_ERT_START_FA: hsa_amd_aie_ert_cmd_opcode_t = 12;
/// Initialize a CU.
pub const HSA_AMD_AIE_ERT_CLK_CALIB: hsa_amd_aie_ert_cmd_opcode_t = 13;
/// Initialize a CU.
pub const HSA_AMD_AIE_ERT_MB_VALIDATE: hsa_amd_aie_ert_cmd_opcode_t = 14;
/// Same as HSA_AMD_AIE_ERT_START_CU but with a key-value pair.
pub const HSA_AMD_AIE_ERT_START_KEY_VAL: hsa_amd_aie_ert_cmd_opcode_t = 15;
/// Same as HSA_AMD_AIE_ERT_START_CU but with a key-value pair.
pub const HSA_AMD_AIE_ERT_ACCESS_TEST_C: hsa_amd_aie_ert_cmd_opcode_t = 16;
/// Same as HSA_AMD_AIE_ERT_START_CU but with a key-value pair.
pub const HSA_AMD_AIE_ERT_ACCESS_TEST: hsa_amd_aie_ert_cmd_opcode_t = 17;
/// Instruction buffer command format.
pub const HSA_AMD_AIE_ERT_START_DPU: hsa_amd_aie_ert_cmd_opcode_t = 18;
/// Command chain.
pub const HSA_AMD_AIE_ERT_CMD_CHAIN: hsa_amd_aie_ert_cmd_opcode_t = 19;
/// Instruction buffer command format on NPU.
pub const HSA_AMD_AIE_ERT_START_NPU: hsa_amd_aie_ert_cmd_opcode_t = 20;
/// Instruction buffer command with pre-emption format on the NPU.
pub const HSA_AMD_AIE_ERT_START_NPU_PREEMPT: hsa_amd_aie_ert_cmd_opcode_t = 21;
/// Opcode types for HSA AIE ERT commands.
pub type hsa_amd_aie_ert_cmd_opcode_t = ::core::ffi::c_uint;
/** Payload data for AIE ERT start kernel packets (i.e., when the opcode is
 HSA_AMD_AIE_ERT_START_KERNEL).*/
#[repr(C)]
#[derive(Debug)]
pub struct hsa_amd_aie_ert_start_kernel_data_s {
    /// Address to the PDI.
    pub pdi_addr: *mut ::core::ffi::c_void,
    /// Opcode, instructions and kernel arguments.
    pub data: __IncompleteArrayField<u32>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_aie_ert_start_kernel_data_s",
    ][::core::mem::size_of::<hsa_amd_aie_ert_start_kernel_data_s>() - 8usize];
    [
        "Alignment of hsa_amd_aie_ert_start_kernel_data_s",
    ][::core::mem::align_of::<hsa_amd_aie_ert_start_kernel_data_s>() - 8usize];
    [
        "Offset of field: hsa_amd_aie_ert_start_kernel_data_s::pdi_addr",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_start_kernel_data_s, pdi_addr) - 0usize];
    [
        "Offset of field: hsa_amd_aie_ert_start_kernel_data_s::data",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_start_kernel_data_s, data) - 8usize];
};
impl Default for hsa_amd_aie_ert_start_kernel_data_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** Payload data for AIE ERT start kernel packets (i.e., when the opcode is
 HSA_AMD_AIE_ERT_START_KERNEL).*/
pub type hsa_amd_aie_ert_start_kernel_data_t = hsa_amd_aie_ert_start_kernel_data_s;
/// AMD AIE ERT packet. Used for sending a command to an AIE agent.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_aie_ert_packet_s {
    /// AMD vendor specific packet header.
    pub header: hsa_amd_vendor_packet_header_t,
    pub __bindgen_anon_1: hsa_amd_aie_ert_packet_s__bindgen_ty_1,
    /// Reserved. Must be 0.
    pub reserved0: u64,
    /// Reserved. Must be 0.
    pub reserved1: u64,
    /// Reserved. Must be 0.
    pub reserved2: u64,
    /// Reserved. Must be 0.
    pub reserved3: u64,
    /// Reserved. Must be 0.
    pub reserved4: u64,
    /// Reserved. Must be 0.
    pub reserved5: u64,
    /** Address of packet data payload. ERT commands contain arbitrarily sized
 data payloads.*/
    pub payload_data: u64,
}
/** Format for packets interpreted by the ERT to understand the command and
 payload data.*/
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_aie_ert_packet_s__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_aie_ert_packet_s__bindgen_ty_1",
    ][::core::mem::size_of::<hsa_amd_aie_ert_packet_s__bindgen_ty_1>() - 4usize];
    [
        "Alignment of hsa_amd_aie_ert_packet_s__bindgen_ty_1",
    ][::core::mem::align_of::<hsa_amd_aie_ert_packet_s__bindgen_ty_1>() - 4usize];
};
impl hsa_amd_aie_ert_packet_s__bindgen_ty_1 {
    #[inline]
    pub fn state(&self) -> u32 {
        self._bitfield_1.get(0usize, 4u8) as u32
    }
    #[inline]
    pub fn set_state(&mut self, val: u32) {
        {
            let val: u32 = val;
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn state_raw(this: *const Self) -> u32 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 0usize, 4u8) as u32
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_state_raw(this: *mut Self, val: u32) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u32 = val;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn custom(&self) -> u32 {
        self._bitfield_1.get(4usize, 8u8) as u32
    }
    #[inline]
    pub fn set_custom(&mut self, val: u32) {
        {
            let val: u32 = val;
            self._bitfield_1.set(4usize, 8u8, val as u64)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn custom_raw(this: *const Self) -> u32 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 4usize, 8u8) as u32
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_custom_raw(this: *mut Self, val: u32) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u32 = val;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn count(&self) -> u32 {
        self._bitfield_1.get(12usize, 11u8) as u32
    }
    #[inline]
    pub fn set_count(&mut self, val: u32) {
        {
            let val: u32 = val;
            self._bitfield_1.set(12usize, 11u8, val as u64)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn count_raw(this: *const Self) -> u32 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 12usize, 11u8) as u32
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_count_raw(this: *mut Self, val: u32) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u32 = val;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                12usize,
                11u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn opcode(&self) -> u32 {
        self._bitfield_1.get(23usize, 5u8) as u32
    }
    #[inline]
    pub fn set_opcode(&mut self, val: u32) {
        {
            let val: u32 = val;
            self._bitfield_1.set(23usize, 5u8, val as u64)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn opcode_raw(this: *const Self) -> u32 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 23usize, 5u8) as u32
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_opcode_raw(this: *mut Self, val: u32) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u32 = val;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                23usize,
                5u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn type_(&self) -> u32 {
        self._bitfield_1.get(28usize, 4u8) as u32
    }
    #[inline]
    pub fn set_type(&mut self, val: u32) {
        {
            let val: u32 = val;
            self._bitfield_1.set(28usize, 4u8, val as u64)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn type__raw(this: *const Self) -> u32 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 28usize, 4u8) as u32
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_type_raw(this: *mut Self, val: u32) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u32 = val;
            <__BindgenBitfieldUnit<
                [u8; 4usize],
            >>::raw_set(
                ::core::ptr::addr_of_mut!((*this)._bitfield_1),
                28usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        state: u32,
        custom: u32,
        count: u32,
        opcode: u32,
        type_: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                4u8,
                {
                    let state: u32 = state;
                    state as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                4usize,
                8u8,
                {
                    let custom: u32 = custom;
                    custom as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                12usize,
                11u8,
                {
                    let count: u32 = count;
                    count as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                23usize,
                5u8,
                {
                    let opcode: u32 = opcode;
                    opcode as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                28usize,
                4u8,
                {
                    let type_: u32 = type_;
                    type_ as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_aie_ert_packet_s",
    ][::core::mem::size_of::<hsa_amd_aie_ert_packet_s>() - 64usize];
    [
        "Alignment of hsa_amd_aie_ert_packet_s",
    ][::core::mem::align_of::<hsa_amd_aie_ert_packet_s>() - 8usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::header",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, header) - 0usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::reserved0",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, reserved0) - 8usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::reserved1",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, reserved1) - 16usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::reserved2",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, reserved2) - 24usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::reserved3",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, reserved3) - 32usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::reserved4",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, reserved4) - 40usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::reserved5",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, reserved5) - 48usize];
    [
        "Offset of field: hsa_amd_aie_ert_packet_s::payload_data",
    ][::core::mem::offset_of!(hsa_amd_aie_ert_packet_s, payload_data) - 56usize];
};
/// AMD AIE ERT packet. Used for sending a command to an AIE agent.
pub type hsa_amd_aie_ert_packet_t = hsa_amd_aie_ert_packet_s;
/// The memory pool is invalid.
pub const HSA_STATUS_ERROR_INVALID_MEMORY_POOL: _bindgen_ty_20 = 40;
/// Agent accessed memory beyond the maximum legal address.
pub const HSA_STATUS_ERROR_MEMORY_APERTURE_VIOLATION: _bindgen_ty_20 = 41;
/// Agent executed an invalid shader instruction.
pub const HSA_STATUS_ERROR_ILLEGAL_INSTRUCTION: _bindgen_ty_20 = 42;
/** Agent attempted to access an inaccessible address.
 See hsa_amd_register_system_event_handler and
 HSA_AMD_GPU_MEMORY_FAULT_EVENT for more information on illegal accesses.*/
pub const HSA_STATUS_ERROR_MEMORY_FAULT: _bindgen_ty_20 = 43;
/** The CU mask was successfully set but the mask attempted to enable a CU
 which was disabled for the process.  CUs disabled for the process remain
 disabled.*/
pub const HSA_STATUS_CU_MASK_REDUCED: _bindgen_ty_20 = 44;
/// Exceeded number of VGPRs available on this agent
pub const HSA_STATUS_ERROR_OUT_OF_REGISTERS: _bindgen_ty_20 = 45;
/// Resource is busy or temporarily unavailable
pub const HSA_STATUS_ERROR_RESOURCE_BUSY: _bindgen_ty_20 = 46;
/// Request is not supported by this system
pub const HSA_STATUS_ERROR_NOT_SUPPORTED: _bindgen_ty_20 = 47;
/** @brief Enumeration constants added to ::hsa_status_t.

 @remark Additions to hsa_status_t*/
pub type _bindgen_ty_20 = ::core::ffi::c_uint;
/// IOMMU not supported
pub const HSA_IOMMU_SUPPORT_NONE: hsa_amd_iommu_version_t = 0;
/// IOMMU V2 supported
pub const HSA_IOMMU_SUPPORT_V2: hsa_amd_iommu_version_t = 1;
/// @brief IOMMU version supported
pub type hsa_amd_iommu_version_t = ::core::ffi::c_uint;
/// @brief Structure containing information on the agent's clock counters.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_clock_counters_s {
    pub gpu_clock_counter: u64,
    pub cpu_clock_counter: u64,
    pub system_clock_counter: u64,
    pub system_clock_frequency: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_clock_counters_s",
    ][::core::mem::size_of::<hsa_amd_clock_counters_s>() - 32usize];
    [
        "Alignment of hsa_amd_clock_counters_s",
    ][::core::mem::align_of::<hsa_amd_clock_counters_s>() - 8usize];
    [
        "Offset of field: hsa_amd_clock_counters_s::gpu_clock_counter",
    ][::core::mem::offset_of!(hsa_amd_clock_counters_s, gpu_clock_counter) - 0usize];
    [
        "Offset of field: hsa_amd_clock_counters_s::cpu_clock_counter",
    ][::core::mem::offset_of!(hsa_amd_clock_counters_s, cpu_clock_counter) - 8usize];
    [
        "Offset of field: hsa_amd_clock_counters_s::system_clock_counter",
    ][::core::mem::offset_of!(hsa_amd_clock_counters_s, system_clock_counter) - 16usize];
    [
        "Offset of field: hsa_amd_clock_counters_s::system_clock_frequency",
    ][::core::mem::offset_of!(hsa_amd_clock_counters_s, system_clock_frequency)
        - 24usize];
};
/// @brief Structure containing information on the agent's clock counters.
pub type hsa_amd_clock_counters_t = hsa_amd_clock_counters_s;
/// Chip identifier. The type of this attribute is uint32_t.
pub const HSA_AMD_AGENT_INFO_CHIP_ID: hsa_amd_agent_info_s = 40960;
/// Size of a cacheline in bytes. The type of this attribute is uint32_t.
pub const HSA_AMD_AGENT_INFO_CACHELINE_SIZE: hsa_amd_agent_info_s = 40961;
/** The number of compute unit available in the agent. The type of this
 attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_COMPUTE_UNIT_COUNT: hsa_amd_agent_info_s = 40962;
/** The maximum clock frequency of the agent in MHz. The type of this
 attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_MAX_CLOCK_FREQUENCY: hsa_amd_agent_info_s = 40963;
/// Internal driver node identifier. The type of this attribute is uint32_t.
pub const HSA_AMD_AGENT_INFO_DRIVER_NODE_ID: hsa_amd_agent_info_s = 40964;
/** Max number of watch points on memory address ranges to generate exception
 events when the watched addresses are accessed.  The type of this
 attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_MAX_ADDRESS_WATCH_POINTS: hsa_amd_agent_info_s = 40965;
/** Agent BDF_ID, named LocationID in thunk. The type of this attribute is
 uint32_t.*/
pub const HSA_AMD_AGENT_INFO_BDFID: hsa_amd_agent_info_s = 40966;
/** Memory Interface width, the return value type is uint32_t.
 This attribute is deprecated.*/
pub const HSA_AMD_AGENT_INFO_MEMORY_WIDTH: hsa_amd_agent_info_s = 40967;
/// Max Memory Clock, the return value type is uint32_t.
pub const HSA_AMD_AGENT_INFO_MEMORY_MAX_FREQUENCY: hsa_amd_agent_info_s = 40968;
/** Board name of Agent - populated from MarketingName of Kfd Node
 The value is an Ascii string of 64 chars.*/
pub const HSA_AMD_AGENT_INFO_PRODUCT_NAME: hsa_amd_agent_info_s = 40969;
/** Maximum number of waves possible in a Compute Unit.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_MAX_WAVES_PER_CU: hsa_amd_agent_info_s = 40970;
/** Number of SIMD's per compute unit CU
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_NUM_SIMDS_PER_CU: hsa_amd_agent_info_s = 40971;
/** Number of Shader Engines (SE) in Gpu
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_NUM_SHADER_ENGINES: hsa_amd_agent_info_s = 40972;
/** Number of Shader Arrays Per Shader Engines in Gpu
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_NUM_SHADER_ARRAYS_PER_SE: hsa_amd_agent_info_s = 40973;
/** Address of the HDP flush registers.  Use of these registers does not conform to the HSA memory
 model and should be treated with caution.
 The type of this attribute is hsa_amd_hdp_flush_t.*/
pub const HSA_AMD_AGENT_INFO_HDP_FLUSH: hsa_amd_agent_info_s = 40974;
/** PCIe domain for the agent.  Pairs with HSA_AMD_AGENT_INFO_BDFID
 to give the full physical location of the Agent.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_DOMAIN: hsa_amd_agent_info_s = 40975;
/** Queries for support of cooperative queues.  See ::HSA_QUEUE_TYPE_COOPERATIVE.
 The type of this attribute is bool.*/
pub const HSA_AMD_AGENT_INFO_COOPERATIVE_QUEUES: hsa_amd_agent_info_s = 40976;
/** Queries UUID of an agent. The value is an Ascii string with a maximum
 of 21 chars including NUL. The string value consists of two parts: header
 and body. The header identifies device type (GPU, CPU, DSP) while body
 encodes UUID as a 16 digit hex string

 Agents that do not support UUID will return the string "GPU-XX" or
 "CPU-XX" or "DSP-XX" depending upon their device type ::hsa_device_type_t*/
pub const HSA_AMD_AGENT_INFO_UUID: hsa_amd_agent_info_s = 40977;
/** Queries for the ASIC revision of an agent. The value is an integer that
 increments for each revision. This can be used by user-level software to
 change how it operates, depending on the hardware version. This allows
 selective workarounds for hardware errata.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_ASIC_REVISION: hsa_amd_agent_info_s = 40978;
/** Queries whether or not the host can directly access SVM memory that is
 physically resident in the agent's local memory.
 The type of this attribute is bool.*/
pub const HSA_AMD_AGENT_INFO_SVM_DIRECT_HOST_ACCESS: hsa_amd_agent_info_s = 40979;
/** Some processors support more CUs than can reliably be used in a cooperative
 dispatch.  This queries the count of CUs which are fully enabled for
 cooperative dispatch.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_COOPERATIVE_COMPUTE_UNIT_COUNT: hsa_amd_agent_info_s = 40980;
/** Queries the amount of memory available in bytes accross all global pools
 owned by the agent.
 The type of this attribute is uint64_t.*/
pub const HSA_AMD_AGENT_INFO_MEMORY_AVAIL: hsa_amd_agent_info_s = 40981;
/** Timestamp value increase rate, in Hz. The timestamp (clock) frequency is
 in the range 1-400MHz.
 The type of this attribute is uint64_t.*/
pub const HSA_AMD_AGENT_INFO_TIMESTAMP_FREQUENCY: hsa_amd_agent_info_s = 40982;
/** Queries for the ASIC family ID of an agent.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_ASIC_FAMILY_ID: hsa_amd_agent_info_s = 41223;
/** Queries for the Packet Processor(CP Firmware) ucode version of an agent.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_UCODE_VERSION: hsa_amd_agent_info_s = 41224;
/** Queries for the SDMA engine ucode of an agent.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_SDMA_UCODE_VERSION: hsa_amd_agent_info_s = 41225;
/** Queries the number of SDMA engines.
 If HSA_AMD_AGENT_INFO_NUM_SDMA_XGMI_ENG query returns non-zero,
 this query returns the the number of SDMA engines optimized for
 host to device bidirectional traffic.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_NUM_SDMA_ENG: hsa_amd_agent_info_s = 41226;
/** Queries the number of additional SDMA engines optimized for D2D xGMI copies.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_NUM_SDMA_XGMI_ENG: hsa_amd_agent_info_s = 41227;
/** Queries for version of IOMMU supported by agent.
 The type of this attribute is hsa_amd_iommu_version_t.*/
pub const HSA_AMD_AGENT_INFO_IOMMU_SUPPORT: hsa_amd_agent_info_s = 41232;
/** Queries for number of XCCs within the agent.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_NUM_XCC: hsa_amd_agent_info_s = 41233;
/** Queries for driver unique identifier.
 The type of this attribute is uint32_t.*/
pub const HSA_AMD_AGENT_INFO_DRIVER_UID: hsa_amd_agent_info_s = 41234;
/** Returns the hsa_agent_t of the nearest CPU agent
 The type of this attribute is hsa_agent_t.*/
pub const HSA_AMD_AGENT_INFO_NEAREST_CPU: hsa_amd_agent_info_s = 41235;
/** Bit-mask indicating memory properties of this agent. A memory property is set if the flag bit
 is set at that position. User may use the hsa_flag_isset64 macro to verify whether a flag
 is set. The type of this attribute is uint8_t[8].*/
pub const HSA_AMD_AGENT_INFO_MEMORY_PROPERTIES: hsa_amd_agent_info_s = 41236;
/** Bit-mask indicating AQL Extensions supported by this agent. An AQL extension is set if the flag
 bit is set at that position. User may use the hsa_flag_isset64 macro to verify whether a flag
 is set. The type of this attribute is uint8_t[8].*/
pub const HSA_AMD_AGENT_INFO_AQL_EXTENSIONS: hsa_amd_agent_info_s = 41237;
/** Maximum allowed value in bytes for scratch limit for this agent. This amount
 is shared accross all queues created on this agent.
 The type of this attribute is uint64_t.*/
pub const HSA_AMD_AGENT_INFO_SCRATCH_LIMIT_MAX: hsa_amd_agent_info_s = 41238;
/** Current scratch limit threshold in bytes for this agent. This limit can be
 modified using the hsa_amd_agent_set_async_scratch_limit call.
 - AQL dispatches that require scratch-memory above this threshold will trigger a
   scratch use-once.
 - AQL dispatches using less scratch-memory than this threshold, ROCr will
   permanently assign the allocated scratch memory to the queue handling the dispatch.
   This memory can be reclaimed by calling hsa_amd_agent_set_async_scratch_limit
   with a lower threshold by current value.

 The type of this attribute is uint64_t.*/
pub const HSA_AMD_AGENT_INFO_SCRATCH_LIMIT_CURRENT: hsa_amd_agent_info_s = 41239;
/** Queries the driver for clock counters of the agent.
 The type of this attribute is hsa_amd_clock_counters_t.*/
pub const HSA_AMD_AGENT_INFO_CLOCK_COUNTERS: hsa_amd_agent_info_s = 41240;
/// @brief Agent attributes.
pub type hsa_amd_agent_info_s = ::core::ffi::c_uint;
/// @brief Agent attributes.
pub use self::hsa_amd_agent_info_s as hsa_amd_agent_info_t;
pub const HSA_AMD_MEMORY_PROPERTY_AGENT_IS_APU: hsa_amd_agent_memory_properties_s = 1;
/// @brief Agent memory properties attributes
pub type hsa_amd_agent_memory_properties_s = ::core::ffi::c_uint;
/// @brief Agent memory properties attributes
pub use self::hsa_amd_agent_memory_properties_s as hsa_amd_agent_memory_properties_t;
pub const HSA_AMD_SDMA_ENGINE_0: hsa_amd_sdma_engine_id = 1;
pub const HSA_AMD_SDMA_ENGINE_1: hsa_amd_sdma_engine_id = 2;
pub const HSA_AMD_SDMA_ENGINE_2: hsa_amd_sdma_engine_id = 4;
pub const HSA_AMD_SDMA_ENGINE_3: hsa_amd_sdma_engine_id = 8;
pub const HSA_AMD_SDMA_ENGINE_4: hsa_amd_sdma_engine_id = 16;
pub const HSA_AMD_SDMA_ENGINE_5: hsa_amd_sdma_engine_id = 32;
pub const HSA_AMD_SDMA_ENGINE_6: hsa_amd_sdma_engine_id = 64;
pub const HSA_AMD_SDMA_ENGINE_7: hsa_amd_sdma_engine_id = 128;
pub const HSA_AMD_SDMA_ENGINE_8: hsa_amd_sdma_engine_id = 256;
pub const HSA_AMD_SDMA_ENGINE_9: hsa_amd_sdma_engine_id = 512;
pub const HSA_AMD_SDMA_ENGINE_10: hsa_amd_sdma_engine_id = 1024;
pub const HSA_AMD_SDMA_ENGINE_11: hsa_amd_sdma_engine_id = 2048;
pub const HSA_AMD_SDMA_ENGINE_12: hsa_amd_sdma_engine_id = 4096;
pub const HSA_AMD_SDMA_ENGINE_13: hsa_amd_sdma_engine_id = 8192;
pub const HSA_AMD_SDMA_ENGINE_14: hsa_amd_sdma_engine_id = 16384;
pub const HSA_AMD_SDMA_ENGINE_15: hsa_amd_sdma_engine_id = 32768;
/// @brief SDMA engine IDs unique by single set bit position.
pub type hsa_amd_sdma_engine_id = ::core::ffi::c_uint;
/// @brief SDMA engine IDs unique by single set bit position.
pub use self::hsa_amd_sdma_engine_id as hsa_amd_sdma_engine_id_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_amd_hdp_flush_s {
    pub HDP_MEM_FLUSH_CNTL: *mut u32,
    pub HDP_REG_FLUSH_CNTL: *mut u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_hdp_flush_s",
    ][::core::mem::size_of::<hsa_amd_hdp_flush_s>() - 16usize];
    [
        "Alignment of hsa_amd_hdp_flush_s",
    ][::core::mem::align_of::<hsa_amd_hdp_flush_s>() - 8usize];
    [
        "Offset of field: hsa_amd_hdp_flush_s::HDP_MEM_FLUSH_CNTL",
    ][::core::mem::offset_of!(hsa_amd_hdp_flush_s, HDP_MEM_FLUSH_CNTL) - 0usize];
    [
        "Offset of field: hsa_amd_hdp_flush_s::HDP_REG_FLUSH_CNTL",
    ][::core::mem::offset_of!(hsa_amd_hdp_flush_s, HDP_REG_FLUSH_CNTL) - 8usize];
};
impl Default for hsa_amd_hdp_flush_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type hsa_amd_hdp_flush_t = hsa_amd_hdp_flush_s;
/** Determine if host can access the region. The type of this attribute
 is bool.*/
pub const HSA_AMD_REGION_INFO_HOST_ACCESSIBLE: hsa_amd_region_info_s = 40960;
/// Base address of the region in flat address space.
pub const HSA_AMD_REGION_INFO_BASE: hsa_amd_region_info_s = 40961;
/** Memory Interface width, the return value type is uint32_t.
 This attribute is deprecated. Use HSA_AMD_AGENT_INFO_MEMORY_WIDTH.*/
pub const HSA_AMD_REGION_INFO_BUS_WIDTH: hsa_amd_region_info_s = 40962;
/** Max Memory Clock, the return value type is uint32_t.
 This attribute is deprecated. Use HSA_AMD_AGENT_INFO_MEMORY_MAX_FREQUENCY.*/
pub const HSA_AMD_REGION_INFO_MAX_CLOCK_FREQUENCY: hsa_amd_region_info_s = 40963;
pub type hsa_amd_region_info_s = ::core::ffi::c_uint;
pub use self::hsa_amd_region_info_s as hsa_amd_region_info_t;
/// Coherent region.
pub const HSA_AMD_COHERENCY_TYPE_COHERENT: hsa_amd_coherency_type_s = 0;
/// Non coherent region.
pub const HSA_AMD_COHERENCY_TYPE_NONCOHERENT: hsa_amd_coherency_type_s = 1;
/// @brief Coherency attributes of fine grain region.
pub type hsa_amd_coherency_type_s = ::core::ffi::c_uint;
/// @brief Coherency attributes of fine grain region.
pub use self::hsa_amd_coherency_type_s as hsa_amd_coherency_type_t;
pub const HSA_AMD_DMABUF_MAPPING_TYPE_NONE: hsa_amd_dma_buf_mapping_type_s = 0;
pub const HSA_AMD_DMABUF_MAPPING_TYPE_PCIE: hsa_amd_dma_buf_mapping_type_s = 1;
pub type hsa_amd_dma_buf_mapping_type_s = ::core::ffi::c_uint;
pub use self::hsa_amd_dma_buf_mapping_type_s as hsa_amd_dma_buf_mapping_type_t;
/** @brief Structure containing profiling dispatch time information.

 Times are reported as ticks in the domain of the HSA system clock.
 The HSA system clock tick and frequency is obtained via hsa_system_get_info.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_profiling_dispatch_time_s {
    /// Dispatch packet processing start time.
    pub start: u64,
    /// Dispatch packet completion time.
    pub end: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_profiling_dispatch_time_s",
    ][::core::mem::size_of::<hsa_amd_profiling_dispatch_time_s>() - 16usize];
    [
        "Alignment of hsa_amd_profiling_dispatch_time_s",
    ][::core::mem::align_of::<hsa_amd_profiling_dispatch_time_s>() - 8usize];
    [
        "Offset of field: hsa_amd_profiling_dispatch_time_s::start",
    ][::core::mem::offset_of!(hsa_amd_profiling_dispatch_time_s, start) - 0usize];
    [
        "Offset of field: hsa_amd_profiling_dispatch_time_s::end",
    ][::core::mem::offset_of!(hsa_amd_profiling_dispatch_time_s, end) - 8usize];
};
/** @brief Structure containing profiling dispatch time information.

 Times are reported as ticks in the domain of the HSA system clock.
 The HSA system clock tick and frequency is obtained via hsa_system_get_info.*/
pub type hsa_amd_profiling_dispatch_time_t = hsa_amd_profiling_dispatch_time_s;
/** @brief Structure containing profiling async copy time information.

 Times are reported as ticks in the domain of the HSA system clock.
 The HSA system clock tick and frequency is obtained via hsa_system_get_info.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_profiling_async_copy_time_s {
    /// Async copy processing start time.
    pub start: u64,
    /// Async copy completion time.
    pub end: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_profiling_async_copy_time_s",
    ][::core::mem::size_of::<hsa_amd_profiling_async_copy_time_s>() - 16usize];
    [
        "Alignment of hsa_amd_profiling_async_copy_time_s",
    ][::core::mem::align_of::<hsa_amd_profiling_async_copy_time_s>() - 8usize];
    [
        "Offset of field: hsa_amd_profiling_async_copy_time_s::start",
    ][::core::mem::offset_of!(hsa_amd_profiling_async_copy_time_s, start) - 0usize];
    [
        "Offset of field: hsa_amd_profiling_async_copy_time_s::end",
    ][::core::mem::offset_of!(hsa_amd_profiling_async_copy_time_s, end) - 8usize];
};
/** @brief Structure containing profiling async copy time information.

 Times are reported as ticks in the domain of the HSA system clock.
 The HSA system clock tick and frequency is obtained via hsa_system_get_info.*/
pub type hsa_amd_profiling_async_copy_time_t = hsa_amd_profiling_async_copy_time_s;
/** Signal will only be consumed by AMD GPUs.  Limits signal consumption to
 AMD GPU agents only.  Ignored if @p num_consumers is not zero (all agents).*/
pub const HSA_AMD_SIGNAL_AMD_GPU_ONLY: hsa_amd_signal_attribute_t = 1;
/** Signal may be used for interprocess communication.
 IPC signals can be read, written, and waited on from any process.
 Profiling using an IPC enabled signal is only supported in a single process
 at a time.  Producing profiling data in one process and consuming it in
 another process is undefined.*/
pub const HSA_AMD_SIGNAL_IPC: hsa_amd_signal_attribute_t = 2;
/// @brief Signal attribute flags.
pub type hsa_amd_signal_attribute_t = ::core::ffi::c_uint;
/** @brief Asyncronous signal handler function type.

 @details Type definition of callback function to be used with
 hsa_amd_signal_async_handler. This callback is invoked if the associated
 signal and condition are met. The callback receives the value of the signal
 which satisfied the associated wait condition and a user provided value. If
 the callback returns true then the callback will be called again if the
 associated signal and condition are satisfied again. If the callback returns
 false then it will not be called again.

 @param[in] value Contains the value of the signal observed by
 hsa_amd_signal_async_handler which caused the signal handler to be invoked.

 @param[in] arg Contains the user provided value given when the signal handler
 was registered with hsa_amd_signal_async_handler

 @retval true resumes monitoring the signal with this handler (as if calling
 hsa_amd_signal_async_handler again with identical parameters)

 @retval false stops monitoring the signal with this handler (handler will
 not be called again for this signal)
*/
pub type hsa_amd_signal_handler = ::core::option::Option<
    unsafe extern "C" fn(
        value: hsa_signal_value_t,
        arg: *mut ::core::ffi::c_void,
    ) -> bool,
>;
/** @brief Encodes an opaque vendor specific image format.  The length of data
 depends on the underlying format.  This structure must not be copied as its
 true length can not be determined.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_image_descriptor_s {
    pub version: u32,
    pub deviceID: u32,
    pub data: [u32; 1usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_image_descriptor_s",
    ][::core::mem::size_of::<hsa_amd_image_descriptor_s>() - 12usize];
    [
        "Alignment of hsa_amd_image_descriptor_s",
    ][::core::mem::align_of::<hsa_amd_image_descriptor_s>() - 4usize];
    [
        "Offset of field: hsa_amd_image_descriptor_s::version",
    ][::core::mem::offset_of!(hsa_amd_image_descriptor_s, version) - 0usize];
    [
        "Offset of field: hsa_amd_image_descriptor_s::deviceID",
    ][::core::mem::offset_of!(hsa_amd_image_descriptor_s, deviceID) - 4usize];
    [
        "Offset of field: hsa_amd_image_descriptor_s::data",
    ][::core::mem::offset_of!(hsa_amd_image_descriptor_s, data) - 8usize];
};
/** @brief Encodes an opaque vendor specific image format.  The length of data
 depends on the underlying format.  This structure must not be copied as its
 true length can not be determined.*/
pub type hsa_amd_image_descriptor_t = hsa_amd_image_descriptor_s;
/// Global segment. Used to hold data that is shared by all agents.
pub const HSA_AMD_SEGMENT_GLOBAL: hsa_amd_segment_t = 0;
/** Read-only segment. Used to hold data that remains constant during the
 execution of a kernel.*/
pub const HSA_AMD_SEGMENT_READONLY: hsa_amd_segment_t = 1;
/// Private segment. Used to hold data that is local to a single work-item.
pub const HSA_AMD_SEGMENT_PRIVATE: hsa_amd_segment_t = 2;
/** Group segment. Used to hold data that is shared by the work-items of a
 work-group.*/
pub const HSA_AMD_SEGMENT_GROUP: hsa_amd_segment_t = 3;
/// @brief Memory segments associated with a memory pool.
pub type hsa_amd_segment_t = ::core::ffi::c_uint;
/** @brief A memory pool encapsulates physical storage on an agent
 along with a memory access model.

 @details A memory pool encapsulates a physical partition of an agent's
 memory system along with a memory access model.  Division of a single
 memory system into separate pools allows querying each partition's access
 path properties (see ::hsa_amd_agent_memory_pool_get_info). Allocations
 from a pool are preferentially bound to that pool's physical partition.
 Binding to the pool's preferential physical partition may not be
 possible or persistent depending on the system's memory policy
 and/or state which is beyond the scope of HSA APIs.

 For example, a multi-node NUMA memory system may be represented by multiple
 pool's with each pool providing size and access path information for the
 partition it represents.  Allocations from a pool are preferentially bound
 to the pool's partition (which in this example is a NUMA node) while
 following its memory access model. The actual placement may vary or migrate
 due to the system's NUMA policy and state, which is beyond the scope of
 HSA APIs.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_memory_pool_s {
    /// Opaque handle.
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_memory_pool_s",
    ][::core::mem::size_of::<hsa_amd_memory_pool_s>() - 8usize];
    [
        "Alignment of hsa_amd_memory_pool_s",
    ][::core::mem::align_of::<hsa_amd_memory_pool_s>() - 8usize];
    [
        "Offset of field: hsa_amd_memory_pool_s::handle",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_s, handle) - 0usize];
};
/** @brief A memory pool encapsulates physical storage on an agent
 along with a memory access model.

 @details A memory pool encapsulates a physical partition of an agent's
 memory system along with a memory access model.  Division of a single
 memory system into separate pools allows querying each partition's access
 path properties (see ::hsa_amd_agent_memory_pool_get_info). Allocations
 from a pool are preferentially bound to that pool's physical partition.
 Binding to the pool's preferential physical partition may not be
 possible or persistent depending on the system's memory policy
 and/or state which is beyond the scope of HSA APIs.

 For example, a multi-node NUMA memory system may be represented by multiple
 pool's with each pool providing size and access path information for the
 partition it represents.  Allocations from a pool are preferentially bound
 to the pool's partition (which in this example is a NUMA node) while
 following its memory access model. The actual placement may vary or migrate
 due to the system's NUMA policy and state, which is beyond the scope of
 HSA APIs.*/
pub type hsa_amd_memory_pool_t = hsa_amd_memory_pool_s;
/** The application can use allocations in the memory pool to store kernel
 arguments, and provide the values for the kernarg segment of
 a kernel dispatch.*/
pub const HSA_AMD_MEMORY_POOL_GLOBAL_FLAG_KERNARG_INIT: hsa_amd_memory_pool_global_flag_s = 1;
/** Updates to memory in this pool conform to HSA memory consistency model.
 If this flag is set, then ::HSA_AMD_MEMORY_POOL_GLOBAL_FLAG_COARSE_GRAINED
 must not be set.*/
pub const HSA_AMD_MEMORY_POOL_GLOBAL_FLAG_FINE_GRAINED: hsa_amd_memory_pool_global_flag_s = 2;
/// Writes to memory in this pool can be performed by a single agent at a time.
pub const HSA_AMD_MEMORY_POOL_GLOBAL_FLAG_COARSE_GRAINED: hsa_amd_memory_pool_global_flag_s = 4;
/** Updates to memory in this memory pool have extended scope, acting as
 system-scope atomics for variables in memory regions of this type.
 Note: On non-compliant systems, device-specific actions may be required
 for system-scope coherence.*/
pub const HSA_AMD_MEMORY_POOL_GLOBAL_FLAG_EXTENDED_SCOPE_FINE_GRAINED: hsa_amd_memory_pool_global_flag_s = 8;
pub type hsa_amd_memory_pool_global_flag_s = ::core::ffi::c_uint;
pub use self::hsa_amd_memory_pool_global_flag_s as hsa_amd_memory_pool_global_flag_t;
/// This memory pool resides on the host (CPU)
pub const HSA_AMD_MEMORY_POOL_LOCATION_CPU: hsa_amd_memory_pool_location_s = 0;
/// This memory pool resides on a GPU
pub const HSA_AMD_MEMORY_POOL_LOCATION_GPU: hsa_amd_memory_pool_location_s = 1;
pub type hsa_amd_memory_pool_location_s = ::core::ffi::c_uint;
pub use self::hsa_amd_memory_pool_location_s as hsa_amd_memory_pool_location_t;
/** Segment where the memory pool resides. The type of this attribute is
 ::hsa_amd_segment_t.*/
pub const HSA_AMD_MEMORY_POOL_INFO_SEGMENT: hsa_amd_memory_pool_info_t = 0;
/** Flag mask. The value of this attribute is undefined if the value of
 ::HSA_AMD_MEMORY_POOL_INFO_SEGMENT is not ::HSA_AMD_SEGMENT_GLOBAL. The type
 of
 this attribute is uint32_t, a bit-field of
 ::hsa_amd_memory_pool_global_flag_t
 values.*/
pub const HSA_AMD_MEMORY_POOL_INFO_GLOBAL_FLAGS: hsa_amd_memory_pool_info_t = 1;
/// Size of this pool, in bytes. The type of this attribute is size_t.
pub const HSA_AMD_MEMORY_POOL_INFO_SIZE: hsa_amd_memory_pool_info_t = 2;
/** Indicates whether memory in this pool can be allocated using
 ::hsa_amd_memory_pool_allocate. The type of this attribute is bool.

 The value of this flag is always false for memory pools in the group and
 private segments.*/
pub const HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_ALLOWED: hsa_amd_memory_pool_info_t = 5;
/** Allocation granularity of buffers allocated by
 ::hsa_amd_memory_pool_allocate
 in this memory pool. The size of a buffer allocated in this pool is a
 multiple of the value of this attribute. While this is the minimum size of
 allocation allowed, it is recommened to use
 HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_REC_GRANULE to obtain the recommended
 allocation granularity size for this pool.
 The value of this attribute is only defined if
 ::HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_ALLOWED is true for
 this pool. The type of this attribute is size_t.*/
pub const HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_GRANULE: hsa_amd_memory_pool_info_t = 6;
/** Alignment of buffers allocated by ::hsa_amd_memory_pool_allocate in this
 pool. The value of this attribute is only defined if
 ::HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_ALLOWED is true for this pool, and
 must be a power of 2. The type of this attribute is size_t.*/
pub const HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_ALIGNMENT: hsa_amd_memory_pool_info_t = 7;
/** This memory_pool can be made directly accessible by all the agents in the
 system (::hsa_amd_agent_memory_pool_get_info does not return
 ::HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED for any agent). The type of this
 attribute is bool.*/
pub const HSA_AMD_MEMORY_POOL_INFO_ACCESSIBLE_BY_ALL: hsa_amd_memory_pool_info_t = 15;
/** Maximum aggregate allocation size in bytes. The type of this attribute
 is size_t.*/
pub const HSA_AMD_MEMORY_POOL_INFO_ALLOC_MAX_SIZE: hsa_amd_memory_pool_info_t = 16;
/** Location of this memory pool. The type of this attribute
 is hsa_amd_memory_pool_location_t.*/
pub const HSA_AMD_MEMORY_POOL_INFO_LOCATION: hsa_amd_memory_pool_info_t = 17;
/** Internal block size for allocations. This would also be the recommended
 granularity size for allocations as this prevents internal fragmentation.
 The value of this attribute is only defined if
 ::HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_ALLOWED is true for this pool.
 The size of this attribute is size_t.*/
pub const HSA_AMD_MEMORY_POOL_INFO_RUNTIME_ALLOC_REC_GRANULE: hsa_amd_memory_pool_info_t = 18;
/// @brief Memory pool features.
pub type hsa_amd_memory_pool_info_t = ::core::ffi::c_uint;
/// Allocates memory that conforms to standard HSA memory consistency model
pub const HSA_AMD_MEMORY_POOL_STANDARD_FLAG: hsa_amd_memory_pool_flag_s = 0;
/** Allocates fine grain memory type where memory ordering is per point to point
 connection. Atomic memory operations on these memory buffers are not
 guaranteed to be visible at system scope.*/
pub const HSA_AMD_MEMORY_POOL_PCIE_FLAG: hsa_amd_memory_pool_flag_s = 1;
///  Allocates physically contiguous memory
pub const HSA_AMD_MEMORY_POOL_CONTIGUOUS_FLAG: hsa_amd_memory_pool_flag_s = 2;
///  Allocates executable memory
pub const HSA_AMD_MEMORY_POOL_EXECUTABLE_FLAG: hsa_amd_memory_pool_flag_s = 4;
///  Allocates uncached memory
pub const HSA_AMD_MEMORY_POOL_UNCACHED_FLAG: hsa_amd_memory_pool_flag_s = 8;
/** @brief Memory pool flag used to specify allocation directives
*/
pub type hsa_amd_memory_pool_flag_s = ::core::ffi::c_uint;
/** @brief Memory pool flag used to specify allocation directives
*/
pub use self::hsa_amd_memory_pool_flag_s as hsa_amd_memory_pool_flag_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_pitched_ptr_s {
    pub base: *mut ::core::ffi::c_void,
    pub pitch: usize,
    pub slice: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_pitched_ptr_s"][::core::mem::size_of::<hsa_pitched_ptr_s>() - 24usize];
    [
        "Alignment of hsa_pitched_ptr_s",
    ][::core::mem::align_of::<hsa_pitched_ptr_s>() - 8usize];
    [
        "Offset of field: hsa_pitched_ptr_s::base",
    ][::core::mem::offset_of!(hsa_pitched_ptr_s, base) - 0usize];
    [
        "Offset of field: hsa_pitched_ptr_s::pitch",
    ][::core::mem::offset_of!(hsa_pitched_ptr_s, pitch) - 8usize];
    [
        "Offset of field: hsa_pitched_ptr_s::slice",
    ][::core::mem::offset_of!(hsa_pitched_ptr_s, slice) - 16usize];
};
impl Default for hsa_pitched_ptr_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type hsa_pitched_ptr_t = hsa_pitched_ptr_s;
pub const hsaHostToHost: hsa_amd_copy_direction_t = 0;
pub const hsaHostToDevice: hsa_amd_copy_direction_t = 1;
pub const hsaDeviceToHost: hsa_amd_copy_direction_t = 2;
pub const hsaDeviceToDevice: hsa_amd_copy_direction_t = 3;
pub type hsa_amd_copy_direction_t = ::core::ffi::c_uint;
/// The agent cannot directly access any buffer in the memory pool.
pub const HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED: hsa_amd_memory_pool_access_t = 0;
/** The agent can directly access a buffer located in the pool; the application
 does not need to invoke ::hsa_amd_agents_allow_access.*/
pub const HSA_AMD_MEMORY_POOL_ACCESS_ALLOWED_BY_DEFAULT: hsa_amd_memory_pool_access_t = 1;
/** The agent can directly access a buffer located in the pool, but only if the
 application has previously requested access to that buffer using
 ::hsa_amd_agents_allow_access.*/
pub const HSA_AMD_MEMORY_POOL_ACCESS_DISALLOWED_BY_DEFAULT: hsa_amd_memory_pool_access_t = 2;
/// @brief Type of accesses to a memory pool from a given agent.
pub type hsa_amd_memory_pool_access_t = ::core::ffi::c_uint;
/// Hyper-transport bus type.
pub const HSA_AMD_LINK_INFO_TYPE_HYPERTRANSPORT: hsa_amd_link_info_type_t = 0;
/// QPI bus type.
pub const HSA_AMD_LINK_INFO_TYPE_QPI: hsa_amd_link_info_type_t = 1;
/// PCIe bus type.
pub const HSA_AMD_LINK_INFO_TYPE_PCIE: hsa_amd_link_info_type_t = 2;
/// Infiniband bus type.
pub const HSA_AMD_LINK_INFO_TYPE_INFINBAND: hsa_amd_link_info_type_t = 3;
/// xGMI link type.
pub const HSA_AMD_LINK_INFO_TYPE_XGMI: hsa_amd_link_info_type_t = 4;
/// @brief Properties of the relationship between an agent a memory pool.
pub type hsa_amd_link_info_type_t = ::core::ffi::c_uint;
/** @brief Link properties when accessing the memory pool from the specified
 agent.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_amd_memory_pool_link_info_s {
    /// Minimum transfer latency (rounded to ns).
    pub min_latency: u32,
    /// Maximum transfer latency (rounded to ns).
    pub max_latency: u32,
    /// Minimum link interface bandwidth in MB/s.
    pub min_bandwidth: u32,
    /// Maximum link interface bandwidth in MB/s.
    pub max_bandwidth: u32,
    /// Support for 32-bit atomic transactions.
    pub atomic_support_32bit: bool,
    /// Support for 64-bit atomic transactions.
    pub atomic_support_64bit: bool,
    /// Support for cache coherent transactions.
    pub coherent_support: bool,
    /// The type of bus/link.
    pub link_type: hsa_amd_link_info_type_t,
    /// NUMA distance of memory pool relative to querying agent
    pub numa_distance: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_memory_pool_link_info_s",
    ][::core::mem::size_of::<hsa_amd_memory_pool_link_info_s>() - 28usize];
    [
        "Alignment of hsa_amd_memory_pool_link_info_s",
    ][::core::mem::align_of::<hsa_amd_memory_pool_link_info_s>() - 4usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::min_latency",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, min_latency) - 0usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::max_latency",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, max_latency) - 4usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::min_bandwidth",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, min_bandwidth) - 8usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::max_bandwidth",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, max_bandwidth) - 12usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::atomic_support_32bit",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, atomic_support_32bit)
        - 16usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::atomic_support_64bit",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, atomic_support_64bit)
        - 17usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::coherent_support",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, coherent_support)
        - 18usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::link_type",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, link_type) - 20usize];
    [
        "Offset of field: hsa_amd_memory_pool_link_info_s::numa_distance",
    ][::core::mem::offset_of!(hsa_amd_memory_pool_link_info_s, numa_distance) - 24usize];
};
impl Default for hsa_amd_memory_pool_link_info_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** @brief Link properties when accessing the memory pool from the specified
 agent.*/
pub type hsa_amd_memory_pool_link_info_t = hsa_amd_memory_pool_link_info_s;
/** Access to buffers located in the memory pool. The type of this attribute
 is ::hsa_amd_memory_pool_access_t.

 An agent can always directly access buffers currently located in a memory
 pool that is associated (the memory_pool is one of the values returned by
 ::hsa_amd_agent_iterate_memory_pools on the agent) with that agent. If the
 buffer is currently located in a memory pool that is not associated with
 the agent, and the value returned by this function for the given
 combination of agent and memory pool is not
 HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED, the application still needs to invoke
 ::hsa_amd_agents_allow_access in order to gain direct access to the buffer.

 If the given agent can directly access buffers the pool, the result is not
 HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED. If the memory pool is associated with
 the agent, or it is of fined-grained type, the result must not be
 HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED. If the memory pool is not associated
 with the agent, and does not reside in the global segment, the result must
 be HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED.*/
pub const HSA_AMD_AGENT_MEMORY_POOL_INFO_ACCESS: hsa_amd_agent_memory_pool_info_t = 0;
/** Number of links to hop when accessing the memory pool from the specified
 agent. The value of this attribute is zero if the memory pool is associated
 with the agent, or if the access type is
 HSA_AMD_MEMORY_POOL_ACCESS_NEVER_ALLOWED. The type of this attribute is
 uint32_t.*/
pub const HSA_AMD_AGENT_MEMORY_POOL_INFO_NUM_LINK_HOPS: hsa_amd_agent_memory_pool_info_t = 1;
/** Details of each link hop when accessing the memory pool starting from the
 specified agent. The type of this attribute is an array size of
 HSA_AMD_AGENT_MEMORY_POOL_INFO_NUM_LINK_HOPS with each element containing
 ::hsa_amd_memory_pool_link_info_t.*/
pub const HSA_AMD_AGENT_MEMORY_POOL_INFO_LINK_INFO: hsa_amd_agent_memory_pool_info_t = 2;
/// @brief Properties of the relationship between an agent a memory pool.
pub type hsa_amd_agent_memory_pool_info_t = ::core::ffi::c_uint;
pub const HSA_EXT_POINTER_TYPE_UNKNOWN: hsa_amd_pointer_type_t = 0;
pub const HSA_EXT_POINTER_TYPE_HSA: hsa_amd_pointer_type_t = 1;
pub const HSA_EXT_POINTER_TYPE_LOCKED: hsa_amd_pointer_type_t = 2;
pub const HSA_EXT_POINTER_TYPE_GRAPHICS: hsa_amd_pointer_type_t = 3;
pub const HSA_EXT_POINTER_TYPE_IPC: hsa_amd_pointer_type_t = 4;
pub const HSA_EXT_POINTER_TYPE_RESERVED_ADDR: hsa_amd_pointer_type_t = 5;
pub const HSA_EXT_POINTER_TYPE_HSA_VMEM: hsa_amd_pointer_type_t = 6;
/// @brief Denotes the type of memory in a pointer info query.
pub type hsa_amd_pointer_type_t = ::core::ffi::c_uint;
/** @brief Describes a memory allocation known to ROCr.
 Within a ROCr major version this structure can only grow.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_amd_pointer_info_s {
    pub size: u32,
    pub type_: hsa_amd_pointer_type_t,
    pub agentBaseAddress: *mut ::core::ffi::c_void,
    pub hostBaseAddress: *mut ::core::ffi::c_void,
    pub sizeInBytes: usize,
    pub userData: *mut ::core::ffi::c_void,
    pub agentOwner: hsa_agent_t,
    pub global_flags: u32,
    pub registered: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_pointer_info_s",
    ][::core::mem::size_of::<hsa_amd_pointer_info_s>() - 56usize];
    [
        "Alignment of hsa_amd_pointer_info_s",
    ][::core::mem::align_of::<hsa_amd_pointer_info_s>() - 8usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::size",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, size) - 0usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::type_",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, type_) - 4usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::agentBaseAddress",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, agentBaseAddress) - 8usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::hostBaseAddress",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, hostBaseAddress) - 16usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::sizeInBytes",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, sizeInBytes) - 24usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::userData",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, userData) - 32usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::agentOwner",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, agentOwner) - 40usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::global_flags",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, global_flags) - 48usize];
    [
        "Offset of field: hsa_amd_pointer_info_s::registered",
    ][::core::mem::offset_of!(hsa_amd_pointer_info_s, registered) - 52usize];
};
impl Default for hsa_amd_pointer_info_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** @brief Describes a memory allocation known to ROCr.
 Within a ROCr major version this structure can only grow.*/
pub type hsa_amd_pointer_info_t = hsa_amd_pointer_info_s;
/** @brief 256-bit process independent identifier for a ROCr shared memory
 allocation.*/
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_ipc_memory_s {
    pub handle: [u32; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_ipc_memory_s",
    ][::core::mem::size_of::<hsa_amd_ipc_memory_s>() - 32usize];
    [
        "Alignment of hsa_amd_ipc_memory_s",
    ][::core::mem::align_of::<hsa_amd_ipc_memory_s>() - 4usize];
    [
        "Offset of field: hsa_amd_ipc_memory_s::handle",
    ][::core::mem::offset_of!(hsa_amd_ipc_memory_s, handle) - 0usize];
};
/** @brief 256-bit process independent identifier for a ROCr shared memory
 allocation.*/
pub type hsa_amd_ipc_memory_t = hsa_amd_ipc_memory_s;
/// @brief 256-bit process independent identifier for a ROCr IPC signal.
pub type hsa_amd_ipc_signal_t = hsa_amd_ipc_memory_t;
pub const HSA_AMD_GPU_MEMORY_FAULT_EVENT: hsa_amd_event_type_s = 0;
pub const HSA_AMD_GPU_HW_EXCEPTION_EVENT: hsa_amd_event_type_s = 1;
pub const HSA_AMD_GPU_MEMORY_ERROR_EVENT: hsa_amd_event_type_s = 2;
/// @brief GPU system event type.
pub type hsa_amd_event_type_s = ::core::ffi::c_uint;
/// @brief GPU system event type.
pub use self::hsa_amd_event_type_s as hsa_amd_event_type_t;
pub const HSA_AMD_MEMORY_FAULT_PAGE_NOT_PRESENT: hsa_amd_memory_fault_reason_t = 1;
pub const HSA_AMD_MEMORY_FAULT_READ_ONLY: hsa_amd_memory_fault_reason_t = 2;
pub const HSA_AMD_MEMORY_FAULT_NX: hsa_amd_memory_fault_reason_t = 4;
pub const HSA_AMD_MEMORY_FAULT_HOST_ONLY: hsa_amd_memory_fault_reason_t = 8;
pub const HSA_AMD_MEMORY_FAULT_DRAMECC: hsa_amd_memory_fault_reason_t = 16;
pub const HSA_AMD_MEMORY_FAULT_IMPRECISE: hsa_amd_memory_fault_reason_t = 32;
pub const HSA_AMD_MEMORY_FAULT_SRAMECC: hsa_amd_memory_fault_reason_t = 64;
pub const HSA_AMD_MEMORY_FAULT_HANG: hsa_amd_memory_fault_reason_t = 2147483648;
/// @brief Flags denoting the cause of a memory fault.
pub type hsa_amd_memory_fault_reason_t = ::core::ffi::c_uint;
/// @brief AMD GPU memory fault event data.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_gpu_memory_fault_info_s {
    pub agent: hsa_agent_t,
    pub virtual_address: u64,
    pub fault_reason_mask: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_gpu_memory_fault_info_s",
    ][::core::mem::size_of::<hsa_amd_gpu_memory_fault_info_s>() - 24usize];
    [
        "Alignment of hsa_amd_gpu_memory_fault_info_s",
    ][::core::mem::align_of::<hsa_amd_gpu_memory_fault_info_s>() - 8usize];
    [
        "Offset of field: hsa_amd_gpu_memory_fault_info_s::agent",
    ][::core::mem::offset_of!(hsa_amd_gpu_memory_fault_info_s, agent) - 0usize];
    [
        "Offset of field: hsa_amd_gpu_memory_fault_info_s::virtual_address",
    ][::core::mem::offset_of!(hsa_amd_gpu_memory_fault_info_s, virtual_address)
        - 8usize];
    [
        "Offset of field: hsa_amd_gpu_memory_fault_info_s::fault_reason_mask",
    ][::core::mem::offset_of!(hsa_amd_gpu_memory_fault_info_s, fault_reason_mask)
        - 16usize];
};
/// @brief AMD GPU memory fault event data.
pub type hsa_amd_gpu_memory_fault_info_t = hsa_amd_gpu_memory_fault_info_s;
pub const HSA_AMD_MEMORY_ERROR_MEMORY_IN_USE: hsa_amd_memory_error_reason_t = 1;
/// @brief Flags denoting the cause of a memory error.
pub type hsa_amd_memory_error_reason_t = ::core::ffi::c_uint;
/// @brief AMD GPU memory error event data.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_gpu_memory_error_info_s {
    pub agent: hsa_agent_t,
    pub virtual_address: u64,
    pub error_reason_mask: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_gpu_memory_error_info_s",
    ][::core::mem::size_of::<hsa_amd_gpu_memory_error_info_s>() - 24usize];
    [
        "Alignment of hsa_amd_gpu_memory_error_info_s",
    ][::core::mem::align_of::<hsa_amd_gpu_memory_error_info_s>() - 8usize];
    [
        "Offset of field: hsa_amd_gpu_memory_error_info_s::agent",
    ][::core::mem::offset_of!(hsa_amd_gpu_memory_error_info_s, agent) - 0usize];
    [
        "Offset of field: hsa_amd_gpu_memory_error_info_s::virtual_address",
    ][::core::mem::offset_of!(hsa_amd_gpu_memory_error_info_s, virtual_address)
        - 8usize];
    [
        "Offset of field: hsa_amd_gpu_memory_error_info_s::error_reason_mask",
    ][::core::mem::offset_of!(hsa_amd_gpu_memory_error_info_s, error_reason_mask)
        - 16usize];
};
/// @brief AMD GPU memory error event data.
pub type hsa_amd_gpu_memory_error_info_t = hsa_amd_gpu_memory_error_info_s;
pub const HSA_AMD_HW_EXCEPTION_RESET_TYPE_OTHER: hsa_amd_hw_exception_reset_type_t = 1;
/// @brief Flags denoting the type of a HW exception
pub type hsa_amd_hw_exception_reset_type_t = ::core::ffi::c_uint;
pub const HSA_AMD_HW_EXCEPTION_CAUSE_GPU_HANG: hsa_amd_hw_exception_reset_cause_t = 1;
pub const HSA_AMD_HW_EXCEPTION_CAUSE_ECC: hsa_amd_hw_exception_reset_cause_t = 2;
/// @brief Flags denoting the cause of a HW exception
pub type hsa_amd_hw_exception_reset_cause_t = ::core::ffi::c_uint;
/// @brief AMD GPU HW Exception event data.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_amd_gpu_hw_exception_info_s {
    pub agent: hsa_agent_t,
    pub reset_type: hsa_amd_hw_exception_reset_type_t,
    pub reset_cause: hsa_amd_hw_exception_reset_cause_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_gpu_hw_exception_info_s",
    ][::core::mem::size_of::<hsa_amd_gpu_hw_exception_info_s>() - 16usize];
    [
        "Alignment of hsa_amd_gpu_hw_exception_info_s",
    ][::core::mem::align_of::<hsa_amd_gpu_hw_exception_info_s>() - 8usize];
    [
        "Offset of field: hsa_amd_gpu_hw_exception_info_s::agent",
    ][::core::mem::offset_of!(hsa_amd_gpu_hw_exception_info_s, agent) - 0usize];
    [
        "Offset of field: hsa_amd_gpu_hw_exception_info_s::reset_type",
    ][::core::mem::offset_of!(hsa_amd_gpu_hw_exception_info_s, reset_type) - 8usize];
    [
        "Offset of field: hsa_amd_gpu_hw_exception_info_s::reset_cause",
    ][::core::mem::offset_of!(hsa_amd_gpu_hw_exception_info_s, reset_cause) - 12usize];
};
impl Default for hsa_amd_gpu_hw_exception_info_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// @brief AMD GPU HW Exception event data.
pub type hsa_amd_gpu_hw_exception_info_t = hsa_amd_gpu_hw_exception_info_s;
/// @brief AMD GPU event data passed to event handler.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsa_amd_event_s {
    pub event_type: hsa_amd_event_type_t,
    pub __bindgen_anon_1: hsa_amd_event_s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hsa_amd_event_s__bindgen_ty_1 {
    pub memory_fault: hsa_amd_gpu_memory_fault_info_t,
    pub hw_exception: hsa_amd_gpu_hw_exception_info_t,
    pub memory_error: hsa_amd_gpu_memory_error_info_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_event_s__bindgen_ty_1",
    ][::core::mem::size_of::<hsa_amd_event_s__bindgen_ty_1>() - 24usize];
    [
        "Alignment of hsa_amd_event_s__bindgen_ty_1",
    ][::core::mem::align_of::<hsa_amd_event_s__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: hsa_amd_event_s__bindgen_ty_1::memory_fault",
    ][::core::mem::offset_of!(hsa_amd_event_s__bindgen_ty_1, memory_fault) - 0usize];
    [
        "Offset of field: hsa_amd_event_s__bindgen_ty_1::hw_exception",
    ][::core::mem::offset_of!(hsa_amd_event_s__bindgen_ty_1, hw_exception) - 0usize];
    [
        "Offset of field: hsa_amd_event_s__bindgen_ty_1::memory_error",
    ][::core::mem::offset_of!(hsa_amd_event_s__bindgen_ty_1, memory_error) - 0usize];
};
impl Default for hsa_amd_event_s__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of hsa_amd_event_s"][::core::mem::size_of::<hsa_amd_event_s>() - 32usize];
    [
        "Alignment of hsa_amd_event_s",
    ][::core::mem::align_of::<hsa_amd_event_s>() - 8usize];
    [
        "Offset of field: hsa_amd_event_s::event_type",
    ][::core::mem::offset_of!(hsa_amd_event_s, event_type) - 0usize];
};
impl Default for hsa_amd_event_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// @brief AMD GPU event data passed to event handler.
pub type hsa_amd_event_t = hsa_amd_event_s;
pub type hsa_amd_system_event_callback_t = ::core::option::Option<
    unsafe extern "C" fn(
        event: *const hsa_amd_event_t,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t,
>;
pub const HSA_AMD_QUEUE_PRIORITY_LOW: hsa_amd_queue_priority_s = 0;
pub const HSA_AMD_QUEUE_PRIORITY_NORMAL: hsa_amd_queue_priority_s = 1;
pub const HSA_AMD_QUEUE_PRIORITY_HIGH: hsa_amd_queue_priority_s = 2;
/// @brief Per-queue dispatch and wavefront scheduling priority.
pub type hsa_amd_queue_priority_s = ::core::ffi::c_uint;
/// @brief Per-queue dispatch and wavefront scheduling priority.
pub use self::hsa_amd_queue_priority_s as hsa_amd_queue_priority_t;
/** The queue's packet buffer and queue descriptor struct should be
 allocated in system memory (default). Mutually exclusive with
 HSA_AMD_QUEUE_CREATE_DEVICE_MEM_RING_BUF and
 HSA_AMD_QUEUE_CREATE_DEVICE_MEM_QUEUE_DESCRIPTOR.*/
pub const HSA_AMD_QUEUE_CREATE_SYSTEM_MEM: hsa_amd_queue_create_flag_t = 0;
/** The queue's packet buffer should be allocated in the agent's
 fine-grain device memory region.*/
pub const HSA_AMD_QUEUE_CREATE_DEVICE_MEM_RING_BUF: hsa_amd_queue_create_flag_t = 1;
/** The queue desciptor struct should be allocated in the agent's
 fine-grain device memory region. Not supported for devices
 connected via PCIe because the CPU's atomic read-modify-write
 operations cannot be promoted to PCIe atomic read-modify-write
 operations.*/
pub const HSA_AMD_QUEUE_CREATE_DEVICE_MEM_QUEUE_DESCRIPTOR: hsa_amd_queue_create_flag_t = 2;
/// @brief Queue creation attributes.
pub type hsa_amd_queue_create_flag_t = ::core::ffi::c_uint;
/// @brief Deallocation notifier function type.
pub type hsa_amd_deallocation_callback_t = ::core::option::Option<
    unsafe extern "C" fn(
        ptr: *mut ::core::ffi::c_void,
        user_data: *mut ::core::ffi::c_void,
    ),
>;
/** Updates to memory with this attribute conform to HSA memory consistency
 model.*/
pub const HSA_AMD_SVM_GLOBAL_FLAG_FINE_GRAINED: hsa_amd_svm_model_s = 0;
/** Writes to memory with this attribute can be performed by a single agent
 at a time.*/
pub const HSA_AMD_SVM_GLOBAL_FLAG_COARSE_GRAINED: hsa_amd_svm_model_s = 1;
/** Memory region queried contains subregions with both
 HSA_AMD_SVM_GLOBAL_FLAG_COARSE_GRAINED and
 HSA_AMD_SVM_GLOBAL_FLAG_FINE_GRAINED attributes.

 This attribute can not be used in hsa_amd_svm_attributes_set.  It is a
 possible return from hsa_amd_svm_attributes_get indicating that the query
 region contains both coarse and fine grained memory.*/
pub const HSA_AMD_SVM_GLOBAL_FLAG_INDETERMINATE: hsa_amd_svm_model_s = 2;
pub type hsa_amd_svm_model_s = ::core::ffi::c_uint;
pub use self::hsa_amd_svm_model_s as hsa_amd_svm_model_t;
pub const HSA_AMD_SVM_ATTRIB_GLOBAL_FLAG: hsa_amd_svm_attribute_s = 0;
pub const HSA_AMD_SVM_ATTRIB_READ_ONLY: hsa_amd_svm_attribute_s = 1;
pub const HSA_AMD_SVM_ATTRIB_HIVE_LOCAL: hsa_amd_svm_attribute_s = 2;
pub const HSA_AMD_SVM_ATTRIB_MIGRATION_GRANULARITY: hsa_amd_svm_attribute_s = 3;
pub const HSA_AMD_SVM_ATTRIB_PREFERRED_LOCATION: hsa_amd_svm_attribute_s = 4;
pub const HSA_AMD_SVM_ATTRIB_PREFETCH_LOCATION: hsa_amd_svm_attribute_s = 5;
pub const HSA_AMD_SVM_ATTRIB_READ_MOSTLY: hsa_amd_svm_attribute_s = 6;
pub const HSA_AMD_SVM_ATTRIB_GPU_EXEC: hsa_amd_svm_attribute_s = 7;
pub const HSA_AMD_SVM_ATTRIB_AGENT_ACCESSIBLE: hsa_amd_svm_attribute_s = 512;
pub const HSA_AMD_SVM_ATTRIB_AGENT_ACCESSIBLE_IN_PLACE: hsa_amd_svm_attribute_s = 513;
pub const HSA_AMD_SVM_ATTRIB_AGENT_NO_ACCESS: hsa_amd_svm_attribute_s = 514;
pub const HSA_AMD_SVM_ATTRIB_ACCESS_QUERY: hsa_amd_svm_attribute_s = 515;
pub type hsa_amd_svm_attribute_s = ::core::ffi::c_uint;
pub use self::hsa_amd_svm_attribute_s as hsa_amd_svm_attribute_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_svm_attribute_pair_s {
    pub attribute: u64,
    pub value: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_svm_attribute_pair_s",
    ][::core::mem::size_of::<hsa_amd_svm_attribute_pair_s>() - 16usize];
    [
        "Alignment of hsa_amd_svm_attribute_pair_s",
    ][::core::mem::align_of::<hsa_amd_svm_attribute_pair_s>() - 8usize];
    [
        "Offset of field: hsa_amd_svm_attribute_pair_s::attribute",
    ][::core::mem::offset_of!(hsa_amd_svm_attribute_pair_s, attribute) - 0usize];
    [
        "Offset of field: hsa_amd_svm_attribute_pair_s::value",
    ][::core::mem::offset_of!(hsa_amd_svm_attribute_pair_s, value) - 8usize];
};
pub type hsa_amd_svm_attribute_pair_t = hsa_amd_svm_attribute_pair_s;
pub const HSA_AMD_VMEM_ADDRESS_NO_REGISTER: hsa_amd_vmem_address_reserve_flag_s = 1;
pub type hsa_amd_vmem_address_reserve_flag_s = ::core::ffi::c_uint;
pub use self::hsa_amd_vmem_address_reserve_flag_s as hsa_amd_vmem_address_reserve_flag_t;
/// @brief Struct containing an opaque handle to a memory allocation handle
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_amd_vmem_alloc_handle_s {
    /** Opaque handle. Two handles reference the same object of the enclosing type
 if and only if they are equal.*/
    pub handle: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_vmem_alloc_handle_s",
    ][::core::mem::size_of::<hsa_amd_vmem_alloc_handle_s>() - 8usize];
    [
        "Alignment of hsa_amd_vmem_alloc_handle_s",
    ][::core::mem::align_of::<hsa_amd_vmem_alloc_handle_s>() - 8usize];
    [
        "Offset of field: hsa_amd_vmem_alloc_handle_s::handle",
    ][::core::mem::offset_of!(hsa_amd_vmem_alloc_handle_s, handle) - 0usize];
};
/// @brief Struct containing an opaque handle to a memory allocation handle
pub type hsa_amd_vmem_alloc_handle_t = hsa_amd_vmem_alloc_handle_s;
pub const MEMORY_TYPE_NONE: hsa_amd_memory_type_t = 0;
pub const MEMORY_TYPE_PINNED: hsa_amd_memory_type_t = 1;
pub type hsa_amd_memory_type_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_amd_memory_access_desc_s {
    pub permissions: hsa_access_permission_t,
    pub agent_handle: hsa_agent_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_memory_access_desc_s",
    ][::core::mem::size_of::<hsa_amd_memory_access_desc_s>() - 16usize];
    [
        "Alignment of hsa_amd_memory_access_desc_s",
    ][::core::mem::align_of::<hsa_amd_memory_access_desc_s>() - 8usize];
    [
        "Offset of field: hsa_amd_memory_access_desc_s::permissions",
    ][::core::mem::offset_of!(hsa_amd_memory_access_desc_s, permissions) - 0usize];
    [
        "Offset of field: hsa_amd_memory_access_desc_s::agent_handle",
    ][::core::mem::offset_of!(hsa_amd_memory_access_desc_s, agent_handle) - 8usize];
};
impl Default for hsa_amd_memory_access_desc_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type hsa_amd_memory_access_desc_t = hsa_amd_memory_access_desc_s;
pub const HSA_AMD_QUEUE_INFO_AGENT: hsa_queue_info_attribute_t = 0;
pub const HSA_AMD_QUEUE_INFO_DOORBELL_ID: hsa_queue_info_attribute_t = 1;
pub type hsa_queue_info_attribute_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsa_amd_ais_file_handle_s {
    pub __bindgen_anon_1: hsa_amd_ais_file_handle_s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hsa_amd_ais_file_handle_s__bindgen_ty_1 {
    pub handle: *mut ::core::ffi::c_void,
    pub fd: ::core::ffi::c_int,
    pub pad: [u8; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_ais_file_handle_s__bindgen_ty_1",
    ][::core::mem::size_of::<hsa_amd_ais_file_handle_s__bindgen_ty_1>() - 8usize];
    [
        "Alignment of hsa_amd_ais_file_handle_s__bindgen_ty_1",
    ][::core::mem::align_of::<hsa_amd_ais_file_handle_s__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: hsa_amd_ais_file_handle_s__bindgen_ty_1::handle",
    ][::core::mem::offset_of!(hsa_amd_ais_file_handle_s__bindgen_ty_1, handle) - 0usize];
    [
        "Offset of field: hsa_amd_ais_file_handle_s__bindgen_ty_1::fd",
    ][::core::mem::offset_of!(hsa_amd_ais_file_handle_s__bindgen_ty_1, fd) - 0usize];
    [
        "Offset of field: hsa_amd_ais_file_handle_s__bindgen_ty_1::pad",
    ][::core::mem::offset_of!(hsa_amd_ais_file_handle_s__bindgen_ty_1, pad) - 0usize];
};
impl Default for hsa_amd_ais_file_handle_s__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_amd_ais_file_handle_s",
    ][::core::mem::size_of::<hsa_amd_ais_file_handle_s>() - 8usize];
    [
        "Alignment of hsa_amd_ais_file_handle_s",
    ][::core::mem::align_of::<hsa_amd_ais_file_handle_s>() - 8usize];
};
impl Default for hsa_amd_ais_file_handle_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type hsa_amd_ais_file_handle_t = hsa_amd_ais_file_handle_s;
pub const HSA_AMD_LOG_FLAG_BLIT_KERNEL_PKTS: hsa_amd_log_flag_s = 0;
pub const HSA_AMD_LOG_FLAG_AQL: hsa_amd_log_flag_s = 0;
pub const HSA_AMD_LOG_FLAG_SDMA: hsa_amd_log_flag_s = 1;
pub const HSA_AMD_LOG_FLAG_INFO: hsa_amd_log_flag_s = 2;
/// @brief logging types
pub type hsa_amd_log_flag_s = ::core::ffi::c_uint;
/// @brief logging types
pub use self::hsa_amd_log_flag_s as hsa_amd_log_flag_t;
/** Loaded memory segment is not backed by any code object (anonymous), as the
 case would be with BSS (uninitialized data).*/
pub const HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_NONE: hsa_ven_amd_loader_code_object_storage_type_t = 0;
/** Loaded memory segment is backed by the code object that is stored in the
 file.*/
pub const HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_FILE: hsa_ven_amd_loader_code_object_storage_type_t = 1;
/** Loaded memory segment is backed by the code object that is stored in the
 memory.*/
pub const HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_MEMORY: hsa_ven_amd_loader_code_object_storage_type_t = 2;
/** @brief The storage type of the code object that is backing loaded memory
 segment.*/
pub type hsa_ven_amd_loader_code_object_storage_type_t = ::core::ffi::c_uint;
/** @brief Loaded memory segment descriptor.


 @details Loaded memory segment descriptor describes underlying loaded memory
 segment. Loaded memory segment is created/allocated by the executable during
 the loading of the code object that is backing underlying memory segment.

 The lifetime of underlying memory segment is limited by the lifetime of the
 executable that is managing underlying memory segment.*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hsa_ven_amd_loader_segment_descriptor_s {
    /** Agent underlying memory segment is allocated on. If the code object that is
 backing underlying memory segment is program code object, then 0.*/
    pub agent: hsa_agent_t,
    /// Executable that is managing this underlying memory segment.
    pub executable: hsa_executable_t,
    /// Storage type of the code object that is backing underlying memory segment.
    pub code_object_storage_type: hsa_ven_amd_loader_code_object_storage_type_t,
    /** If the storage type of the code object that is backing underlying memory
 segment is:
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_NONE, then null;
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_FILE, then null-terminated
     filepath to the code object;
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_MEMORY, then host
     accessible pointer to the first byte of the code object.*/
    pub code_object_storage_base: *const ::core::ffi::c_void,
    /** If the storage type of the code object that is backing underlying memory
 segment is:
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_NONE, then 0;
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_FILE, then the length of
     the filepath to the code object (including null-terminating character);
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_MEMORY, then the size, in
     bytes, of the memory occupied by the code object.*/
    pub code_object_storage_size: usize,
    /** If the storage type of the code object that is backing underlying memory
 segment is:
   - HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_NONE, then 0;
   - other, then offset, in bytes, from the beginning of the code object to
     the first byte in the code object data is copied from.*/
    pub code_object_storage_offset: usize,
    /// Starting address of the underlying memory segment.
    pub segment_base: *const ::core::ffi::c_void,
    /// Size, in bytes, of the underlying memory segment.
    pub segment_size: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_loader_segment_descriptor_s",
    ][::core::mem::size_of::<hsa_ven_amd_loader_segment_descriptor_s>() - 64usize];
    [
        "Alignment of hsa_ven_amd_loader_segment_descriptor_s",
    ][::core::mem::align_of::<hsa_ven_amd_loader_segment_descriptor_s>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::agent",
    ][::core::mem::offset_of!(hsa_ven_amd_loader_segment_descriptor_s, agent) - 0usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::executable",
    ][::core::mem::offset_of!(hsa_ven_amd_loader_segment_descriptor_s, executable)
        - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::code_object_storage_type",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_segment_descriptor_s, code_object_storage_type
    ) - 16usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::code_object_storage_base",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_segment_descriptor_s, code_object_storage_base
    ) - 24usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::code_object_storage_size",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_segment_descriptor_s, code_object_storage_size
    ) - 32usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::code_object_storage_offset",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_segment_descriptor_s, code_object_storage_offset
    ) - 40usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::segment_base",
    ][::core::mem::offset_of!(hsa_ven_amd_loader_segment_descriptor_s, segment_base)
        - 48usize];
    [
        "Offset of field: hsa_ven_amd_loader_segment_descriptor_s::segment_size",
    ][::core::mem::offset_of!(hsa_ven_amd_loader_segment_descriptor_s, segment_size)
        - 56usize];
};
impl Default for hsa_ven_amd_loader_segment_descriptor_s {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/** @brief Loaded memory segment descriptor.


 @details Loaded memory segment descriptor describes underlying loaded memory
 segment. Loaded memory segment is created/allocated by the executable during
 the loading of the code object that is backing underlying memory segment.

 The lifetime of underlying memory segment is limited by the lifetime of the
 executable that is managing underlying memory segment.*/
pub type hsa_ven_amd_loader_segment_descriptor_t = hsa_ven_amd_loader_segment_descriptor_s;
/// Program code object.
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_KIND_PROGRAM: hsa_ven_amd_loader_loaded_code_object_kind_t = 1;
/// Agent code object.
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_KIND_AGENT: hsa_ven_amd_loader_loaded_code_object_kind_t = 2;
/// @brief Loaded code object kind.
pub type hsa_ven_amd_loader_loaded_code_object_kind_t = ::core::ffi::c_uint;
/** The executable in which this loaded code object is loaded. The
 type of this attribute is ::hsa_executable_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_EXECUTABLE: hsa_ven_amd_loader_loaded_code_object_info_e = 1;
/** The kind of this loaded code object. The type of this attribute is
 ::uint32_t interpreted as ::hsa_ven_amd_loader_loaded_code_object_kind_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_KIND: hsa_ven_amd_loader_loaded_code_object_info_e = 2;
/** The agent on which this loaded code object is loaded. The
 value of this attribute is only defined if
 ::HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_KIND is
 ::HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_KIND_AGENT. The type of this
 attribute is ::hsa_agent_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_AGENT: hsa_ven_amd_loader_loaded_code_object_info_e = 3;
/** The storage type of the code object reader used to load the loaded code object.
 The type of this attribute is ::uint32_t interpreted as a
 ::hsa_ven_amd_loader_code_object_storage_type_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_TYPE: hsa_ven_amd_loader_loaded_code_object_info_e = 4;
/** The memory address of the first byte of the code object that was loaaded.
 The value of this attribute is only defined if
 ::HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_TYPE is
 ::HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_MEMORY. The type of this
 attribute is ::uint64_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_MEMORY_BASE: hsa_ven_amd_loader_loaded_code_object_info_e = 5;
/** The memory size in bytes of the code object that was loaaded.
 The value of this attribute is only defined if
 ::HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_TYPE is
 ::HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_MEMORY. The type of this
 attribute is ::uint64_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_MEMORY_SIZE: hsa_ven_amd_loader_loaded_code_object_info_e = 6;
/** The file descriptor of the code object that was loaaded.
 The value of this attribute is only defined if
 ::HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_TYPE is
 ::HSA_VEN_AMD_LOADER_CODE_OBJECT_STORAGE_TYPE_FILE. The type of this
 attribute is ::int.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_CODE_OBJECT_STORAGE_FILE: hsa_ven_amd_loader_loaded_code_object_info_e = 7;
/** The signed byte address difference of the memory address at which the code
 object is loaded minus the virtual address specified in the code object
 that is loaded. The value of this attribute is only defined if the
 executable in which the code object is loaded is froozen. The type of this
 attribute is ::int64_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_LOAD_DELTA: hsa_ven_amd_loader_loaded_code_object_info_e = 8;
/** The base memory address at which the code object is loaded. This is the
 base address of the allocation for the lowest addressed segment of the code
 object that is loaded. Note that any non-loaded segments before the first
 loaded segment are ignored. The value of this attribute is only defined if
 the executable in which the code object is loaded is froozen. The type of
 this attribute is ::uint64_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_LOAD_BASE: hsa_ven_amd_loader_loaded_code_object_info_e = 9;
/** The byte size of the loaded code objects contiguous memory allocation. The
 value of this attribute is only defined if the executable in which the code
 object is loaded is froozen. The type of this attribute is ::uint64_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_LOAD_SIZE: hsa_ven_amd_loader_loaded_code_object_info_e = 10;
/** The length of the URI in bytes, not including the NUL terminator. The type
 of this attribute is uint32_t.*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_URI_LENGTH: hsa_ven_amd_loader_loaded_code_object_info_e = 11;
/** The URI name from which the code object was loaded. The type of this
 attribute is a NUL terminated \p char* with the length equal to the value
 of ::HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_URI_LENGTH attribute.
 The URI name syntax is defined by the following BNF syntax:

     code_object_uri ::== file_uri | memory_uri
     file_uri        ::== "file://" file_path [ range_specifier ]
     memory_uri      ::== "memory://" process_id range_specifier
     range_specifier ::== [ "#" | "?" ] "offset=" number "&" "size=" number
     file_path       ::== URI_ENCODED_OS_FILE_PATH
     process_id      ::== DECIMAL_NUMBER
     number          ::== HEX_NUMBER | DECIMAL_NUMBER | OCTAL_NUMBER

 ``number`` is a C integral literal where hexadecimal values are prefixed by
 "0x" or "0X", and octal values by "0".

 ``file_path`` is the file's path specified as a URI encoded UTF-8 string.
 In URI encoding, every character that is not in the regular expression
 ``[a-zA-Z0-9/_.~-]`` is encoded as two uppercase hexidecimal digits
 proceeded by "%".  Directories in the path are separated by "/".

 ``offset`` is a 0-based byte offset to the start of the code object.  For a
 file URI, it is from the start of the file specified by the ``file_path``,
 and if omitted defaults to 0. For a memory URI, it is the memory address
 and is required.

 ``size`` is the number of bytes in the code object.  For a file URI, if
 omitted it defaults to the size of the file.  It is required for a memory
 URI.

 ``process_id`` is the identity of the process owning the memory.  For Linux
 it is the C unsigned integral decimal literal for the process ID (PID).

 For example:

     file:///dir1/dir2/file1
     file:///dir3/dir4/file2#offset=0x2000&size=3000
     memory://1234#offset=0x20000&size=3000*/
pub const HSA_VEN_AMD_LOADER_LOADED_CODE_OBJECT_INFO_URI: hsa_ven_amd_loader_loaded_code_object_info_e = 12;
/// @brief Loaded code object attributes.
pub type hsa_ven_amd_loader_loaded_code_object_info_e = ::core::ffi::c_uint;
/// @brief Loaded code object attributes.
pub use self::hsa_ven_amd_loader_loaded_code_object_info_e as hsa_ven_amd_loader_loaded_code_object_info_t;
/// @brief Extension function table version 1.00.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ven_amd_loader_1_00_pfn_s {
    pub hsa_ven_amd_loader_query_host_address: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            host_address: *mut *const ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_segment_descriptors: ::core::option::Option<
        unsafe extern "C" fn(
            segment_descriptors: *mut hsa_ven_amd_loader_segment_descriptor_t,
            num_segment_descriptors: *mut usize,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_executable: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            executable: *mut hsa_executable_t,
        ) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_loader_1_00_pfn_s",
    ][::core::mem::size_of::<hsa_ven_amd_loader_1_00_pfn_s>() - 24usize];
    [
        "Alignment of hsa_ven_amd_loader_1_00_pfn_s",
    ][::core::mem::align_of::<hsa_ven_amd_loader_1_00_pfn_s>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_00_pfn_s::hsa_ven_amd_loader_query_host_address",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_00_pfn_s, hsa_ven_amd_loader_query_host_address
    ) - 0usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_00_pfn_s::hsa_ven_amd_loader_query_segment_descriptors",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_00_pfn_s, hsa_ven_amd_loader_query_segment_descriptors
    ) - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_00_pfn_s::hsa_ven_amd_loader_query_executable",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_00_pfn_s, hsa_ven_amd_loader_query_executable
    ) - 16usize];
};
/// @brief Extension function table version 1.00.
pub type hsa_ven_amd_loader_1_00_pfn_t = hsa_ven_amd_loader_1_00_pfn_s;
/// @brief Extension function table version 1.01.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ven_amd_loader_1_01_pfn_s {
    pub hsa_ven_amd_loader_query_host_address: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            host_address: *mut *const ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_segment_descriptors: ::core::option::Option<
        unsafe extern "C" fn(
            segment_descriptors: *mut hsa_ven_amd_loader_segment_descriptor_t,
            num_segment_descriptors: *mut usize,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_executable: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            executable: *mut hsa_executable_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_executable_iterate_loaded_code_objects: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    executable: hsa_executable_t,
                    loaded_code_object: hsa_loaded_code_object_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_loaded_code_object_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            loaded_code_object: hsa_loaded_code_object_t,
            attribute: hsa_ven_amd_loader_loaded_code_object_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_loader_1_01_pfn_s",
    ][::core::mem::size_of::<hsa_ven_amd_loader_1_01_pfn_s>() - 40usize];
    [
        "Alignment of hsa_ven_amd_loader_1_01_pfn_s",
    ][::core::mem::align_of::<hsa_ven_amd_loader_1_01_pfn_s>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_01_pfn_s::hsa_ven_amd_loader_query_host_address",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_01_pfn_s, hsa_ven_amd_loader_query_host_address
    ) - 0usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_01_pfn_s::hsa_ven_amd_loader_query_segment_descriptors",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_01_pfn_s, hsa_ven_amd_loader_query_segment_descriptors
    ) - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_01_pfn_s::hsa_ven_amd_loader_query_executable",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_01_pfn_s, hsa_ven_amd_loader_query_executable
    ) - 16usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_01_pfn_s::hsa_ven_amd_loader_executable_iterate_loaded_code_objects",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_01_pfn_s,
        hsa_ven_amd_loader_executable_iterate_loaded_code_objects
    ) - 24usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_01_pfn_s::hsa_ven_amd_loader_loaded_code_object_get_info",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_01_pfn_s, hsa_ven_amd_loader_loaded_code_object_get_info
    ) - 32usize];
};
/// @brief Extension function table version 1.01.
pub type hsa_ven_amd_loader_1_01_pfn_t = hsa_ven_amd_loader_1_01_pfn_s;
/// @brief Extension function table version 1.02.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ven_amd_loader_1_02_pfn_s {
    pub hsa_ven_amd_loader_query_host_address: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            host_address: *mut *const ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_segment_descriptors: ::core::option::Option<
        unsafe extern "C" fn(
            segment_descriptors: *mut hsa_ven_amd_loader_segment_descriptor_t,
            num_segment_descriptors: *mut usize,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_executable: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            executable: *mut hsa_executable_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_executable_iterate_loaded_code_objects: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    executable: hsa_executable_t,
                    loaded_code_object: hsa_loaded_code_object_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_loaded_code_object_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            loaded_code_object: hsa_loaded_code_object_t,
            attribute: hsa_ven_amd_loader_loaded_code_object_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_code_object_reader_create_from_file_with_offset_size: ::core::option::Option<
        unsafe extern "C" fn(
            file: hsa_file_t,
            offset: usize,
            size: usize,
            code_object_reader: *mut hsa_code_object_reader_t,
        ) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_loader_1_02_pfn_s",
    ][::core::mem::size_of::<hsa_ven_amd_loader_1_02_pfn_s>() - 48usize];
    [
        "Alignment of hsa_ven_amd_loader_1_02_pfn_s",
    ][::core::mem::align_of::<hsa_ven_amd_loader_1_02_pfn_s>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_02_pfn_s::hsa_ven_amd_loader_query_host_address",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_02_pfn_s, hsa_ven_amd_loader_query_host_address
    ) - 0usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_02_pfn_s::hsa_ven_amd_loader_query_segment_descriptors",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_02_pfn_s, hsa_ven_amd_loader_query_segment_descriptors
    ) - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_02_pfn_s::hsa_ven_amd_loader_query_executable",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_02_pfn_s, hsa_ven_amd_loader_query_executable
    ) - 16usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_02_pfn_s::hsa_ven_amd_loader_executable_iterate_loaded_code_objects",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_02_pfn_s,
        hsa_ven_amd_loader_executable_iterate_loaded_code_objects
    ) - 24usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_02_pfn_s::hsa_ven_amd_loader_loaded_code_object_get_info",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_02_pfn_s, hsa_ven_amd_loader_loaded_code_object_get_info
    ) - 32usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_02_pfn_s::hsa_ven_amd_loader_code_object_reader_create_from_file_with_offset_size",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_02_pfn_s,
        hsa_ven_amd_loader_code_object_reader_create_from_file_with_offset_size
    ) - 40usize];
};
/// @brief Extension function table version 1.02.
pub type hsa_ven_amd_loader_1_02_pfn_t = hsa_ven_amd_loader_1_02_pfn_s;
/// @brief Extension function table version 1.03.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct hsa_ven_amd_loader_1_03_pfn_s {
    pub hsa_ven_amd_loader_query_host_address: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            host_address: *mut *const ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_segment_descriptors: ::core::option::Option<
        unsafe extern "C" fn(
            segment_descriptors: *mut hsa_ven_amd_loader_segment_descriptor_t,
            num_segment_descriptors: *mut usize,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_query_executable: ::core::option::Option<
        unsafe extern "C" fn(
            device_address: *const ::core::ffi::c_void,
            executable: *mut hsa_executable_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_executable_iterate_loaded_code_objects: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    executable: hsa_executable_t,
                    loaded_code_object: hsa_loaded_code_object_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_loaded_code_object_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            loaded_code_object: hsa_loaded_code_object_t,
            attribute: hsa_ven_amd_loader_loaded_code_object_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_code_object_reader_create_from_file_with_offset_size: ::core::option::Option<
        unsafe extern "C" fn(
            file: hsa_file_t,
            offset: usize,
            size: usize,
            code_object_reader: *mut hsa_code_object_reader_t,
        ) -> hsa_status_t,
    >,
    pub hsa_ven_amd_loader_iterate_executables: ::core::option::Option<
        unsafe extern "C" fn(
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    executable: hsa_executable_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of hsa_ven_amd_loader_1_03_pfn_s",
    ][::core::mem::size_of::<hsa_ven_amd_loader_1_03_pfn_s>() - 56usize];
    [
        "Alignment of hsa_ven_amd_loader_1_03_pfn_s",
    ][::core::mem::align_of::<hsa_ven_amd_loader_1_03_pfn_s>() - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_query_host_address",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s, hsa_ven_amd_loader_query_host_address
    ) - 0usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_query_segment_descriptors",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s, hsa_ven_amd_loader_query_segment_descriptors
    ) - 8usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_query_executable",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s, hsa_ven_amd_loader_query_executable
    ) - 16usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_executable_iterate_loaded_code_objects",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s,
        hsa_ven_amd_loader_executable_iterate_loaded_code_objects
    ) - 24usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_loaded_code_object_get_info",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s, hsa_ven_amd_loader_loaded_code_object_get_info
    ) - 32usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_code_object_reader_create_from_file_with_offset_size",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s,
        hsa_ven_amd_loader_code_object_reader_create_from_file_with_offset_size
    ) - 40usize];
    [
        "Offset of field: hsa_ven_amd_loader_1_03_pfn_s::hsa_ven_amd_loader_iterate_executables",
    ][::core::mem::offset_of!(
        hsa_ven_amd_loader_1_03_pfn_s, hsa_ven_amd_loader_iterate_executables
    ) - 48usize];
};
/// @brief Extension function table version 1.03.
pub type hsa_ven_amd_loader_1_03_pfn_t = hsa_ven_amd_loader_1_03_pfn_s;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_dynamic_library_t {
    _unused: [u8; 0],
}
pub type iree_hal_amdgpu_libhsa_flags_t = u32;
pub const IREE_HAL_AMDGPU_LIBHSA_FLAG_NONE: iree_hal_amdgpu_libhsa_flag_bits_e = 0;
pub type iree_hal_amdgpu_libhsa_flag_bits_e = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_amdgpu_libhsa_t {
    pub initialized: bool,
    pub library: *mut iree_dynamic_library_t,
    pub hsa_init: ::core::option::Option<unsafe extern "C" fn() -> hsa_status_t>,
    pub hsa_shut_down: ::core::option::Option<unsafe extern "C" fn() -> hsa_status_t>,
    pub hsa_amd_register_system_event_handler: ::core::option::Option<
        unsafe extern "C" fn(
            callback: hsa_amd_system_event_callback_t,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_system_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            attribute: hsa_system_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_system_major_extension_supported: ::core::option::Option<
        unsafe extern "C" fn(
            extension: u16,
            version_major: u16,
            version_minor: *mut u16,
            result: *mut bool,
        ) -> hsa_status_t,
    >,
    pub hsa_system_get_major_extension_table: ::core::option::Option<
        unsafe extern "C" fn(
            extension: u16,
            version_major: u16,
            table_length: usize,
            table: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_agent_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            attribute: hsa_agent_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_iterate_agents: ::core::option::Option<
        unsafe extern "C" fn(
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    agent: hsa_agent_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_cache_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            cache: hsa_cache_t,
            attribute: hsa_cache_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_agent_iterate_caches: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    cache: hsa_cache_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_region_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            region: hsa_region_t,
            attribute: hsa_region_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_agent_iterate_regions: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    region: hsa_region_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_memory_pool_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            memory_pool: hsa_amd_memory_pool_t,
            attribute: hsa_amd_memory_pool_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_agent_iterate_memory_pools: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    memory_pool: hsa_amd_memory_pool_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_agent_memory_pool_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            memory_pool: hsa_amd_memory_pool_t,
            attribute: hsa_amd_agent_memory_pool_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_agents_allow_access: ::core::option::Option<
        unsafe extern "C" fn(
            num_agents: u32,
            agents: *const hsa_agent_t,
            flags: *const u32,
            ptr: *const ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_memory_copy: ::core::option::Option<
        unsafe extern "C" fn(
            dst: *mut ::core::ffi::c_void,
            src: *const ::core::ffi::c_void,
            size: usize,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_memory_async_copy: ::core::option::Option<
        unsafe extern "C" fn(
            dst: *mut ::core::ffi::c_void,
            dst_agent: hsa_agent_t,
            src: *const ::core::ffi::c_void,
            src_agent: hsa_agent_t,
            size: usize,
            num_dep_signals: u32,
            dep_signals: *const hsa_signal_t,
            completion_signal: hsa_signal_t,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_memory_pool_allocate: ::core::option::Option<
        unsafe extern "C" fn(
            memory_pool: hsa_amd_memory_pool_t,
            size: usize,
            flags: u32,
            ptr: *mut *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_memory_pool_free: ::core::option::Option<
        unsafe extern "C" fn(ptr: *mut ::core::ffi::c_void) -> hsa_status_t,
    >,
    pub hsa_amd_memory_lock: ::core::option::Option<
        unsafe extern "C" fn(
            host_ptr: *mut ::core::ffi::c_void,
            size: usize,
            agents: *mut hsa_agent_t,
            num_agent: ::core::ffi::c_int,
            agent_ptr: *mut *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_memory_lock_to_pool: ::core::option::Option<
        unsafe extern "C" fn(
            host_ptr: *mut ::core::ffi::c_void,
            size: usize,
            agents: *mut hsa_agent_t,
            num_agent: ::core::ffi::c_int,
            pool: hsa_amd_memory_pool_t,
            flags: u32,
            agent_ptr: *mut *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_memory_unlock: ::core::option::Option<
        unsafe extern "C" fn(host_ptr: *mut ::core::ffi::c_void) -> hsa_status_t,
    >,
    pub hsa_amd_interop_map_buffer: ::core::option::Option<
        unsafe extern "C" fn(
            num_agents: u32,
            agents: *mut hsa_agent_t,
            interop_handle: ::core::ffi::c_int,
            flags: u32,
            size: *mut usize,
            ptr: *mut *mut ::core::ffi::c_void,
            metadata_size: *mut usize,
            metadata: *mut *const ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_interop_unmap_buffer: ::core::option::Option<
        unsafe extern "C" fn(ptr: *mut ::core::ffi::c_void) -> hsa_status_t,
    >,
    pub hsa_amd_pointer_info: ::core::option::Option<
        unsafe extern "C" fn(
            ptr: *const ::core::ffi::c_void,
            info: *mut hsa_amd_pointer_info_t,
            alloc: ::core::option::Option<
                unsafe extern "C" fn(arg1: usize) -> *mut ::core::ffi::c_void,
            >,
            num_agents_accessible: *mut u32,
            accessible: *mut *mut hsa_agent_t,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_ipc_memory_create: ::core::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::core::ffi::c_void,
            len: usize,
            handle: *mut hsa_amd_ipc_memory_t,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_ipc_memory_attach: ::core::option::Option<
        unsafe extern "C" fn(
            handle: *const hsa_amd_ipc_memory_t,
            len: usize,
            num_agents: u32,
            mapping_agents: *const hsa_agent_t,
            mapped_ptr: *mut *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_ipc_memory_detach: ::core::option::Option<
        unsafe extern "C" fn(mapped_ptr: *mut ::core::ffi::c_void) -> hsa_status_t,
    >,
    pub hsa_amd_portable_export_dmabuf: ::core::option::Option<
        unsafe extern "C" fn(
            ptr: *const ::core::ffi::c_void,
            size: usize,
            dmabuf: *mut ::core::ffi::c_int,
            offset: *mut u64,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_portable_close_dmabuf: ::core::option::Option<
        unsafe extern "C" fn(dmabuf: ::core::ffi::c_int) -> hsa_status_t,
    >,
    pub hsa_amd_svm_attributes_get: ::core::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::core::ffi::c_void,
            size: usize,
            attribute_list: *mut hsa_amd_svm_attribute_pair_t,
            attribute_count: usize,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_svm_attributes_set: ::core::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::core::ffi::c_void,
            size: usize,
            attribute_list: *mut hsa_amd_svm_attribute_pair_t,
            attribute_count: usize,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_svm_prefetch_async: ::core::option::Option<
        unsafe extern "C" fn(
            ptr: *mut ::core::ffi::c_void,
            size: usize,
            agent: hsa_agent_t,
            num_dep_signals: u32,
            dep_signals: *const hsa_signal_t,
            completion_signal: hsa_signal_t,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_address_reserve_align: ::core::option::Option<
        unsafe extern "C" fn(
            va: *mut *mut ::core::ffi::c_void,
            size: usize,
            address: u64,
            alignment: u64,
            flags: u64,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_address_free: ::core::option::Option<
        unsafe extern "C" fn(va: *mut ::core::ffi::c_void, size: usize) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_set_access: ::core::option::Option<
        unsafe extern "C" fn(
            va: *mut ::core::ffi::c_void,
            size: usize,
            desc: *const hsa_amd_memory_access_desc_t,
            desc_cnt: usize,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_handle_create: ::core::option::Option<
        unsafe extern "C" fn(
            pool: hsa_amd_memory_pool_t,
            size: usize,
            type_: hsa_amd_memory_type_t,
            flags: u64,
            memory_handle: *mut hsa_amd_vmem_alloc_handle_t,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_handle_release: ::core::option::Option<
        unsafe extern "C" fn(memory_handle: hsa_amd_vmem_alloc_handle_t) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_map: ::core::option::Option<
        unsafe extern "C" fn(
            va: *mut ::core::ffi::c_void,
            size: usize,
            in_offset: usize,
            memory_handle: hsa_amd_vmem_alloc_handle_t,
            flags: u64,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_vmem_unmap: ::core::option::Option<
        unsafe extern "C" fn(va: *mut ::core::ffi::c_void, size: usize) -> hsa_status_t,
    >,
    pub hsa_amd_signal_create: ::core::option::Option<
        unsafe extern "C" fn(
            initial_value: hsa_signal_value_t,
            num_consumers: u32,
            consumers: *const hsa_agent_t,
            attributes: u64,
            signal: *mut hsa_signal_t,
        ) -> hsa_status_t,
    >,
    pub hsa_signal_destroy: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t) -> hsa_status_t,
    >,
    pub hsa_signal_load_scacquire: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t) -> hsa_signal_value_t,
    >,
    pub hsa_signal_load_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t) -> hsa_signal_value_t,
    >,
    pub hsa_signal_store_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_store_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_silent_store_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_silent_store_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_exchange_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_exchange_scacquire: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_exchange_relaxed: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_exchange_screlease: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_cas_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            expected: hsa_signal_value_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_cas_scacquire: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            expected: hsa_signal_value_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_cas_relaxed: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            expected: hsa_signal_value_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_cas_screlease: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            expected: hsa_signal_value_t,
            value: hsa_signal_value_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_add_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_add_scacquire: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_add_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_add_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_subtract_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_subtract_scacquire: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_subtract_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_subtract_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_and_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_and_scacquire: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_and_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_and_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_or_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_or_scacquire: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_or_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_or_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_xor_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_xor_scacquire: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_xor_relaxed: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_xor_screlease: ::core::option::Option<
        unsafe extern "C" fn(signal: hsa_signal_t, value: hsa_signal_value_t),
    >,
    pub hsa_signal_wait_scacquire: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            condition: hsa_signal_condition_t,
            compare_value: hsa_signal_value_t,
            timeout_hint: u64,
            wait_state_hint: hsa_wait_state_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_signal_wait_relaxed: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            condition: hsa_signal_condition_t,
            compare_value: hsa_signal_value_t,
            timeout_hint: u64,
            wait_state_hint: hsa_wait_state_t,
        ) -> hsa_signal_value_t,
    >,
    pub hsa_amd_signal_wait_all: ::core::option::Option<
        unsafe extern "C" fn(
            signal_count: u32,
            signals: *mut hsa_signal_t,
            conds: *mut hsa_signal_condition_t,
            values: *mut hsa_signal_value_t,
            timeout_hint: u64,
            wait_hint: hsa_wait_state_t,
            satisfying_values: *mut hsa_signal_value_t,
        ) -> u32,
    >,
    pub hsa_amd_signal_wait_any: ::core::option::Option<
        unsafe extern "C" fn(
            signal_count: u32,
            signals: *mut hsa_signal_t,
            conds: *mut hsa_signal_condition_t,
            values: *mut hsa_signal_value_t,
            timeout_hint: u64,
            wait_hint: hsa_wait_state_t,
            satisfying_value: *mut hsa_signal_value_t,
        ) -> u32,
    >,
    pub hsa_amd_ipc_signal_create: ::core::option::Option<
        unsafe extern "C" fn(
            signal: hsa_signal_t,
            handle: *mut hsa_amd_ipc_signal_t,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_ipc_signal_attach: ::core::option::Option<
        unsafe extern "C" fn(
            handle: *const hsa_amd_ipc_signal_t,
            signal: *mut hsa_signal_t,
        ) -> hsa_status_t,
    >,
    pub hsa_queue_create: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            size: u32,
            type_: hsa_queue_type32_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    status: hsa_status_t,
                    source: *mut hsa_queue_t,
                    data: *mut ::core::ffi::c_void,
                ),
            >,
            data: *mut ::core::ffi::c_void,
            private_segment_size: u32,
            group_segment_size: u32,
            queue: *mut *mut hsa_queue_t,
        ) -> hsa_status_t,
    >,
    pub hsa_soft_queue_create: ::core::option::Option<
        unsafe extern "C" fn(
            region: hsa_region_t,
            size: u32,
            type_: hsa_queue_type32_t,
            features: u32,
            doorbell_signal: hsa_signal_t,
            queue: *mut *mut hsa_queue_t,
        ) -> hsa_status_t,
    >,
    pub hsa_queue_destroy: ::core::option::Option<
        unsafe extern "C" fn(queue: *mut hsa_queue_t) -> hsa_status_t,
    >,
    pub hsa_queue_inactivate: ::core::option::Option<
        unsafe extern "C" fn(queue: *mut hsa_queue_t) -> hsa_status_t,
    >,
    pub hsa_amd_profiling_set_profiler_enabled: ::core::option::Option<
        unsafe extern "C" fn(
            queue: *mut hsa_queue_t,
            enable: ::core::ffi::c_int,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_queue_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            queue: *mut hsa_queue_t,
            attribute: hsa_queue_info_attribute_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_queue_cu_set_mask: ::core::option::Option<
        unsafe extern "C" fn(
            queue: *const hsa_queue_t,
            num_cu_mask_count: u32,
            cu_mask: *const u32,
        ) -> hsa_status_t,
    >,
    pub hsa_amd_queue_set_priority: ::core::option::Option<
        unsafe extern "C" fn(
            queue: *mut hsa_queue_t,
            priority: hsa_amd_queue_priority_t,
        ) -> hsa_status_t,
    >,
    pub hsa_queue_load_read_index_scacquire: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t) -> u64,
    >,
    pub hsa_queue_load_read_index_relaxed: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t) -> u64,
    >,
    pub hsa_queue_load_write_index_relaxed: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t) -> u64,
    >,
    pub hsa_queue_load_write_index_scacquire: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t) -> u64,
    >,
    pub hsa_queue_store_write_index_relaxed: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64),
    >,
    pub hsa_queue_store_write_index_screlease: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64),
    >,
    pub hsa_queue_cas_write_index_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, expected: u64, value: u64) -> u64,
    >,
    pub hsa_queue_cas_write_index_scacquire: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, expected: u64, value: u64) -> u64,
    >,
    pub hsa_queue_cas_write_index_relaxed: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, expected: u64, value: u64) -> u64,
    >,
    pub hsa_queue_cas_write_index_screlease: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, expected: u64, value: u64) -> u64,
    >,
    pub hsa_queue_add_write_index_scacq_screl: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64) -> u64,
    >,
    pub hsa_queue_add_write_index_scacquire: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64) -> u64,
    >,
    pub hsa_queue_add_write_index_relaxed: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64) -> u64,
    >,
    pub hsa_queue_add_write_index_screlease: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64) -> u64,
    >,
    pub hsa_queue_store_read_index_relaxed: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64),
    >,
    pub hsa_queue_store_read_index_screlease: ::core::option::Option<
        unsafe extern "C" fn(queue: *const hsa_queue_t, value: u64),
    >,
    pub hsa_agent_iterate_isas: ::core::option::Option<
        unsafe extern "C" fn(
            agent: hsa_agent_t,
            callback: ::core::option::Option<
                unsafe extern "C" fn(
                    isa: hsa_isa_t,
                    data: *mut ::core::ffi::c_void,
                ) -> hsa_status_t,
            >,
            data: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_isa_get_info_alt: ::core::option::Option<
        unsafe extern "C" fn(
            isa: hsa_isa_t,
            attribute: hsa_isa_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub hsa_code_object_reader_create_from_memory: ::core::option::Option<
        unsafe extern "C" fn(
            code_object: *const ::core::ffi::c_void,
            size: usize,
            code_object_reader: *mut hsa_code_object_reader_t,
        ) -> hsa_status_t,
    >,
    pub hsa_code_object_reader_destroy: ::core::option::Option<
        unsafe extern "C" fn(
            code_object_reader: hsa_code_object_reader_t,
        ) -> hsa_status_t,
    >,
    pub hsa_executable_create_alt: ::core::option::Option<
        unsafe extern "C" fn(
            profile: hsa_profile_t,
            default_float_rounding_mode: hsa_default_float_rounding_mode_t,
            options: *const ::core::ffi::c_char,
            executable: *mut hsa_executable_t,
        ) -> hsa_status_t,
    >,
    pub hsa_executable_destroy: ::core::option::Option<
        unsafe extern "C" fn(executable: hsa_executable_t) -> hsa_status_t,
    >,
    pub hsa_executable_load_agent_code_object: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            agent: hsa_agent_t,
            code_object_reader: hsa_code_object_reader_t,
            options: *const ::core::ffi::c_char,
            loaded_code_object: *mut hsa_loaded_code_object_t,
        ) -> hsa_status_t,
    >,
    pub hsa_executable_freeze: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            options: *const ::core::ffi::c_char,
        ) -> hsa_status_t,
    >,
    pub hsa_executable_validate_alt: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            options: *const ::core::ffi::c_char,
            result: *mut u32,
        ) -> hsa_status_t,
    >,
    pub hsa_executable_get_symbol_by_name: ::core::option::Option<
        unsafe extern "C" fn(
            executable: hsa_executable_t,
            symbol_name: *const ::core::ffi::c_char,
            agent: *const hsa_agent_t,
            symbol: *mut hsa_executable_symbol_t,
        ) -> hsa_status_t,
    >,
    pub hsa_executable_symbol_get_info: ::core::option::Option<
        unsafe extern "C" fn(
            executable_symbol: hsa_executable_symbol_t,
            attribute: hsa_executable_symbol_info_t,
            value: *mut ::core::ffi::c_void,
        ) -> hsa_status_t,
    >,
    pub amd_loader: hsa_ven_amd_loader_1_03_pfn_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_libhsa_t",
    ][::core::mem::size_of::<iree_hal_amdgpu_libhsa_t>() - 1016usize];
    [
        "Alignment of iree_hal_amdgpu_libhsa_t",
    ][::core::mem::align_of::<iree_hal_amdgpu_libhsa_t>() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::initialized",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, initialized) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::library",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, library) - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_init",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_init) - 16usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_shut_down",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_shut_down) - 24usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_register_system_event_handler",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_amd_register_system_event_handler
    ) - 32usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_system_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_system_get_info) - 40usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_system_major_extension_supported",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_system_major_extension_supported
    ) - 48usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_system_get_major_extension_table",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_system_get_major_extension_table
    ) - 56usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_agent_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_agent_get_info) - 64usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_iterate_agents",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_iterate_agents) - 72usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_cache_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_cache_get_info) - 80usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_agent_iterate_caches",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_agent_iterate_caches)
        - 88usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_region_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_region_get_info) - 96usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_agent_iterate_regions",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_agent_iterate_regions)
        - 104usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_pool_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_pool_get_info)
        - 112usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_agent_iterate_memory_pools",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_amd_agent_iterate_memory_pools
    ) - 120usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_agent_memory_pool_get_info",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_amd_agent_memory_pool_get_info
    ) - 128usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_agents_allow_access",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_agents_allow_access)
        - 136usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_memory_copy",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_memory_copy) - 144usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_async_copy",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_async_copy)
        - 152usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_pool_allocate",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_pool_allocate)
        - 160usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_pool_free",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_pool_free)
        - 168usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_lock",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_lock) - 176usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_lock_to_pool",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_lock_to_pool)
        - 184usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_memory_unlock",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_memory_unlock)
        - 192usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_interop_map_buffer",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_interop_map_buffer)
        - 200usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_interop_unmap_buffer",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_interop_unmap_buffer)
        - 208usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_pointer_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_pointer_info)
        - 216usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_ipc_memory_create",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_ipc_memory_create)
        - 224usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_ipc_memory_attach",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_ipc_memory_attach)
        - 232usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_ipc_memory_detach",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_ipc_memory_detach)
        - 240usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_portable_export_dmabuf",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_portable_export_dmabuf)
        - 248usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_portable_close_dmabuf",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_portable_close_dmabuf)
        - 256usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_svm_attributes_get",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_svm_attributes_get)
        - 264usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_svm_attributes_set",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_svm_attributes_set)
        - 272usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_svm_prefetch_async",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_svm_prefetch_async)
        - 280usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_address_reserve_align",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_address_reserve_align
    ) - 288usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_address_free",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_address_free)
        - 296usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_set_access",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_set_access)
        - 304usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_handle_create",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_handle_create)
        - 312usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_handle_release",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_handle_release)
        - 320usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_map",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_map) - 328usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_vmem_unmap",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_vmem_unmap) - 336usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_signal_create",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_signal_create)
        - 344usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_destroy",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_destroy) - 352usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_load_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_load_scacquire)
        - 360usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_load_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_load_relaxed)
        - 368usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_store_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_store_relaxed)
        - 376usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_store_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_store_screlease)
        - 384usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_silent_store_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_silent_store_relaxed)
        - 392usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_silent_store_screlease",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_signal_silent_store_screlease
    ) - 400usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_exchange_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_exchange_scacq_screl)
        - 408usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_exchange_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_exchange_scacquire)
        - 416usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_exchange_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_exchange_relaxed)
        - 424usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_exchange_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_exchange_screlease)
        - 432usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_cas_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_cas_scacq_screl)
        - 440usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_cas_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_cas_scacquire)
        - 448usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_cas_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_cas_relaxed)
        - 456usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_cas_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_cas_screlease)
        - 464usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_add_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_add_scacq_screl)
        - 472usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_add_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_add_scacquire)
        - 480usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_add_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_add_relaxed)
        - 488usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_add_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_add_screlease)
        - 496usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_subtract_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_subtract_scacq_screl)
        - 504usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_subtract_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_subtract_scacquire)
        - 512usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_subtract_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_subtract_relaxed)
        - 520usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_subtract_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_subtract_screlease)
        - 528usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_and_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_and_scacq_screl)
        - 536usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_and_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_and_scacquire)
        - 544usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_and_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_and_relaxed)
        - 552usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_and_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_and_screlease)
        - 560usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_or_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_or_scacq_screl)
        - 568usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_or_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_or_scacquire)
        - 576usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_or_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_or_relaxed)
        - 584usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_or_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_or_screlease)
        - 592usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_xor_scacq_screl",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_xor_scacq_screl)
        - 600usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_xor_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_xor_scacquire)
        - 608usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_xor_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_xor_relaxed)
        - 616usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_xor_screlease",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_xor_screlease)
        - 624usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_wait_scacquire",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_wait_scacquire)
        - 632usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_signal_wait_relaxed",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_signal_wait_relaxed)
        - 640usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_signal_wait_all",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_signal_wait_all)
        - 648usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_signal_wait_any",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_signal_wait_any)
        - 656usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_ipc_signal_create",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_ipc_signal_create)
        - 664usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_ipc_signal_attach",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_ipc_signal_attach)
        - 672usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_create",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_queue_create) - 680usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_soft_queue_create",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_soft_queue_create)
        - 688usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_destroy",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_queue_destroy) - 696usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_inactivate",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_queue_inactivate)
        - 704usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_profiling_set_profiler_enabled",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_amd_profiling_set_profiler_enabled
    ) - 712usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_queue_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_queue_get_info)
        - 720usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_queue_cu_set_mask",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_queue_cu_set_mask)
        - 728usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_amd_queue_set_priority",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_amd_queue_set_priority)
        - 736usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_load_read_index_scacquire",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_load_read_index_scacquire
    ) - 744usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_load_read_index_relaxed",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_load_read_index_relaxed
    ) - 752usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_load_write_index_relaxed",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_load_write_index_relaxed
    ) - 760usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_load_write_index_scacquire",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_load_write_index_scacquire
    ) - 768usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_store_write_index_relaxed",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_store_write_index_relaxed
    ) - 776usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_store_write_index_screlease",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_store_write_index_screlease
    ) - 784usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_cas_write_index_scacq_screl",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_cas_write_index_scacq_screl
    ) - 792usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_cas_write_index_scacquire",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_cas_write_index_scacquire
    ) - 800usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_cas_write_index_relaxed",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_cas_write_index_relaxed
    ) - 808usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_cas_write_index_screlease",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_cas_write_index_screlease
    ) - 816usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_add_write_index_scacq_screl",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_add_write_index_scacq_screl
    ) - 824usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_add_write_index_scacquire",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_add_write_index_scacquire
    ) - 832usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_add_write_index_relaxed",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_add_write_index_relaxed
    ) - 840usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_add_write_index_screlease",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_add_write_index_screlease
    ) - 848usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_store_read_index_relaxed",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_store_read_index_relaxed
    ) - 856usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_queue_store_read_index_screlease",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_queue_store_read_index_screlease
    ) - 864usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_agent_iterate_isas",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_agent_iterate_isas)
        - 872usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_isa_get_info_alt",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_isa_get_info_alt)
        - 880usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_code_object_reader_create_from_memory",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_code_object_reader_create_from_memory
    ) - 888usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_code_object_reader_destroy",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_code_object_reader_destroy)
        - 896usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_create_alt",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_executable_create_alt)
        - 904usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_destroy",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_executable_destroy)
        - 912usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_load_agent_code_object",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_executable_load_agent_code_object
    ) - 920usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_freeze",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_executable_freeze)
        - 928usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_validate_alt",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_executable_validate_alt)
        - 936usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_get_symbol_by_name",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_libhsa_t, hsa_executable_get_symbol_by_name
    ) - 944usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::hsa_executable_symbol_get_info",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, hsa_executable_symbol_get_info)
        - 952usize];
    [
        "Offset of field: iree_hal_amdgpu_libhsa_t::amd_loader",
    ][::core::mem::offset_of!(iree_hal_amdgpu_libhsa_t, amd_loader) - 960usize];
};
impl Default for iree_hal_amdgpu_libhsa_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_libhsa_initialize(
        flags: iree_hal_amdgpu_libhsa_flags_t,
        search_paths: iree_string_view_list_t,
        host_allocator: iree_allocator_t,
        out_libhsa: *mut iree_hal_amdgpu_libhsa_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_libhsa_deinitialize(libhsa: *mut iree_hal_amdgpu_libhsa_t);
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_libhsa_copy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        out_libhsa: *mut iree_hal_amdgpu_libhsa_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_libhsa_append_path_to_builder(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        builder: *mut iree_string_builder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_status_from_hsa_status(
        file: *const ::core::ffi::c_char,
        line: u32,
        hsa_status: hsa_status_t,
        symbol: *const ::core::ffi::c_char,
        message: *const ::core::ffi::c_char,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_init(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_init_raw(libhsa: *const iree_hal_amdgpu_libhsa_t) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_shut_down(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_shut_down_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_register_system_event_handler(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        callback: hsa_amd_system_event_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_register_system_event_handler_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        callback: hsa_amd_system_event_callback_t,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_system_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        attribute: hsa_system_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_system_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        attribute: hsa_system_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_system_major_extension_supported(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        extension: u16,
        version_major: u16,
        version_minor: *mut u16,
        result: *mut bool,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_system_major_extension_supported_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        extension: u16,
        version_major: u16,
        version_minor: *mut u16,
        result: *mut bool,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_system_get_major_extension_table(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        extension: u16,
        version_major: u16,
        table_length: usize,
        table: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_system_get_major_extension_table_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        extension: u16,
        version_major: u16,
        table_length: usize,
        table: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        attribute: hsa_agent_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        attribute: hsa_agent_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_iterate_agents(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                agent: hsa_agent_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_iterate_agents_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                agent: hsa_agent_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_cache_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        cache: hsa_cache_t,
        attribute: hsa_cache_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_cache_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        cache: hsa_cache_t,
        attribute: hsa_cache_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_iterate_caches(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                cache: hsa_cache_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_iterate_caches_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                cache: hsa_cache_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_region_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        region: hsa_region_t,
        attribute: hsa_region_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_region_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        region: hsa_region_t,
        attribute: hsa_region_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_iterate_regions(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                region: hsa_region_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_iterate_regions_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                region: hsa_region_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_pool_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        memory_pool: hsa_amd_memory_pool_t,
        attribute: hsa_amd_memory_pool_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_pool_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        memory_pool: hsa_amd_memory_pool_t,
        attribute: hsa_amd_memory_pool_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_agent_iterate_memory_pools(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                memory_pool: hsa_amd_memory_pool_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_agent_iterate_memory_pools_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                memory_pool: hsa_amd_memory_pool_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_agent_memory_pool_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        memory_pool: hsa_amd_memory_pool_t,
        attribute: hsa_amd_agent_memory_pool_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_agent_memory_pool_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        memory_pool: hsa_amd_memory_pool_t,
        attribute: hsa_amd_agent_memory_pool_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_agents_allow_access(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        num_agents: u32,
        agents: *const hsa_agent_t,
        flags: *const u32,
        ptr: *const ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_agents_allow_access_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        num_agents: u32,
        agents: *const hsa_agent_t,
        flags: *const u32,
        ptr: *const ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_memory_copy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        size: usize,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_memory_copy_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        size: usize,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_async_copy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        dst: *mut ::core::ffi::c_void,
        dst_agent: hsa_agent_t,
        src: *const ::core::ffi::c_void,
        src_agent: hsa_agent_t,
        size: usize,
        num_dep_signals: u32,
        dep_signals: *const hsa_signal_t,
        completion_signal: hsa_signal_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_async_copy_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        dst: *mut ::core::ffi::c_void,
        dst_agent: hsa_agent_t,
        src: *const ::core::ffi::c_void,
        src_agent: hsa_agent_t,
        size: usize,
        num_dep_signals: u32,
        dep_signals: *const hsa_signal_t,
        completion_signal: hsa_signal_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_pool_allocate(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        memory_pool: hsa_amd_memory_pool_t,
        size: usize,
        flags: u32,
        ptr: *mut *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_pool_allocate_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        memory_pool: hsa_amd_memory_pool_t,
        size: usize,
        flags: u32,
        ptr: *mut *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_pool_free(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_pool_free_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_lock(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        host_ptr: *mut ::core::ffi::c_void,
        size: usize,
        agents: *mut hsa_agent_t,
        num_agent: ::core::ffi::c_int,
        agent_ptr: *mut *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_lock_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        host_ptr: *mut ::core::ffi::c_void,
        size: usize,
        agents: *mut hsa_agent_t,
        num_agent: ::core::ffi::c_int,
        agent_ptr: *mut *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_lock_to_pool(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        host_ptr: *mut ::core::ffi::c_void,
        size: usize,
        agents: *mut hsa_agent_t,
        num_agent: ::core::ffi::c_int,
        pool: hsa_amd_memory_pool_t,
        flags: u32,
        agent_ptr: *mut *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_lock_to_pool_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        host_ptr: *mut ::core::ffi::c_void,
        size: usize,
        agents: *mut hsa_agent_t,
        num_agent: ::core::ffi::c_int,
        pool: hsa_amd_memory_pool_t,
        flags: u32,
        agent_ptr: *mut *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_unlock(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        host_ptr: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_memory_unlock_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        host_ptr: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_interop_map_buffer(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        num_agents: u32,
        agents: *mut hsa_agent_t,
        interop_handle: ::core::ffi::c_int,
        flags: u32,
        size: *mut usize,
        ptr: *mut *mut ::core::ffi::c_void,
        metadata_size: *mut usize,
        metadata: *mut *const ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_interop_map_buffer_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        num_agents: u32,
        agents: *mut hsa_agent_t,
        interop_handle: ::core::ffi::c_int,
        flags: u32,
        size: *mut usize,
        ptr: *mut *mut ::core::ffi::c_void,
        metadata_size: *mut usize,
        metadata: *mut *const ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_interop_unmap_buffer(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_interop_unmap_buffer_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_pointer_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *const ::core::ffi::c_void,
        info: *mut hsa_amd_pointer_info_t,
        alloc: ::core::option::Option<
            unsafe extern "C" fn(arg1: usize) -> *mut ::core::ffi::c_void,
        >,
        num_agents_accessible: *mut u32,
        accessible: *mut *mut hsa_agent_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_pointer_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *const ::core::ffi::c_void,
        info: *mut hsa_amd_pointer_info_t,
        alloc: ::core::option::Option<
            unsafe extern "C" fn(arg1: usize) -> *mut ::core::ffi::c_void,
        >,
        num_agents_accessible: *mut u32,
        accessible: *mut *mut hsa_agent_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_memory_create(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *mut ::core::ffi::c_void,
        len: usize,
        handle: *mut hsa_amd_ipc_memory_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_memory_create_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *mut ::core::ffi::c_void,
        len: usize,
        handle: *mut hsa_amd_ipc_memory_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_memory_attach(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        handle: *const hsa_amd_ipc_memory_t,
        len: usize,
        num_agents: u32,
        mapping_agents: *const hsa_agent_t,
        mapped_ptr: *mut *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_memory_attach_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        handle: *const hsa_amd_ipc_memory_t,
        len: usize,
        num_agents: u32,
        mapping_agents: *const hsa_agent_t,
        mapped_ptr: *mut *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_memory_detach(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        mapped_ptr: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_memory_detach_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        mapped_ptr: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_portable_export_dmabuf(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *const ::core::ffi::c_void,
        size: usize,
        dmabuf: *mut ::core::ffi::c_int,
        offset: *mut u64,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_portable_export_dmabuf_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *const ::core::ffi::c_void,
        size: usize,
        dmabuf: *mut ::core::ffi::c_int,
        offset: *mut u64,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_portable_close_dmabuf(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        dmabuf: ::core::ffi::c_int,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_portable_close_dmabuf_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        dmabuf: ::core::ffi::c_int,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_svm_attributes_get(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        attribute_list: *mut hsa_amd_svm_attribute_pair_t,
        attribute_count: usize,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_svm_attributes_get_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        attribute_list: *mut hsa_amd_svm_attribute_pair_t,
        attribute_count: usize,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_svm_attributes_set(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        attribute_list: *mut hsa_amd_svm_attribute_pair_t,
        attribute_count: usize,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_svm_attributes_set_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        attribute_list: *mut hsa_amd_svm_attribute_pair_t,
        attribute_count: usize,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_svm_prefetch_async(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        agent: hsa_agent_t,
        num_dep_signals: u32,
        dep_signals: *const hsa_signal_t,
        completion_signal: hsa_signal_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_svm_prefetch_async_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        ptr: *mut ::core::ffi::c_void,
        size: usize,
        agent: hsa_agent_t,
        num_dep_signals: u32,
        dep_signals: *const hsa_signal_t,
        completion_signal: hsa_signal_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_address_reserve_align(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        va: *mut *mut ::core::ffi::c_void,
        size: usize,
        address: u64,
        alignment: u64,
        flags: u64,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_address_reserve_align_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        va: *mut *mut ::core::ffi::c_void,
        size: usize,
        address: u64,
        alignment: u64,
        flags: u64,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_address_free(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        va: *mut ::core::ffi::c_void,
        size: usize,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_address_free_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        va: *mut ::core::ffi::c_void,
        size: usize,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_set_access(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        va: *mut ::core::ffi::c_void,
        size: usize,
        desc: *const hsa_amd_memory_access_desc_t,
        desc_cnt: usize,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_set_access_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        va: *mut ::core::ffi::c_void,
        size: usize,
        desc: *const hsa_amd_memory_access_desc_t,
        desc_cnt: usize,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_handle_create(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        pool: hsa_amd_memory_pool_t,
        size: usize,
        type_: hsa_amd_memory_type_t,
        flags: u64,
        memory_handle: *mut hsa_amd_vmem_alloc_handle_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_handle_create_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        pool: hsa_amd_memory_pool_t,
        size: usize,
        type_: hsa_amd_memory_type_t,
        flags: u64,
        memory_handle: *mut hsa_amd_vmem_alloc_handle_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_handle_release(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        memory_handle: hsa_amd_vmem_alloc_handle_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_handle_release_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        memory_handle: hsa_amd_vmem_alloc_handle_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_map(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        va: *mut ::core::ffi::c_void,
        size: usize,
        in_offset: usize,
        memory_handle: hsa_amd_vmem_alloc_handle_t,
        flags: u64,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_map_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        va: *mut ::core::ffi::c_void,
        size: usize,
        in_offset: usize,
        memory_handle: hsa_amd_vmem_alloc_handle_t,
        flags: u64,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_unmap(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        va: *mut ::core::ffi::c_void,
        size: usize,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_vmem_unmap_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        va: *mut ::core::ffi::c_void,
        size: usize,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_signal_create(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        initial_value: hsa_signal_value_t,
        num_consumers: u32,
        consumers: *const hsa_agent_t,
        attributes: u64,
        signal: *mut hsa_signal_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_signal_create_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        initial_value: hsa_signal_value_t,
        num_consumers: u32,
        consumers: *const hsa_agent_t,
        attributes: u64,
        signal: *mut hsa_signal_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_destroy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_destroy_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        signal: hsa_signal_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_load_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_load_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_store_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_store_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_silent_store_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_silent_store_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_exchange_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_exchange_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_exchange_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_exchange_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_cas_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        expected: hsa_signal_value_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_cas_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        expected: hsa_signal_value_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_cas_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        expected: hsa_signal_value_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_cas_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        expected: hsa_signal_value_t,
        value: hsa_signal_value_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_add_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_add_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_add_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_add_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_subtract_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_subtract_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_subtract_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_subtract_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_and_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_and_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_and_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_and_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_or_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_or_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_or_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_or_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_xor_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_xor_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_xor_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_xor_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        value: hsa_signal_value_t,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_signal_wait_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        condition: hsa_signal_condition_t,
        compare_value: hsa_signal_value_t,
        timeout_hint: u64,
        wait_state_hint: hsa_wait_state_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_signal_wait_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        condition: hsa_signal_condition_t,
        compare_value: hsa_signal_value_t,
        timeout_hint: u64,
        wait_state_hint: hsa_wait_state_t,
    ) -> hsa_signal_value_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_signal_wait_all(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal_count: u32,
        signals: *mut hsa_signal_t,
        conds: *mut hsa_signal_condition_t,
        values: *mut hsa_signal_value_t,
        timeout_hint: u64,
        wait_hint: hsa_wait_state_t,
        satisfying_values: *mut hsa_signal_value_t,
    ) -> u32;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_signal_wait_any(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal_count: u32,
        signals: *mut hsa_signal_t,
        conds: *mut hsa_signal_condition_t,
        values: *mut hsa_signal_value_t,
        timeout_hint: u64,
        wait_hint: hsa_wait_state_t,
        satisfying_value: *mut hsa_signal_value_t,
    ) -> u32;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_signal_create(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        signal: hsa_signal_t,
        handle: *mut hsa_amd_ipc_signal_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_signal_create_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        signal: hsa_signal_t,
        handle: *mut hsa_amd_ipc_signal_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_signal_attach(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        handle: *const hsa_amd_ipc_signal_t,
        signal: *mut hsa_signal_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_ipc_signal_attach_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        handle: *const hsa_amd_ipc_signal_t,
        signal: *mut hsa_signal_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_create(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        size: u32,
        type_: hsa_queue_type32_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                status: hsa_status_t,
                source: *mut hsa_queue_t,
                data: *mut ::core::ffi::c_void,
            ),
        >,
        data: *mut ::core::ffi::c_void,
        private_segment_size: u32,
        group_segment_size: u32,
        queue: *mut *mut hsa_queue_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_create_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        size: u32,
        type_: hsa_queue_type32_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                status: hsa_status_t,
                source: *mut hsa_queue_t,
                data: *mut ::core::ffi::c_void,
            ),
        >,
        data: *mut ::core::ffi::c_void,
        private_segment_size: u32,
        group_segment_size: u32,
        queue: *mut *mut hsa_queue_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_soft_queue_create(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        region: hsa_region_t,
        size: u32,
        type_: hsa_queue_type32_t,
        features: u32,
        doorbell_signal: hsa_signal_t,
        queue: *mut *mut hsa_queue_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_soft_queue_create_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        region: hsa_region_t,
        size: u32,
        type_: hsa_queue_type32_t,
        features: u32,
        doorbell_signal: hsa_signal_t,
        queue: *mut *mut hsa_queue_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_destroy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *mut hsa_queue_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_destroy_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        queue: *mut hsa_queue_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_inactivate(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *mut hsa_queue_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_inactivate_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        queue: *mut hsa_queue_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_profiling_set_profiler_enabled(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *mut hsa_queue_t,
        enable: ::core::ffi::c_int,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_profiling_set_profiler_enabled_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        queue: *mut hsa_queue_t,
        enable: ::core::ffi::c_int,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_queue_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *mut hsa_queue_t,
        attribute: hsa_queue_info_attribute_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_queue_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        queue: *mut hsa_queue_t,
        attribute: hsa_queue_info_attribute_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_queue_cu_set_mask(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        num_cu_mask_count: u32,
        cu_mask: *const u32,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_queue_cu_set_mask_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        queue: *const hsa_queue_t,
        num_cu_mask_count: u32,
        cu_mask: *const u32,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_queue_set_priority(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *mut hsa_queue_t,
        priority: hsa_amd_queue_priority_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_amd_queue_set_priority_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        queue: *mut hsa_queue_t,
        priority: hsa_amd_queue_priority_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_load_read_index_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_load_read_index_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_load_write_index_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_load_write_index_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_store_write_index_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_queue_store_write_index_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_queue_cas_write_index_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        expected: u64,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_cas_write_index_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        expected: u64,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_cas_write_index_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        expected: u64,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_cas_write_index_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        expected: u64,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_add_write_index_scacq_screl(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_add_write_index_scacquire(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_add_write_index_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_add_write_index_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    ) -> u64;
}
unsafe extern "C" {
    pub fn iree_hsa_queue_store_read_index_relaxed(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_queue_store_read_index_screlease(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        queue: *const hsa_queue_t,
        value: u64,
    );
}
unsafe extern "C" {
    pub fn iree_hsa_agent_iterate_isas(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                isa: hsa_isa_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_agent_iterate_isas_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        callback: ::core::option::Option<
            unsafe extern "C" fn(
                isa: hsa_isa_t,
                data: *mut ::core::ffi::c_void,
            ) -> hsa_status_t,
        >,
        data: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_isa_get_info_alt(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        isa: hsa_isa_t,
        attribute: hsa_isa_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_isa_get_info_alt_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        isa: hsa_isa_t,
        attribute: hsa_isa_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_code_object_reader_create_from_memory(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        code_object: *const ::core::ffi::c_void,
        size: usize,
        code_object_reader: *mut hsa_code_object_reader_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_code_object_reader_create_from_memory_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        code_object: *const ::core::ffi::c_void,
        size: usize,
        code_object_reader: *mut hsa_code_object_reader_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_code_object_reader_destroy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        code_object_reader: hsa_code_object_reader_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_code_object_reader_destroy_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        code_object_reader: hsa_code_object_reader_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_create_alt(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        profile: hsa_profile_t,
        default_float_rounding_mode: hsa_default_float_rounding_mode_t,
        options: *const ::core::ffi::c_char,
        executable: *mut hsa_executable_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_create_alt_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        profile: hsa_profile_t,
        default_float_rounding_mode: hsa_default_float_rounding_mode_t,
        options: *const ::core::ffi::c_char,
        executable: *mut hsa_executable_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_destroy(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        executable: hsa_executable_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_destroy_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        executable: hsa_executable_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_load_agent_code_object(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        executable: hsa_executable_t,
        agent: hsa_agent_t,
        code_object_reader: hsa_code_object_reader_t,
        options: *const ::core::ffi::c_char,
        loaded_code_object: *mut hsa_loaded_code_object_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_load_agent_code_object_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        executable: hsa_executable_t,
        agent: hsa_agent_t,
        code_object_reader: hsa_code_object_reader_t,
        options: *const ::core::ffi::c_char,
        loaded_code_object: *mut hsa_loaded_code_object_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_freeze(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        executable: hsa_executable_t,
        options: *const ::core::ffi::c_char,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_freeze_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        executable: hsa_executable_t,
        options: *const ::core::ffi::c_char,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_validate_alt(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        executable: hsa_executable_t,
        options: *const ::core::ffi::c_char,
        result: *mut u32,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_validate_alt_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        executable: hsa_executable_t,
        options: *const ::core::ffi::c_char,
        result: *mut u32,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_get_symbol_by_name(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        executable: hsa_executable_t,
        symbol_name: *const ::core::ffi::c_char,
        agent: *const hsa_agent_t,
        symbol: *mut hsa_executable_symbol_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_get_symbol_by_name_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        executable: hsa_executable_t,
        symbol_name: *const ::core::ffi::c_char,
        agent: *const hsa_agent_t,
        symbol: *mut hsa_executable_symbol_t,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_symbol_get_info(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        file: *const ::core::ffi::c_char,
        line: u32,
        executable_symbol: hsa_executable_symbol_t,
        attribute: hsa_executable_symbol_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hsa_executable_symbol_get_info_raw(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        executable_symbol: hsa_executable_symbol_t,
        attribute: hsa_executable_symbol_info_t,
        value: *mut ::core::ffi::c_void,
    ) -> hsa_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_hal_amdgpu_hsa_cleanup_assert_success__extern"]
    pub fn iree_hal_amdgpu_hsa_cleanup_assert_success(status: hsa_status_t);
}
#[repr(C)]
pub struct iree_hal_amdgpu_topology_t {
    pub all_agent_count: iree_host_size_t,
    pub all_agents: [hsa_agent_t; 128usize],
    pub cpu_agent_count: iree_host_size_t,
    pub cpu_agents: [hsa_agent_t; 64usize],
    pub gpu_agent_count: iree_host_size_t,
    pub gpu_agents: [hsa_agent_t; 64usize],
    pub gpu_agent_queue_count: iree_host_size_t,
    pub gpu_cpu_map: [u8; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_topology_t",
    ][::core::mem::size_of::<iree_hal_amdgpu_topology_t>() - 2144usize];
    [
        "Alignment of iree_hal_amdgpu_topology_t",
    ][::core::mem::align_of::<iree_hal_amdgpu_topology_t>() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::all_agent_count",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, all_agent_count) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::all_agents",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, all_agents) - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::cpu_agent_count",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, cpu_agent_count) - 1032usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::cpu_agents",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, cpu_agents) - 1040usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::gpu_agent_count",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, gpu_agent_count) - 1552usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::gpu_agents",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, gpu_agents) - 1560usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::gpu_agent_queue_count",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, gpu_agent_queue_count)
        - 2072usize];
    [
        "Offset of field: iree_hal_amdgpu_topology_t::gpu_cpu_map",
    ][::core::mem::offset_of!(iree_hal_amdgpu_topology_t, gpu_cpu_map) - 2080usize];
};
impl Default for iree_hal_amdgpu_topology_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_initialize(
        out_topology: *mut iree_hal_amdgpu_topology_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_deinitialize(
        topology: *mut iree_hal_amdgpu_topology_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_insert_cpu_agent(
        topology: *mut iree_hal_amdgpu_topology_t,
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        cpu_agent: hsa_agent_t,
        out_index: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_insert_gpu_agent(
        topology: *mut iree_hal_amdgpu_topology_t,
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        gpu_agent: hsa_agent_t,
        cpu_agent: hsa_agent_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_insert_gpu_agent_with_nearest_cpu_agent(
        topology: *mut iree_hal_amdgpu_topology_t,
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        gpu_agent: hsa_agent_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_query_agent_compatibility(
        topology: *mut iree_hal_amdgpu_topology_t,
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        agent: hsa_agent_t,
        out_is_compatible: *mut bool,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_verify(
        topology: *const iree_hal_amdgpu_topology_t,
        libhsa: *const iree_hal_amdgpu_libhsa_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_initialize_with_defaults(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        out_topology: *mut iree_hal_amdgpu_topology_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_initialize_from_path(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        path: iree_string_view_t,
        out_topology: *mut iree_hal_amdgpu_topology_t,
    ) -> iree_status_t;
}
pub type iree_hal_amdgpu_gpu_agent_mask_t = u64;
unsafe extern "C" {
    pub fn iree_hal_amdgpu_topology_initialize_from_gpu_agent_mask(
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        gpu_agent_mask: u64,
        out_topology: *mut iree_hal_amdgpu_topology_t,
    ) -> iree_status_t;
}
pub const IREE_HAL_AMDGPU_QUEUE_PLACEMENT_ANY: iree_hal_amdgpu_queue_placement_e = 0;
pub const IREE_HAL_AMDGPU_QUEUE_PLACEMENT_HOST: iree_hal_amdgpu_queue_placement_e = 1;
pub const IREE_HAL_AMDGPU_QUEUE_PLACEMENT_DEVICE: iree_hal_amdgpu_queue_placement_e = 2;
pub type iree_hal_amdgpu_queue_placement_e = ::core::ffi::c_uint;
pub use self::iree_hal_amdgpu_queue_placement_e as iree_hal_amdgpu_queue_placement_t;
pub const IREE_HAL_AMDGPU_COMMAND_BUFFER_MODE_AQL: iree_hal_amdgpu_command_buffer_mode_e = 0;
pub const IREE_HAL_AMDGPU_COMMAND_BUFFER_MODE_PM4: iree_hal_amdgpu_command_buffer_mode_e = 1;
pub const IREE_HAL_AMDGPU_COMMAND_BUFFER_MODE_AUTO: iree_hal_amdgpu_command_buffer_mode_e = 2;
pub type iree_hal_amdgpu_command_buffer_mode_e = ::core::ffi::c_uint;
pub use self::iree_hal_amdgpu_command_buffer_mode_e as iree_hal_amdgpu_command_buffer_mode_t;
pub const IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_DIRECT: iree_hal_amdgpu_pm4_command_buffer_publication_mode_e = 0;
pub const IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_HOST_COPY: iree_hal_amdgpu_pm4_command_buffer_publication_mode_e = 1;
pub const IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_HOST_ASYNC_COPY: iree_hal_amdgpu_pm4_command_buffer_publication_mode_e = 2;
pub const IREE_HAL_AMDGPU_PM4_COMMAND_BUFFER_PUBLICATION_MODE_HOST_ASYNC_COPY_NONBLOCKING: iree_hal_amdgpu_pm4_command_buffer_publication_mode_e = 3;
pub type iree_hal_amdgpu_pm4_command_buffer_publication_mode_e = ::core::ffi::c_uint;
pub use self::iree_hal_amdgpu_pm4_command_buffer_publication_mode_e as iree_hal_amdgpu_pm4_command_buffer_publication_mode_t;
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t {
    pub host_block_pools: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1,
    pub device_block_pools: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2,
    pub default_pool: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3,
    pub queue_placement: iree_hal_amdgpu_queue_placement_t,
    pub command_buffer_mode: iree_hal_amdgpu_command_buffer_mode_t,
    pub pm4_command_buffer_publication_mode: iree_hal_amdgpu_pm4_command_buffer_publication_mode_t,
    pub host_queues: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub wait_active_for_ns: iree_duration_t,
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1 {
    pub small: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1,
    pub large: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2,
    pub command_buffer: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1 {
    pub block_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1",
    ][::core::mem::size_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1,
    >() - 8usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1",
    ][::core::mem::align_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1,
    >() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1::block_size",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1, block_size
    ) - 0usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2 {
    pub block_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2",
    ][::core::mem::size_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2,
    >() - 8usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2",
    ][::core::mem::align_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2,
    >() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2::block_size",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2, block_size
    ) - 0usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3 {
    pub usable_block_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3",
    ][::core::mem::size_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3,
    >() - 8usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3",
    ][::core::mem::align_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3,
    >() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3::usable_block_size",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3,
        usable_block_size
    ) - 0usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1",
    ][::core::mem::size_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1>()
        - 24usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1",
    ][::core::mem::align_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1>()
        - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1::small",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1, small
    ) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1::large",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1, large
    ) - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1::command_buffer",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1, command_buffer
    ) - 16usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2 {
    pub small: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1,
    pub large: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2,
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1 {
    pub block_size: iree_device_size_t,
    pub initial_capacity: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1",
    ][::core::mem::size_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1,
    >() - 16usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1",
    ][::core::mem::align_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1,
    >() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1::block_size",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1, block_size
    ) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1::initial_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1,
        initial_capacity
    ) - 8usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2 {
    pub block_size: iree_device_size_t,
    pub initial_capacity: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2",
    ][::core::mem::size_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2,
    >() - 16usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2",
    ][::core::mem::align_of::<
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2,
    >() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2::block_size",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2, block_size
    ) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2::initial_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2,
        initial_capacity
    ) - 8usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2",
    ][::core::mem::size_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2>()
        - 32usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2",
    ][::core::mem::align_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2>()
        - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2::small",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2, small
    ) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2::large",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2, large
    ) - 16usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3 {
    pub range_length: iree_device_size_t,
    pub alignment: iree_device_size_t,
    pub frontier_capacity: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3",
    ][::core::mem::size_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3>()
        - 24usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3",
    ][::core::mem::align_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3>()
        - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3::range_length",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3, range_length
    ) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3::alignment",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3, alignment
    ) - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3::frontier_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3, frontier_capacity
    ) - 16usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4 {
    pub aql_capacity: u32,
    pub notification_capacity: u32,
    pub kernarg_capacity: u32,
    pub upload_capacity: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4",
    ][::core::mem::size_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4>()
        - 16usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4",
    ][::core::mem::align_of::<iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4>()
        - 4usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4::aql_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4, aql_capacity
    ) - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4::notification_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4, notification_capacity
    ) - 4usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4::kernarg_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4, kernarg_capacity
    ) - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4::upload_capacity",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t__bindgen_ty_4, upload_capacity
    ) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_logical_device_options_t",
    ][::core::mem::size_of::<iree_hal_amdgpu_logical_device_options_t>() - 120usize];
    [
        "Alignment of iree_hal_amdgpu_logical_device_options_t",
    ][::core::mem::align_of::<iree_hal_amdgpu_logical_device_options_t>() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::host_block_pools",
    ][::core::mem::offset_of!(iree_hal_amdgpu_logical_device_options_t, host_block_pools)
        - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::device_block_pools",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t, device_block_pools
    ) - 24usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::default_pool",
    ][::core::mem::offset_of!(iree_hal_amdgpu_logical_device_options_t, default_pool)
        - 56usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::queue_placement",
    ][::core::mem::offset_of!(iree_hal_amdgpu_logical_device_options_t, queue_placement)
        - 80usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::command_buffer_mode",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t, command_buffer_mode
    ) - 84usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::pm4_command_buffer_publication_mode",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t, pm4_command_buffer_publication_mode
    ) - 88usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::host_queues",
    ][::core::mem::offset_of!(iree_hal_amdgpu_logical_device_options_t, host_queues)
        - 92usize];
    [
        "Offset of field: iree_hal_amdgpu_logical_device_options_t::wait_active_for_ns",
    ][::core::mem::offset_of!(
        iree_hal_amdgpu_logical_device_options_t, wait_active_for_ns
    ) - 112usize];
};
impl Default for iree_hal_amdgpu_logical_device_options_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl iree_hal_amdgpu_logical_device_options_t {
    #[inline]
    pub fn preallocate_pools(&self) -> u64 {
        self._bitfield_1.get(0usize, 1u8)
    }
    #[inline]
    pub fn set_preallocate_pools(&mut self, val: u64) {
        {
            let val: u64 = val;
            self._bitfield_1.set(0usize, 1u8, val)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn preallocate_pools_raw(this: *const Self) -> u64 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 0usize, 1u8)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_preallocate_pools_raw(this: *mut Self, val: u64) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u64 = val;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 0usize, 1u8, val)
        }
    }
    #[inline]
    pub fn exclusive_execution(&self) -> u64 {
        self._bitfield_1.get(1usize, 1u8)
    }
    #[inline]
    pub fn set_exclusive_execution(&mut self, val: u64) {
        {
            let val: u64 = val;
            self._bitfield_1.set(1usize, 1u8, val)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn exclusive_execution_raw(this: *const Self) -> u64 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 1usize, 1u8)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_exclusive_execution_raw(this: *mut Self, val: u64) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u64 = val;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 1usize, 1u8, val)
        }
    }
    #[inline]
    pub fn force_wait_barrier_defer(&self) -> u64 {
        self._bitfield_1.get(2usize, 1u8)
    }
    #[inline]
    pub fn set_force_wait_barrier_defer(&mut self, val: u64) {
        {
            let val: u64 = val;
            self._bitfield_1.set(2usize, 1u8, val)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn force_wait_barrier_defer_raw(this: *const Self) -> u64 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 2usize, 1u8)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_force_wait_barrier_defer_raw(this: *mut Self, val: u64) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u64 = val;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 2usize, 1u8, val)
        }
    }
    #[inline]
    pub fn enable_experimental_pm4_command_buffers(&self) -> u64 {
        self._bitfield_1.get(3usize, 1u8)
    }
    #[inline]
    pub fn set_enable_experimental_pm4_command_buffers(&mut self, val: u64) {
        {
            let val: u64 = val;
            self._bitfield_1.set(3usize, 1u8, val)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn enable_experimental_pm4_command_buffers_raw(this: *const Self) -> u64 {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 3usize, 1u8)
        }
    }
    #[inline]
    ///# Safety
    ///The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned.
    pub unsafe fn set_enable_experimental_pm4_command_buffers_raw(
        this: *mut Self,
        val: u64,
    ) {
        // SAFETY: The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.
        unsafe {
            let val: u64 = val;
            <__BindgenBitfieldUnit<
                [u8; 1usize],
            >>::raw_set(::core::ptr::addr_of_mut!((*this)._bitfield_1), 3usize, 1u8, val)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        preallocate_pools: u64,
        exclusive_execution: u64,
        force_wait_barrier_defer: u64,
        enable_experimental_pm4_command_buffers: u64,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                1u8,
                {
                    let preallocate_pools: u64 = preallocate_pools;
                    preallocate_pools
                },
            );
        __bindgen_bitfield_unit
            .set(
                1usize,
                1u8,
                {
                    let exclusive_execution: u64 = exclusive_execution;
                    exclusive_execution
                },
            );
        __bindgen_bitfield_unit
            .set(
                2usize,
                1u8,
                {
                    let force_wait_barrier_defer: u64 = force_wait_barrier_defer;
                    force_wait_barrier_defer
                },
            );
        __bindgen_bitfield_unit
            .set(
                3usize,
                1u8,
                {
                    let enable_experimental_pm4_command_buffers: u64 = enable_experimental_pm4_command_buffers;
                    enable_experimental_pm4_command_buffers
                },
            );
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_logical_device_options_initialize(
        out_options: *mut iree_hal_amdgpu_logical_device_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_logical_device_options_parse(
        options: *mut iree_hal_amdgpu_logical_device_options_t,
        params: iree_string_pair_list_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_logical_device_create(
        identifier: iree_string_view_t,
        options: *const iree_hal_amdgpu_logical_device_options_t,
        libhsa: *const iree_hal_amdgpu_libhsa_t,
        topology: *const iree_hal_amdgpu_topology_t,
        create_params: *const iree_hal_device_create_params_t,
        host_allocator: iree_allocator_t,
        out_device: *mut *mut iree_hal_device_t,
    ) -> iree_status_t;
}
#[repr(C)]
pub struct iree_hal_amdgpu_driver_options_t {
    pub libhsa_search_paths: iree_string_view_list_t,
    pub default_device_options: iree_hal_amdgpu_logical_device_options_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_amdgpu_driver_options_t",
    ][::core::mem::size_of::<iree_hal_amdgpu_driver_options_t>() - 136usize];
    [
        "Alignment of iree_hal_amdgpu_driver_options_t",
    ][::core::mem::align_of::<iree_hal_amdgpu_driver_options_t>() - 8usize];
    [
        "Offset of field: iree_hal_amdgpu_driver_options_t::libhsa_search_paths",
    ][::core::mem::offset_of!(iree_hal_amdgpu_driver_options_t, libhsa_search_paths)
        - 0usize];
    [
        "Offset of field: iree_hal_amdgpu_driver_options_t::default_device_options",
    ][::core::mem::offset_of!(iree_hal_amdgpu_driver_options_t, default_device_options)
        - 16usize];
};
impl Default for iree_hal_amdgpu_driver_options_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_driver_options_initialize(
        out_options: *mut iree_hal_amdgpu_driver_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_driver_options_parse(
        options: *mut iree_hal_amdgpu_driver_options_t,
        params: iree_string_pair_list_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_amdgpu_driver_create(
        identifier: iree_string_view_t,
        options: *const iree_hal_amdgpu_driver_options_t,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
