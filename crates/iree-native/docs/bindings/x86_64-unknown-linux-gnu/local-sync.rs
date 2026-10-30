use super::*;
#[repr(C)]
pub struct iree_hal_sync_device_params_t {
    pub arena_block_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_sync_device_params_t",
    ][::core::mem::size_of::<iree_hal_sync_device_params_t>() - 8usize];
    [
        "Alignment of iree_hal_sync_device_params_t",
    ][::core::mem::align_of::<iree_hal_sync_device_params_t>() - 8usize];
    [
        "Offset of field: iree_hal_sync_device_params_t::arena_block_size",
    ][::core::mem::offset_of!(iree_hal_sync_device_params_t, arena_block_size) - 0usize];
};
impl Default for iree_hal_sync_device_params_t {
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
    pub fn iree_hal_sync_device_params_initialize(
        out_params: *mut iree_hal_sync_device_params_t,
    );
}
unsafe extern "C" {
    pub fn iree_hal_sync_device_create(
        identifier: iree_string_view_t,
        params: *const iree_hal_sync_device_params_t,
        create_params: *const iree_hal_device_create_params_t,
        loader_count: iree_host_size_t,
        loaders: *mut *mut iree_hal_executable_loader_t,
        device_allocator: *mut iree_hal_allocator_t,
        host_allocator: iree_allocator_t,
        out_device: *mut *mut iree_hal_device_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_hal_sync_driver_create(
        identifier: iree_string_view_t,
        default_params: *const iree_hal_sync_device_params_t,
        loader_count: iree_host_size_t,
        loaders: *mut *mut iree_hal_executable_loader_t,
        device_allocator: *mut iree_hal_allocator_t,
        host_allocator: iree_allocator_t,
        out_driver: *mut *mut iree_hal_driver_t,
    ) -> iree_status_t;
}
