use super::*;
pub const IREE_TASK_TOPOLOGY_MAX_GROUP_COUNT: u32 = 256;
pub const IREE_TASK_EXECUTOR_MAX_WORKER_COUNT: u32 = 256;
pub const IREE_TASK_AFFINITY_SET_WORD_COUNT: u32 = 4;
pub const IREE_TASK_EXECUTOR_MAX_COMPUTE_SLOTS: u32 = 16;
pub const IREE_TASK_WAKE_FANOUT: u32 = 1;
pub const IREE_TASK_WARM_WAIT_SPIN_NS: u32 = 10000;
pub const IREE_TASK_WARM_WAIT_LOOKAHEAD_SPIN_NS: u32 = 10000;
pub const IREE_TASK_WARM_WAIT_TAIL_SPIN_DRAINER_LIMIT: u32 = 8;
pub const IREE_TASK_WARM_WAIT_TRANSITION_SPIN_NS: u32 = 3000;
pub const IREE_TASK_WARM_WAIT_SLEEP_NS: u32 = 1000000;
pub const IREE_TASK_TOPOLOGY_MAX_NODES: u32 = 64;
pub type iree_atomic_slist_intrusive_ptr_t = *mut ::core::ffi::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_atomic_slist_entry_t {
    pub next: *mut iree_atomic_slist_entry_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_atomic_slist_entry_t",
    ][::core::mem::size_of::<iree_atomic_slist_entry_t>() - 8usize];
    [
        "Alignment of iree_atomic_slist_entry_t",
    ][::core::mem::align_of::<iree_atomic_slist_entry_t>() - 8usize];
    [
        "Offset of field: iree_atomic_slist_entry_t::next",
    ][::core::mem::offset_of!(iree_atomic_slist_entry_t, next) - 0usize];
};
impl Default for iree_atomic_slist_entry_t {
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
pub struct iree_atomic_slist_t {
    pub mutex: iree_slim_mutex_t,
    pub head: iree_atomic_intptr_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_atomic_slist_t",
    ][::core::mem::size_of::<iree_atomic_slist_t>() - 16usize];
    [
        "Alignment of iree_atomic_slist_t",
    ][::core::mem::align_of::<iree_atomic_slist_t>() - 8usize];
    [
        "Offset of field: iree_atomic_slist_t::mutex",
    ][::core::mem::offset_of!(iree_atomic_slist_t, mutex) - 0usize];
    [
        "Offset of field: iree_atomic_slist_t::head",
    ][::core::mem::offset_of!(iree_atomic_slist_t, head) - 8usize];
};
impl Default for iree_atomic_slist_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const IREE_ATOMIC_SLIST_FLUSH_ORDER_APPROXIMATE_LIFO: iree_atomic_slist_flush_order_e = 0;
pub const IREE_ATOMIC_SLIST_FLUSH_ORDER_APPROXIMATE_FIFO: iree_atomic_slist_flush_order_e = 1;
pub type iree_atomic_slist_flush_order_e = ::core::ffi::c_uint;
pub use self::iree_atomic_slist_flush_order_e as iree_atomic_slist_flush_order_t;
pub type iree_cpu_processor_id_t = u32;
pub type iree_cpu_processor_tag_t = u32;
unsafe extern "C" {
    #[link_name = "iree_wait_address_wait_int32__extern"]
    pub fn iree_wait_address_wait_int32(
        address: *mut iree_atomic_int32_t,
        expected_value: i32,
        deadline_ns: iree_time_t,
    ) -> iree_status_code_t;
}
unsafe extern "C" {
    #[link_name = "iree_wait_address_wake_all__extern"]
    pub fn iree_wait_address_wake_all(address: *mut iree_atomic_int32_t);
}
pub type iree_task_process_flags_t = u32;
pub const IREE_TASK_PROCESS_FLAG_NONE: iree_task_process_flag_bits_e = 0;
pub const IREE_TASK_PROCESS_FLAG_COMPUTE_SLOT: iree_task_process_flag_bits_e = 1;
pub type iree_task_process_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_task_process_flag_bits_e as iree_task_process_flag_bits_t;
#[repr(C)]
pub struct iree_task_worker_context_t {
    pub worker_index: u32,
    pub processor_id: iree_cpu_processor_id_t,
    pub local_memory: iree_byte_span_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_worker_context_t",
    ][::core::mem::size_of::<iree_task_worker_context_t>() - 24usize];
    [
        "Alignment of iree_task_worker_context_t",
    ][::core::mem::align_of::<iree_task_worker_context_t>() - 8usize];
    [
        "Offset of field: iree_task_worker_context_t::worker_index",
    ][::core::mem::offset_of!(iree_task_worker_context_t, worker_index) - 0usize];
    [
        "Offset of field: iree_task_worker_context_t::processor_id",
    ][::core::mem::offset_of!(iree_task_worker_context_t, processor_id) - 4usize];
    [
        "Offset of field: iree_task_worker_context_t::local_memory",
    ][::core::mem::offset_of!(iree_task_worker_context_t, local_memory) - 8usize];
};
impl Default for iree_task_worker_context_t {
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
pub struct iree_task_process_drain_result_t {
    pub completed: bool,
    pub did_work: bool,
    pub keep_active: bool,
    pub keep_warm: bool,
    pub keep_warm_retainer_limit: i32,
    pub keep_warm_spin: bool,
    pub publish_keep_active: bool,
    pub keep_warm_epoch: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_process_drain_result_t",
    ][::core::mem::size_of::<iree_task_process_drain_result_t>() - 16usize];
    [
        "Alignment of iree_task_process_drain_result_t",
    ][::core::mem::align_of::<iree_task_process_drain_result_t>() - 4usize];
    [
        "Offset of field: iree_task_process_drain_result_t::completed",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, completed) - 0usize];
    [
        "Offset of field: iree_task_process_drain_result_t::did_work",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, did_work) - 1usize];
    [
        "Offset of field: iree_task_process_drain_result_t::keep_active",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, keep_active) - 2usize];
    [
        "Offset of field: iree_task_process_drain_result_t::keep_warm",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, keep_warm) - 3usize];
    [
        "Offset of field: iree_task_process_drain_result_t::keep_warm_retainer_limit",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, keep_warm_retainer_limit)
        - 4usize];
    [
        "Offset of field: iree_task_process_drain_result_t::keep_warm_spin",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, keep_warm_spin)
        - 8usize];
    [
        "Offset of field: iree_task_process_drain_result_t::publish_keep_active",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, publish_keep_active)
        - 9usize];
    [
        "Offset of field: iree_task_process_drain_result_t::keep_warm_epoch",
    ][::core::mem::offset_of!(iree_task_process_drain_result_t, keep_warm_epoch)
        - 12usize];
};
pub type iree_task_process_drain_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        process: *mut iree_task_process_t,
        worker_context: *const iree_task_worker_context_t,
        out_result: *mut iree_task_process_drain_result_t,
    ) -> iree_status_t,
>;
pub type iree_task_process_completion_fn_t = ::core::option::Option<
    unsafe extern "C" fn(process: *mut iree_task_process_t, status: iree_status_t),
>;
pub type iree_task_process_release_fn_t = ::core::option::Option<
    unsafe extern "C" fn(process: *mut iree_task_process_t),
>;
pub const IREE_TASK_PROCESS_STATE_SUSPENDED: iree_task_process_state_e = 0;
pub const IREE_TASK_PROCESS_STATE_RUNNABLE: iree_task_process_state_e = 1;
pub const IREE_TASK_PROCESS_STATE_COMPLETED: iree_task_process_state_e = 2;
pub const IREE_TASK_PROCESS_STATE_CANCELLED: iree_task_process_state_e = 3;
pub type iree_task_process_state_e = ::core::ffi::c_uint;
pub use self::iree_task_process_state_e as iree_task_process_state_t;
pub const IREE_TASK_PROCESS_SCHEDULE_IDLE: iree_task_process_schedule_state_e = 0;
pub const IREE_TASK_PROCESS_SCHEDULE_QUEUED: iree_task_process_schedule_state_e = 1;
pub const IREE_TASK_PROCESS_SCHEDULE_DRAINING: iree_task_process_schedule_state_e = 2;
pub type iree_task_process_schedule_state_e = ::core::ffi::c_uint;
pub use self::iree_task_process_schedule_state_e as iree_task_process_schedule_state_t;
#[repr(C)]
#[repr(align(64))]
pub struct iree_task_process_t {
    pub drain: iree_task_process_drain_fn_t,
    pub completion_fn: iree_task_process_completion_fn_t,
    pub release_fn: iree_task_process_release_fn_t,
    pub user_data: *mut ::core::ffi::c_void,
    pub dependents: *mut *mut iree_task_process_t,
    pub dependent_count: u16,
    pub reserved0: [u8; 22usize],
    pub suspend_count: iree_atomic_int32_t,
    pub state: iree_atomic_int32_t,
    pub error_status: iree_atomic_intptr_t,
    pub __bindgen_padding_0: [u32; 12usize],
    pub wake_budget: iree_atomic_int32_t,
    pub flags: iree_task_process_flags_t,
    pub placement_epoch: iree_atomic_int32_t,
    pub schedule_state: iree_atomic_int32_t,
    pub needs_drain: iree_atomic_int32_t,
    pub retain_drain: iree_atomic_int32_t,
    pub retention_epoch: iree_atomic_int32_t,
    pub warm_retainers: iree_atomic_int32_t,
    pub retention_sleepers: iree_atomic_int32_t,
    pub retention_spin_deadline_ns: iree_atomic_int64_t,
    pub __bindgen_padding_1: [u64; 2usize],
    pub slist_next: iree_atomic_slist_intrusive_ptr_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_process_t",
    ][::core::mem::size_of::<iree_task_process_t>() - 256usize];
    [
        "Alignment of iree_task_process_t",
    ][::core::mem::align_of::<iree_task_process_t>() - 64usize];
    [
        "Offset of field: iree_task_process_t::drain",
    ][::core::mem::offset_of!(iree_task_process_t, drain) - 0usize];
    [
        "Offset of field: iree_task_process_t::completion_fn",
    ][::core::mem::offset_of!(iree_task_process_t, completion_fn) - 8usize];
    [
        "Offset of field: iree_task_process_t::release_fn",
    ][::core::mem::offset_of!(iree_task_process_t, release_fn) - 16usize];
    [
        "Offset of field: iree_task_process_t::user_data",
    ][::core::mem::offset_of!(iree_task_process_t, user_data) - 24usize];
    [
        "Offset of field: iree_task_process_t::dependents",
    ][::core::mem::offset_of!(iree_task_process_t, dependents) - 32usize];
    [
        "Offset of field: iree_task_process_t::dependent_count",
    ][::core::mem::offset_of!(iree_task_process_t, dependent_count) - 40usize];
    [
        "Offset of field: iree_task_process_t::reserved0",
    ][::core::mem::offset_of!(iree_task_process_t, reserved0) - 42usize];
    [
        "Offset of field: iree_task_process_t::suspend_count",
    ][::core::mem::offset_of!(iree_task_process_t, suspend_count) - 64usize];
    [
        "Offset of field: iree_task_process_t::state",
    ][::core::mem::offset_of!(iree_task_process_t, state) - 68usize];
    [
        "Offset of field: iree_task_process_t::error_status",
    ][::core::mem::offset_of!(iree_task_process_t, error_status) - 72usize];
    [
        "Offset of field: iree_task_process_t::wake_budget",
    ][::core::mem::offset_of!(iree_task_process_t, wake_budget) - 128usize];
    [
        "Offset of field: iree_task_process_t::flags",
    ][::core::mem::offset_of!(iree_task_process_t, flags) - 132usize];
    [
        "Offset of field: iree_task_process_t::placement_epoch",
    ][::core::mem::offset_of!(iree_task_process_t, placement_epoch) - 136usize];
    [
        "Offset of field: iree_task_process_t::schedule_state",
    ][::core::mem::offset_of!(iree_task_process_t, schedule_state) - 140usize];
    [
        "Offset of field: iree_task_process_t::needs_drain",
    ][::core::mem::offset_of!(iree_task_process_t, needs_drain) - 144usize];
    [
        "Offset of field: iree_task_process_t::retain_drain",
    ][::core::mem::offset_of!(iree_task_process_t, retain_drain) - 148usize];
    [
        "Offset of field: iree_task_process_t::retention_epoch",
    ][::core::mem::offset_of!(iree_task_process_t, retention_epoch) - 152usize];
    [
        "Offset of field: iree_task_process_t::warm_retainers",
    ][::core::mem::offset_of!(iree_task_process_t, warm_retainers) - 156usize];
    [
        "Offset of field: iree_task_process_t::retention_sleepers",
    ][::core::mem::offset_of!(iree_task_process_t, retention_sleepers) - 160usize];
    [
        "Offset of field: iree_task_process_t::retention_spin_deadline_ns",
    ][::core::mem::offset_of!(iree_task_process_t, retention_spin_deadline_ns)
        - 168usize];
    [
        "Offset of field: iree_task_process_t::slist_next",
    ][::core::mem::offset_of!(iree_task_process_t, slist_next) - 192usize];
};
impl Default for iree_task_process_t {
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
    #[link_name = "iree_task_process_slist_entry_from_ptr__extern"]
    pub fn iree_task_process_slist_entry_from_ptr(
        entry: *mut iree_task_process_t,
    ) -> *mut iree_atomic_slist_entry_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_entry_to_ptr__extern"]
    pub fn iree_task_process_slist_entry_to_ptr(
        entry: *mut iree_atomic_slist_entry_t,
    ) -> *mut iree_task_process_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_get_next__extern"]
    pub fn iree_task_process_slist_get_next(
        entry: *mut iree_task_process_t,
    ) -> *mut iree_task_process_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_set_next__extern"]
    pub fn iree_task_process_slist_set_next(
        entry: *mut iree_task_process_t,
        next: *mut iree_task_process_t,
    );
}
#[repr(C)]
#[repr(align(16))]
pub struct iree_task_process_slist_t {
    pub impl_: iree_atomic_slist_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_process_slist_t",
    ][::core::mem::size_of::<iree_task_process_slist_t>() - 16usize];
    [
        "Alignment of iree_task_process_slist_t",
    ][::core::mem::align_of::<iree_task_process_slist_t>() - 16usize];
    [
        "Offset of field: iree_task_process_slist_t::impl_",
    ][::core::mem::offset_of!(iree_task_process_slist_t, impl_) - 0usize];
};
impl Default for iree_task_process_slist_t {
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
    #[link_name = "iree_task_process_slist_initialize__extern"]
    pub fn iree_task_process_slist_initialize(out_list: *mut iree_task_process_slist_t);
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_deinitialize__extern"]
    pub fn iree_task_process_slist_deinitialize(list: *mut iree_task_process_slist_t);
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_push__extern"]
    pub fn iree_task_process_slist_push(
        list: *mut iree_task_process_slist_t,
        entry: *mut iree_task_process_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_push_unsafe__extern"]
    pub fn iree_task_process_slist_push_unsafe(
        list: *mut iree_task_process_slist_t,
        entry: *mut iree_task_process_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_concat__extern"]
    pub fn iree_task_process_slist_concat(
        list: *mut iree_task_process_slist_t,
        head: *mut iree_task_process_t,
        tail: *mut iree_task_process_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_discard__extern"]
    pub fn iree_task_process_slist_discard(list: *mut iree_task_process_slist_t);
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_pop__extern"]
    pub fn iree_task_process_slist_pop(
        list: *mut iree_task_process_slist_t,
    ) -> *mut iree_task_process_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_slist_flush__extern"]
    pub fn iree_task_process_slist_flush(
        list: *mut iree_task_process_slist_t,
        flush_order: iree_atomic_slist_flush_order_t,
        out_head: *mut *mut iree_task_process_t,
        out_tail: *mut *mut iree_task_process_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_task_process_initialize(
        drain_fn: iree_task_process_drain_fn_t,
        suspend_count: i32,
        wake_budget: i32,
        out_process: *mut iree_task_process_t,
    );
}
unsafe extern "C" {
    pub fn iree_task_process_set_flags(
        process: *mut iree_task_process_t,
        flags: iree_task_process_flags_t,
    );
}
unsafe extern "C" {
    pub fn iree_task_process_wake(process: *mut iree_task_process_t) -> bool;
}
unsafe extern "C" {
    pub fn iree_task_process_complete(
        process: *mut iree_task_process_t,
        out_activated_head: *mut *mut iree_task_process_t,
        out_activated_tail: *mut *mut iree_task_process_t,
    );
}
unsafe extern "C" {
    pub fn iree_task_process_report_error(
        process: *mut iree_task_process_t,
        status: iree_status_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_process_has_error__extern"]
    pub fn iree_task_process_has_error(process: *const iree_task_process_t) -> bool;
}
unsafe extern "C" {
    pub fn iree_task_process_cancel(
        process: *mut iree_task_process_t,
        out_activated_head: *mut *mut iree_task_process_t,
        out_activated_tail: *mut *mut iree_task_process_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_process_state__extern"]
    pub fn iree_task_process_state(
        process: *const iree_task_process_t,
    ) -> iree_task_process_state_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_is_terminal__extern"]
    pub fn iree_task_process_is_terminal(process: *const iree_task_process_t) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_wake_budget__extern"]
    pub fn iree_task_process_wake_budget(process: *const iree_task_process_t) -> i32;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_uses_compute_slots__extern"]
    pub fn iree_task_process_uses_compute_slots(
        process: *const iree_task_process_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_placement_epoch__extern"]
    pub fn iree_task_process_placement_epoch(process: *const iree_task_process_t) -> i32;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_advance_placement_epoch__extern"]
    pub fn iree_task_process_advance_placement_epoch(
        process: *mut iree_task_process_t,
    ) -> i32;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_retention_epoch__extern"]
    pub fn iree_task_process_retention_epoch(process: *const iree_task_process_t) -> i32;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_warm_retainer_count__extern"]
    pub fn iree_task_process_warm_retainer_count(
        process: *const iree_task_process_t,
    ) -> i32;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_try_retain_warm__extern"]
    pub fn iree_task_process_try_retain_warm(
        process: *mut iree_task_process_t,
        retainer_limit: i32,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_task_process_advance_retention_epoch__extern"]
    pub fn iree_task_process_advance_retention_epoch(
        process: *mut iree_task_process_t,
    ) -> i32;
}
pub const IREE_TASK_SCOPE_FLAG_NONE: iree_task_scope_flag_bits_e = 0;
pub const IREE_TASK_SCOPE_FLAG_ABORT_ON_FAILURE: iree_task_scope_flag_bits_e = 1;
pub type iree_task_scope_flag_bits_e = ::core::ffi::c_uint;
pub type iree_task_scope_flags_t = u32;
#[repr(C)]
pub struct iree_task_scope_t {
    pub name: [::core::ffi::c_char; 16usize],
    pub flags: iree_task_scope_flags_t,
    pub permanent_status: iree_atomic_intptr_t,
    pub pending_submissions: iree_atomic_ref_count_t,
    pub idle_notification: iree_notification_t,
    pub pending_idle_notification_posts: iree_atomic_int32_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of iree_task_scope_t"][::core::mem::size_of::<iree_task_scope_t>() - 56usize];
    [
        "Alignment of iree_task_scope_t",
    ][::core::mem::align_of::<iree_task_scope_t>() - 8usize];
    [
        "Offset of field: iree_task_scope_t::name",
    ][::core::mem::offset_of!(iree_task_scope_t, name) - 0usize];
    [
        "Offset of field: iree_task_scope_t::flags",
    ][::core::mem::offset_of!(iree_task_scope_t, flags) - 16usize];
    [
        "Offset of field: iree_task_scope_t::permanent_status",
    ][::core::mem::offset_of!(iree_task_scope_t, permanent_status) - 24usize];
    [
        "Offset of field: iree_task_scope_t::pending_submissions",
    ][::core::mem::offset_of!(iree_task_scope_t, pending_submissions) - 32usize];
    [
        "Offset of field: iree_task_scope_t::idle_notification",
    ][::core::mem::offset_of!(iree_task_scope_t, idle_notification) - 40usize];
    [
        "Offset of field: iree_task_scope_t::pending_idle_notification_posts",
    ][::core::mem::offset_of!(iree_task_scope_t, pending_idle_notification_posts)
        - 48usize];
};
impl Default for iree_task_scope_t {
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
    pub fn iree_task_scope_initialize(
        name: iree_string_view_t,
        flags: iree_task_scope_flags_t,
        out_scope: *mut iree_task_scope_t,
    );
}
unsafe extern "C" {
    pub fn iree_task_scope_deinitialize(scope: *mut iree_task_scope_t);
}
unsafe extern "C" {
    pub fn iree_task_scope_name(scope: *mut iree_task_scope_t) -> iree_string_view_t;
}
unsafe extern "C" {
    pub fn iree_task_scope_has_failed(scope: *mut iree_task_scope_t) -> bool;
}
unsafe extern "C" {
    pub fn iree_task_scope_consume_status(
        scope: *mut iree_task_scope_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_scope_abort(scope: *mut iree_task_scope_t);
}
unsafe extern "C" {
    pub fn iree_task_scope_fail(scope: *mut iree_task_scope_t, status: iree_status_t);
}
unsafe extern "C" {
    pub fn iree_task_scope_begin(scope: *mut iree_task_scope_t);
}
unsafe extern "C" {
    pub fn iree_task_scope_end(scope: *mut iree_task_scope_t);
}
unsafe extern "C" {
    pub fn iree_task_scope_is_idle(scope: *mut iree_task_scope_t) -> bool;
}
unsafe extern "C" {
    pub fn iree_task_scope_wait_idle(
        scope: *mut iree_task_scope_t,
        deadline_ns: iree_time_t,
    ) -> iree_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_thread_t {
    _unused: [u8; 0],
}
pub const IREE_THREAD_PRIORITY_CLASS_LOWEST: iree_thread_priority_class_e = -2;
pub const IREE_THREAD_PRIORITY_CLASS_LOW: iree_thread_priority_class_e = -1;
pub const IREE_THREAD_PRIORITY_CLASS_NORMAL: iree_thread_priority_class_e = 0;
pub const IREE_THREAD_PRIORITY_CLASS_HIGH: iree_thread_priority_class_e = 1;
pub const IREE_THREAD_PRIORITY_CLASS_HIGHEST: iree_thread_priority_class_e = 2;
pub type iree_thread_priority_class_e = ::core::ffi::c_int;
pub use self::iree_thread_priority_class_e as iree_thread_priority_class_t;
#[repr(C)]
pub struct iree_thread_create_params_t {
    pub name: iree_string_view_t,
    pub stack_size: usize,
    pub create_suspended: bool,
    pub priority_class: iree_thread_priority_class_t,
    pub initial_affinity: iree_thread_affinity_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_thread_create_params_t",
    ][::core::mem::size_of::<iree_thread_create_params_t>() - 40usize];
    [
        "Alignment of iree_thread_create_params_t",
    ][::core::mem::align_of::<iree_thread_create_params_t>() - 8usize];
    [
        "Offset of field: iree_thread_create_params_t::name",
    ][::core::mem::offset_of!(iree_thread_create_params_t, name) - 0usize];
    [
        "Offset of field: iree_thread_create_params_t::stack_size",
    ][::core::mem::offset_of!(iree_thread_create_params_t, stack_size) - 16usize];
    [
        "Offset of field: iree_thread_create_params_t::create_suspended",
    ][::core::mem::offset_of!(iree_thread_create_params_t, create_suspended) - 24usize];
    [
        "Offset of field: iree_thread_create_params_t::priority_class",
    ][::core::mem::offset_of!(iree_thread_create_params_t, priority_class) - 28usize];
    [
        "Offset of field: iree_thread_create_params_t::initial_affinity",
    ][::core::mem::offset_of!(iree_thread_create_params_t, initial_affinity) - 32usize];
};
impl Default for iree_thread_create_params_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_thread_entry_t = ::core::option::Option<
    unsafe extern "C" fn(entry_arg: *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
>;
unsafe extern "C" {
    pub fn iree_thread_create(
        entry: iree_thread_entry_t,
        entry_arg: *mut ::core::ffi::c_void,
        params: iree_thread_create_params_t,
        allocator: iree_allocator_t,
        out_thread: *mut *mut iree_thread_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_thread_retain(thread: *mut iree_thread_t);
}
unsafe extern "C" {
    pub fn iree_thread_release(thread: *mut iree_thread_t);
}
unsafe extern "C" {
    pub fn iree_thread_id(thread: *mut iree_thread_t) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_thread_override_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn iree_thread_priority_class_override_begin(
        thread: *mut iree_thread_t,
        priority_class: iree_thread_priority_class_t,
    ) -> *mut iree_thread_override_t;
}
unsafe extern "C" {
    pub fn iree_thread_override_end(override_token: *mut iree_thread_override_t);
}
unsafe extern "C" {
    pub fn iree_thread_request_affinity(
        thread: *mut iree_thread_t,
        affinity: iree_thread_affinity_t,
    );
}
unsafe extern "C" {
    pub fn iree_thread_resume(thread: *mut iree_thread_t);
}
unsafe extern "C" {
    pub fn iree_thread_join(thread: *mut iree_thread_t);
}
unsafe extern "C" {
    pub fn iree_thread_yield();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_task_affinity_bit_t {
    pub word_index: u8,
    pub bit_mask: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_affinity_bit_t",
    ][::core::mem::size_of::<iree_task_affinity_bit_t>() - 16usize];
    [
        "Alignment of iree_task_affinity_bit_t",
    ][::core::mem::align_of::<iree_task_affinity_bit_t>() - 8usize];
    [
        "Offset of field: iree_task_affinity_bit_t::word_index",
    ][::core::mem::offset_of!(iree_task_affinity_bit_t, word_index) - 0usize];
    [
        "Offset of field: iree_task_affinity_bit_t::bit_mask",
    ][::core::mem::offset_of!(iree_task_affinity_bit_t, bit_mask) - 8usize];
};
unsafe extern "C" {
    #[link_name = "iree_task_affinity_bit_for_worker__extern"]
    pub fn iree_task_affinity_bit_for_worker(
        worker_index: iree_host_size_t,
    ) -> iree_task_affinity_bit_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_task_affinity_set_t {
    pub words: [u64; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_affinity_set_t",
    ][::core::mem::size_of::<iree_task_affinity_set_t>() - 32usize];
    [
        "Alignment of iree_task_affinity_set_t",
    ][::core::mem::align_of::<iree_task_affinity_set_t>() - 8usize];
    [
        "Offset of field: iree_task_affinity_set_t::words",
    ][::core::mem::offset_of!(iree_task_affinity_set_t, words) - 0usize];
};
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_empty__extern"]
    pub fn iree_task_affinity_set_empty() -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_for_worker__extern"]
    pub fn iree_task_affinity_for_worker(
        worker_index: iree_host_size_t,
    ) -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_ones__extern"]
    pub fn iree_task_affinity_set_ones(
        count: iree_host_size_t,
    ) -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_for_any_worker__extern"]
    pub fn iree_task_affinity_for_any_worker() -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_equal__extern"]
    pub fn iree_task_affinity_set_equal(
        a: iree_task_affinity_set_t,
        b: iree_task_affinity_set_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_is_empty__extern"]
    pub fn iree_task_affinity_set_is_empty(set: iree_task_affinity_set_t) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_count_ones__extern"]
    pub fn iree_task_affinity_set_count_ones(
        set: iree_task_affinity_set_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_find_first__extern"]
    pub fn iree_task_affinity_set_find_first(
        set: iree_task_affinity_set_t,
    ) -> ::core::ffi::c_int;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_test__extern"]
    pub fn iree_task_affinity_set_test(
        set: iree_task_affinity_set_t,
        bit: iree_task_affinity_bit_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_set__extern"]
    pub fn iree_task_affinity_set_set(
        set: *mut iree_task_affinity_set_t,
        bit: iree_task_affinity_bit_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_clear__extern"]
    pub fn iree_task_affinity_set_clear(
        set: *mut iree_task_affinity_set_t,
        bit: iree_task_affinity_bit_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_set_index__extern"]
    pub fn iree_task_affinity_set_set_index(
        set: *mut iree_task_affinity_set_t,
        index: iree_host_size_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_clear_index__extern"]
    pub fn iree_task_affinity_set_clear_index(
        set: *mut iree_task_affinity_set_t,
        index: iree_host_size_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_and__extern"]
    pub fn iree_task_affinity_set_and(
        a: iree_task_affinity_set_t,
        b: iree_task_affinity_set_t,
    ) -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_or__extern"]
    pub fn iree_task_affinity_set_or(
        a: iree_task_affinity_set_t,
        b: iree_task_affinity_set_t,
    ) -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_task_affinity_set_not__extern"]
    pub fn iree_task_affinity_set_not(
        set: iree_task_affinity_set_t,
    ) -> iree_task_affinity_set_t;
}
#[repr(C)]
pub struct iree_atomic_task_affinity_set_t {
    pub words: [iree_atomic_int64_t; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_atomic_task_affinity_set_t",
    ][::core::mem::size_of::<iree_atomic_task_affinity_set_t>() - 32usize];
    [
        "Alignment of iree_atomic_task_affinity_set_t",
    ][::core::mem::align_of::<iree_atomic_task_affinity_set_t>() - 8usize];
    [
        "Offset of field: iree_atomic_task_affinity_set_t::words",
    ][::core::mem::offset_of!(iree_atomic_task_affinity_set_t, words) - 0usize];
};
impl Default for iree_atomic_task_affinity_set_t {
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
    #[link_name = "iree_atomic_task_affinity_set_set__extern"]
    pub fn iree_atomic_task_affinity_set_set(
        set: *mut iree_atomic_task_affinity_set_t,
        bit: iree_task_affinity_bit_t,
        order: iree_memory_order_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_atomic_task_affinity_set_clear__extern"]
    pub fn iree_atomic_task_affinity_set_clear(
        set: *mut iree_atomic_task_affinity_set_t,
        bit: iree_task_affinity_bit_t,
        order: iree_memory_order_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_atomic_task_affinity_set_load__extern"]
    pub fn iree_atomic_task_affinity_set_load(
        set: *const iree_atomic_task_affinity_set_t,
        order: iree_memory_order_t,
    ) -> iree_task_affinity_set_t;
}
unsafe extern "C" {
    #[link_name = "iree_atomic_task_affinity_set_store__extern"]
    pub fn iree_atomic_task_affinity_set_store(
        set: *mut iree_atomic_task_affinity_set_t,
        value: iree_task_affinity_set_t,
        order: iree_memory_order_t,
    );
}
unsafe extern "C" {
    pub fn iree_task_topology_format_processor_debug_ids(
        processor: u32,
        buffer_capacity: iree_host_size_t,
        buffer: *mut ::core::ffi::c_char,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_set_snapshot_path(
        path: *const ::core::ffi::c_char,
    ) -> iree_status_t;
}
pub type iree_task_topology_node_id_t = u32;
unsafe extern "C" {
    pub fn iree_task_topology_query_node_ids(
        capacity: iree_host_size_t,
        out_ids: *mut iree_task_topology_node_id_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_query_node_count() -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_dense_node_ids(
        count: iree_host_size_t,
        capacity: iree_host_size_t,
        out_ids: *mut iree_task_topology_node_id_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_query_current_node() -> iree_task_topology_node_id_t;
}
pub type iree_task_topology_group_mask_t = iree_task_affinity_set_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_task_topology_caches_t {
    pub l1_data: u32,
    pub l2_data: u32,
    pub l3_data: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_topology_caches_t",
    ][::core::mem::size_of::<iree_task_topology_caches_t>() - 12usize];
    [
        "Alignment of iree_task_topology_caches_t",
    ][::core::mem::align_of::<iree_task_topology_caches_t>() - 4usize];
    [
        "Offset of field: iree_task_topology_caches_t::l1_data",
    ][::core::mem::offset_of!(iree_task_topology_caches_t, l1_data) - 0usize];
    [
        "Offset of field: iree_task_topology_caches_t::l2_data",
    ][::core::mem::offset_of!(iree_task_topology_caches_t, l2_data) - 4usize];
    [
        "Offset of field: iree_task_topology_caches_t::l3_data",
    ][::core::mem::offset_of!(iree_task_topology_caches_t, l3_data) - 8usize];
};
#[repr(C)]
pub struct iree_task_topology_group_t {
    pub group_index: u8,
    pub name: [::core::ffi::c_char; 31usize],
    pub processor_index: u32,
    pub caches: iree_task_topology_caches_t,
    pub ideal_thread_affinity: iree_thread_affinity_t,
    pub constructive_sharing_mask: iree_task_topology_group_mask_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_topology_group_t",
    ][::core::mem::size_of::<iree_task_topology_group_t>() - 88usize];
    [
        "Alignment of iree_task_topology_group_t",
    ][::core::mem::align_of::<iree_task_topology_group_t>() - 8usize];
    [
        "Offset of field: iree_task_topology_group_t::group_index",
    ][::core::mem::offset_of!(iree_task_topology_group_t, group_index) - 0usize];
    [
        "Offset of field: iree_task_topology_group_t::name",
    ][::core::mem::offset_of!(iree_task_topology_group_t, name) - 1usize];
    [
        "Offset of field: iree_task_topology_group_t::processor_index",
    ][::core::mem::offset_of!(iree_task_topology_group_t, processor_index) - 32usize];
    [
        "Offset of field: iree_task_topology_group_t::caches",
    ][::core::mem::offset_of!(iree_task_topology_group_t, caches) - 36usize];
    [
        "Offset of field: iree_task_topology_group_t::ideal_thread_affinity",
    ][::core::mem::offset_of!(iree_task_topology_group_t, ideal_thread_affinity)
        - 48usize];
    [
        "Offset of field: iree_task_topology_group_t::constructive_sharing_mask",
    ][::core::mem::offset_of!(iree_task_topology_group_t, constructive_sharing_mask)
        - 56usize];
};
impl Default for iree_task_topology_group_t {
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
    pub fn iree_task_topology_group_initialize(
        group_index: u8,
        out_group: *mut iree_task_topology_group_t,
    );
}
#[repr(C)]
pub struct iree_task_topology_t {
    pub numa_node_id: iree_task_topology_node_id_t,
    pub group_count: iree_host_size_t,
    pub groups: [iree_task_topology_group_t; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_topology_t",
    ][::core::mem::size_of::<iree_task_topology_t>() - 22544usize];
    [
        "Alignment of iree_task_topology_t",
    ][::core::mem::align_of::<iree_task_topology_t>() - 8usize];
    [
        "Offset of field: iree_task_topology_t::numa_node_id",
    ][::core::mem::offset_of!(iree_task_topology_t, numa_node_id) - 0usize];
    [
        "Offset of field: iree_task_topology_t::group_count",
    ][::core::mem::offset_of!(iree_task_topology_t, group_count) - 8usize];
    [
        "Offset of field: iree_task_topology_t::groups",
    ][::core::mem::offset_of!(iree_task_topology_t, groups) - 16usize];
};
impl Default for iree_task_topology_t {
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
    pub fn iree_task_topology_initialize(out_topology: *mut iree_task_topology_t);
}
unsafe extern "C" {
    pub fn iree_task_topology_deinitialize(topology: *mut iree_task_topology_t);
}
unsafe extern "C" {
    pub fn iree_task_topology_parse(
        value: iree_string_view_t,
        out_topology: *mut iree_task_topology_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_format(
        topology: *const iree_task_topology_t,
        buffer_capacity: iree_host_size_t,
        buffer: *mut ::core::ffi::c_char,
        out_buffer_length: *mut iree_host_size_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_task_topology_group_capacity(
        topology: *const iree_task_topology_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_group_count(
        topology: *const iree_task_topology_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_get_group(
        topology: *const iree_task_topology_t,
        group_index: iree_host_size_t,
    ) -> *const iree_task_topology_group_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_push_group(
        topology: *mut iree_task_topology_t,
        group: *const iree_task_topology_group_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_initialize_from_group_count(
        group_count: iree_host_size_t,
        out_topology: *mut iree_task_topology_t,
    );
}
unsafe extern "C" {
    pub fn iree_task_topology_initialize_from_thread_affinities(
        group_count: iree_host_size_t,
        group_affinities: *const iree_thread_affinity_t,
        out_topology: *mut iree_task_topology_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_initialize_from_logical_cpu_set(
        cpu_count: iree_host_size_t,
        cpu_ids: *const u32,
        out_topology: *mut iree_task_topology_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_initialize_from_logical_cpu_set_string(
        cpu_id_set: iree_string_view_t,
        out_topology: *mut iree_task_topology_t,
    ) -> iree_status_t;
}
pub const IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_ANY: iree_task_topology_performance_level_e = 0;
pub const IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_LOW: iree_task_topology_performance_level_e = 1;
pub const IREE_TASK_TOPOLOGY_PERFORMANCE_LEVEL_HIGH: iree_task_topology_performance_level_e = 2;
pub type iree_task_topology_performance_level_e = ::core::ffi::c_uint;
pub use self::iree_task_topology_performance_level_e as iree_task_topology_performance_level_t;
pub const IREE_TASK_TOPOLOGY_DISTRIBUTION_COMPACT: iree_task_topology_distribution_e = 0;
pub const IREE_TASK_TOPOLOGY_DISTRIBUTION_SCATTER: iree_task_topology_distribution_e = 1;
pub type iree_task_topology_distribution_e = ::core::ffi::c_uint;
pub use self::iree_task_topology_distribution_e as iree_task_topology_distribution_t;
unsafe extern "C" {
    pub fn iree_task_topology_initialize_from_physical_cores(
        node_id: iree_task_topology_node_id_t,
        performance_level: iree_task_topology_performance_level_t,
        distribution: iree_task_topology_distribution_t,
        max_core_count: iree_host_size_t,
        out_topology: *mut iree_task_topology_t,
    ) -> iree_status_t;
}
pub const IREE_TASK_SCHEDULING_MODE_RESERVED: iree_task_scheduling_mode_bits_t = 0;
pub type iree_task_scheduling_mode_bits_t = ::core::ffi::c_uint;
pub type iree_task_scheduling_mode_t = u32;
#[repr(C)]
pub struct iree_task_executor_options_t {
    pub scheduling_mode: iree_task_scheduling_mode_t,
    pub worker_base_index: iree_host_size_t,
    pub worker_spin_ns: iree_duration_t,
    pub worker_stack_size: iree_host_size_t,
    pub worker_local_memory_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_executor_options_t",
    ][::core::mem::size_of::<iree_task_executor_options_t>() - 40usize];
    [
        "Alignment of iree_task_executor_options_t",
    ][::core::mem::align_of::<iree_task_executor_options_t>() - 8usize];
    [
        "Offset of field: iree_task_executor_options_t::scheduling_mode",
    ][::core::mem::offset_of!(iree_task_executor_options_t, scheduling_mode) - 0usize];
    [
        "Offset of field: iree_task_executor_options_t::worker_base_index",
    ][::core::mem::offset_of!(iree_task_executor_options_t, worker_base_index) - 8usize];
    [
        "Offset of field: iree_task_executor_options_t::worker_spin_ns",
    ][::core::mem::offset_of!(iree_task_executor_options_t, worker_spin_ns) - 16usize];
    [
        "Offset of field: iree_task_executor_options_t::worker_stack_size",
    ][::core::mem::offset_of!(iree_task_executor_options_t, worker_stack_size)
        - 24usize];
    [
        "Offset of field: iree_task_executor_options_t::worker_local_memory_size",
    ][::core::mem::offset_of!(iree_task_executor_options_t, worker_local_memory_size)
        - 32usize];
};
impl Default for iree_task_executor_options_t {
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
    pub fn iree_task_executor_options_initialize(
        out_options: *mut iree_task_executor_options_t,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_task_executor_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn iree_task_executor_create(
        options: iree_task_executor_options_t,
        topology: *const iree_task_topology_t,
        allocator: iree_allocator_t,
        out_executor: *mut *mut iree_task_executor_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_executor_retain(executor: *mut iree_task_executor_t);
}
unsafe extern "C" {
    pub fn iree_task_executor_release(executor: *mut iree_task_executor_t);
}
unsafe extern "C" {
    pub fn iree_task_executor_node_id(
        executor: *mut iree_task_executor_t,
    ) -> iree_task_topology_node_id_t;
}
unsafe extern "C" {
    pub fn iree_task_executor_trim(executor: *mut iree_task_executor_t);
}
unsafe extern "C" {
    pub fn iree_task_executor_worker_count(
        executor: *mut iree_task_executor_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_task_executor_wake_workers(
        executor: *mut iree_task_executor_t,
        count: i32,
    );
}
unsafe extern "C" {
    pub fn iree_task_executor_schedule_process(
        executor: *mut iree_task_executor_t,
        process: *mut iree_task_process_t,
    );
}
#[repr(C)]
pub struct iree_task_node_selection_t {
    pub count: iree_host_size_t,
    pub ids: [iree_task_topology_node_id_t; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_task_node_selection_t",
    ][::core::mem::size_of::<iree_task_node_selection_t>() - 264usize];
    [
        "Alignment of iree_task_node_selection_t",
    ][::core::mem::align_of::<iree_task_node_selection_t>() - 8usize];
    [
        "Offset of field: iree_task_node_selection_t::count",
    ][::core::mem::offset_of!(iree_task_node_selection_t, count) - 0usize];
    [
        "Offset of field: iree_task_node_selection_t::ids",
    ][::core::mem::offset_of!(iree_task_node_selection_t, ids) - 8usize];
};
impl Default for iree_task_node_selection_t {
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
    pub fn iree_task_topology_select_nodes(
        spec: iree_string_view_t,
        out_selection: *mut iree_task_node_selection_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_executor_options_initialize_from_flags(
        out_options: *mut iree_task_executor_options_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_topology_initialize_from_flags(
        node_id: iree_task_topology_node_id_t,
        out_topology: *mut iree_task_topology_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_task_executors_create_from_flags(
        host_allocator: iree_allocator_t,
        executor_capacity: iree_host_size_t,
        executors: *mut *mut iree_task_executor_t,
        out_executor_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
