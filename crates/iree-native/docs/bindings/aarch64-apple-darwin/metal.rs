use super::*;
pub const IREE_HAL_METAL_COMMAND_DISPATCH_TYPE_CONCURRENT: iree_hal_metal_command_dispatch_type_e = 0;
pub const IREE_HAL_METAL_COMMAND_DISPATCH_TYPE_SERIAL: iree_hal_metal_command_dispatch_type_e = 1;
pub type iree_hal_metal_command_dispatch_type_e = ::core::ffi::c_uint;
pub use self::iree_hal_metal_command_dispatch_type_e as iree_hal_metal_command_dispatch_type_t;
pub const IREE_HAL_METAL_COMMAND_BUFFER_RESOURCE_REFERENCE_MODE_UNRETAINED: iree_hal_metal_command_buffer_resource_reference_mode_e = 0;
pub const IREE_HAL_METAL_COMMAND_BUFFER_RESOURCE_REFERENCE_MODE_RETAINED: iree_hal_metal_command_buffer_resource_reference_mode_e = 1;
pub type iree_hal_metal_command_buffer_resource_reference_mode_e = ::core::ffi::c_uint;
pub use self::iree_hal_metal_command_buffer_resource_reference_mode_e as iree_hal_metal_command_buffer_resource_reference_mode_t;
pub const IREE_HAL_METAL_RESOURCE_HAZARD_TRACKING_MODE_UNTRACKED: iree_hal_metal_resource_hazard_tracking_mode_e = 0;
pub const IREE_HAL_METAL_RESOURCE_HAZARD_TRACKING_MODE_TRACKED: iree_hal_metal_resource_hazard_tracking_mode_e = 1;
pub type iree_hal_metal_resource_hazard_tracking_mode_e = ::core::ffi::c_uint;
pub use self::iree_hal_metal_resource_hazard_tracking_mode_e as iree_hal_metal_resource_hazard_tracking_mode_t;
#[repr(C)]
pub struct iree_hal_metal_device_params_t {
    pub arena_block_size: iree_host_size_t,
    pub queue_uniform_buffer_size: iree_host_size_t,
    pub command_dispatch_type: iree_hal_metal_command_dispatch_type_t,
    pub command_buffer_resource_reference_mode: iree_hal_metal_command_buffer_resource_reference_mode_t,
    pub resource_hazard_tracking_mode: iree_hal_metal_resource_hazard_tracking_mode_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_metal_device_params_t",
    ][::core::mem::size_of::<iree_hal_metal_device_params_t>() - 32usize];
    [
        "Alignment of iree_hal_metal_device_params_t",
    ][::core::mem::align_of::<iree_hal_metal_device_params_t>() - 8usize];
    [
        "Offset of field: iree_hal_metal_device_params_t::arena_block_size",
    ][::core::mem::offset_of!(iree_hal_metal_device_params_t, arena_block_size)
        - 0usize];
    [
        "Offset of field: iree_hal_metal_device_params_t::queue_uniform_buffer_size",
    ][::core::mem::offset_of!(iree_hal_metal_device_params_t, queue_uniform_buffer_size)
        - 8usize];
    [
        "Offset of field: iree_hal_metal_device_params_t::command_dispatch_type",
    ][::core::mem::offset_of!(iree_hal_metal_device_params_t, command_dispatch_type)
        - 16usize];
    [
        "Offset of field: iree_hal_metal_device_params_t::command_buffer_resource_reference_mode",
    ][::core::mem::offset_of!(
        iree_hal_metal_device_params_t, command_buffer_resource_reference_mode
    ) - 20usize];
    [
        "Offset of field: iree_hal_metal_device_params_t::resource_hazard_tracking_mode",
    ][::core::mem::offset_of!(
        iree_hal_metal_device_params_t, resource_hazard_tracking_mode
    ) - 24usize];
};
impl Default for iree_hal_metal_device_params_t {
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
    pub fn iree_hal_metal_device_params_initialize(
        out_params: *mut iree_hal_metal_device_params_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_metal_driver_create(
        identifier: iree_string_view_t,
        device_params: *const iree_hal_metal_device_params_t,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
