use super::*;
pub const IREE_HAL_VULKAN_KHR_SHADER_BFLOAT16_EXTENSION_NAME: &[u8; 23] = b"VK_KHR_shader_bfloat16\0";
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkInstance_T {
    _unused: [u8; 0],
}
pub type VkInstance = *mut VkInstance_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDevice_T {
    _unused: [u8; 0],
}
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDevice_T {
    _unused: [u8; 0],
}
pub type VkDevice = *mut VkDevice_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceMemory_T {
    _unused: [u8; 0],
}
pub type VkDeviceMemory = *mut VkDeviceMemory_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBuffer_T {
    _unused: [u8; 0],
}
pub type VkBuffer = *mut VkBuffer_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphore_T {
    _unused: [u8; 0],
}
pub type VkSemaphore = *mut VkSemaphore_T;
pub const IREE_HAL_VULKAN_REQUEST_FLAG_NONE: iree_hal_vulkan_request_flag_bits_t = 0;
pub const IREE_HAL_VULKAN_REQUEST_FLAG_VALIDATION_LAYERS: iree_hal_vulkan_request_flag_bits_t = 1;
pub const IREE_HAL_VULKAN_REQUEST_FLAG_DEBUG_UTILS: iree_hal_vulkan_request_flag_bits_t = 2;
pub const IREE_HAL_VULKAN_REQUEST_FLAG_TRACING: iree_hal_vulkan_request_flag_bits_t = 4;
pub const IREE_HAL_VULKAN_REQUEST_FLAG_ALL_RECOGNIZED: iree_hal_vulkan_request_flag_bits_t = 7;
pub type iree_hal_vulkan_request_flag_bits_t = ::core::ffi::c_uint;
pub type iree_hal_vulkan_request_flags_t = u32;
pub const IREE_HAL_VULKAN_FEATURE_NONE: iree_hal_vulkan_feature_bits_t = 0;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_ROBUST_BUFFER_ACCESS: iree_hal_vulkan_feature_bits_t = 8;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SPARSE_BINDING: iree_hal_vulkan_feature_bits_t = 16;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SPARSE_RESIDENCY_ALIASED: iree_hal_vulkan_feature_bits_t = 48;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_BUFFER_DEVICE_ADDRESSES: iree_hal_vulkan_feature_bits_t = 64;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_TIMELINE_SEMAPHORES: iree_hal_vulkan_feature_bits_t = 128;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SYNCHRONIZATION2: iree_hal_vulkan_feature_bits_t = 256;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SCALAR_BLOCK_LAYOUT: iree_hal_vulkan_feature_bits_t = 512;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SUBGROUP_SIZE_CONTROL: iree_hal_vulkan_feature_bits_t = 1024;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_COOPERATIVE_MATRIX: iree_hal_vulkan_feature_bits_t = 2048;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_STORAGE_BUFFER_8BIT_ACCESS: iree_hal_vulkan_feature_bits_t = 4096;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_FLOAT16: iree_hal_vulkan_feature_bits_t = 8192;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_FLOAT64: iree_hal_vulkan_feature_bits_t = 16384;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INT8: iree_hal_vulkan_feature_bits_t = 32768;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INT16: iree_hal_vulkan_feature_bits_t = 65536;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INT64: iree_hal_vulkan_feature_bits_t = 131072;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_INTEGER_DOT_PRODUCT: iree_hal_vulkan_feature_bits_t = 262144;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_VULKAN_MEMORY_MODEL: iree_hal_vulkan_feature_bits_t = 524288;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_VULKAN_MEMORY_MODEL_DEVICE_SCOPE: iree_hal_vulkan_feature_bits_t = 1048576;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_STORAGE_BUFFER_16BIT_ACCESS: iree_hal_vulkan_feature_bits_t = 2097152;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_BFLOAT16_TYPE: iree_hal_vulkan_feature_bits_t = 4194304;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_BFLOAT16_DOT_PRODUCT: iree_hal_vulkan_feature_bits_t = 8388608;
pub const IREE_HAL_VULKAN_FEATURE_ENABLE_SHADER_BFLOAT16_COOPERATIVE_MATRIX: iree_hal_vulkan_feature_bits_t = 16777216;
pub const IREE_HAL_VULKAN_FEATURE_REQUIRED_BASELINE: iree_hal_vulkan_feature_bits_t = 896;
pub const IREE_HAL_VULKAN_FEATURE_ALL_RECOGNIZED: iree_hal_vulkan_feature_bits_t = 33554424;
pub type iree_hal_vulkan_feature_bits_t = ::core::ffi::c_uint;
pub type iree_hal_vulkan_features_t = u32;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_NONE: iree_hal_vulkan_device_extension_bits_t = 0;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_PORTABILITY_SUBSET: iree_hal_vulkan_device_extension_bits_t = 1;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_EXTERNAL_MEMORY: iree_hal_vulkan_device_extension_bits_t = 2;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_EXTERNAL_MEMORY_FD: iree_hal_vulkan_device_extension_bits_t = 4;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_EXTERNAL_MEMORY_WIN32: iree_hal_vulkan_device_extension_bits_t = 8;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_EXT_EXTERNAL_MEMORY_HOST: iree_hal_vulkan_device_extension_bits_t = 16;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_EXT_CALIBRATED_TIMESTAMPS: iree_hal_vulkan_device_extension_bits_t = 32;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_PUSH_DESCRIPTOR: iree_hal_vulkan_device_extension_bits_t = 64;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_COOPERATIVE_MATRIX: iree_hal_vulkan_device_extension_bits_t = 128;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_KHR_SHADER_BFLOAT16: iree_hal_vulkan_device_extension_bits_t = 256;
pub const IREE_HAL_VULKAN_DEVICE_EXTENSION_ALL_RECOGNIZED: iree_hal_vulkan_device_extension_bits_t = 511;
pub type iree_hal_vulkan_device_extension_bits_t = ::core::ffi::c_uint;
pub type iree_hal_vulkan_device_extensions_t = u32;
unsafe extern "C" {
    pub fn iree_hal_vulkan_device_extensions_from_names(
        extension_count: iree_host_size_t,
        extension_names: *const *const ::core::ffi::c_char,
        out_extensions: *mut iree_hal_vulkan_device_extensions_t,
    ) -> iree_status_t;
}
pub const IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_LAYERS_REQUIRED: iree_hal_vulkan_extensibility_set_e = 0;
pub const IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_LAYERS_OPTIONAL: iree_hal_vulkan_extensibility_set_e = 1;
pub const IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_EXTENSIONS_REQUIRED: iree_hal_vulkan_extensibility_set_e = 2;
pub const IREE_HAL_VULKAN_EXTENSIBILITY_INSTANCE_EXTENSIONS_OPTIONAL: iree_hal_vulkan_extensibility_set_e = 3;
pub const IREE_HAL_VULKAN_EXTENSIBILITY_DEVICE_EXTENSIONS_REQUIRED: iree_hal_vulkan_extensibility_set_e = 4;
pub const IREE_HAL_VULKAN_EXTENSIBILITY_DEVICE_EXTENSIONS_OPTIONAL: iree_hal_vulkan_extensibility_set_e = 5;
pub const IREE_HAL_VULKAN_EXTENSIBILITY_SET_COUNT: iree_hal_vulkan_extensibility_set_e = 6;
pub type iree_hal_vulkan_extensibility_set_e = ::core::ffi::c_uint;
pub use self::iree_hal_vulkan_extensibility_set_e as iree_hal_vulkan_extensibility_set_t;
unsafe extern "C" {
    pub fn iree_hal_vulkan_query_extensibility_set(
        request_flags: iree_hal_vulkan_request_flags_t,
        set: iree_hal_vulkan_extensibility_set_t,
        string_capacity: iree_host_size_t,
        out_string_count: *mut iree_host_size_t,
        out_string_values: *mut *const ::core::ffi::c_char,
    ) -> iree_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_vulkan_syms_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_syms_create(
        vkGetInstanceProcAddr_fn: *mut ::core::ffi::c_void,
        host_allocator: iree_allocator_t,
        out_syms: *mut *mut iree_hal_vulkan_syms_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_syms_create_from_system_loader(
        host_allocator: iree_allocator_t,
        out_syms: *mut *mut iree_hal_vulkan_syms_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_syms_retain(syms: *mut iree_hal_vulkan_syms_t);
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_syms_release(syms: *mut iree_hal_vulkan_syms_t);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_vulkan_queue_set_t {
    pub queue_family_index: u32,
    pub queue_indices: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_vulkan_queue_set_t",
    ][::core::mem::size_of::<iree_hal_vulkan_queue_set_t>() - 16usize];
    [
        "Alignment of iree_hal_vulkan_queue_set_t",
    ][::core::mem::align_of::<iree_hal_vulkan_queue_set_t>() - 8usize];
    [
        "Offset of field: iree_hal_vulkan_queue_set_t::queue_family_index",
    ][::core::mem::offset_of!(iree_hal_vulkan_queue_set_t, queue_family_index) - 0usize];
    [
        "Offset of field: iree_hal_vulkan_queue_set_t::queue_indices",
    ][::core::mem::offset_of!(iree_hal_vulkan_queue_set_t, queue_indices) - 8usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_vulkan_external_device_params_t {
    pub enabled_features: iree_hal_vulkan_features_t,
    pub enabled_extensions: iree_hal_vulkan_device_extensions_t,
    pub request_flags: iree_hal_vulkan_request_flags_t,
    pub compute_queue_set: iree_hal_vulkan_queue_set_t,
    pub transfer_queue_set: iree_hal_vulkan_queue_set_t,
    pub sparse_binding_queue_set: iree_hal_vulkan_queue_set_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_vulkan_external_device_params_t",
    ][::core::mem::size_of::<iree_hal_vulkan_external_device_params_t>() - 64usize];
    [
        "Alignment of iree_hal_vulkan_external_device_params_t",
    ][::core::mem::align_of::<iree_hal_vulkan_external_device_params_t>() - 8usize];
    [
        "Offset of field: iree_hal_vulkan_external_device_params_t::enabled_features",
    ][::core::mem::offset_of!(iree_hal_vulkan_external_device_params_t, enabled_features)
        - 0usize];
    [
        "Offset of field: iree_hal_vulkan_external_device_params_t::enabled_extensions",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_external_device_params_t, enabled_extensions
    ) - 4usize];
    [
        "Offset of field: iree_hal_vulkan_external_device_params_t::request_flags",
    ][::core::mem::offset_of!(iree_hal_vulkan_external_device_params_t, request_flags)
        - 8usize];
    [
        "Offset of field: iree_hal_vulkan_external_device_params_t::compute_queue_set",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_external_device_params_t, compute_queue_set
    ) - 16usize];
    [
        "Offset of field: iree_hal_vulkan_external_device_params_t::transfer_queue_set",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_external_device_params_t, transfer_queue_set
    ) - 32usize];
    [
        "Offset of field: iree_hal_vulkan_external_device_params_t::sparse_binding_queue_set",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_external_device_params_t, sparse_binding_queue_set
    ) - 48usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_vulkan_cooperative_matrix_property_t {
    pub m_size: u32,
    pub n_size: u32,
    pub k_size: u32,
    pub a_type: u32,
    pub b_type: u32,
    pub c_type: u32,
    pub result_type: u32,
    pub saturating_accumulation: u32,
    pub scope: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_vulkan_cooperative_matrix_property_t",
    ][::core::mem::size_of::<iree_hal_vulkan_cooperative_matrix_property_t>() - 36usize];
    [
        "Alignment of iree_hal_vulkan_cooperative_matrix_property_t",
    ][::core::mem::align_of::<iree_hal_vulkan_cooperative_matrix_property_t>() - 4usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::m_size",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, m_size)
        - 0usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::n_size",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, n_size)
        - 4usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::k_size",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, k_size)
        - 8usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::a_type",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, a_type)
        - 12usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::b_type",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, b_type)
        - 16usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::c_type",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, c_type)
        - 20usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::result_type",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, result_type)
        - 24usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::saturating_accumulation",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_cooperative_matrix_property_t, saturating_accumulation
    ) - 28usize];
    [
        "Offset of field: iree_hal_vulkan_cooperative_matrix_property_t::scope",
    ][::core::mem::offset_of!(iree_hal_vulkan_cooperative_matrix_property_t, scope)
        - 32usize];
};
unsafe extern "C" {
    pub fn iree_hal_vulkan_device_query_cooperative_matrix_properties(
        device: *mut iree_hal_device_t,
        property_capacity: iree_host_size_t,
        out_property_count: *mut iree_host_size_t,
        out_properties: *mut iree_hal_vulkan_cooperative_matrix_property_t,
    ) -> iree_status_t;
}
pub const IREE_HAL_VULKAN_DEVICE_FLAG_NONE: iree_hal_vulkan_device_flag_bits_t = 0;
pub const IREE_HAL_VULKAN_DEVICE_FLAG_DEDICATED_COMPUTE_QUEUE: iree_hal_vulkan_device_flag_bits_t = 1;
pub type iree_hal_vulkan_device_flag_bits_t = ::core::ffi::c_uint;
pub type iree_hal_vulkan_device_flags_t = u32;
pub const IREE_HAL_VULKAN_DISPATCH_ABI_NONE: iree_hal_vulkan_dispatch_abi_bits_t = 0;
pub const IREE_HAL_VULKAN_DISPATCH_ABI_DESCRIPTOR: iree_hal_vulkan_dispatch_abi_bits_t = 1;
pub const IREE_HAL_VULKAN_DISPATCH_ABI_BDA: iree_hal_vulkan_dispatch_abi_bits_t = 2;
pub const IREE_HAL_VULKAN_DISPATCH_ABI_ALL_RECOGNIZED: iree_hal_vulkan_dispatch_abi_bits_t = 3;
pub type iree_hal_vulkan_dispatch_abi_bits_t = ::core::ffi::c_uint;
pub type iree_hal_vulkan_dispatch_abis_t = u32;
unsafe extern "C" {
    pub fn iree_hal_vulkan_dispatch_abis_parse(
        value: iree_string_view_t,
        out_abis: *mut iree_hal_vulkan_dispatch_abis_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_dispatch_abis_verify(
        dispatch_abis: iree_hal_vulkan_dispatch_abis_t,
    ) -> iree_status_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_vulkan_device_options_t {
    pub flags: iree_hal_vulkan_device_flags_t,
    pub dispatch_abis: iree_hal_vulkan_dispatch_abis_t,
    pub max_cached_bda_replay_instances: u32,
    pub max_cached_bda_replay_publication_bytes: u64,
    pub retained_cached_bda_replay_instances: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_vulkan_device_options_t",
    ][::core::mem::size_of::<iree_hal_vulkan_device_options_t>() - 32usize];
    [
        "Alignment of iree_hal_vulkan_device_options_t",
    ][::core::mem::align_of::<iree_hal_vulkan_device_options_t>() - 8usize];
    [
        "Offset of field: iree_hal_vulkan_device_options_t::flags",
    ][::core::mem::offset_of!(iree_hal_vulkan_device_options_t, flags) - 0usize];
    [
        "Offset of field: iree_hal_vulkan_device_options_t::dispatch_abis",
    ][::core::mem::offset_of!(iree_hal_vulkan_device_options_t, dispatch_abis) - 4usize];
    [
        "Offset of field: iree_hal_vulkan_device_options_t::max_cached_bda_replay_instances",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_device_options_t, max_cached_bda_replay_instances
    ) - 8usize];
    [
        "Offset of field: iree_hal_vulkan_device_options_t::max_cached_bda_replay_publication_bytes",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_device_options_t, max_cached_bda_replay_publication_bytes
    ) - 16usize];
    [
        "Offset of field: iree_hal_vulkan_device_options_t::retained_cached_bda_replay_instances",
    ][::core::mem::offset_of!(
        iree_hal_vulkan_device_options_t, retained_cached_bda_replay_instances
    ) - 24usize];
};
unsafe extern "C" {
    pub fn iree_hal_vulkan_device_options_initialize(
        out_options: *mut iree_hal_vulkan_device_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_wrap_device(
        identifier: iree_string_view_t,
        options: *const iree_hal_vulkan_device_options_t,
        create_params: *const iree_hal_device_create_params_t,
        instance_syms: *const iree_hal_vulkan_syms_t,
        instance: VkInstance,
        physical_device: VkPhysicalDevice,
        logical_device: VkDevice,
        external_device_params: *const iree_hal_vulkan_external_device_params_t,
        host_allocator: iree_allocator_t,
        out_device: *mut *mut iree_hal_device_t,
    ) -> iree_status_t;
}
#[repr(C)]
pub struct iree_hal_vulkan_driver_options_t {
    pub libvulkan_search_paths: iree_string_view_list_t,
    pub api_version: u32,
    pub request_flags: iree_hal_vulkan_request_flags_t,
    pub requested_features: iree_hal_vulkan_features_t,
    pub debug_verbosity: i32,
    pub device_options: iree_hal_vulkan_device_options_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_vulkan_driver_options_t",
    ][::core::mem::size_of::<iree_hal_vulkan_driver_options_t>() - 64usize];
    [
        "Alignment of iree_hal_vulkan_driver_options_t",
    ][::core::mem::align_of::<iree_hal_vulkan_driver_options_t>() - 8usize];
    [
        "Offset of field: iree_hal_vulkan_driver_options_t::libvulkan_search_paths",
    ][::core::mem::offset_of!(iree_hal_vulkan_driver_options_t, libvulkan_search_paths)
        - 0usize];
    [
        "Offset of field: iree_hal_vulkan_driver_options_t::api_version",
    ][::core::mem::offset_of!(iree_hal_vulkan_driver_options_t, api_version) - 16usize];
    [
        "Offset of field: iree_hal_vulkan_driver_options_t::request_flags",
    ][::core::mem::offset_of!(iree_hal_vulkan_driver_options_t, request_flags)
        - 20usize];
    [
        "Offset of field: iree_hal_vulkan_driver_options_t::requested_features",
    ][::core::mem::offset_of!(iree_hal_vulkan_driver_options_t, requested_features)
        - 24usize];
    [
        "Offset of field: iree_hal_vulkan_driver_options_t::debug_verbosity",
    ][::core::mem::offset_of!(iree_hal_vulkan_driver_options_t, debug_verbosity)
        - 28usize];
    [
        "Offset of field: iree_hal_vulkan_driver_options_t::device_options",
    ][::core::mem::offset_of!(iree_hal_vulkan_driver_options_t, device_options)
        - 32usize];
};
impl Default for iree_hal_vulkan_driver_options_t {
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
    pub fn iree_hal_vulkan_driver_options_initialize(
        out_options: *mut iree_hal_vulkan_driver_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_driver_create(
        identifier: iree_string_view_t,
        options: *const iree_hal_vulkan_driver_options_t,
        syms: *mut iree_hal_vulkan_syms_t,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_driver_create_using_instance(
        identifier: iree_string_view_t,
        options: *const iree_hal_vulkan_driver_options_t,
        instance_syms: *mut iree_hal_vulkan_syms_t,
        instance: VkInstance,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_allocated_buffer_handle(
        allocated_buffer: *mut iree_hal_buffer_t,
        out_memory: *mut VkDeviceMemory,
        out_handle: *mut VkBuffer,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_vulkan_semaphore_handle(
        semaphore: *mut iree_hal_semaphore_t,
        out_handle: *mut VkSemaphore,
    ) -> iree_status_t;
}
