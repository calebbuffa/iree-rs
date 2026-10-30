use super::*;
pub const IREE_HAL_HIP_COMMAND_BUFFER_MODE_GRAPH: iree_hal_hip_command_buffer_mode_e = 0;
pub const IREE_HAL_HIP_COMMAND_BUFFER_MODE_STREAM: iree_hal_hip_command_buffer_mode_e = 1;
pub type iree_hal_hip_command_buffer_mode_e = ::core::ffi::c_uint;
pub use self::iree_hal_hip_command_buffer_mode_e as iree_hal_hip_command_buffer_mode_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_hip_nccl_id_t {
    pub data: [::core::ffi::c_char; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_hip_nccl_id_t",
    ][::core::mem::size_of::<iree_hal_hip_nccl_id_t>() - 128usize];
    [
        "Alignment of iree_hal_hip_nccl_id_t",
    ][::core::mem::align_of::<iree_hal_hip_nccl_id_t>() - 1usize];
    [
        "Offset of field: iree_hal_hip_nccl_id_t::data",
    ][::core::mem::offset_of!(iree_hal_hip_nccl_id_t, data) - 0usize];
};
impl Default for iree_hal_hip_nccl_id_t {
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
pub struct iree_hal_hip_memory_pool_params_t {
    pub minimum_capacity: u64,
    pub release_threshold: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_hip_memory_pool_params_t",
    ][::core::mem::size_of::<iree_hal_hip_memory_pool_params_t>() - 16usize];
    [
        "Alignment of iree_hal_hip_memory_pool_params_t",
    ][::core::mem::align_of::<iree_hal_hip_memory_pool_params_t>() - 8usize];
    [
        "Offset of field: iree_hal_hip_memory_pool_params_t::minimum_capacity",
    ][::core::mem::offset_of!(iree_hal_hip_memory_pool_params_t, minimum_capacity)
        - 0usize];
    [
        "Offset of field: iree_hal_hip_memory_pool_params_t::release_threshold",
    ][::core::mem::offset_of!(iree_hal_hip_memory_pool_params_t, release_threshold)
        - 8usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_hip_memory_pooling_params_t {
    pub device_local: iree_hal_hip_memory_pool_params_t,
    pub other: iree_hal_hip_memory_pool_params_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_hip_memory_pooling_params_t",
    ][::core::mem::size_of::<iree_hal_hip_memory_pooling_params_t>() - 32usize];
    [
        "Alignment of iree_hal_hip_memory_pooling_params_t",
    ][::core::mem::align_of::<iree_hal_hip_memory_pooling_params_t>() - 8usize];
    [
        "Offset of field: iree_hal_hip_memory_pooling_params_t::device_local",
    ][::core::mem::offset_of!(iree_hal_hip_memory_pooling_params_t, device_local)
        - 0usize];
    [
        "Offset of field: iree_hal_hip_memory_pooling_params_t::other",
    ][::core::mem::offset_of!(iree_hal_hip_memory_pooling_params_t, other) - 16usize];
};
#[repr(C)]
pub struct iree_hal_hip_device_params_t {
    pub queue_count: iree_host_size_t,
    pub arena_block_size: iree_host_size_t,
    pub event_pool_capacity: iree_host_size_t,
    pub command_buffer_mode: iree_hal_hip_command_buffer_mode_t,
    pub stream_tracing: i32,
    pub async_allocations: bool,
    pub file_transfer_buffer_size: iree_device_size_t,
    pub file_transfer_chunk_size: iree_device_size_t,
    pub memory_pools: iree_hal_hip_memory_pooling_params_t,
    pub allow_inline_execution: bool,
    pub async_caching: bool,
    pub external_stream: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_hip_device_params_t",
    ][::core::mem::size_of::<iree_hal_hip_device_params_t>() - 104usize];
    [
        "Alignment of iree_hal_hip_device_params_t",
    ][::core::mem::align_of::<iree_hal_hip_device_params_t>() - 8usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::queue_count",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, queue_count) - 0usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::arena_block_size",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, arena_block_size) - 8usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::event_pool_capacity",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, event_pool_capacity)
        - 16usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::command_buffer_mode",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, command_buffer_mode)
        - 24usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::stream_tracing",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, stream_tracing) - 28usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::async_allocations",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, async_allocations)
        - 32usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::file_transfer_buffer_size",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, file_transfer_buffer_size)
        - 40usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::file_transfer_chunk_size",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, file_transfer_chunk_size)
        - 48usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::memory_pools",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, memory_pools) - 56usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::allow_inline_execution",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, allow_inline_execution)
        - 88usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::async_caching",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, async_caching) - 89usize];
    [
        "Offset of field: iree_hal_hip_device_params_t::external_stream",
    ][::core::mem::offset_of!(iree_hal_hip_device_params_t, external_stream) - 96usize];
};
impl Default for iree_hal_hip_device_params_t {
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
    pub fn iree_hal_hip_device_params_initialize(
        out_params: *mut iree_hal_hip_device_params_t,
    );
}
#[repr(C)]
pub struct iree_hal_hip_driver_options_t {
    pub default_device_index: i32,
    pub hip_lib_search_paths: *mut iree_string_view_t,
    pub hip_lib_search_path_count: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_hip_driver_options_t",
    ][::core::mem::size_of::<iree_hal_hip_driver_options_t>() - 24usize];
    [
        "Alignment of iree_hal_hip_driver_options_t",
    ][::core::mem::align_of::<iree_hal_hip_driver_options_t>() - 8usize];
    [
        "Offset of field: iree_hal_hip_driver_options_t::default_device_index",
    ][::core::mem::offset_of!(iree_hal_hip_driver_options_t, default_device_index)
        - 0usize];
    [
        "Offset of field: iree_hal_hip_driver_options_t::hip_lib_search_paths",
    ][::core::mem::offset_of!(iree_hal_hip_driver_options_t, hip_lib_search_paths)
        - 8usize];
    [
        "Offset of field: iree_hal_hip_driver_options_t::hip_lib_search_path_count",
    ][::core::mem::offset_of!(iree_hal_hip_driver_options_t, hip_lib_search_path_count)
        - 16usize];
};
impl Default for iree_hal_hip_driver_options_t {
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
    pub fn iree_hal_hip_driver_options_initialize(
        out_options: *mut iree_hal_hip_driver_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_hip_driver_create(
        identifier: iree_string_view_t,
        options: *const iree_hal_hip_driver_options_t,
        default_params: *const iree_hal_hip_device_params_t,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
