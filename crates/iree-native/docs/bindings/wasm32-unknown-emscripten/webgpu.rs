use super::*;
pub type iree_hal_webgpu_handle_t = u32;
#[repr(C)]
pub struct iree_hal_webgpu_handle_table_t {
    pub entries: *mut *mut ::core::ffi::c_void,
    pub free_stack: *mut u32,
    pub capacity: u32,
    pub count: u32,
    pub high_water: u32,
    pub free_count: u32,
    pub allocator: iree_allocator_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_webgpu_handle_table_t",
    ][::core::mem::size_of::<iree_hal_webgpu_handle_table_t>() - 32usize];
    [
        "Alignment of iree_hal_webgpu_handle_table_t",
    ][::core::mem::align_of::<iree_hal_webgpu_handle_table_t>() - 4usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::entries",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, entries) - 0usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::free_stack",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, free_stack) - 4usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::capacity",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, capacity) - 8usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::count",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, count) - 12usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::high_water",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, high_water) - 16usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::free_count",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, free_count) - 20usize];
    [
        "Offset of field: iree_hal_webgpu_handle_table_t::allocator",
    ][::core::mem::offset_of!(iree_hal_webgpu_handle_table_t, allocator) - 24usize];
};
impl Default for iree_hal_webgpu_handle_table_t {
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
    #[link_name = "iree_hal_webgpu_handle_table_count__extern"]
    pub fn iree_hal_webgpu_handle_table_count(
        table: *const iree_hal_webgpu_handle_table_t,
    ) -> u32;
}
unsafe extern "C" {
    #[link_name = "iree_hal_webgpu_handle_table_is_empty__extern"]
    pub fn iree_hal_webgpu_handle_table_is_empty(
        table: *const iree_hal_webgpu_handle_table_t,
    ) -> bool;
}
pub const IREE_HAL_WEBGPU_DEVICE_FLAG_NONE: iree_hal_webgpu_device_flag_bits_e = 0;
pub const IREE_HAL_WEBGPU_DEVICE_FLAG_OWNS_DEVICE_HANDLE: iree_hal_webgpu_device_flag_bits_e = 1;
pub type iree_hal_webgpu_device_flag_bits_e = ::core::ffi::c_uint;
pub type iree_hal_webgpu_device_flags_t = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_webgpu_driver_options_t {
    pub reserved: ::core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_webgpu_driver_options_t",
    ][::core::mem::size_of::<iree_hal_webgpu_driver_options_t>() - 4usize];
    [
        "Alignment of iree_hal_webgpu_driver_options_t",
    ][::core::mem::align_of::<iree_hal_webgpu_driver_options_t>() - 4usize];
    [
        "Offset of field: iree_hal_webgpu_driver_options_t::reserved",
    ][::core::mem::offset_of!(iree_hal_webgpu_driver_options_t, reserved) - 0usize];
};
