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
pub const IREE_TOKENIZER_DECODER_MIN_BUFFER_SIZE: u32 = 4;
pub const IREE_TOKENIZER_DECODER_CTC_MAX_TOKEN_SIZE: u32 = 64;
pub const IREE_TOKENIZER_DECODER_SEQUENCE_MAX_DEPTH: u32 = 8;
pub const IREE_TOKENIZER_DECODER_STRIP_MAX_CONTENT_LENGTH: u32 = 8;
pub const IREE_TOKENIZER_WORDPIECE_MAX_PREFIX_LENGTH: u32 = 8;
pub const IREE_TOKENIZER_POSTPROCESSOR_MAX_PIECES: u32 = 7;
pub const IREE_TOKENIZER_SPECIAL_TOKENS_MAX_BUCKETS: u32 = 16;
pub const IREE_TOKENIZER_SPECIAL_TOKENS_NO_BUCKET: u32 = 255;
pub const IREE_TOKENIZER_TRANSFORM_BUFFER_MAX_SIZE: u32 = 16384;
pub const IREE_TOKENIZER_TRANSFORM_BUFFER_EXPANSION_FACTOR: u32 = 3;
pub const IREE_TOKENIZER_TRANSFORM_BUFFER_MIN_SIZE: u32 = 4096;
pub const IREE_TOKENIZER_DECODE_OUTPUT_RECOMMENDED_SIZE: u32 = 2048;
pub const IREE_TOKENIZER_NORMALIZER_DEFERRED_BUFFER_SIZE: u32 = 512;
pub const IREE_TOKENIZER_NORMALIZER_PREPEND_MAX_LENGTH: u32 = 16;
pub const IREE_TOKENIZER_REGEX_REPLACE_MAX_CONTENT: u32 = 32;
pub const IREE_TOKENIZER_REPLACE_MAX_PATTERN: u32 = 32;
pub const IREE_TOKENIZER_REPLACE_MAX_CONTENT: u32 = 32;
pub const IREE_TOKENIZER_NORMALIZER_SEQUENCE_MAX_DEPTH: u32 = 16;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_VERSION: u32 = 1;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_MAX_VERSION: u32 = 1;
pub const IREE_TOKENIZER_UTIL_REGEX_NO_TRANSITION: u32 = 65535;
pub const IREE_TOKENIZER_UTIL_REGEX_REWIND_BUFFER_SIZE: u32 = 256;
pub const IREE_TOKENIZER_UTIL_REGEX_MAX_CHAR_CLASS_RANGES: u32 = 10;
pub const IREE_TOKENIZER_METASPACE_DEFAULT_REPLACEMENT: u32 = 9601;
pub const IREE_TOKENIZER_SEGMENTER_SEQUENCE_MAX_DEPTH: u32 = 8;
pub const IREE_TOKENIZER_VOCAB_HASH_DEFAULT_LOAD_PERCENT: u32 = 75;
pub const IREE_TOKENIZER_MERGE_HASH_EMPTY_KEY: i32 = -1;
#[repr(C)]
pub struct iree_tokenizer_string_list_t {
    pub count: iree_host_size_t,
    pub values: *const iree_string_view_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_string_list_t",
    ][::core::mem::size_of::<iree_tokenizer_string_list_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_string_list_t",
    ][::core::mem::align_of::<iree_tokenizer_string_list_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_string_list_t::count",
    ][::core::mem::offset_of!(iree_tokenizer_string_list_t, count) - 0usize];
    [
        "Offset of field: iree_tokenizer_string_list_t::values",
    ][::core::mem::offset_of!(iree_tokenizer_string_list_t, values) - 8usize];
};
impl Default for iree_tokenizer_string_list_t {
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
    #[link_name = "iree_tokenizer_make_string_list__extern"]
    pub fn iree_tokenizer_make_string_list(
        values: *const iree_string_view_t,
        count: iree_host_size_t,
    ) -> iree_tokenizer_string_list_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_string_list_empty__extern"]
    pub fn iree_tokenizer_string_list_empty() -> iree_tokenizer_string_list_t;
}
pub type iree_tokenizer_decoder_flags_t = u32;
pub const IREE_TOKENIZER_DECODER_FLAG_NONE: iree_tokenizer_decoder_flags_e = 0;
pub const IREE_TOKENIZER_DECODER_FLAG_SHRINKING: iree_tokenizer_decoder_flags_e = 1;
pub type iree_tokenizer_decoder_flags_e = ::core::ffi::c_uint;
pub type iree_tokenizer_decoder_capability_t = u32;
pub const IREE_TOKENIZER_DECODER_CAPABILITY_NONE: iree_tokenizer_decoder_capability_e = 0;
pub const IREE_TOKENIZER_DECODER_CAPABILITY_STATELESS: iree_tokenizer_decoder_capability_e = 1;
pub const IREE_TOKENIZER_DECODER_CAPABILITY_POSITION_SENSITIVE: iree_tokenizer_decoder_capability_e = 2;
pub const IREE_TOKENIZER_DECODER_CAPABILITY_STATELESS_EXCEPT_BYTE_TOKENS: iree_tokenizer_decoder_capability_e = 4;
pub const IREE_TOKENIZER_DECODER_CAPABILITY_STATELESS_EXCEPT_PARTIAL_UTF8: iree_tokenizer_decoder_capability_e = 8;
pub type iree_tokenizer_decoder_capability_e = ::core::ffi::c_uint;
#[repr(C)]
pub struct iree_tokenizer_decoder_t {
    pub vtable: *const iree_tokenizer_decoder_vtable_t,
    pub state_size: iree_host_size_t,
    pub capabilities: iree_tokenizer_decoder_capability_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_decoder_t",
    ][::core::mem::size_of::<iree_tokenizer_decoder_t>() - 24usize];
    [
        "Alignment of iree_tokenizer_decoder_t",
    ][::core::mem::align_of::<iree_tokenizer_decoder_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_decoder_t::vtable",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_t, vtable) - 0usize];
    [
        "Offset of field: iree_tokenizer_decoder_t::state_size",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_t, state_size) - 8usize];
    [
        "Offset of field: iree_tokenizer_decoder_t::capabilities",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_t, capabilities) - 16usize];
};
impl Default for iree_tokenizer_decoder_t {
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
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_decoder_state_t {
    pub decoder: *const iree_tokenizer_decoder_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_decoder_state_t",
    ][::core::mem::size_of::<iree_tokenizer_decoder_state_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_decoder_state_t",
    ][::core::mem::align_of::<iree_tokenizer_decoder_state_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_decoder_state_t::decoder",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_state_t, decoder) - 0usize];
};
impl Default for iree_tokenizer_decoder_state_t {
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
pub struct iree_tokenizer_decoder_vtable_t {
    pub destroy: ::core::option::Option<
        unsafe extern "C" fn(decoder: *mut iree_tokenizer_decoder_t),
    >,
    pub state_initialize: ::core::option::Option<
        unsafe extern "C" fn(
            decoder: *const iree_tokenizer_decoder_t,
            storage: *mut ::core::ffi::c_void,
            out_state: *mut *mut iree_tokenizer_decoder_state_t,
        ) -> iree_status_t,
    >,
    pub state_deinitialize: ::core::option::Option<
        unsafe extern "C" fn(state: *mut iree_tokenizer_decoder_state_t),
    >,
    pub state_process: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_decoder_state_t,
            token_strings: iree_tokenizer_string_list_t,
            output: iree_mutable_string_view_t,
            out_strings_consumed: *mut iree_host_size_t,
            out_bytes_written: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_finalize: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_decoder_state_t,
            output: iree_mutable_string_view_t,
            out_written: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_has_pending: ::core::option::Option<
        unsafe extern "C" fn(state: *const iree_tokenizer_decoder_state_t) -> bool,
    >,
    pub flags: iree_tokenizer_decoder_flags_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_decoder_vtable_t",
    ][::core::mem::size_of::<iree_tokenizer_decoder_vtable_t>() - 56usize];
    [
        "Alignment of iree_tokenizer_decoder_vtable_t",
    ][::core::mem::align_of::<iree_tokenizer_decoder_vtable_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::destroy",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, destroy) - 0usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::state_initialize",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, state_initialize)
        - 8usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::state_deinitialize",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, state_deinitialize)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::state_process",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, state_process) - 24usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::state_finalize",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, state_finalize)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::state_has_pending",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, state_has_pending)
        - 40usize];
    [
        "Offset of field: iree_tokenizer_decoder_vtable_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_vtable_t, flags) - 48usize];
};
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_initialize(
        decoder: *mut iree_tokenizer_decoder_t,
        vtable: *const iree_tokenizer_decoder_vtable_t,
        state_size: iree_host_size_t,
        capabilities: iree_tokenizer_decoder_capability_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_free(decoder: *mut iree_tokenizer_decoder_t);
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_state_size__extern"]
    pub fn iree_tokenizer_decoder_state_size(
        decoder: *const iree_tokenizer_decoder_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_capabilities__extern"]
    pub fn iree_tokenizer_decoder_capabilities(
        decoder: *const iree_tokenizer_decoder_t,
    ) -> iree_tokenizer_decoder_capability_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_state_initialize__extern"]
    pub fn iree_tokenizer_decoder_state_initialize(
        decoder: *const iree_tokenizer_decoder_t,
        storage: *mut ::core::ffi::c_void,
        out_state: *mut *mut iree_tokenizer_decoder_state_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_state_deinitialize__extern"]
    pub fn iree_tokenizer_decoder_state_deinitialize(
        state: *mut iree_tokenizer_decoder_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_state_process__extern"]
    pub fn iree_tokenizer_decoder_state_process(
        state: *mut iree_tokenizer_decoder_state_t,
        token_strings: iree_tokenizer_string_list_t,
        output: iree_mutable_string_view_t,
        out_strings_consumed: *mut iree_host_size_t,
        out_bytes_written: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_state_finalize__extern"]
    pub fn iree_tokenizer_decoder_state_finalize(
        state: *mut iree_tokenizer_decoder_state_t,
        output: iree_mutable_string_view_t,
        out_written: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_decoder_state_has_pending__extern"]
    pub fn iree_tokenizer_decoder_state_has_pending(
        state: *const iree_tokenizer_decoder_state_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_byte_fallback_parse_byte_token(
        token: iree_string_view_t,
        out_byte: *mut u8,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_byte_fallback_allocate(
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_byte_level_allocate(
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_ctc_allocate(
        pad_token: iree_string_view_t,
        word_delimiter_token: iree_string_view_t,
        cleanup: bool,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_DECODER_METASPACE_PREPEND_ALWAYS: iree_tokenizer_decoder_metaspace_prepend_scheme_e = 0;
pub const IREE_TOKENIZER_DECODER_METASPACE_PREPEND_NEVER: iree_tokenizer_decoder_metaspace_prepend_scheme_e = 1;
pub const IREE_TOKENIZER_DECODER_METASPACE_PREPEND_FIRST: iree_tokenizer_decoder_metaspace_prepend_scheme_e = 2;
pub type iree_tokenizer_decoder_metaspace_prepend_scheme_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_decoder_metaspace_prepend_scheme_e as iree_tokenizer_decoder_metaspace_prepend_scheme_t;
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_metaspace_allocate(
        replacement_codepoint: u32,
        prepend_scheme: iree_tokenizer_decoder_metaspace_prepend_scheme_t,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_passthrough_allocate(
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_replace_allocate(
        pattern: iree_string_view_t,
        content: iree_string_view_t,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_sequence_allocate(
        children: *const *mut iree_tokenizer_decoder_t,
        child_count: iree_host_size_t,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_strip_allocate(
        content: iree_string_view_t,
        start_count: iree_host_size_t,
        stop_count: iree_host_size_t,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
#[repr(C)]
pub struct iree_tokenizer_decoder_wordpiece_config_t {
    pub prefix: iree_string_view_t,
    pub cleanup: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_decoder_wordpiece_config_t",
    ][::core::mem::size_of::<iree_tokenizer_decoder_wordpiece_config_t>() - 24usize];
    [
        "Alignment of iree_tokenizer_decoder_wordpiece_config_t",
    ][::core::mem::align_of::<iree_tokenizer_decoder_wordpiece_config_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_decoder_wordpiece_config_t::prefix",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_wordpiece_config_t, prefix)
        - 0usize];
    [
        "Offset of field: iree_tokenizer_decoder_wordpiece_config_t::cleanup",
    ][::core::mem::offset_of!(iree_tokenizer_decoder_wordpiece_config_t, cleanup)
        - 16usize];
};
impl Default for iree_tokenizer_decoder_wordpiece_config_t {
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
    #[link_name = "iree_tokenizer_make_decoder_wordpiece_config__extern"]
    pub fn iree_tokenizer_make_decoder_wordpiece_config(
        prefix: iree_string_view_t,
        cleanup: bool,
    ) -> iree_tokenizer_decoder_wordpiece_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decoder_wordpiece_allocate(
        config: iree_tokenizer_decoder_wordpiece_config_t,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
pub type iree_tokenizer_token_id_t = i32;
#[repr(C)]
pub struct iree_tokenizer_token_id_list_t {
    pub count: iree_host_size_t,
    pub values: *const iree_tokenizer_token_id_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_token_id_list_t",
    ][::core::mem::size_of::<iree_tokenizer_token_id_list_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_token_id_list_t",
    ][::core::mem::align_of::<iree_tokenizer_token_id_list_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_token_id_list_t::count",
    ][::core::mem::offset_of!(iree_tokenizer_token_id_list_t, count) - 0usize];
    [
        "Offset of field: iree_tokenizer_token_id_list_t::values",
    ][::core::mem::offset_of!(iree_tokenizer_token_id_list_t, values) - 8usize];
};
impl Default for iree_tokenizer_token_id_list_t {
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
    #[link_name = "iree_tokenizer_token_id_list_empty__extern"]
    pub fn iree_tokenizer_token_id_list_empty() -> iree_tokenizer_token_id_list_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_make_token_id_list__extern"]
    pub fn iree_tokenizer_make_token_id_list(
        values: *const iree_tokenizer_token_id_t,
        count: iree_host_size_t,
    ) -> iree_tokenizer_token_id_list_t;
}
#[repr(C)]
pub struct iree_tokenizer_offset_t {
    pub start: iree_host_size_t,
    pub end: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_offset_t",
    ][::core::mem::size_of::<iree_tokenizer_offset_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_offset_t",
    ][::core::mem::align_of::<iree_tokenizer_offset_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_offset_t::start",
    ][::core::mem::offset_of!(iree_tokenizer_offset_t, start) - 0usize];
    [
        "Offset of field: iree_tokenizer_offset_t::end",
    ][::core::mem::offset_of!(iree_tokenizer_offset_t, end) - 8usize];
};
impl Default for iree_tokenizer_offset_t {
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
pub struct iree_tokenizer_token_output_t {
    pub capacity: iree_host_size_t,
    pub token_ids: *mut iree_tokenizer_token_id_t,
    pub token_offsets: *mut iree_tokenizer_offset_t,
    pub type_ids: *mut u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_token_output_t",
    ][::core::mem::size_of::<iree_tokenizer_token_output_t>() - 32usize];
    [
        "Alignment of iree_tokenizer_token_output_t",
    ][::core::mem::align_of::<iree_tokenizer_token_output_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_token_output_t::capacity",
    ][::core::mem::offset_of!(iree_tokenizer_token_output_t, capacity) - 0usize];
    [
        "Offset of field: iree_tokenizer_token_output_t::token_ids",
    ][::core::mem::offset_of!(iree_tokenizer_token_output_t, token_ids) - 8usize];
    [
        "Offset of field: iree_tokenizer_token_output_t::token_offsets",
    ][::core::mem::offset_of!(iree_tokenizer_token_output_t, token_offsets) - 16usize];
    [
        "Offset of field: iree_tokenizer_token_output_t::type_ids",
    ][::core::mem::offset_of!(iree_tokenizer_token_output_t, type_ids) - 24usize];
};
impl Default for iree_tokenizer_token_output_t {
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
    #[link_name = "iree_tokenizer_make_token_output__extern"]
    pub fn iree_tokenizer_make_token_output(
        token_ids: *mut iree_tokenizer_token_id_t,
        token_offsets: *mut iree_tokenizer_offset_t,
        type_ids: *mut u8,
        capacity: iree_host_size_t,
    ) -> iree_tokenizer_token_output_t;
}
pub const IREE_TOKENIZER_HUGGINGFACE_ADDED_TOKEN_FLAG_NONE: iree_tokenizer_huggingface_added_token_flag_bits_e = 0;
pub const IREE_TOKENIZER_HUGGINGFACE_ADDED_TOKEN_FLAG_SINGLE_WORD: iree_tokenizer_huggingface_added_token_flag_bits_e = 1;
pub const IREE_TOKENIZER_HUGGINGFACE_ADDED_TOKEN_FLAG_LSTRIP: iree_tokenizer_huggingface_added_token_flag_bits_e = 2;
pub const IREE_TOKENIZER_HUGGINGFACE_ADDED_TOKEN_FLAG_RSTRIP: iree_tokenizer_huggingface_added_token_flag_bits_e = 4;
pub const IREE_TOKENIZER_HUGGINGFACE_ADDED_TOKEN_FLAG_NORMALIZED: iree_tokenizer_huggingface_added_token_flag_bits_e = 8;
pub const IREE_TOKENIZER_HUGGINGFACE_ADDED_TOKEN_FLAG_SPECIAL: iree_tokenizer_huggingface_added_token_flag_bits_e = 16;
pub type iree_tokenizer_huggingface_added_token_flag_bits_e = ::core::ffi::c_uint;
pub type iree_tokenizer_huggingface_added_token_flags_t = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_huggingface_added_token_t {
    pub id: iree_tokenizer_token_id_t,
    pub flags: iree_tokenizer_huggingface_added_token_flags_t,
    pub content_offset: u32,
    pub content_length: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_huggingface_added_token_t",
    ][::core::mem::size_of::<iree_tokenizer_huggingface_added_token_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_huggingface_added_token_t",
    ][::core::mem::align_of::<iree_tokenizer_huggingface_added_token_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_token_t::id",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_token_t, id) - 0usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_token_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_token_t, flags) - 4usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_token_t::content_offset",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_token_t, content_offset)
        - 8usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_token_t::content_length",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_token_t, content_length)
        - 12usize];
};
#[repr(C)]
pub struct iree_tokenizer_huggingface_added_tokens_t {
    pub count: iree_host_size_t,
    pub string_pool_size: iree_host_size_t,
    pub allocator: iree_allocator_t,
    pub tokens: *const iree_tokenizer_huggingface_added_token_t,
    pub string_pool: *const ::core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_huggingface_added_tokens_t",
    ][::core::mem::size_of::<iree_tokenizer_huggingface_added_tokens_t>() - 48usize];
    [
        "Alignment of iree_tokenizer_huggingface_added_tokens_t",
    ][::core::mem::align_of::<iree_tokenizer_huggingface_added_tokens_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_tokens_t::count",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_tokens_t, count)
        - 0usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_tokens_t::string_pool_size",
    ][::core::mem::offset_of!(
        iree_tokenizer_huggingface_added_tokens_t, string_pool_size
    ) - 8usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_tokens_t::allocator",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_tokens_t, allocator)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_tokens_t::tokens",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_tokens_t, tokens)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_huggingface_added_tokens_t::string_pool",
    ][::core::mem::offset_of!(iree_tokenizer_huggingface_added_tokens_t, string_pool)
        - 40usize];
};
impl Default for iree_tokenizer_huggingface_added_tokens_t {
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
    pub fn iree_tokenizer_huggingface_added_tokens_free(
        tokens: *mut iree_tokenizer_huggingface_added_tokens_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_huggingface_added_tokens_get__extern"]
    pub fn iree_tokenizer_huggingface_added_tokens_get(
        list: *const iree_tokenizer_huggingface_added_tokens_t,
        index: iree_host_size_t,
    ) -> *const iree_tokenizer_huggingface_added_token_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_huggingface_added_token_content__extern"]
    pub fn iree_tokenizer_huggingface_added_token_content(
        list: *const iree_tokenizer_huggingface_added_tokens_t,
        token: *const iree_tokenizer_huggingface_added_token_t,
    ) -> iree_string_view_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_added_tokens_json(
        json_root: iree_string_view_t,
        allocator: iree_allocator_t,
        out_tokens: *mut iree_tokenizer_huggingface_added_tokens_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_decoder(
        decoder_value: iree_string_view_t,
        allocator: iree_allocator_t,
        out_decoder: *mut *mut iree_tokenizer_decoder_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_NONE: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 0;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_BYTE_LEVEL: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 1;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_ADD_PREFIX_SPACE: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 2;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_METASPACE_PREPEND: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 4;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_METASPACE_REPLACE: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 8;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_WORD_LEVEL_SPLIT: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 16;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_TRIM_OFFSETS: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 32;
pub const IREE_TOKENIZER_HUGGINGFACE_PRE_TOKENIZER_FLAG_HAS_WHITESPACE_SPLIT: iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = 64;
pub type iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_huggingface_pre_tokenizer_flag_bits_e as iree_tokenizer_huggingface_pre_tokenizer_flag_bits_t;
pub type iree_tokenizer_huggingface_pre_tokenizer_flags_t = u32;
#[repr(C)]
pub struct iree_tokenizer_segment_t {
    pub start: iree_host_size_t,
    pub end: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_segment_t",
    ][::core::mem::size_of::<iree_tokenizer_segment_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_segment_t",
    ][::core::mem::align_of::<iree_tokenizer_segment_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_segment_t::start",
    ][::core::mem::offset_of!(iree_tokenizer_segment_t, start) - 0usize];
    [
        "Offset of field: iree_tokenizer_segment_t::end",
    ][::core::mem::offset_of!(iree_tokenizer_segment_t, end) - 8usize];
};
impl Default for iree_tokenizer_segment_t {
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
pub struct iree_tokenizer_segment_output_t {
    pub capacity: iree_host_size_t,
    pub values: *mut iree_tokenizer_segment_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_segment_output_t",
    ][::core::mem::size_of::<iree_tokenizer_segment_output_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_segment_output_t",
    ][::core::mem::align_of::<iree_tokenizer_segment_output_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_segment_output_t::capacity",
    ][::core::mem::offset_of!(iree_tokenizer_segment_output_t, capacity) - 0usize];
    [
        "Offset of field: iree_tokenizer_segment_output_t::values",
    ][::core::mem::offset_of!(iree_tokenizer_segment_output_t, values) - 8usize];
};
impl Default for iree_tokenizer_segment_output_t {
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
    #[link_name = "iree_tokenizer_make_segment_output__extern"]
    pub fn iree_tokenizer_make_segment_output(
        values: *mut iree_tokenizer_segment_t,
        capacity: iree_host_size_t,
    ) -> iree_tokenizer_segment_output_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segment_output_empty__extern"]
    pub fn iree_tokenizer_segment_output_empty() -> iree_tokenizer_segment_output_t;
}
#[repr(C)]
pub struct iree_tokenizer_segmenter_t {
    pub vtable: *const iree_tokenizer_segmenter_vtable_t,
    pub state_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_segmenter_t",
    ][::core::mem::size_of::<iree_tokenizer_segmenter_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_segmenter_t",
    ][::core::mem::align_of::<iree_tokenizer_segmenter_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_segmenter_t::vtable",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_t, vtable) - 0usize];
    [
        "Offset of field: iree_tokenizer_segmenter_t::state_size",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_t, state_size) - 8usize];
};
impl Default for iree_tokenizer_segmenter_t {
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
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_segmenter_state_t {
    pub segmenter: *const iree_tokenizer_segmenter_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_segmenter_state_t",
    ][::core::mem::size_of::<iree_tokenizer_segmenter_state_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_segmenter_state_t",
    ][::core::mem::align_of::<iree_tokenizer_segmenter_state_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_segmenter_state_t::segmenter",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_state_t, segmenter) - 0usize];
};
impl Default for iree_tokenizer_segmenter_state_t {
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
pub struct iree_tokenizer_segmenter_vtable_t {
    pub destroy: ::core::option::Option<
        unsafe extern "C" fn(segmenter: *mut iree_tokenizer_segmenter_t),
    >,
    pub state_initialize: ::core::option::Option<
        unsafe extern "C" fn(
            segmenter: *const iree_tokenizer_segmenter_t,
            storage: *mut ::core::ffi::c_void,
            out_state: *mut *mut iree_tokenizer_segmenter_state_t,
        ) -> iree_status_t,
    >,
    pub state_deinitialize: ::core::option::Option<
        unsafe extern "C" fn(state: *mut iree_tokenizer_segmenter_state_t),
    >,
    pub state_process: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_segmenter_state_t,
            input: iree_string_view_t,
            output: iree_tokenizer_segment_output_t,
            out_consumed: *mut iree_host_size_t,
            out_segment_count: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_finalize: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_segmenter_state_t,
            remaining_input: iree_string_view_t,
            output: iree_tokenizer_segment_output_t,
            out_segment_count: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_has_pending: ::core::option::Option<
        unsafe extern "C" fn(state: *const iree_tokenizer_segmenter_state_t) -> bool,
    >,
    pub state_flush: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_segmenter_state_t,
            output: iree_tokenizer_segment_output_t,
            out_segment_count: *mut iree_host_size_t,
            out_bytes_committed: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_segmenter_vtable_t",
    ][::core::mem::size_of::<iree_tokenizer_segmenter_vtable_t>() - 56usize];
    [
        "Alignment of iree_tokenizer_segmenter_vtable_t",
    ][::core::mem::align_of::<iree_tokenizer_segmenter_vtable_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::destroy",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, destroy) - 0usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::state_initialize",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, state_initialize)
        - 8usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::state_deinitialize",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, state_deinitialize)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::state_process",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, state_process)
        - 24usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::state_finalize",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, state_finalize)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::state_has_pending",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, state_has_pending)
        - 40usize];
    [
        "Offset of field: iree_tokenizer_segmenter_vtable_t::state_flush",
    ][::core::mem::offset_of!(iree_tokenizer_segmenter_vtable_t, state_flush) - 48usize];
};
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_initialize(
        segmenter: *mut iree_tokenizer_segmenter_t,
        vtable: *const iree_tokenizer_segmenter_vtable_t,
        state_size: iree_host_size_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_free(segmenter: *mut iree_tokenizer_segmenter_t);
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_size__extern"]
    pub fn iree_tokenizer_segmenter_state_size(
        segmenter: *const iree_tokenizer_segmenter_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_initialize__extern"]
    pub fn iree_tokenizer_segmenter_state_initialize(
        segmenter: *const iree_tokenizer_segmenter_t,
        storage: *mut ::core::ffi::c_void,
        out_state: *mut *mut iree_tokenizer_segmenter_state_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_deinitialize__extern"]
    pub fn iree_tokenizer_segmenter_state_deinitialize(
        state: *mut iree_tokenizer_segmenter_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_process__extern"]
    pub fn iree_tokenizer_segmenter_state_process(
        state: *mut iree_tokenizer_segmenter_state_t,
        input: iree_string_view_t,
        output: iree_tokenizer_segment_output_t,
        out_consumed: *mut iree_host_size_t,
        out_segment_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_finalize__extern"]
    pub fn iree_tokenizer_segmenter_state_finalize(
        state: *mut iree_tokenizer_segmenter_state_t,
        remaining_input: iree_string_view_t,
        output: iree_tokenizer_segment_output_t,
        out_segment_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_has_pending__extern"]
    pub fn iree_tokenizer_segmenter_state_has_pending(
        state: *const iree_tokenizer_segmenter_state_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segmenter_state_flush__extern"]
    pub fn iree_tokenizer_segmenter_state_flush(
        state: *mut iree_tokenizer_segmenter_state_t,
        output: iree_tokenizer_segment_output_t,
        out_segment_count: *mut iree_host_size_t,
        out_bytes_committed: *mut iree_host_size_t,
    ) -> iree_status_t;
}
#[repr(C)]
pub struct iree_tokenizer_segment_list_t {
    pub count: iree_host_size_t,
    pub values: *const iree_tokenizer_segment_t,
    pub last_is_partial: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_segment_list_t",
    ][::core::mem::size_of::<iree_tokenizer_segment_list_t>() - 24usize];
    [
        "Alignment of iree_tokenizer_segment_list_t",
    ][::core::mem::align_of::<iree_tokenizer_segment_list_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_segment_list_t::count",
    ][::core::mem::offset_of!(iree_tokenizer_segment_list_t, count) - 0usize];
    [
        "Offset of field: iree_tokenizer_segment_list_t::values",
    ][::core::mem::offset_of!(iree_tokenizer_segment_list_t, values) - 8usize];
    [
        "Offset of field: iree_tokenizer_segment_list_t::last_is_partial",
    ][::core::mem::offset_of!(iree_tokenizer_segment_list_t, last_is_partial) - 16usize];
};
impl Default for iree_tokenizer_segment_list_t {
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
    #[link_name = "iree_tokenizer_make_segment_list__extern"]
    pub fn iree_tokenizer_make_segment_list(
        values: *const iree_tokenizer_segment_t,
        count: iree_host_size_t,
    ) -> iree_tokenizer_segment_list_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_segment_list_empty__extern"]
    pub fn iree_tokenizer_segment_list_empty() -> iree_tokenizer_segment_list_t;
}
#[repr(C)]
pub struct iree_tokenizer_model_t {
    pub vtable: *const iree_tokenizer_model_vtable_t,
    pub state_size: iree_host_size_t,
    pub type_name: iree_string_view_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_model_t",
    ][::core::mem::size_of::<iree_tokenizer_model_t>() - 32usize];
    [
        "Alignment of iree_tokenizer_model_t",
    ][::core::mem::align_of::<iree_tokenizer_model_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_model_t::vtable",
    ][::core::mem::offset_of!(iree_tokenizer_model_t, vtable) - 0usize];
    [
        "Offset of field: iree_tokenizer_model_t::state_size",
    ][::core::mem::offset_of!(iree_tokenizer_model_t, state_size) - 8usize];
    [
        "Offset of field: iree_tokenizer_model_t::type_name",
    ][::core::mem::offset_of!(iree_tokenizer_model_t, type_name) - 16usize];
};
impl Default for iree_tokenizer_model_t {
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
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_model_state_t {
    pub model: *const iree_tokenizer_model_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_model_state_t",
    ][::core::mem::size_of::<iree_tokenizer_model_state_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_model_state_t",
    ][::core::mem::align_of::<iree_tokenizer_model_state_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_model_state_t::model",
    ][::core::mem::offset_of!(iree_tokenizer_model_state_t, model) - 0usize];
};
impl Default for iree_tokenizer_model_state_t {
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
pub struct iree_tokenizer_model_vtable_t {
    pub destroy: ::core::option::Option<
        unsafe extern "C" fn(model: *mut iree_tokenizer_model_t),
    >,
    pub state_initialize: ::core::option::Option<
        unsafe extern "C" fn(
            model: *const iree_tokenizer_model_t,
            storage: *mut ::core::ffi::c_void,
            out_state: *mut *mut iree_tokenizer_model_state_t,
        ) -> iree_status_t,
    >,
    pub state_deinitialize: ::core::option::Option<
        unsafe extern "C" fn(state: *mut iree_tokenizer_model_state_t),
    >,
    pub state_encode: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_model_state_t,
            transform_buffer: iree_const_byte_span_t,
            segments: iree_tokenizer_segment_list_t,
            output: iree_tokenizer_token_output_t,
            out_segments_consumed: *mut iree_host_size_t,
            out_token_count: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_finalize: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_model_state_t,
            output: iree_tokenizer_token_output_t,
            out_token_count: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_has_pending: ::core::option::Option<
        unsafe extern "C" fn(state: *const iree_tokenizer_model_state_t) -> bool,
    >,
    pub state_reclaim: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_model_state_t,
        ) -> iree_host_size_t,
    >,
    pub get_token_string: ::core::option::Option<
        unsafe extern "C" fn(
            model: *const iree_tokenizer_model_t,
            token_id: iree_tokenizer_token_id_t,
            out_string: *mut iree_string_view_t,
        ) -> iree_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_model_vtable_t",
    ][::core::mem::size_of::<iree_tokenizer_model_vtable_t>() - 64usize];
    [
        "Alignment of iree_tokenizer_model_vtable_t",
    ][::core::mem::align_of::<iree_tokenizer_model_vtable_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::destroy",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, destroy) - 0usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::state_initialize",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, state_initialize) - 8usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::state_deinitialize",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, state_deinitialize)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::state_encode",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, state_encode) - 24usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::state_finalize",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, state_finalize) - 32usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::state_has_pending",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, state_has_pending)
        - 40usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::state_reclaim",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, state_reclaim) - 48usize];
    [
        "Offset of field: iree_tokenizer_model_vtable_t::get_token_string",
    ][::core::mem::offset_of!(iree_tokenizer_model_vtable_t, get_token_string)
        - 56usize];
};
unsafe extern "C" {
    pub fn iree_tokenizer_model_initialize(
        model: *mut iree_tokenizer_model_t,
        vtable: *const iree_tokenizer_model_vtable_t,
        state_size: iree_host_size_t,
        type_name: iree_string_view_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_model_free(model: *mut iree_tokenizer_model_t);
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_size__extern"]
    pub fn iree_tokenizer_model_state_size(
        model: *const iree_tokenizer_model_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_initialize__extern"]
    pub fn iree_tokenizer_model_state_initialize(
        model: *const iree_tokenizer_model_t,
        storage: *mut ::core::ffi::c_void,
        out_state: *mut *mut iree_tokenizer_model_state_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_deinitialize__extern"]
    pub fn iree_tokenizer_model_state_deinitialize(
        state: *mut iree_tokenizer_model_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_encode__extern"]
    pub fn iree_tokenizer_model_state_encode(
        state: *mut iree_tokenizer_model_state_t,
        transform_buffer: iree_const_byte_span_t,
        segments: iree_tokenizer_segment_list_t,
        output: iree_tokenizer_token_output_t,
        out_segments_consumed: *mut iree_host_size_t,
        out_token_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_finalize__extern"]
    pub fn iree_tokenizer_model_state_finalize(
        state: *mut iree_tokenizer_model_state_t,
        output: iree_tokenizer_token_output_t,
        out_token_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_has_pending__extern"]
    pub fn iree_tokenizer_model_state_has_pending(
        state: *const iree_tokenizer_model_state_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_state_reclaim__extern"]
    pub fn iree_tokenizer_model_state_reclaim(
        state: *mut iree_tokenizer_model_state_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_model_get_token_string__extern"]
    pub fn iree_tokenizer_model_get_token_string(
        model: *const iree_tokenizer_model_t,
        token_id: iree_tokenizer_token_id_t,
        out_string: *mut iree_string_view_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_TOKEN_ATTR_NONE: iree_tokenizer_token_attr_bits_e = 0;
pub const IREE_TOKENIZER_TOKEN_ATTR_UNKNOWN: iree_tokenizer_token_attr_bits_e = 1;
pub const IREE_TOKENIZER_TOKEN_ATTR_CONTROL: iree_tokenizer_token_attr_bits_e = 2;
pub const IREE_TOKENIZER_TOKEN_ATTR_BYTE: iree_tokenizer_token_attr_bits_e = 4;
pub const IREE_TOKENIZER_TOKEN_ATTR_SPECIAL: iree_tokenizer_token_attr_bits_e = 8;
pub const IREE_TOKENIZER_TOKEN_ATTR_UNUSED: iree_tokenizer_token_attr_bits_e = 16;
pub type iree_tokenizer_token_attr_bits_e = ::core::ffi::c_uint;
pub type iree_tokenizer_token_attr_t = u16;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_token_t {
    pub string_offset: u32,
    pub string_length: u16,
    pub attributes: iree_tokenizer_token_attr_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_token_t",
    ][::core::mem::size_of::<iree_tokenizer_token_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_token_t",
    ][::core::mem::align_of::<iree_tokenizer_token_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_token_t::string_offset",
    ][::core::mem::offset_of!(iree_tokenizer_token_t, string_offset) - 0usize];
    [
        "Offset of field: iree_tokenizer_token_t::string_length",
    ][::core::mem::offset_of!(iree_tokenizer_token_t, string_length) - 4usize];
    [
        "Offset of field: iree_tokenizer_token_t::attributes",
    ][::core::mem::offset_of!(iree_tokenizer_token_t, attributes) - 6usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_merge_t {
    pub left_id: u32,
    pub right_id: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_merge_t",
    ][::core::mem::size_of::<iree_tokenizer_merge_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_merge_t",
    ][::core::mem::align_of::<iree_tokenizer_merge_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_merge_t::left_id",
    ][::core::mem::offset_of!(iree_tokenizer_merge_t, left_id) - 0usize];
    [
        "Offset of field: iree_tokenizer_merge_t::right_id",
    ][::core::mem::offset_of!(iree_tokenizer_merge_t, right_id) - 4usize];
};
pub const IREE_TOKENIZER_SPECIAL_TOKEN_UNK: iree_tokenizer_special_token_e = 0;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_BOS: iree_tokenizer_special_token_e = 1;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_EOS: iree_tokenizer_special_token_e = 2;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_PAD: iree_tokenizer_special_token_e = 3;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_SEP: iree_tokenizer_special_token_e = 4;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_CLS: iree_tokenizer_special_token_e = 5;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_MASK: iree_tokenizer_special_token_e = 6;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_COUNT: iree_tokenizer_special_token_e = 7;
pub type iree_tokenizer_special_token_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_special_token_e as iree_tokenizer_special_token_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_special_ids_t {
    pub bos: i32,
    pub eos: i32,
    pub unk: i32,
    pub pad: i32,
    pub sep: i32,
    pub cls: i32,
    pub mask: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_special_ids_t",
    ][::core::mem::size_of::<iree_tokenizer_special_ids_t>() - 28usize];
    [
        "Alignment of iree_tokenizer_special_ids_t",
    ][::core::mem::align_of::<iree_tokenizer_special_ids_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::bos",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, bos) - 0usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::eos",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, eos) - 4usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::unk",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, unk) - 8usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::pad",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, pad) - 12usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::sep",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, sep) - 16usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::cls",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, cls) - 20usize];
    [
        "Offset of field: iree_tokenizer_special_ids_t::mask",
    ][::core::mem::offset_of!(iree_tokenizer_special_ids_t, mask) - 24usize];
};
unsafe extern "C" {
    #[link_name = "iree_tokenizer_special_ids_none__extern"]
    pub fn iree_tokenizer_special_ids_none() -> iree_tokenizer_special_ids_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_vocab_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_lookup(
        vocab: *const iree_tokenizer_vocab_t,
        text: iree_string_view_t,
    ) -> i32;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_token_text(
        vocab: *const iree_tokenizer_vocab_t,
        token_id: i32,
    ) -> iree_string_view_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_token_score(
        vocab: *const iree_tokenizer_vocab_t,
        token_id: i32,
    ) -> f32;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_token_attrs(
        vocab: *const iree_tokenizer_vocab_t,
        token_id: i32,
    ) -> iree_tokenizer_token_attr_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_capacity(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_token_count(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_tokens(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> *const iree_tokenizer_token_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_special_ids(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> iree_tokenizer_special_ids_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_merge_count(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_string_table(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> iree_const_byte_span_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_merge(
        vocab: *const iree_tokenizer_vocab_t,
        rank: iree_host_size_t,
    ) -> iree_tokenizer_merge_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_max_token_length(
        vocab: *const iree_tokenizer_vocab_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_free(vocab: *mut iree_tokenizer_vocab_t);
}
pub const IREE_TOKENIZER_BPE_FLAG_NONE: iree_tokenizer_bpe_flag_bits_e = 0;
pub const IREE_TOKENIZER_BPE_FLAG_NO_BYTE_FALLBACK: iree_tokenizer_bpe_flag_bits_e = 1;
pub const IREE_TOKENIZER_BPE_FLAG_IGNORE_MERGES: iree_tokenizer_bpe_flag_bits_e = 2;
pub const IREE_TOKENIZER_BPE_FLAG_FUSE_UNK: iree_tokenizer_bpe_flag_bits_e = 4;
pub const IREE_TOKENIZER_BPE_FLAG_BYTE_LEVEL_INPUT: iree_tokenizer_bpe_flag_bits_e = 8;
pub const IREE_TOKENIZER_BPE_FLAG_ENABLE_WORD_CACHE: iree_tokenizer_bpe_flag_bits_e = 16;
pub type iree_tokenizer_bpe_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_bpe_flag_bits_e as iree_tokenizer_bpe_flag_bits_t;
pub type iree_tokenizer_bpe_flags_t = u32;
unsafe extern "C" {
    pub fn iree_tokenizer_bpe_model_allocate(
        vocab: *const iree_tokenizer_vocab_t,
        flags: iree_tokenizer_bpe_flags_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_bpe_model_set_end_of_word_suffix(
        model: *mut iree_tokenizer_model_t,
        suffix: iree_string_view_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_UNIGRAM_FLAG_NONE: iree_tokenizer_unigram_flag_bits_e = 0;
pub const IREE_TOKENIZER_UNIGRAM_FLAG_NO_BYTE_FALLBACK: iree_tokenizer_unigram_flag_bits_e = 1;
pub const IREE_TOKENIZER_UNIGRAM_FLAG_NO_FUSE_UNK: iree_tokenizer_unigram_flag_bits_e = 2;
pub type iree_tokenizer_unigram_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_unigram_flag_bits_e as iree_tokenizer_unigram_flag_bits_t;
pub type iree_tokenizer_unigram_flags_t = u32;
unsafe extern "C" {
    pub fn iree_tokenizer_unigram_model_allocate(
        vocab: *const iree_tokenizer_vocab_t,
        unk_token_id: iree_tokenizer_token_id_t,
        unk_score: f32,
        flags: iree_tokenizer_unigram_flags_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_bpe_model(
        model_json: iree_string_view_t,
        added_tokens: *const iree_tokenizer_huggingface_added_tokens_t,
        extra_flags: iree_tokenizer_bpe_flags_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
        out_vocab: *mut *mut iree_tokenizer_vocab_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_wordpiece_model(
        model_json: iree_string_view_t,
        added_tokens: *const iree_tokenizer_huggingface_added_tokens_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
        out_vocab: *mut *mut iree_tokenizer_vocab_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_unigram_model(
        model_json: iree_string_view_t,
        added_tokens: *const iree_tokenizer_huggingface_added_tokens_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
        out_vocab: *mut *mut iree_tokenizer_vocab_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_model(
        model_json: iree_string_view_t,
        added_tokens: *const iree_tokenizer_huggingface_added_tokens_t,
        pre_tokenizer_flags: iree_tokenizer_huggingface_pre_tokenizer_flags_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
        out_vocab: *mut *mut iree_tokenizer_vocab_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_estimate_vocab_capacity(
        json_size: iree_host_size_t,
    ) -> iree_host_size_t;
}
pub type iree_tokenizer_normalizer_flags_t = u32;
pub const IREE_TOKENIZER_NORMALIZER_FLAG_NONE: iree_tokenizer_normalizer_flag_bits_e = 0;
pub const IREE_TOKENIZER_NORMALIZER_FLAG_SEGMENT_END: iree_tokenizer_normalizer_flag_bits_e = 1;
pub const IREE_TOKENIZER_NORMALIZER_FLAG_FIRST_CONSUMED: iree_tokenizer_normalizer_flag_bits_e = 2;
pub type iree_tokenizer_normalizer_flag_bits_e = ::core::ffi::c_uint;
#[repr(C)]
pub struct iree_tokenizer_normalizer_t {
    pub vtable: *const iree_tokenizer_normalizer_vtable_t,
    pub state_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_normalizer_t",
    ][::core::mem::size_of::<iree_tokenizer_normalizer_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_normalizer_t",
    ][::core::mem::align_of::<iree_tokenizer_normalizer_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_normalizer_t::vtable",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_t, vtable) - 0usize];
    [
        "Offset of field: iree_tokenizer_normalizer_t::state_size",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_t, state_size) - 8usize];
};
impl Default for iree_tokenizer_normalizer_t {
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
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_normalizer_state_t {
    pub normalizer: *const iree_tokenizer_normalizer_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_normalizer_state_t",
    ][::core::mem::size_of::<iree_tokenizer_normalizer_state_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_normalizer_state_t",
    ][::core::mem::align_of::<iree_tokenizer_normalizer_state_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_normalizer_state_t::normalizer",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_state_t, normalizer) - 0usize];
};
impl Default for iree_tokenizer_normalizer_state_t {
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
pub struct iree_tokenizer_normalizer_vtable_t {
    pub destroy: ::core::option::Option<
        unsafe extern "C" fn(normalizer: *mut iree_tokenizer_normalizer_t),
    >,
    pub state_initialize: ::core::option::Option<
        unsafe extern "C" fn(
            normalizer: *const iree_tokenizer_normalizer_t,
            storage: *mut ::core::ffi::c_void,
            out_state: *mut *mut iree_tokenizer_normalizer_state_t,
        ) -> iree_status_t,
    >,
    pub state_deinitialize: ::core::option::Option<
        unsafe extern "C" fn(state: *mut iree_tokenizer_normalizer_state_t),
    >,
    pub state_process: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_normalizer_state_t,
            input: iree_string_view_t,
            output: iree_mutable_string_view_t,
            flags: iree_tokenizer_normalizer_flags_t,
            out_consumed: *mut iree_host_size_t,
            out_written: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_finalize: ::core::option::Option<
        unsafe extern "C" fn(
            state: *mut iree_tokenizer_normalizer_state_t,
            output: iree_mutable_string_view_t,
            out_written: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub state_has_pending: ::core::option::Option<
        unsafe extern "C" fn(state: *const iree_tokenizer_normalizer_state_t) -> bool,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_normalizer_vtable_t",
    ][::core::mem::size_of::<iree_tokenizer_normalizer_vtable_t>() - 48usize];
    [
        "Alignment of iree_tokenizer_normalizer_vtable_t",
    ][::core::mem::align_of::<iree_tokenizer_normalizer_vtable_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_normalizer_vtable_t::destroy",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_vtable_t, destroy) - 0usize];
    [
        "Offset of field: iree_tokenizer_normalizer_vtable_t::state_initialize",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_vtable_t, state_initialize)
        - 8usize];
    [
        "Offset of field: iree_tokenizer_normalizer_vtable_t::state_deinitialize",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_vtable_t, state_deinitialize)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_normalizer_vtable_t::state_process",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_vtable_t, state_process)
        - 24usize];
    [
        "Offset of field: iree_tokenizer_normalizer_vtable_t::state_finalize",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_vtable_t, state_finalize)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_normalizer_vtable_t::state_has_pending",
    ][::core::mem::offset_of!(iree_tokenizer_normalizer_vtable_t, state_has_pending)
        - 40usize];
};
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_initialize(
        normalizer: *mut iree_tokenizer_normalizer_t,
        vtable: *const iree_tokenizer_normalizer_vtable_t,
        state_size: iree_host_size_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_free(normalizer: *mut iree_tokenizer_normalizer_t);
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_normalizer_state_size__extern"]
    pub fn iree_tokenizer_normalizer_state_size(
        normalizer: *const iree_tokenizer_normalizer_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_normalizer_state_initialize__extern"]
    pub fn iree_tokenizer_normalizer_state_initialize(
        normalizer: *const iree_tokenizer_normalizer_t,
        storage: *mut ::core::ffi::c_void,
        out_state: *mut *mut iree_tokenizer_normalizer_state_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_normalizer_state_deinitialize__extern"]
    pub fn iree_tokenizer_normalizer_state_deinitialize(
        state: *mut iree_tokenizer_normalizer_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_normalizer_state_process__extern"]
    pub fn iree_tokenizer_normalizer_state_process(
        state: *mut iree_tokenizer_normalizer_state_t,
        input: iree_string_view_t,
        output: iree_mutable_string_view_t,
        flags: iree_tokenizer_normalizer_flags_t,
        out_consumed: *mut iree_host_size_t,
        out_written: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_normalizer_state_finalize__extern"]
    pub fn iree_tokenizer_normalizer_state_finalize(
        state: *mut iree_tokenizer_normalizer_state_t,
        output: iree_mutable_string_view_t,
        out_written: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_normalizer_state_has_pending__extern"]
    pub fn iree_tokenizer_normalizer_state_has_pending(
        state: *const iree_tokenizer_normalizer_state_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_normalizer(
        normalizer_value: iree_string_view_t,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_postprocessor_template_t {
    pub prefix_count: u8,
    pub infix_count: u8,
    pub suffix_count: u8,
    pub sequence_a_type_id: u8,
    pub sequence_b_type_id: u8,
    pub _pad: [u8; 3usize],
    pub token_ids: [iree_tokenizer_token_id_t; 7usize],
    pub type_ids: [u8; 7usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_postprocessor_template_t",
    ][::core::mem::size_of::<iree_tokenizer_postprocessor_template_t>() - 44usize];
    [
        "Alignment of iree_tokenizer_postprocessor_template_t",
    ][::core::mem::align_of::<iree_tokenizer_postprocessor_template_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::prefix_count",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_template_t, prefix_count)
        - 0usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::infix_count",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_template_t, infix_count)
        - 1usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::suffix_count",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_template_t, suffix_count)
        - 2usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::sequence_a_type_id",
    ][::core::mem::offset_of!(
        iree_tokenizer_postprocessor_template_t, sequence_a_type_id
    ) - 3usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::sequence_b_type_id",
    ][::core::mem::offset_of!(
        iree_tokenizer_postprocessor_template_t, sequence_b_type_id
    ) - 4usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::_pad",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_template_t, _pad) - 5usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::token_ids",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_template_t, token_ids)
        - 8usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_template_t::type_ids",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_template_t, type_ids)
        - 36usize];
};
unsafe extern "C" {
    #[link_name = "iree_tokenizer_postprocessor_template_total_count__extern"]
    pub fn iree_tokenizer_postprocessor_template_total_count(
        t: *const iree_tokenizer_postprocessor_template_t,
    ) -> u8;
}
pub type iree_tokenizer_postprocessor_flags_t = u32;
pub const IREE_TOKENIZER_POSTPROCESSOR_FLAG_NONE: iree_tokenizer_postprocessor_flags_e = 0;
pub const IREE_TOKENIZER_POSTPROCESSOR_FLAG_TRIM_OFFSETS: iree_tokenizer_postprocessor_flags_e = 1;
pub const IREE_TOKENIZER_POSTPROCESSOR_FLAG_ADD_PREFIX_SPACE: iree_tokenizer_postprocessor_flags_e = 2;
pub type iree_tokenizer_postprocessor_flags_e = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_postprocessor_t {
    pub single: iree_tokenizer_postprocessor_template_t,
    pub has_pair: bool,
    pub pair: iree_tokenizer_postprocessor_template_t,
    pub flags: iree_tokenizer_postprocessor_flags_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_postprocessor_t",
    ][::core::mem::size_of::<iree_tokenizer_postprocessor_t>() - 96usize];
    [
        "Alignment of iree_tokenizer_postprocessor_t",
    ][::core::mem::align_of::<iree_tokenizer_postprocessor_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_t::single",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_t, single) - 0usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_t::has_pair",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_t, has_pair) - 44usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_t::pair",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_t, pair) - 48usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_t, flags) - 92usize];
};
unsafe extern "C" {
    #[link_name = "iree_tokenizer_postprocessor_supports_pair__extern"]
    pub fn iree_tokenizer_postprocessor_supports_pair(
        postprocessor: *const iree_tokenizer_postprocessor_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_initialize(
        single_template: *const iree_tokenizer_postprocessor_template_t,
        pair_template: *const iree_tokenizer_postprocessor_template_t,
        flags: iree_tokenizer_postprocessor_flags_t,
        out_postprocessor: *mut iree_tokenizer_postprocessor_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_deinitialize(
        postprocessor: *mut iree_tokenizer_postprocessor_t,
    );
}
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_IDLE: iree_tokenizer_postprocessor_phase_t = 0;
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_PREFIX: iree_tokenizer_postprocessor_phase_t = 1;
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_SEQUENCE_A: iree_tokenizer_postprocessor_phase_t = 2;
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_INFIX: iree_tokenizer_postprocessor_phase_t = 3;
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_SEQUENCE_B: iree_tokenizer_postprocessor_phase_t = 4;
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_SUFFIX: iree_tokenizer_postprocessor_phase_t = 5;
pub const IREE_TOKENIZER_POSTPROCESSOR_PHASE_DONE: iree_tokenizer_postprocessor_phase_t = 6;
pub type iree_tokenizer_postprocessor_phase_t = ::core::ffi::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_postprocessor_encode_state_t {
    pub active_template: *const iree_tokenizer_postprocessor_template_t,
    pub phase: iree_tokenizer_postprocessor_phase_t,
    pub position: u8,
    pub flags: iree_tokenizer_postprocessor_flags_t,
    pub first_model_token_trimmed: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_postprocessor_encode_state_t",
    ][::core::mem::size_of::<iree_tokenizer_postprocessor_encode_state_t>() - 24usize];
    [
        "Alignment of iree_tokenizer_postprocessor_encode_state_t",
    ][::core::mem::align_of::<iree_tokenizer_postprocessor_encode_state_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_encode_state_t::active_template",
    ][::core::mem::offset_of!(
        iree_tokenizer_postprocessor_encode_state_t, active_template
    ) - 0usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_encode_state_t::phase",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_encode_state_t, phase)
        - 8usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_encode_state_t::position",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_encode_state_t, position)
        - 12usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_encode_state_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_postprocessor_encode_state_t, flags)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_postprocessor_encode_state_t::first_model_token_trimmed",
    ][::core::mem::offset_of!(
        iree_tokenizer_postprocessor_encode_state_t, first_model_token_trimmed
    ) - 20usize];
};
impl Default for iree_tokenizer_postprocessor_encode_state_t {
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
    #[link_name = "iree_tokenizer_postprocessor_encode_state_initialize__extern"]
    pub fn iree_tokenizer_postprocessor_encode_state_initialize(
        postprocessor: *const iree_tokenizer_postprocessor_t,
        active_template: *const iree_tokenizer_postprocessor_template_t,
        out_state: *mut iree_tokenizer_postprocessor_encode_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_postprocessor_encode_state_has_pending__extern"]
    pub fn iree_tokenizer_postprocessor_encode_state_has_pending(
        state: *const iree_tokenizer_postprocessor_encode_state_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_emit_prefix(
        state: *mut iree_tokenizer_postprocessor_encode_state_t,
        output: iree_tokenizer_token_output_t,
        output_offset: iree_host_size_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_emit_infix(
        state: *mut iree_tokenizer_postprocessor_encode_state_t,
        output: iree_tokenizer_token_output_t,
        output_offset: iree_host_size_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_assign_type_ids(
        state: *const iree_tokenizer_postprocessor_encode_state_t,
        output: iree_tokenizer_token_output_t,
        offset: iree_host_size_t,
        count: iree_host_size_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_postprocessor_begin_infix__extern"]
    pub fn iree_tokenizer_postprocessor_begin_infix(
        state: *mut iree_tokenizer_postprocessor_encode_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_postprocessor_begin_suffix__extern"]
    pub fn iree_tokenizer_postprocessor_begin_suffix(
        state: *mut iree_tokenizer_postprocessor_encode_state_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_emit_suffix(
        state: *mut iree_tokenizer_postprocessor_encode_state_t,
        output: iree_tokenizer_token_output_t,
        output_offset: iree_host_size_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_postprocessor_trim_token_offsets(
        state: *mut iree_tokenizer_postprocessor_encode_state_t,
        vocab: *const iree_tokenizer_vocab_t,
        output: iree_tokenizer_token_output_t,
        model_token_start: iree_host_size_t,
        model_token_count: iree_host_size_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_postprocessor(
        postprocessor_value: iree_string_view_t,
        out_postprocessor: *mut iree_tokenizer_postprocessor_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_huggingface_parse_segmenter(
        pre_tokenizer_value: iree_string_view_t,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
        out_flags: *mut iree_tokenizer_huggingface_pre_tokenizer_flags_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_SPECIAL_TOKEN_FLAG_NONE: iree_tokenizer_special_token_flag_bits_e = 0;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_FLAG_LSTRIP: iree_tokenizer_special_token_flag_bits_e = 1;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_FLAG_RSTRIP: iree_tokenizer_special_token_flag_bits_e = 2;
pub const IREE_TOKENIZER_SPECIAL_TOKEN_FLAG_SINGLE_WORD: iree_tokenizer_special_token_flag_bits_e = 4;
pub type iree_tokenizer_special_token_flag_bits_e = ::core::ffi::c_uint;
pub type iree_tokenizer_special_token_flags_t = u8;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_special_tokens_bucket_t {
    pub prefix: [u8; 4usize],
    pub prefix_length: u8,
    pub start: u16,
    pub end: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_special_tokens_bucket_t",
    ][::core::mem::size_of::<iree_tokenizer_special_tokens_bucket_t>() - 10usize];
    [
        "Alignment of iree_tokenizer_special_tokens_bucket_t",
    ][::core::mem::align_of::<iree_tokenizer_special_tokens_bucket_t>() - 2usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_bucket_t::prefix",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_bucket_t, prefix) - 0usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_bucket_t::prefix_length",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_bucket_t, prefix_length)
        - 4usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_bucket_t::start",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_bucket_t, start) - 6usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_bucket_t::end",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_bucket_t, end) - 8usize];
};
#[repr(C)]
pub struct iree_tokenizer_special_tokens_t {
    pub count: iree_host_size_t,
    pub max_length: iree_host_size_t,
    pub min_length: iree_host_size_t,
    pub first_byte_to_bucket: [u8; 256usize],
    pub buckets: [iree_tokenizer_special_tokens_bucket_t; 16usize],
    pub bucket_count: u8,
    pub slab: *mut ::core::ffi::c_void,
    pub ids: *mut iree_tokenizer_token_id_t,
    pub flags: *mut iree_tokenizer_special_token_flags_t,
    pub bstring_offsets: *mut u32,
    pub data: iree_byte_span_t,
    pub allocator: iree_allocator_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_special_tokens_t",
    ][::core::mem::size_of::<iree_tokenizer_special_tokens_t>() - 512usize];
    [
        "Alignment of iree_tokenizer_special_tokens_t",
    ][::core::mem::align_of::<iree_tokenizer_special_tokens_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::count",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, count) - 0usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::max_length",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, max_length) - 8usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::min_length",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, min_length) - 16usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::first_byte_to_bucket",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, first_byte_to_bucket)
        - 24usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::buckets",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, buckets) - 280usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::bucket_count",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, bucket_count) - 440usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::slab",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, slab) - 448usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::ids",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, ids) - 456usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, flags) - 464usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::bstring_offsets",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, bstring_offsets)
        - 472usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::data",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, data) - 480usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_t::allocator",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_t, allocator) - 496usize];
};
impl Default for iree_tokenizer_special_tokens_t {
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
    #[link_name = "iree_tokenizer_special_tokens_is_empty__extern"]
    pub fn iree_tokenizer_special_tokens_is_empty(
        special_tokens: *const iree_tokenizer_special_tokens_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_initialize(
        out_special_tokens: *mut iree_tokenizer_special_tokens_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_deinitialize(
        special_tokens: *mut iree_tokenizer_special_tokens_t,
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_special_tokens_builder_entry_t {
    pub id: iree_tokenizer_token_id_t,
    pub string_offset: u32,
    pub string_length: u16,
    pub flags: iree_tokenizer_special_token_flags_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_special_tokens_builder_entry_t",
    ][::core::mem::size_of::<iree_tokenizer_special_tokens_builder_entry_t>() - 12usize];
    [
        "Alignment of iree_tokenizer_special_tokens_builder_entry_t",
    ][::core::mem::align_of::<iree_tokenizer_special_tokens_builder_entry_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_entry_t::id",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_entry_t, id)
        - 0usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_entry_t::string_offset",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_builder_entry_t, string_offset
    ) - 4usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_entry_t::string_length",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_builder_entry_t, string_length
    ) - 8usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_entry_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_entry_t, flags)
        - 10usize];
};
#[repr(C)]
pub struct iree_tokenizer_special_tokens_builder_t {
    pub allocator: iree_allocator_t,
    pub entries: *mut iree_tokenizer_special_tokens_builder_entry_t,
    pub entry_count: iree_host_size_t,
    pub entry_capacity: iree_host_size_t,
    pub string_data: *mut u8,
    pub string_size: iree_host_size_t,
    pub string_capacity: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_special_tokens_builder_t",
    ][::core::mem::size_of::<iree_tokenizer_special_tokens_builder_t>() - 64usize];
    [
        "Alignment of iree_tokenizer_special_tokens_builder_t",
    ][::core::mem::align_of::<iree_tokenizer_special_tokens_builder_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::allocator",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, allocator)
        - 0usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::entries",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, entries)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::entry_count",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, entry_count)
        - 24usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::entry_capacity",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, entry_capacity)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::string_data",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, string_data)
        - 40usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::string_size",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, string_size)
        - 48usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_builder_t::string_capacity",
    ][::core::mem::offset_of!(iree_tokenizer_special_tokens_builder_t, string_capacity)
        - 56usize];
};
impl Default for iree_tokenizer_special_tokens_builder_t {
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
    pub fn iree_tokenizer_special_tokens_builder_initialize(
        allocator: iree_allocator_t,
        out_builder: *mut iree_tokenizer_special_tokens_builder_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_builder_add(
        builder: *mut iree_tokenizer_special_tokens_builder_t,
        content: iree_string_view_t,
        token_id: iree_tokenizer_token_id_t,
        flags: iree_tokenizer_special_token_flags_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_builder_build(
        builder: *mut iree_tokenizer_special_tokens_builder_t,
        allocator: iree_allocator_t,
        out_special_tokens: *mut iree_tokenizer_special_tokens_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_builder_deinitialize(
        builder: *mut iree_tokenizer_special_tokens_builder_t,
    );
}
pub const IREE_TOKENIZER_SPECIAL_TOKENS_NO_MATCH: iree_tokenizer_special_tokens_match_result_e = 0;
pub const IREE_TOKENIZER_SPECIAL_TOKENS_MATCHED: iree_tokenizer_special_tokens_match_result_e = 1;
pub const IREE_TOKENIZER_SPECIAL_TOKENS_NEED_MORE: iree_tokenizer_special_tokens_match_result_e = 2;
pub type iree_tokenizer_special_tokens_match_result_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_special_tokens_match_result_e as iree_tokenizer_special_tokens_match_result_t;
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_match(
        special_tokens: *const iree_tokenizer_special_tokens_t,
        input: iree_string_view_t,
        out_length: *mut iree_host_size_t,
        out_id: *mut iree_tokenizer_token_id_t,
        state: *mut iree_tokenizer_special_tokens_encode_state_t,
    ) -> iree_tokenizer_special_tokens_match_result_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_special_tokens_safe_prefix_length__extern"]
    pub fn iree_tokenizer_special_tokens_safe_prefix_length(
        special_tokens: *const iree_tokenizer_special_tokens_t,
        input: iree_string_view_t,
    ) -> iree_host_size_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_special_tokens_encode_state_t {
    pub match_position: u16,
    pub partial_token_index: u16,
    pub drain_position: u16,
    pub prev_byte_plus_one: u8,
    pub matched_flags: iree_tokenizer_special_token_flags_t,
    pub at_start_of_input: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_special_tokens_encode_state_t",
    ][::core::mem::size_of::<iree_tokenizer_special_tokens_encode_state_t>() - 10usize];
    [
        "Alignment of iree_tokenizer_special_tokens_encode_state_t",
    ][::core::mem::align_of::<iree_tokenizer_special_tokens_encode_state_t>() - 2usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_encode_state_t::match_position",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_encode_state_t, match_position
    ) - 0usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_encode_state_t::partial_token_index",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_encode_state_t, partial_token_index
    ) - 2usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_encode_state_t::drain_position",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_encode_state_t, drain_position
    ) - 4usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_encode_state_t::prev_byte_plus_one",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_encode_state_t, prev_byte_plus_one
    ) - 6usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_encode_state_t::matched_flags",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_encode_state_t, matched_flags
    ) - 7usize];
    [
        "Offset of field: iree_tokenizer_special_tokens_encode_state_t::at_start_of_input",
    ][::core::mem::offset_of!(
        iree_tokenizer_special_tokens_encode_state_t, at_start_of_input
    ) - 8usize];
};
unsafe extern "C" {
    #[link_name = "iree_tokenizer_special_tokens_encode_state_initialize__extern"]
    pub fn iree_tokenizer_special_tokens_encode_state_initialize(
        out_state: *mut iree_tokenizer_special_tokens_encode_state_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_special_tokens_encode_state_has_partial__extern"]
    pub fn iree_tokenizer_special_tokens_encode_state_has_partial(
        state: *const iree_tokenizer_special_tokens_encode_state_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_special_tokens_encode_state_update_prev_byte__extern"]
    pub fn iree_tokenizer_special_tokens_encode_state_update_prev_byte(
        state: *mut iree_tokenizer_special_tokens_encode_state_t,
        byte: u8,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_is_word_boundary_byte__extern"]
    pub fn iree_tokenizer_is_word_boundary_byte(byte: u8) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_special_tokens_encode_state_get_partial(
        state: *const iree_tokenizer_special_tokens_encode_state_t,
        special_tokens: *const iree_tokenizer_special_tokens_t,
        out_buffer: *mut u8,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_special_tokens_encode_state_clear_partial__extern"]
    pub fn iree_tokenizer_special_tokens_encode_state_clear_partial(
        state: *mut iree_tokenizer_special_tokens_encode_state_t,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_encode_state_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_decode_state_t {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct iree_tokenizer_offset_run_t {
    pub transform_position: iree_host_size_t,
    pub original_offset: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_offset_run_t",
    ][::core::mem::size_of::<iree_tokenizer_offset_run_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_offset_run_t",
    ][::core::mem::align_of::<iree_tokenizer_offset_run_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_offset_run_t::transform_position",
    ][::core::mem::offset_of!(iree_tokenizer_offset_run_t, transform_position) - 0usize];
    [
        "Offset of field: iree_tokenizer_offset_run_t::original_offset",
    ][::core::mem::offset_of!(iree_tokenizer_offset_run_t, original_offset) - 8usize];
};
impl Default for iree_tokenizer_offset_run_t {
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
pub struct iree_tokenizer_offset_run_list_t {
    pub capacity: iree_host_size_t,
    pub values: *mut iree_tokenizer_offset_run_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_offset_run_list_t",
    ][::core::mem::size_of::<iree_tokenizer_offset_run_list_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_offset_run_list_t",
    ][::core::mem::align_of::<iree_tokenizer_offset_run_list_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_offset_run_list_t::capacity",
    ][::core::mem::offset_of!(iree_tokenizer_offset_run_list_t, capacity) - 0usize];
    [
        "Offset of field: iree_tokenizer_offset_run_list_t::values",
    ][::core::mem::offset_of!(iree_tokenizer_offset_run_list_t, values) - 8usize];
};
impl Default for iree_tokenizer_offset_run_list_t {
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
    #[link_name = "iree_tokenizer_offset_run_list_empty__extern"]
    pub fn iree_tokenizer_offset_run_list_empty() -> iree_tokenizer_offset_run_list_t;
}
pub const IREE_TOKENIZER_ENCODE_FLAG_NONE: iree_tokenizer_encode_flag_bits_e = 0;
pub const IREE_TOKENIZER_ENCODE_FLAG_AT_INPUT_START: iree_tokenizer_encode_flag_bits_e = 1;
pub const IREE_TOKENIZER_ENCODE_FLAG_TRACK_OFFSETS: iree_tokenizer_encode_flag_bits_e = 2;
pub const IREE_TOKENIZER_ENCODE_FLAG_ADD_SPECIAL_TOKENS: iree_tokenizer_encode_flag_bits_e = 4;
pub const IREE_TOKENIZER_ENCODE_FLAG_NO_SPECIAL_TOKEN_MATCHING: iree_tokenizer_encode_flag_bits_e = 8;
pub type iree_tokenizer_encode_flag_bits_e = ::core::ffi::c_uint;
pub type iree_tokenizer_encode_flags_t = u32;
pub const IREE_TOKENIZER_DECODE_FLAG_NONE: iree_tokenizer_decode_flag_bits_e = 0;
pub const IREE_TOKENIZER_DECODE_FLAG_SKIP_SPECIAL_TOKENS: iree_tokenizer_decode_flag_bits_e = 1;
pub type iree_tokenizer_decode_flag_bits_e = ::core::ffi::c_uint;
pub type iree_tokenizer_decode_flags_t = u32;
#[repr(C)]
pub struct iree_tokenizer_builder_t {
    pub allocator: iree_allocator_t,
    pub normalizer: *mut iree_tokenizer_normalizer_t,
    pub segmenter: *mut iree_tokenizer_segmenter_t,
    pub model: *mut iree_tokenizer_model_t,
    pub decoder: *mut iree_tokenizer_decoder_t,
    pub postprocessor: iree_tokenizer_postprocessor_t,
    pub special_tokens: iree_tokenizer_special_tokens_t,
    pub special_tokens_post_norm: iree_tokenizer_special_tokens_t,
    pub vocab: *mut iree_tokenizer_vocab_t,
    pub string_batch_size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_builder_t",
    ][::core::mem::size_of::<iree_tokenizer_builder_t>() - 1184usize];
    [
        "Alignment of iree_tokenizer_builder_t",
    ][::core::mem::align_of::<iree_tokenizer_builder_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_builder_t::allocator",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, allocator) - 0usize];
    [
        "Offset of field: iree_tokenizer_builder_t::normalizer",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, normalizer) - 16usize];
    [
        "Offset of field: iree_tokenizer_builder_t::segmenter",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, segmenter) - 24usize];
    [
        "Offset of field: iree_tokenizer_builder_t::model",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, model) - 32usize];
    [
        "Offset of field: iree_tokenizer_builder_t::decoder",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, decoder) - 40usize];
    [
        "Offset of field: iree_tokenizer_builder_t::postprocessor",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, postprocessor) - 48usize];
    [
        "Offset of field: iree_tokenizer_builder_t::special_tokens",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, special_tokens) - 144usize];
    [
        "Offset of field: iree_tokenizer_builder_t::special_tokens_post_norm",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, special_tokens_post_norm)
        - 656usize];
    [
        "Offset of field: iree_tokenizer_builder_t::vocab",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, vocab) - 1168usize];
    [
        "Offset of field: iree_tokenizer_builder_t::string_batch_size",
    ][::core::mem::offset_of!(iree_tokenizer_builder_t, string_batch_size) - 1176usize];
};
impl Default for iree_tokenizer_builder_t {
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
    pub fn iree_tokenizer_builder_initialize(
        allocator: iree_allocator_t,
        out_builder: *mut iree_tokenizer_builder_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_deinitialize(builder: *mut iree_tokenizer_builder_t);
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_normalizer(
        builder: *mut iree_tokenizer_builder_t,
        normalizer: *mut iree_tokenizer_normalizer_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_segmenter(
        builder: *mut iree_tokenizer_builder_t,
        segmenter: *mut iree_tokenizer_segmenter_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_model(
        builder: *mut iree_tokenizer_builder_t,
        model: *mut iree_tokenizer_model_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_decoder(
        builder: *mut iree_tokenizer_builder_t,
        decoder: *mut iree_tokenizer_decoder_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_postprocessor(
        builder: *mut iree_tokenizer_builder_t,
        postprocessor: iree_tokenizer_postprocessor_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_special_tokens(
        builder: *mut iree_tokenizer_builder_t,
        special_tokens: *mut iree_tokenizer_special_tokens_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_special_tokens_post_norm(
        builder: *mut iree_tokenizer_builder_t,
        special_tokens: *mut iree_tokenizer_special_tokens_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_vocab(
        builder: *mut iree_tokenizer_builder_t,
        vocab: *mut iree_tokenizer_vocab_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_set_string_batch_size(
        builder: *mut iree_tokenizer_builder_t,
        batch_size: iree_host_size_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_builder_build(
        builder: *mut iree_tokenizer_builder_t,
        out_tokenizer: *mut *mut iree_tokenizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_free(tokenizer: *mut iree_tokenizer_t);
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab(
        tokenizer: *const iree_tokenizer_t,
    ) -> *const iree_tokenizer_vocab_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_max_special_token_count(
        tokenizer: *const iree_tokenizer_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_model_type_name(
        tokenizer: *const iree_tokenizer_t,
    ) -> iree_string_view_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode(
        tokenizer: *const iree_tokenizer_t,
        text: iree_string_view_t,
        flags: iree_tokenizer_encode_flags_t,
        output: iree_tokenizer_token_output_t,
        allocator: iree_allocator_t,
        out_token_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decode(
        tokenizer: *const iree_tokenizer_t,
        tokens: iree_tokenizer_token_id_list_t,
        flags: iree_tokenizer_decode_flags_t,
        text_output: iree_mutable_string_view_t,
        allocator: iree_allocator_t,
        out_text_length: *mut iree_host_size_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_ENCODE_BATCH_ITEM_FLAG_NONE: iree_tokenizer_encode_batch_item_flag_bits_e = 0;
pub const IREE_TOKENIZER_ENCODE_BATCH_ITEM_FLAG_HAS_TEXT_PAIR: iree_tokenizer_encode_batch_item_flag_bits_e = 1;
pub type iree_tokenizer_encode_batch_item_flag_bits_e = ::core::ffi::c_uint;
pub type iree_tokenizer_encode_batch_item_flags_t = u32;
#[repr(C)]
pub struct iree_tokenizer_encode_batch_item_t {
    pub text: iree_string_view_t,
    pub text_pair: iree_string_view_t,
    pub flags: iree_tokenizer_encode_batch_item_flags_t,
    pub output: iree_tokenizer_token_output_t,
    pub out_token_count: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_encode_batch_item_t",
    ][::core::mem::size_of::<iree_tokenizer_encode_batch_item_t>() - 80usize];
    [
        "Alignment of iree_tokenizer_encode_batch_item_t",
    ][::core::mem::align_of::<iree_tokenizer_encode_batch_item_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_encode_batch_item_t::text",
    ][::core::mem::offset_of!(iree_tokenizer_encode_batch_item_t, text) - 0usize];
    [
        "Offset of field: iree_tokenizer_encode_batch_item_t::text_pair",
    ][::core::mem::offset_of!(iree_tokenizer_encode_batch_item_t, text_pair) - 16usize];
    [
        "Offset of field: iree_tokenizer_encode_batch_item_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_encode_batch_item_t, flags) - 32usize];
    [
        "Offset of field: iree_tokenizer_encode_batch_item_t::output",
    ][::core::mem::offset_of!(iree_tokenizer_encode_batch_item_t, output) - 40usize];
    [
        "Offset of field: iree_tokenizer_encode_batch_item_t::out_token_count",
    ][::core::mem::offset_of!(iree_tokenizer_encode_batch_item_t, out_token_count)
        - 72usize];
};
impl Default for iree_tokenizer_encode_batch_item_t {
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
    pub fn iree_tokenizer_encode_batch(
        tokenizer: *const iree_tokenizer_t,
        items: *mut iree_tokenizer_encode_batch_item_t,
        item_count: iree_host_size_t,
        flags: iree_tokenizer_encode_flags_t,
        state_storage: iree_byte_span_t,
        transform_buffer: iree_byte_span_t,
        offset_runs: iree_tokenizer_offset_run_list_t,
    ) -> iree_status_t;
}
#[repr(C)]
pub struct iree_tokenizer_decode_batch_item_t {
    pub tokens: iree_tokenizer_token_id_list_t,
    pub text_output: iree_mutable_string_view_t,
    pub out_text_length: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_decode_batch_item_t",
    ][::core::mem::size_of::<iree_tokenizer_decode_batch_item_t>() - 40usize];
    [
        "Alignment of iree_tokenizer_decode_batch_item_t",
    ][::core::mem::align_of::<iree_tokenizer_decode_batch_item_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_decode_batch_item_t::tokens",
    ][::core::mem::offset_of!(iree_tokenizer_decode_batch_item_t, tokens) - 0usize];
    [
        "Offset of field: iree_tokenizer_decode_batch_item_t::text_output",
    ][::core::mem::offset_of!(iree_tokenizer_decode_batch_item_t, text_output)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_decode_batch_item_t::out_text_length",
    ][::core::mem::offset_of!(iree_tokenizer_decode_batch_item_t, out_text_length)
        - 32usize];
};
impl Default for iree_tokenizer_decode_batch_item_t {
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
    pub fn iree_tokenizer_decode_batch(
        tokenizer: *const iree_tokenizer_t,
        items: *mut iree_tokenizer_decode_batch_item_t,
        item_count: iree_host_size_t,
        flags: iree_tokenizer_decode_flags_t,
        state_storage: iree_byte_span_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_calculate_size(
        tokenizer: *const iree_tokenizer_t,
        out_size: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_transform_buffer_recommended_size__extern"]
    pub fn iree_tokenizer_transform_buffer_recommended_size(
        text_size: iree_host_size_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_transform_buffer_oneshot_size__extern"]
    pub fn iree_tokenizer_transform_buffer_oneshot_size(
        text_size: iree_host_size_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_initialize(
        tokenizer: *const iree_tokenizer_t,
        state_storage: iree_byte_span_t,
        transform_buffer: iree_byte_span_t,
        offset_runs: iree_tokenizer_offset_run_list_t,
        flags: iree_tokenizer_encode_flags_t,
        out_state: *mut *mut iree_tokenizer_encode_state_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_deinitialize(
        state: *mut iree_tokenizer_encode_state_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_reset(
        state: *mut iree_tokenizer_encode_state_t,
        flags: iree_tokenizer_encode_flags_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_has_pending(
        state: *const iree_tokenizer_encode_state_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_pending_token_bound(
        state: *const iree_tokenizer_encode_state_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_feed(
        state: *mut iree_tokenizer_encode_state_t,
        chunk: iree_string_view_t,
        output: iree_tokenizer_token_output_t,
        out_bytes_consumed: *mut iree_host_size_t,
        out_token_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_encode_state_finalize(
        state: *mut iree_tokenizer_encode_state_t,
        output: iree_tokenizer_token_output_t,
        out_token_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decode_state_calculate_size(
        tokenizer: *const iree_tokenizer_t,
        out_size: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decode_state_initialize(
        tokenizer: *const iree_tokenizer_t,
        flags: iree_tokenizer_decode_flags_t,
        state_storage: iree_byte_span_t,
        out_state: *mut *mut iree_tokenizer_decode_state_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decode_state_deinitialize(
        state: *mut iree_tokenizer_decode_state_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_decode_state_feed(
        state: *mut iree_tokenizer_decode_state_t,
        tokens: iree_tokenizer_token_id_list_t,
        text_output: iree_mutable_string_view_t,
        out_tokens_consumed: *mut iree_host_size_t,
        out_text_length: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_decode_state_finalize(
        state: *mut iree_tokenizer_decode_state_t,
        text_output: iree_mutable_string_view_t,
        out_text_length: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_from_huggingface_json(
        json: iree_string_view_t,
        allocator: iree_allocator_t,
        out_tokenizer: *mut *mut iree_tokenizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_parse_huggingface_json(
        json: iree_string_view_t,
        builder: *mut iree_tokenizer_builder_t,
    ) -> iree_status_t;
}
#[repr(C)]
pub struct iree_tokenizer_tiktoken_config_t {
    pub pattern: iree_string_view_t,
    pub special_token_count: iree_host_size_t,
    pub special_token_strings: *const iree_string_view_t,
    pub special_token_ids: *const i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_tiktoken_config_t",
    ][::core::mem::size_of::<iree_tokenizer_tiktoken_config_t>() - 40usize];
    [
        "Alignment of iree_tokenizer_tiktoken_config_t",
    ][::core::mem::align_of::<iree_tokenizer_tiktoken_config_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_tiktoken_config_t::pattern",
    ][::core::mem::offset_of!(iree_tokenizer_tiktoken_config_t, pattern) - 0usize];
    [
        "Offset of field: iree_tokenizer_tiktoken_config_t::special_token_count",
    ][::core::mem::offset_of!(iree_tokenizer_tiktoken_config_t, special_token_count)
        - 16usize];
    [
        "Offset of field: iree_tokenizer_tiktoken_config_t::special_token_strings",
    ][::core::mem::offset_of!(iree_tokenizer_tiktoken_config_t, special_token_strings)
        - 24usize];
    [
        "Offset of field: iree_tokenizer_tiktoken_config_t::special_token_ids",
    ][::core::mem::offset_of!(iree_tokenizer_tiktoken_config_t, special_token_ids)
        - 32usize];
};
impl Default for iree_tokenizer_tiktoken_config_t {
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
    pub fn iree_tokenizer_tiktoken_config_cl100k_base() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_o200k_base() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_o200k_harmony() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_r50k_base() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_gpt2() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_p50k_base() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_p50k_edit() -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_tiktoken_config_by_name(
        name: iree_string_view_t,
    ) -> *const iree_tokenizer_tiktoken_config_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_from_tiktoken(
        data: iree_string_view_t,
        config: *const iree_tokenizer_tiktoken_config_t,
        allocator: iree_allocator_t,
        out_tokenizer: *mut *mut iree_tokenizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_parse_tiktoken(
        data: iree_string_view_t,
        config: *const iree_tokenizer_tiktoken_config_t,
        builder: *mut iree_tokenizer_builder_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_WORDPIECE_FLAG_NONE: iree_tokenizer_wordpiece_flag_bits_e = 0;
pub type iree_tokenizer_wordpiece_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_wordpiece_flag_bits_e as iree_tokenizer_wordpiece_flag_bits_t;
pub type iree_tokenizer_wordpiece_flags_t = u32;
unsafe extern "C" {
    pub fn iree_tokenizer_wordpiece_model_allocate(
        vocab: *const iree_tokenizer_vocab_t,
        continuing_subword_prefix: iree_string_view_t,
        max_input_chars_per_word: iree_host_size_t,
        flags: iree_tokenizer_wordpiece_flags_t,
        allocator: iree_allocator_t,
        out_model: *mut *mut iree_tokenizer_model_t,
    ) -> iree_status_t;
}
pub type iree_tokenizer_bert_normalizer_flags_t = u32;
pub const IREE_TOKENIZER_BERT_NORMALIZER_FLAG_NONE: iree_tokenizer_bert_normalizer_flag_bits_e = 0;
pub const IREE_TOKENIZER_BERT_NORMALIZER_FLAG_CLEAN_TEXT: iree_tokenizer_bert_normalizer_flag_bits_e = 1;
pub const IREE_TOKENIZER_BERT_NORMALIZER_FLAG_HANDLE_CHINESE_CHARS: iree_tokenizer_bert_normalizer_flag_bits_e = 2;
pub const IREE_TOKENIZER_BERT_NORMALIZER_FLAG_STRIP_ACCENTS: iree_tokenizer_bert_normalizer_flag_bits_e = 4;
pub const IREE_TOKENIZER_BERT_NORMALIZER_FLAG_LOWERCASE: iree_tokenizer_bert_normalizer_flag_bits_e = 8;
pub type iree_tokenizer_bert_normalizer_flag_bits_e = ::core::ffi::c_uint;
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_bert_allocate(
        flags: iree_tokenizer_bert_normalizer_flags_t,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_deferred_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_lowercase_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_nfc_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_nfd_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_nfkd_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_passthrough_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_precompiled_normalizer_allocate(
        charsmap: iree_const_byte_span_t,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_prepend_allocate(
        prepend_string: iree_string_view_t,
        skip_if_prefix_matches: bool,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_regex_replace_allocate(
        pattern: iree_string_view_t,
        content: iree_string_view_t,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_replace_allocate(
        pattern: iree_string_view_t,
        content: iree_string_view_t,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_sequence_allocate(
        children: *const *mut iree_tokenizer_normalizer_t,
        child_count: iree_host_size_t,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_strip_allocate(
        strip_left: bool,
        strip_right: bool,
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_normalizer_strip_accents_allocate(
        allocator: iree_allocator_t,
        out_normalizer: *mut *mut iree_tokenizer_normalizer_t,
    ) -> iree_status_t;
}
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_LETTER: iree_tokenizer_regex_pseudo_byte_e = 128;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_NUMBER: iree_tokenizer_regex_pseudo_byte_e = 129;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_PUNCT: iree_tokenizer_regex_pseudo_byte_e = 130;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_MARK: iree_tokenizer_regex_pseudo_byte_e = 131;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_SYMBOL: iree_tokenizer_regex_pseudo_byte_e = 132;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_SEPARATOR: iree_tokenizer_regex_pseudo_byte_e = 133;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_OTHER: iree_tokenizer_regex_pseudo_byte_e = 134;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_WHITESPACE: iree_tokenizer_regex_pseudo_byte_e = 135;
pub const IREE_TOKENIZER_UTIL_REGEX_PSEUDO_COUNT: iree_tokenizer_regex_pseudo_byte_e = 8;
pub type iree_tokenizer_regex_pseudo_byte_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_regex_pseudo_byte_e as iree_tokenizer_regex_pseudo_byte_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_regex_codepoint_range_t {
    pub start: u32,
    pub end: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_codepoint_range_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_codepoint_range_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_regex_codepoint_range_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_codepoint_range_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_regex_codepoint_range_t::start",
    ][::core::mem::offset_of!(iree_tokenizer_regex_codepoint_range_t, start) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_codepoint_range_t::end",
    ][::core::mem::offset_of!(iree_tokenizer_regex_codepoint_range_t, end) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_regex_dfa_range_transition_t {
    pub from_state: u16,
    pub target_state: u16,
    pub start: u32,
    pub end: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_dfa_range_transition_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_dfa_range_transition_t>() - 12usize];
    [
        "Alignment of iree_tokenizer_regex_dfa_range_transition_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_dfa_range_transition_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_range_transition_t::from_state",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_range_transition_t, from_state)
        - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_range_transition_t::target_state",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_range_transition_t, target_state)
        - 2usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_range_transition_t::start",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_range_transition_t, start)
        - 4usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_range_transition_t::end",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_range_transition_t, end)
        - 8usize];
};
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_NONE: iree_tokenizer_regex_dfa_flag_bits_e = 0;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_HAS_LOOKAHEAD: iree_tokenizer_regex_dfa_flag_bits_e = 1;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_UNICODE: iree_tokenizer_regex_dfa_flag_bits_e = 2;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_CASE_INSENSITIVE: iree_tokenizer_regex_dfa_flag_bits_e = 4;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_HAS_ANCHORS: iree_tokenizer_regex_dfa_flag_bits_e = 8;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_HAS_BRANCHES: iree_tokenizer_regex_dfa_flag_bits_e = 16;
pub const IREE_TOKENIZER_UTIL_REGEX_DFA_FLAG_HAS_RANGES: iree_tokenizer_regex_dfa_flag_bits_e = 32;
pub type iree_tokenizer_regex_dfa_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_regex_dfa_flag_bits_e as iree_tokenizer_regex_dfa_flag_bits_t;
pub type iree_tokenizer_regex_dfa_flags_t = u16;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_regex_dfa_header_t {
    pub magic: u32,
    pub version: u16,
    pub flags: u16,
    pub num_states: u16,
    pub num_accepting: u16,
    pub start_state: u16,
    pub unanchored_start_state: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_dfa_header_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_dfa_header_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_regex_dfa_header_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_dfa_header_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::magic",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, magic) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::version",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, version) - 4usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, flags) - 6usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::num_states",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, num_states) - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::num_accepting",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, num_accepting)
        - 10usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::start_state",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, start_state) - 12usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_header_t::unanchored_start_state",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_header_t, unanchored_start_state)
        - 14usize];
};
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NONE: iree_tokenizer_regex_lookahead_type_e = 0;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NEG_CHAR: iree_tokenizer_regex_lookahead_type_e = 1;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NEG_SHORTHAND: iree_tokenizer_regex_lookahead_type_e = 2;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NEG_CLASS: iree_tokenizer_regex_lookahead_type_e = 3;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NEG_CHAR_WITH_FALLBACK: iree_tokenizer_regex_lookahead_type_e = 4;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NEG_SHORTHAND_WITH_FALLBACK: iree_tokenizer_regex_lookahead_type_e = 5;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_NEG_CLASS_WITH_FALLBACK: iree_tokenizer_regex_lookahead_type_e = 6;
pub type iree_tokenizer_regex_lookahead_type_e = ::core::ffi::c_uint;
pub type iree_tokenizer_regex_lookahead_type_t = u8;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_FLAG_NONE: iree_tokenizer_regex_lookahead_flag_bits_e = 0;
pub const IREE_TOKENIZER_UTIL_REGEX_LOOKAHEAD_FLAG_HAS_EARLY_NO_LOOKAHEAD: iree_tokenizer_regex_lookahead_flag_bits_e = 1;
pub type iree_tokenizer_regex_lookahead_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_regex_lookahead_flag_bits_e as iree_tokenizer_regex_lookahead_flag_bits_t;
pub type iree_tokenizer_regex_lookahead_flags_t = u8;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_regex_lookahead_t {
    pub type_: iree_tokenizer_regex_lookahead_type_t,
    pub data: u8,
    pub flags: iree_tokenizer_regex_lookahead_flags_t,
    pub reserved: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_lookahead_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_lookahead_t>() - 4usize];
    [
        "Alignment of iree_tokenizer_regex_lookahead_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_lookahead_t>() - 1usize];
    [
        "Offset of field: iree_tokenizer_regex_lookahead_t::type_",
    ][::core::mem::offset_of!(iree_tokenizer_regex_lookahead_t, type_) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_lookahead_t::data",
    ][::core::mem::offset_of!(iree_tokenizer_regex_lookahead_t, data) - 1usize];
    [
        "Offset of field: iree_tokenizer_regex_lookahead_t::flags",
    ][::core::mem::offset_of!(iree_tokenizer_regex_lookahead_t, flags) - 2usize];
    [
        "Offset of field: iree_tokenizer_regex_lookahead_t::reserved",
    ][::core::mem::offset_of!(iree_tokenizer_regex_lookahead_t, reserved) - 3usize];
};
pub const IREE_TOKENIZER_UTIL_REGEX_SHORTHAND_s: iree_tokenizer_regex_shorthand_e = 0;
pub const IREE_TOKENIZER_UTIL_REGEX_SHORTHAND_S: iree_tokenizer_regex_shorthand_e = 1;
pub const IREE_TOKENIZER_UTIL_REGEX_SHORTHAND_d: iree_tokenizer_regex_shorthand_e = 2;
pub const IREE_TOKENIZER_UTIL_REGEX_SHORTHAND_D: iree_tokenizer_regex_shorthand_e = 3;
pub const IREE_TOKENIZER_UTIL_REGEX_SHORTHAND_w: iree_tokenizer_regex_shorthand_e = 4;
pub const IREE_TOKENIZER_UTIL_REGEX_SHORTHAND_W: iree_tokenizer_regex_shorthand_e = 5;
pub type iree_tokenizer_regex_shorthand_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_regex_shorthand_e as iree_tokenizer_regex_shorthand_t;
#[repr(C)]
pub struct iree_tokenizer_regex_match_t {
    pub start: iree_host_size_t,
    pub end: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_match_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_match_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_regex_match_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_match_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_match_t::start",
    ][::core::mem::offset_of!(iree_tokenizer_regex_match_t, start) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_match_t::end",
    ][::core::mem::offset_of!(iree_tokenizer_regex_match_t, end) - 8usize];
};
impl Default for iree_tokenizer_regex_match_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_tokenizer_regex_match_callback_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        user_data: *mut ::core::ffi::c_void,
        match_: iree_tokenizer_regex_match_t,
    ) -> iree_status_t,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_regex_dfa_t {
    pub header: *const iree_tokenizer_regex_dfa_header_t,
    pub transitions: *const u16,
    pub accepting_bitmap: *const u64,
    pub lookahead: *const iree_tokenizer_regex_lookahead_t,
    pub start_anchor_bitmap: *const u64,
    pub end_anchor_bitmap: *const u64,
    pub alive_branches: *const u64,
    pub accepting_branches: *const u64,
    pub ranges: *const iree_tokenizer_regex_dfa_range_transition_t,
    pub num_ranges: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_dfa_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_dfa_t>() - 80usize];
    [
        "Alignment of iree_tokenizer_regex_dfa_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_dfa_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::header",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, header) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::transitions",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, transitions) - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::accepting_bitmap",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, accepting_bitmap) - 16usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::lookahead",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, lookahead) - 24usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::start_anchor_bitmap",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, start_anchor_bitmap)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::end_anchor_bitmap",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, end_anchor_bitmap) - 40usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::alive_branches",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, alive_branches) - 48usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::accepting_branches",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, accepting_branches) - 56usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::ranges",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, ranges) - 64usize];
    [
        "Offset of field: iree_tokenizer_regex_dfa_t::num_ranges",
    ][::core::mem::offset_of!(iree_tokenizer_regex_dfa_t, num_ranges) - 72usize];
};
impl Default for iree_tokenizer_regex_dfa_t {
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
pub struct iree_tokenizer_regex_exec_state_t {
    pub match_start: iree_host_size_t,
    pub last_accept: iree_host_size_t,
    pub dfa_state: u16,
    pub in_match: bool,
    pub has_accept: bool,
    pub pending_lookahead: bool,
    pub pending_lookahead_state: u16,
    pub pending_match_end: iree_host_size_t,
    pub has_accept_fallback: bool,
    pub last_accept_fallback: iree_host_size_t,
    pub best_branch_idx: u8,
    pub best_match_end: iree_host_size_t,
    pub rewind_buffer: [u8; 256usize],
    pub rewind_buffer_length: iree_host_size_t,
    pub rewind_buffer_start: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_exec_state_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_exec_state_t>() - 336usize];
    [
        "Alignment of iree_tokenizer_regex_exec_state_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_exec_state_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::match_start",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, match_start) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::last_accept",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, last_accept) - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::dfa_state",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, dfa_state) - 16usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::in_match",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, in_match) - 18usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::has_accept",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, has_accept) - 19usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::pending_lookahead",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, pending_lookahead)
        - 20usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::pending_lookahead_state",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, pending_lookahead_state)
        - 22usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::pending_match_end",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, pending_match_end)
        - 24usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::has_accept_fallback",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, has_accept_fallback)
        - 32usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::last_accept_fallback",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, last_accept_fallback)
        - 40usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::best_branch_idx",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, best_branch_idx)
        - 48usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::best_match_end",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, best_match_end)
        - 56usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::rewind_buffer",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, rewind_buffer)
        - 64usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::rewind_buffer_length",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, rewind_buffer_length)
        - 320usize];
    [
        "Offset of field: iree_tokenizer_regex_exec_state_t::rewind_buffer_start",
    ][::core::mem::offset_of!(iree_tokenizer_regex_exec_state_t, rewind_buffer_start)
        - 328usize];
};
impl Default for iree_tokenizer_regex_exec_state_t {
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
pub struct iree_tokenizer_regex_stride_t {
    pub category_table: [u8; 128usize],
    pub category_count: u8,
    pub num_states: u16,
    pub boring_bitmap_offset: iree_host_size_t,
    pub self_loop_category_offset: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_stride_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_stride_t>() - 152usize];
    [
        "Alignment of iree_tokenizer_regex_stride_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_stride_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_stride_t::category_table",
    ][::core::mem::offset_of!(iree_tokenizer_regex_stride_t, category_table) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_stride_t::category_count",
    ][::core::mem::offset_of!(iree_tokenizer_regex_stride_t, category_count) - 128usize];
    [
        "Offset of field: iree_tokenizer_regex_stride_t::num_states",
    ][::core::mem::offset_of!(iree_tokenizer_regex_stride_t, num_states) - 130usize];
    [
        "Offset of field: iree_tokenizer_regex_stride_t::boring_bitmap_offset",
    ][::core::mem::offset_of!(iree_tokenizer_regex_stride_t, boring_bitmap_offset)
        - 136usize];
    [
        "Offset of field: iree_tokenizer_regex_stride_t::self_loop_category_offset",
    ][::core::mem::offset_of!(iree_tokenizer_regex_stride_t, self_loop_category_offset)
        - 144usize];
};
impl Default for iree_tokenizer_regex_stride_t {
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
    #[link_name = "iree_tokenizer_regex_stride_boring_bitmap__extern"]
    pub fn iree_tokenizer_regex_stride_boring_bitmap(
        stride: *const iree_tokenizer_regex_stride_t,
    ) -> *const u8;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_stride_is_boring__extern"]
    pub fn iree_tokenizer_regex_stride_is_boring(
        stride: *const iree_tokenizer_regex_stride_t,
        state: u16,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_stride_self_loop_category__extern"]
    pub fn iree_tokenizer_regex_stride_self_loop_category(
        stride: *const iree_tokenizer_regex_stride_t,
        state: u16,
    ) -> u8;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_stride_allocate(
        dfa: *const iree_tokenizer_regex_dfa_t,
        allocator: iree_allocator_t,
        out_stride: *mut *mut iree_tokenizer_regex_stride_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_stride_free(
        stride: *mut iree_tokenizer_regex_stride_t,
        allocator: iree_allocator_t,
    );
}
pub const IREE_TOKENIZER_UTIL_REGEX_SPLIT_REMOVED: iree_tokenizer_regex_split_behavior_e = 0;
pub const IREE_TOKENIZER_UTIL_REGEX_SPLIT_ISOLATED: iree_tokenizer_regex_split_behavior_e = 1;
pub const IREE_TOKENIZER_UTIL_REGEX_SPLIT_MERGED_WITH_PREVIOUS: iree_tokenizer_regex_split_behavior_e = 2;
pub const IREE_TOKENIZER_UTIL_REGEX_SPLIT_MERGED_WITH_NEXT: iree_tokenizer_regex_split_behavior_e = 3;
pub const IREE_TOKENIZER_UTIL_REGEX_SPLIT_CONTIGUOUS: iree_tokenizer_regex_split_behavior_e = 4;
pub type iree_tokenizer_regex_split_behavior_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_regex_split_behavior_e as iree_tokenizer_regex_split_behavior_t;
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_split_creates_bpe_boundaries__extern"]
    pub fn iree_tokenizer_regex_split_creates_bpe_boundaries(
        behavior: iree_tokenizer_regex_split_behavior_t,
    ) -> bool;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_dfa_load(
        data: iree_const_byte_span_t,
        out_dfa: *mut iree_tokenizer_regex_dfa_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_dfa_validate(
        dfa: *const iree_tokenizer_regex_dfa_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_state_count__extern"]
    pub fn iree_tokenizer_regex_dfa_state_count(
        dfa: *const iree_tokenizer_regex_dfa_t,
    ) -> u16;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_flags__extern"]
    pub fn iree_tokenizer_regex_dfa_flags(
        dfa: *const iree_tokenizer_regex_dfa_t,
    ) -> iree_tokenizer_regex_dfa_flags_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_start_state__extern"]
    pub fn iree_tokenizer_regex_dfa_start_state(
        dfa: *const iree_tokenizer_regex_dfa_t,
    ) -> u16;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_uses_unicode__extern"]
    pub fn iree_tokenizer_regex_dfa_uses_unicode(
        dfa: *const iree_tokenizer_regex_dfa_t,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_is_accepting__extern"]
    pub fn iree_tokenizer_regex_dfa_is_accepting(
        dfa: *const iree_tokenizer_regex_dfa_t,
        state: u16,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_step__extern"]
    pub fn iree_tokenizer_regex_dfa_step(
        dfa: *const iree_tokenizer_regex_dfa_t,
        current_state: u16,
        codepoint: u32,
        byte: u8,
    ) -> u16;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_exec_initialize(
        state: *mut iree_tokenizer_regex_exec_state_t,
        dfa: *const iree_tokenizer_regex_dfa_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_exec_feed(
        dfa: *const iree_tokenizer_regex_dfa_t,
        state: *mut iree_tokenizer_regex_exec_state_t,
        chunk: iree_string_view_t,
        base_offset: iree_host_size_t,
        stride: *const iree_tokenizer_regex_stride_t,
        callback: iree_tokenizer_regex_match_callback_fn_t,
        user_data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_exec_finalize(
        dfa: *const iree_tokenizer_regex_dfa_t,
        state: *mut iree_tokenizer_regex_exec_state_t,
        total_length: iree_host_size_t,
        callback: iree_tokenizer_regex_match_callback_fn_t,
        user_data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_exec(
        dfa: *const iree_tokenizer_regex_dfa_t,
        text: iree_string_view_t,
        callback: iree_tokenizer_regex_match_callback_fn_t,
        user_data: *mut ::core::ffi::c_void,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_count_matches(
        dfa: *const iree_tokenizer_regex_dfa_t,
        text: iree_string_view_t,
        out_count: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_has_match(
        dfa: *const iree_tokenizer_regex_dfa_t,
        text: iree_string_view_t,
        out_has_match: *mut bool,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_regex_dfa_binary_size__extern"]
    pub fn iree_tokenizer_regex_dfa_binary_size(
        num_states: u16,
        flags: iree_tokenizer_regex_dfa_flags_t,
        num_ranges: u16,
    ) -> iree_host_size_t;
}
pub const IREE_TOKENIZER_UTIL_REGEX_COMPILE_FLAG_NONE: iree_tokenizer_regex_compile_flag_bits_e = 0;
pub const IREE_TOKENIZER_UTIL_REGEX_COMPILE_FLAG_CASE_INSENSITIVE: iree_tokenizer_regex_compile_flag_bits_e = 1;
pub type iree_tokenizer_regex_compile_flag_bits_e = ::core::ffi::c_uint;
pub use self::iree_tokenizer_regex_compile_flag_bits_e as iree_tokenizer_regex_compile_flag_bits_t;
pub type iree_tokenizer_regex_compile_flags_t = u32;
#[repr(C)]
pub struct iree_tokenizer_regex_compile_error_t {
    pub position: iree_host_size_t,
    pub length: iree_host_size_t,
    pub message: *const ::core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_regex_compile_error_t",
    ][::core::mem::size_of::<iree_tokenizer_regex_compile_error_t>() - 24usize];
    [
        "Alignment of iree_tokenizer_regex_compile_error_t",
    ][::core::mem::align_of::<iree_tokenizer_regex_compile_error_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_compile_error_t::position",
    ][::core::mem::offset_of!(iree_tokenizer_regex_compile_error_t, position) - 0usize];
    [
        "Offset of field: iree_tokenizer_regex_compile_error_t::length",
    ][::core::mem::offset_of!(iree_tokenizer_regex_compile_error_t, length) - 8usize];
    [
        "Offset of field: iree_tokenizer_regex_compile_error_t::message",
    ][::core::mem::offset_of!(iree_tokenizer_regex_compile_error_t, message) - 16usize];
};
impl Default for iree_tokenizer_regex_compile_error_t {
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
    pub fn iree_tokenizer_regex_compile(
        pattern: iree_string_view_t,
        flags: iree_tokenizer_regex_compile_flags_t,
        allocator: iree_allocator_t,
        out_dfa_data: *mut *mut u8,
        out_dfa_size: *mut iree_host_size_t,
        out_error: *mut iree_tokenizer_regex_compile_error_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_regex_compile_and_load(
        pattern: iree_string_view_t,
        flags: iree_tokenizer_regex_compile_flags_t,
        allocator: iree_allocator_t,
        out_dfa: *mut iree_tokenizer_regex_dfa_t,
        out_dfa_storage: *mut *mut u8,
        out_error: *mut iree_tokenizer_regex_compile_error_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_bert_allocate(
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_digits_allocate(
        individual_digits: bool,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_metaspace_allocate(
        replacement_codepoint: u32,
        split_enabled: bool,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_passthrough_allocate(
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_punctuation_allocate(
        behavior: iree_tokenizer_regex_split_behavior_t,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_sequence_allocate(
        children: *const *mut iree_tokenizer_segmenter_t,
        child_count: iree_host_size_t,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_split_allocate(
        dfa: iree_tokenizer_regex_dfa_t,
        dfa_data: *mut u8,
        behavior: iree_tokenizer_regex_split_behavior_t,
        invert: bool,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_split_literal_allocate(
        pattern: iree_string_view_t,
        behavior: iree_tokenizer_regex_split_behavior_t,
        invert: bool,
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_segmenter_whitespace_allocate(
        allocator: iree_allocator_t,
        out_segmenter: *mut *mut iree_tokenizer_segmenter_t,
    ) -> iree_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_vocab_builder_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_allocate(
        capacity_hint: iree_host_size_t,
        allocator: iree_allocator_t,
        out_builder: *mut *mut iree_tokenizer_vocab_builder_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_add_token(
        builder: *mut iree_tokenizer_vocab_builder_t,
        text: iree_string_view_t,
        score: f32,
        attrs: iree_tokenizer_token_attr_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_add_token_with_id(
        builder: *mut iree_tokenizer_vocab_builder_t,
        token_id: i32,
        text: iree_string_view_t,
        score: f32,
        attrs: iree_tokenizer_token_attr_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_set_special_token(
        builder: *mut iree_tokenizer_vocab_builder_t,
        type_: iree_tokenizer_special_token_t,
        token_id: i32,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_add_token_attrs(
        builder: *mut iree_tokenizer_vocab_builder_t,
        token_id: i32,
        attrs: iree_tokenizer_token_attr_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_ensure_token(
        builder: *mut iree_tokenizer_vocab_builder_t,
        token_id: i32,
        text: iree_string_view_t,
        score: f32,
        attrs: iree_tokenizer_token_attr_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_add_merge(
        builder: *mut iree_tokenizer_vocab_builder_t,
        left_id: u32,
        right_id: u32,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_build(
        builder: *mut iree_tokenizer_vocab_builder_t,
        out_vocab: *mut *mut iree_tokenizer_vocab_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_builder_free(
        builder: *mut iree_tokenizer_vocab_builder_t,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_tokenizer_vocab_hash_t {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_hash_storage_size(
        token_count: iree_host_size_t,
        load_percent: u8,
        out_size: *mut iree_host_size_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_hash_initialize(
        tokens: *const iree_tokenizer_token_t,
        token_count: iree_host_size_t,
        string_table: iree_const_byte_span_t,
        load_percent: u8,
        out_hash: *mut iree_tokenizer_vocab_hash_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_hash_deinitialize(
        hash: *mut iree_tokenizer_vocab_hash_t,
    );
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_hash_build(
        tokens: *const iree_tokenizer_token_t,
        token_count: iree_host_size_t,
        string_table: iree_const_byte_span_t,
        load_percent: u8,
        allocator: iree_allocator_t,
        out_hash: *mut *mut iree_tokenizer_vocab_hash_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_hash_free(hash: *mut iree_tokenizer_vocab_hash_t);
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_hash_lookup(
        hash: *const iree_tokenizer_vocab_hash_t,
        text: iree_string_view_t,
    ) -> i32;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_merge_hash_result_t {
    pub rank: u32,
    pub result_id: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_merge_hash_result_t",
    ][::core::mem::size_of::<iree_tokenizer_merge_hash_result_t>() - 8usize];
    [
        "Alignment of iree_tokenizer_merge_hash_result_t",
    ][::core::mem::align_of::<iree_tokenizer_merge_hash_result_t>() - 4usize];
    [
        "Offset of field: iree_tokenizer_merge_hash_result_t::rank",
    ][::core::mem::offset_of!(iree_tokenizer_merge_hash_result_t, rank) - 0usize];
    [
        "Offset of field: iree_tokenizer_merge_hash_result_t::result_id",
    ][::core::mem::offset_of!(iree_tokenizer_merge_hash_result_t, result_id) - 4usize];
};
unsafe extern "C" {
    #[link_name = "iree_tokenizer_merge_hash_result_none__extern"]
    pub fn iree_tokenizer_merge_hash_result_none() -> iree_tokenizer_merge_hash_result_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_merge_hash_result_is_valid__extern"]
    pub fn iree_tokenizer_merge_hash_result_is_valid(
        result: iree_tokenizer_merge_hash_result_t,
    ) -> bool;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_tokenizer_merge_hash_entry_t {
    pub key: u64,
    pub rank: u32,
    pub result_id: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_merge_hash_entry_t",
    ][::core::mem::size_of::<iree_tokenizer_merge_hash_entry_t>() - 16usize];
    [
        "Alignment of iree_tokenizer_merge_hash_entry_t",
    ][::core::mem::align_of::<iree_tokenizer_merge_hash_entry_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_merge_hash_entry_t::key",
    ][::core::mem::offset_of!(iree_tokenizer_merge_hash_entry_t, key) - 0usize];
    [
        "Offset of field: iree_tokenizer_merge_hash_entry_t::rank",
    ][::core::mem::offset_of!(iree_tokenizer_merge_hash_entry_t, rank) - 8usize];
    [
        "Offset of field: iree_tokenizer_merge_hash_entry_t::result_id",
    ][::core::mem::offset_of!(iree_tokenizer_merge_hash_entry_t, result_id) - 12usize];
};
#[repr(C)]
pub struct iree_tokenizer_vocab_merge_hash_t {
    pub allocator: iree_allocator_t,
    pub capacity: iree_host_size_t,
    pub count: iree_host_size_t,
    pub entries: __IncompleteArrayField<iree_tokenizer_merge_hash_entry_t>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_vocab_merge_hash_t",
    ][::core::mem::size_of::<iree_tokenizer_vocab_merge_hash_t>() - 32usize];
    [
        "Alignment of iree_tokenizer_vocab_merge_hash_t",
    ][::core::mem::align_of::<iree_tokenizer_vocab_merge_hash_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_vocab_merge_hash_t::allocator",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_merge_hash_t, allocator) - 0usize];
    [
        "Offset of field: iree_tokenizer_vocab_merge_hash_t::capacity",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_merge_hash_t, capacity) - 16usize];
    [
        "Offset of field: iree_tokenizer_vocab_merge_hash_t::count",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_merge_hash_t, count) - 24usize];
    [
        "Offset of field: iree_tokenizer_vocab_merge_hash_t::entries",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_merge_hash_t, entries) - 32usize];
};
impl Default for iree_tokenizer_vocab_merge_hash_t {
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
    pub fn iree_tokenizer_vocab_merge_hash_build(
        vocab: *const iree_tokenizer_vocab_t,
        allocator: iree_allocator_t,
        out_hash: *mut *mut iree_tokenizer_vocab_merge_hash_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_merge_hash_pack_key__extern"]
    pub fn iree_tokenizer_merge_hash_pack_key(left_id: i32, right_id: i32) -> u64;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_merge_hash_fn__extern"]
    pub fn iree_tokenizer_merge_hash_fn(key: u64) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_vocab_merge_hash_lookup__extern"]
    pub fn iree_tokenizer_vocab_merge_hash_lookup(
        hash: *const iree_tokenizer_vocab_merge_hash_t,
        left_token_id: i32,
        right_token_id: i32,
    ) -> iree_tokenizer_merge_hash_result_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_merge_hash_free(
        hash: *mut iree_tokenizer_vocab_merge_hash_t,
    );
}
#[repr(C)]
pub struct iree_tokenizer_vocab_trie_t {
    pub allocator: iree_allocator_t,
    pub max_depth: iree_host_size_t,
    pub node_count: iree_host_size_t,
    pub array_size: iree_host_size_t,
    pub base: *mut i32,
    pub check: *mut i32,
    pub output: *mut i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_vocab_trie_t",
    ][::core::mem::size_of::<iree_tokenizer_vocab_trie_t>() - 64usize];
    [
        "Alignment of iree_tokenizer_vocab_trie_t",
    ][::core::mem::align_of::<iree_tokenizer_vocab_trie_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::allocator",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, allocator) - 0usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::max_depth",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, max_depth) - 16usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::node_count",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, node_count) - 24usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::array_size",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, array_size) - 32usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::base",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, base) - 40usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::check",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, check) - 48usize];
    [
        "Offset of field: iree_tokenizer_vocab_trie_t::output",
    ][::core::mem::offset_of!(iree_tokenizer_vocab_trie_t, output) - 56usize];
};
impl Default for iree_tokenizer_vocab_trie_t {
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
    pub fn iree_tokenizer_vocab_trie_build(
        tokens: *const iree_tokenizer_token_t,
        token_count: iree_host_size_t,
        string_table: iree_const_byte_span_t,
        allocator: iree_allocator_t,
        out_trie: *mut *mut iree_tokenizer_vocab_trie_t,
    ) -> iree_status_t;
}
unsafe extern "C" {
    pub fn iree_tokenizer_vocab_trie_free(trie: *mut iree_tokenizer_vocab_trie_t);
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_vocab_trie_max_depth__extern"]
    pub fn iree_tokenizer_vocab_trie_max_depth(
        trie: *const iree_tokenizer_vocab_trie_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_vocab_trie_node_count__extern"]
    pub fn iree_tokenizer_vocab_trie_node_count(
        trie: *const iree_tokenizer_vocab_trie_t,
    ) -> iree_host_size_t;
}
#[repr(C)]
pub struct iree_tokenizer_trie_cursor_t {
    pub trie: *const iree_tokenizer_vocab_trie_t,
    pub node_index: u32,
    pub depth: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_tokenizer_trie_cursor_t",
    ][::core::mem::size_of::<iree_tokenizer_trie_cursor_t>() - 24usize];
    [
        "Alignment of iree_tokenizer_trie_cursor_t",
    ][::core::mem::align_of::<iree_tokenizer_trie_cursor_t>() - 8usize];
    [
        "Offset of field: iree_tokenizer_trie_cursor_t::trie",
    ][::core::mem::offset_of!(iree_tokenizer_trie_cursor_t, trie) - 0usize];
    [
        "Offset of field: iree_tokenizer_trie_cursor_t::node_index",
    ][::core::mem::offset_of!(iree_tokenizer_trie_cursor_t, node_index) - 8usize];
    [
        "Offset of field: iree_tokenizer_trie_cursor_t::depth",
    ][::core::mem::offset_of!(iree_tokenizer_trie_cursor_t, depth) - 16usize];
};
impl Default for iree_tokenizer_trie_cursor_t {
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
    #[link_name = "iree_tokenizer_trie_cursor_depth__extern"]
    pub fn iree_tokenizer_trie_cursor_depth(
        cursor: *const iree_tokenizer_trie_cursor_t,
    ) -> iree_host_size_t;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_trie_cursor_reset__extern"]
    pub fn iree_tokenizer_trie_cursor_reset(
        cursor: *mut iree_tokenizer_trie_cursor_t,
        trie: *const iree_tokenizer_vocab_trie_t,
    );
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_trie_cursor_advance__extern"]
    pub fn iree_tokenizer_trie_cursor_advance(
        cursor: *mut iree_tokenizer_trie_cursor_t,
        byte: u8,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_tokenizer_trie_cursor_token_id__extern"]
    pub fn iree_tokenizer_trie_cursor_token_id(
        cursor: *const iree_tokenizer_trie_cursor_t,
    ) -> i32;
}
