use super::*;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_null_device_options_t {
    pub reserved: ::core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_null_device_options_t",
    ][::core::mem::size_of::<iree_hal_null_device_options_t>() - 4usize];
    [
        "Alignment of iree_hal_null_device_options_t",
    ][::core::mem::align_of::<iree_hal_null_device_options_t>() - 4usize];
    [
        "Offset of field: iree_hal_null_device_options_t::reserved",
    ][::core::mem::offset_of!(iree_hal_null_device_options_t, reserved) - 0usize];
};
unsafe extern "C" {
    pub fn iree_hal_null_device_options_initialize(
        out_options: *mut iree_hal_null_device_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_null_device_create(
        identifier: iree_string_view_t,
        options: *const iree_hal_null_device_options_t,
        create_params: *const iree_hal_device_create_params_t,
        host_allocator: iree_allocator_t,
        out_device: *mut *mut iree_hal_device_t,
    ) -> iree_status_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_null_driver_options_t {
    pub default_device_options: iree_hal_null_device_options_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_null_driver_options_t",
    ][::core::mem::size_of::<iree_hal_null_driver_options_t>() - 4usize];
    [
        "Alignment of iree_hal_null_driver_options_t",
    ][::core::mem::align_of::<iree_hal_null_driver_options_t>() - 4usize];
    [
        "Offset of field: iree_hal_null_driver_options_t::default_device_options",
    ][::core::mem::offset_of!(iree_hal_null_driver_options_t, default_device_options)
        - 0usize];
};
unsafe extern "C" {
    pub fn iree_hal_null_driver_options_initialize(
        out_options: *mut iree_hal_null_driver_options_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_null_driver_create(
        identifier: iree_string_view_t,
        options: *const iree_hal_null_driver_options_t,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
