use super::*;
#[repr(C)]
pub struct iree_hal_sync_device_params_t {
    pub arena_block_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_sync_device_params_t",
    ][::core::mem::size_of::<iree_hal_sync_device_params_t>() - 4usize];
    [
        "Alignment of iree_hal_sync_device_params_t",
    ][::core::mem::align_of::<iree_hal_sync_device_params_t>() - 4usize];
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
