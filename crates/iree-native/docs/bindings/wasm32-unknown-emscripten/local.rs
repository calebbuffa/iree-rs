use super::*;
#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    ///# Safety
    ///The backing union storage must contain a valid, initialized T, aligned and live for the returned reference's lifetime. Mutable access requires exclusivity; shared access forbids mutation for that lifetime.
    pub unsafe fn as_ref(&self) -> &T {
        // SAFETY: The caller guarantees this union member is initialized and valid as T, properly aligned, and borrowed with the returned reference's lifetime and exclusivity.
        unsafe { ::core::mem::transmute(self) }
    }
    #[inline]
    ///# Safety
    ///The backing union storage must contain a valid, initialized T, aligned and live for the returned reference's lifetime. Mutable access requires exclusivity; shared access forbids mutation for that lifetime.
    pub unsafe fn as_mut(&mut self) -> &mut T {
        // SAFETY: The caller guarantees this union member is initialized and valid as T, properly aligned, and borrowed with the returned reference's lifetime and exclusivity.
        unsafe { ::core::mem::transmute(self) }
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
pub const IREE_HAL_EXECUTABLE_LIBRARY_VERSION_0_6: u32 = 6;
pub const IREE_HAL_EXECUTABLE_LIBRARY_VERSION_LATEST: u32 = 6;
pub const IREE_HAL_EXECUTABLE_LIBRARY_EXPORT_NAME: &[u8; 34] = b"iree_hal_executable_library_query\0";
pub const IREE_HAL_PROCESSOR_DATA_CAPACITY_V0: u32 = 8;
pub const IREE_HAL_EXECUTABLE_WORKGROUP_LOCAL_MEMORY_PAGE_SIZE: u32 = 4096;
pub const IREE_HAL_EXECUTABLE_MAX_CONSTANT_COUNT: u32 = 64;
pub const IREE_HAL_EXECUTABLE_MAX_BINDING_COUNT: u32 = 64;
pub const IREE_HAL_EXECUTABLE_PLUGIN_VERSION_0_1: u32 = 1;
pub const IREE_HAL_EXECUTABLE_PLUGIN_VERSION_LATEST: u32 = 1;
pub const IREE_HAL_EXECUTABLE_PLUGIN_EXPORT_NAME: &[u8; 33] = b"iree_hal_executable_plugin_query\0";
pub const IREE_HAL_EXECUTABLE_LIBRARY_FEATURE_NONE: iree_hal_executable_library_feature_bits_t = 0;
pub type iree_hal_executable_library_feature_bits_t = ::core::ffi::c_uint;
pub type iree_hal_executable_library_features_t = u32;
pub const IREE_HAL_EXECUTABLE_LIBRARY_SANITIZER_NONE: iree_hal_executable_library_sanitizer_kind_e = 0;
pub const IREE_HAL_EXECUTABLE_LIBRARY_SANITIZER_ADDRESS: iree_hal_executable_library_sanitizer_kind_e = 1;
pub const IREE_HAL_EXECUTABLE_LIBRARY_SANITIZER_MEMORY: iree_hal_executable_library_sanitizer_kind_e = 2;
pub const IREE_HAL_EXECUTABLE_LIBRARY_SANITIZER_THREAD: iree_hal_executable_library_sanitizer_kind_e = 3;
pub const IREE_HAL_EXECUTABLE_LIBRARY_SANITIZER_UNDEFINED: iree_hal_executable_library_sanitizer_kind_e = 4;
pub const IREE_HAL_EXECUTABLE_LIBRARY_SANITIZER_MAX_ENUM: iree_hal_executable_library_sanitizer_kind_e = 2147483647;
pub type iree_hal_executable_library_sanitizer_kind_e = ::core::ffi::c_uint;
pub use self::iree_hal_executable_library_sanitizer_kind_e as iree_hal_executable_library_sanitizer_kind_t;
pub type iree_hal_executable_library_version_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_library_header_t {
    pub version: iree_hal_executable_library_version_t,
    pub name: *const ::core::ffi::c_char,
    pub features: iree_hal_executable_library_features_t,
    pub sanitizer: iree_hal_executable_library_sanitizer_kind_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_library_header_t",
    ][::core::mem::size_of::<iree_hal_executable_library_header_t>() - 16usize];
    [
        "Alignment of iree_hal_executable_library_header_t",
    ][::core::mem::align_of::<iree_hal_executable_library_header_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_library_header_t::version",
    ][::core::mem::offset_of!(iree_hal_executable_library_header_t, version) - 0usize];
    [
        "Offset of field: iree_hal_executable_library_header_t::name",
    ][::core::mem::offset_of!(iree_hal_executable_library_header_t, name) - 4usize];
    [
        "Offset of field: iree_hal_executable_library_header_t::features",
    ][::core::mem::offset_of!(iree_hal_executable_library_header_t, features) - 8usize];
    [
        "Offset of field: iree_hal_executable_library_header_t::sanitizer",
    ][::core::mem::offset_of!(iree_hal_executable_library_header_t, sanitizer)
        - 12usize];
};
impl Default for iree_hal_executable_library_header_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_hal_executable_library_query_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        max_version: iree_hal_executable_library_version_t,
        environment: *const iree_hal_executable_environment_v0_t,
    ) -> *mut *const iree_hal_executable_library_header_t,
>;
pub type iree_hal_executable_import_v0_t = ::core::option::Option<
    unsafe extern "C" fn(
        params: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        reserved: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type iree_hal_executable_import_thunk_v0_t = ::core::option::Option<
    unsafe extern "C" fn(
        fn_ptr: iree_hal_executable_import_v0_t,
        params: *mut ::core::ffi::c_void,
        context: *mut ::core::ffi::c_void,
        reserved: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_import_table_v0_t {
    pub count: u32,
    pub symbols: *const *const ::core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_import_table_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_import_table_v0_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_import_table_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_import_table_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_import_table_v0_t::count",
    ][::core::mem::offset_of!(iree_hal_executable_import_table_v0_t, count) - 0usize];
    [
        "Offset of field: iree_hal_executable_import_table_v0_t::symbols",
    ][::core::mem::offset_of!(iree_hal_executable_import_table_v0_t, symbols) - 4usize];
};
impl Default for iree_hal_executable_import_table_v0_t {
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
pub struct iree_hal_processor_v0_t {
    pub data: [u64; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_processor_v0_t",
    ][::core::mem::size_of::<iree_hal_processor_v0_t>() - 64usize];
    [
        "Alignment of iree_hal_processor_v0_t",
    ][::core::mem::align_of::<iree_hal_processor_v0_t>() - 8usize];
    [
        "Offset of field: iree_hal_processor_v0_t::data",
    ][::core::mem::offset_of!(iree_hal_processor_v0_t, data) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_environment_v0_t {
    pub constants: *const u32,
    pub import_thunk: iree_hal_executable_import_thunk_v0_t,
    pub import_funcs: *const iree_hal_executable_import_v0_t,
    pub import_contexts: *mut *const ::core::ffi::c_void,
    pub processor: iree_hal_processor_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_environment_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_environment_v0_t>() - 80usize];
    [
        "Alignment of iree_hal_executable_environment_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_environment_v0_t>() - 8usize];
    [
        "Offset of field: iree_hal_executable_environment_v0_t::constants",
    ][::core::mem::offset_of!(iree_hal_executable_environment_v0_t, constants) - 0usize];
    [
        "Offset of field: iree_hal_executable_environment_v0_t::import_thunk",
    ][::core::mem::offset_of!(iree_hal_executable_environment_v0_t, import_thunk)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_environment_v0_t::import_funcs",
    ][::core::mem::offset_of!(iree_hal_executable_environment_v0_t, import_funcs)
        - 8usize];
    [
        "Offset of field: iree_hal_executable_environment_v0_t::import_contexts",
    ][::core::mem::offset_of!(iree_hal_executable_environment_v0_t, import_contexts)
        - 12usize];
    [
        "Offset of field: iree_hal_executable_environment_v0_t::processor",
    ][::core::mem::offset_of!(iree_hal_executable_environment_v0_t, processor)
        - 16usize];
};
impl Default for iree_hal_executable_environment_v0_t {
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
pub struct iree_hal_executable_dispatch_state_v0_t {
    pub workgroup_size_x: u32,
    pub workgroup_size_y: u32,
    pub workgroup_size_z: u16,
    pub constant_count: u16,
    pub workgroup_count_x: u32,
    pub workgroup_count_y: u32,
    pub workgroup_count_z: u16,
    pub max_concurrency: u8,
    pub binding_count: u8,
    pub constants: *const u32,
    pub binding_ptrs: *const *mut ::core::ffi::c_void,
    pub binding_lengths: *const usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_dispatch_state_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_dispatch_state_v0_t>() - 36usize];
    [
        "Alignment of iree_hal_executable_dispatch_state_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_dispatch_state_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::workgroup_size_x",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, workgroup_size_x)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::workgroup_size_y",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, workgroup_size_y)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::workgroup_size_z",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, workgroup_size_z)
        - 8usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::constant_count",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, constant_count)
        - 10usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::workgroup_count_x",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, workgroup_count_x)
        - 12usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::workgroup_count_y",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, workgroup_count_y)
        - 16usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::workgroup_count_z",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, workgroup_count_z)
        - 20usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::max_concurrency",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, max_concurrency)
        - 22usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::binding_count",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, binding_count)
        - 23usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::constants",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, constants)
        - 24usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::binding_ptrs",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, binding_ptrs)
        - 28usize];
    [
        "Offset of field: iree_hal_executable_dispatch_state_v0_t::binding_lengths",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_state_v0_t, binding_lengths)
        - 32usize];
};
impl Default for iree_hal_executable_dispatch_state_v0_t {
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
pub struct iree_hal_executable_workgroup_state_v0_t {
    pub workgroup_id_x: u32,
    pub workgroup_id_y: u32,
    pub workgroup_id_z: u16,
    pub reserved: u16,
    pub processor_id: u32,
    pub local_memory: *mut ::core::ffi::c_void,
    pub local_memory_size: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_workgroup_state_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_workgroup_state_v0_t>() - 24usize];
    [
        "Alignment of iree_hal_executable_workgroup_state_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_workgroup_state_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::workgroup_id_x",
    ][::core::mem::offset_of!(iree_hal_executable_workgroup_state_v0_t, workgroup_id_x)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::workgroup_id_y",
    ][::core::mem::offset_of!(iree_hal_executable_workgroup_state_v0_t, workgroup_id_y)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::workgroup_id_z",
    ][::core::mem::offset_of!(iree_hal_executable_workgroup_state_v0_t, workgroup_id_z)
        - 8usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::reserved",
    ][::core::mem::offset_of!(iree_hal_executable_workgroup_state_v0_t, reserved)
        - 10usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::processor_id",
    ][::core::mem::offset_of!(iree_hal_executable_workgroup_state_v0_t, processor_id)
        - 12usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::local_memory",
    ][::core::mem::offset_of!(iree_hal_executable_workgroup_state_v0_t, local_memory)
        - 16usize];
    [
        "Offset of field: iree_hal_executable_workgroup_state_v0_t::local_memory_size",
    ][::core::mem::offset_of!(
        iree_hal_executable_workgroup_state_v0_t, local_memory_size
    ) - 20usize];
};
impl Default for iree_hal_executable_workgroup_state_v0_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_hal_executable_dispatch_v0_t = ::core::option::Option<
    unsafe extern "C" fn(
        environment: *const iree_hal_executable_environment_v0_t,
        dispatch_state: *const iree_hal_executable_dispatch_state_v0_t,
        workgroup_state: *const iree_hal_executable_workgroup_state_v0_t,
    ) -> ::core::ffi::c_int,
>;
pub const IREE_HAL_EXECUTABLE_DISPATCH_FLAG_V0_NONE: iree_hal_executable_dispatch_flag_v0_bits_e = 0;
pub const IREE_HAL_EXECUTABLE_DISPATCH_FLAG_V0_SEQUENTIAL: iree_hal_executable_dispatch_flag_v0_bits_e = 1;
pub const IREE_HAL_EXECUTABLE_DISPATCH_FLAG_V0_WORKGROUP_SIZE_DYNAMIC: iree_hal_executable_dispatch_flag_v0_bits_e = 2;
pub type iree_hal_executable_dispatch_flag_v0_bits_e = ::core::ffi::c_uint;
pub type iree_hal_executable_dispatch_flags_v0_t = u64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_executable_dispatch_attrs_v0_t {
    pub flags: iree_hal_executable_dispatch_flags_v0_t,
    pub local_memory_pages: u16,
    pub constant_count: u8,
    pub binding_count: u8,
    pub workgroup_size_x: u32,
    pub workgroup_size_y: u32,
    pub workgroup_size_z: u16,
    pub parameter_count: u16,
    pub reserved_1: [u64; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_dispatch_attrs_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_dispatch_attrs_v0_t>() - 64usize];
    [
        "Alignment of iree_hal_executable_dispatch_attrs_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_dispatch_attrs_v0_t>() - 8usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::flags",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, flags) - 0usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::local_memory_pages",
    ][::core::mem::offset_of!(
        iree_hal_executable_dispatch_attrs_v0_t, local_memory_pages
    ) - 8usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::constant_count",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, constant_count)
        - 10usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::binding_count",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, binding_count)
        - 11usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::workgroup_size_x",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, workgroup_size_x)
        - 12usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::workgroup_size_y",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, workgroup_size_y)
        - 16usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::workgroup_size_z",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, workgroup_size_z)
        - 20usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::parameter_count",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, parameter_count)
        - 22usize];
    [
        "Offset of field: iree_hal_executable_dispatch_attrs_v0_t::reserved_1",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_attrs_v0_t, reserved_1)
        - 24usize];
};
pub const IREE_HAL_EXECUTABLE_DISPATCH_PARAM_TYPE_V0_CONSTANT: iree_hal_executable_dispatch_parameter_type_v0_e = 0;
pub const IREE_HAL_EXECUTABLE_DISPATCH_PARAM_TYPE_V0_BINDING: iree_hal_executable_dispatch_parameter_type_v0_e = 1;
pub const IREE_HAL_EXECUTABLE_DISPATCH_PARAM_TYPE_V0_BUFFER_PTR: iree_hal_executable_dispatch_parameter_type_v0_e = 2;
pub type iree_hal_executable_dispatch_parameter_type_v0_e = ::core::ffi::c_uint;
pub type iree_hal_executable_dispatch_parameter_type_v0_t = u8;
pub const IREE_HAL_EXECUTABLE_DISPATCH_PARAM_FLAG_V0_NONE: iree_hal_executable_dispatch_parameter_flag_v0_bits_e = 0;
pub type iree_hal_executable_dispatch_parameter_flag_v0_bits_e = ::core::ffi::c_uint;
pub type iree_hal_executable_dispatch_parameter_flags_v0_t = u16;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_executable_dispatch_parameter_v0_t {
    pub type_: iree_hal_executable_dispatch_parameter_type_v0_t,
    pub size: u8,
    pub flags: iree_hal_executable_dispatch_parameter_flags_v0_t,
    pub name: u16,
    pub offset: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_dispatch_parameter_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_dispatch_parameter_v0_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_dispatch_parameter_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_dispatch_parameter_v0_t>() - 2usize];
    [
        "Offset of field: iree_hal_executable_dispatch_parameter_v0_t::type_",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_parameter_v0_t, type_)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_dispatch_parameter_v0_t::size",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_parameter_v0_t, size)
        - 1usize];
    [
        "Offset of field: iree_hal_executable_dispatch_parameter_v0_t::flags",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_parameter_v0_t, flags)
        - 2usize];
    [
        "Offset of field: iree_hal_executable_dispatch_parameter_v0_t::name",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_parameter_v0_t, name)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_dispatch_parameter_v0_t::offset",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_parameter_v0_t, offset)
        - 6usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct iree_hal_executable_dispatch_occupancy_v0_t {
    pub reserved: ::core::ffi::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_dispatch_occupancy_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_dispatch_occupancy_v0_t>() - 4usize];
    [
        "Alignment of iree_hal_executable_dispatch_occupancy_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_dispatch_occupancy_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_dispatch_occupancy_v0_t::reserved",
    ][::core::mem::offset_of!(iree_hal_executable_dispatch_occupancy_v0_t, reserved)
        - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_source_location_v0_t {
    pub line: u32,
    pub path_length: u32,
    pub path: *const ::core::ffi::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_source_location_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_source_location_v0_t>() - 12usize];
    [
        "Alignment of iree_hal_executable_source_location_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_source_location_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_source_location_v0_t::line",
    ][::core::mem::offset_of!(iree_hal_executable_source_location_v0_t, line) - 0usize];
    [
        "Offset of field: iree_hal_executable_source_location_v0_t::path_length",
    ][::core::mem::offset_of!(iree_hal_executable_source_location_v0_t, path_length)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_source_location_v0_t::path",
    ][::core::mem::offset_of!(iree_hal_executable_source_location_v0_t, path) - 8usize];
};
impl Default for iree_hal_executable_source_location_v0_t {
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
pub struct iree_hal_executable_stage_location_table_v0_t {
    pub count: u32,
    pub names: *const *const ::core::ffi::c_char,
    pub locations: *const iree_hal_executable_source_location_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_stage_location_table_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_stage_location_table_v0_t>() - 12usize];
    [
        "Alignment of iree_hal_executable_stage_location_table_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_stage_location_table_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_stage_location_table_v0_t::count",
    ][::core::mem::offset_of!(iree_hal_executable_stage_location_table_v0_t, count)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_stage_location_table_v0_t::names",
    ][::core::mem::offset_of!(iree_hal_executable_stage_location_table_v0_t, names)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_stage_location_table_v0_t::locations",
    ][::core::mem::offset_of!(iree_hal_executable_stage_location_table_v0_t, locations)
        - 8usize];
};
impl Default for iree_hal_executable_stage_location_table_v0_t {
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
pub struct iree_hal_executable_export_table_v0_t {
    pub count: u32,
    pub ptrs: *const iree_hal_executable_dispatch_v0_t,
    pub attrs: *const iree_hal_executable_dispatch_attrs_v0_t,
    pub params: *mut *const iree_hal_executable_dispatch_parameter_v0_t,
    pub occupancy: *const iree_hal_executable_dispatch_occupancy_v0_t,
    pub names: *const *const ::core::ffi::c_char,
    pub tags: *const *const ::core::ffi::c_char,
    pub parameter_names: *const *const ::core::ffi::c_char,
    pub source_locations: *const iree_hal_executable_source_location_v0_t,
    pub stage_locations: *const iree_hal_executable_stage_location_table_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_export_table_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_export_table_v0_t>() - 40usize];
    [
        "Alignment of iree_hal_executable_export_table_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_export_table_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::count",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, count) - 0usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::ptrs",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, ptrs) - 4usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::attrs",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, attrs) - 8usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::params",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, params) - 12usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::occupancy",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, occupancy)
        - 16usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::names",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, names) - 20usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::tags",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, tags) - 24usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::parameter_names",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, parameter_names)
        - 28usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::source_locations",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, source_locations)
        - 32usize];
    [
        "Offset of field: iree_hal_executable_export_table_v0_t::stage_locations",
    ][::core::mem::offset_of!(iree_hal_executable_export_table_v0_t, stage_locations)
        - 36usize];
};
impl Default for iree_hal_executable_export_table_v0_t {
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
pub struct iree_hal_executable_constant_table_v0_t {
    pub count: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_constant_table_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_constant_table_v0_t>() - 4usize];
    [
        "Alignment of iree_hal_executable_constant_table_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_constant_table_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_constant_table_v0_t::count",
    ][::core::mem::offset_of!(iree_hal_executable_constant_table_v0_t, count) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_source_file_v0_t {
    pub path_length: u32,
    pub path: *const ::core::ffi::c_char,
    pub content_length: u32,
    pub content: *const u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_source_file_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_source_file_v0_t>() - 16usize];
    [
        "Alignment of iree_hal_executable_source_file_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_source_file_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_source_file_v0_t::path_length",
    ][::core::mem::offset_of!(iree_hal_executable_source_file_v0_t, path_length)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_source_file_v0_t::path",
    ][::core::mem::offset_of!(iree_hal_executable_source_file_v0_t, path) - 4usize];
    [
        "Offset of field: iree_hal_executable_source_file_v0_t::content_length",
    ][::core::mem::offset_of!(iree_hal_executable_source_file_v0_t, content_length)
        - 8usize];
    [
        "Offset of field: iree_hal_executable_source_file_v0_t::content",
    ][::core::mem::offset_of!(iree_hal_executable_source_file_v0_t, content) - 12usize];
};
impl Default for iree_hal_executable_source_file_v0_t {
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
pub struct iree_hal_executable_source_file_table_v0_t {
    pub count: u32,
    pub files: *const iree_hal_executable_source_file_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_source_file_table_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_source_file_table_v0_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_source_file_table_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_source_file_table_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_source_file_table_v0_t::count",
    ][::core::mem::offset_of!(iree_hal_executable_source_file_table_v0_t, count)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_source_file_table_v0_t::files",
    ][::core::mem::offset_of!(iree_hal_executable_source_file_table_v0_t, files)
        - 4usize];
};
impl Default for iree_hal_executable_source_file_table_v0_t {
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
pub struct iree_hal_executable_library_v0_t {
    pub header: *const iree_hal_executable_library_header_t,
    pub imports: iree_hal_executable_import_table_v0_t,
    pub exports: iree_hal_executable_export_table_v0_t,
    pub constants: iree_hal_executable_constant_table_v0_t,
    pub sources: iree_hal_executable_source_file_table_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_library_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_library_v0_t>() - 64usize];
    [
        "Alignment of iree_hal_executable_library_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_library_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_library_v0_t::header",
    ][::core::mem::offset_of!(iree_hal_executable_library_v0_t, header) - 0usize];
    [
        "Offset of field: iree_hal_executable_library_v0_t::imports",
    ][::core::mem::offset_of!(iree_hal_executable_library_v0_t, imports) - 4usize];
    [
        "Offset of field: iree_hal_executable_library_v0_t::exports",
    ][::core::mem::offset_of!(iree_hal_executable_library_v0_t, exports) - 12usize];
    [
        "Offset of field: iree_hal_executable_library_v0_t::constants",
    ][::core::mem::offset_of!(iree_hal_executable_library_v0_t, constants) - 52usize];
    [
        "Offset of field: iree_hal_executable_library_v0_t::sources",
    ][::core::mem::offset_of!(iree_hal_executable_library_v0_t, sources) - 56usize];
};
impl Default for iree_hal_executable_library_v0_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const IREE_HAL_EXECUTABLE_PLUGIN_FEATURE_NONE: iree_hal_executable_plugin_feature_bits_t = 0;
pub const IREE_HAL_EXECUTABLE_PLUGIN_FEATURE_STANDALONE: iree_hal_executable_plugin_feature_bits_t = 1;
pub const IREE_HAL_EXECUTABLE_PLUGIN_FEATURE_FULL_STATUS: iree_hal_executable_plugin_feature_bits_t = 2;
pub type iree_hal_executable_plugin_feature_bits_t = ::core::ffi::c_uint;
pub type iree_hal_executable_plugin_features_t = u32;
pub const IREE_HAL_EXECUTABLE_PLUGIN_SANITIZER_NONE: iree_hal_executable_plugin_sanitizer_kind_e = 0;
pub const IREE_HAL_EXECUTABLE_PLUGIN_SANITIZER_ADDRESS: iree_hal_executable_plugin_sanitizer_kind_e = 1;
pub const IREE_HAL_EXECUTABLE_PLUGIN_SANITIZER_MEMORY: iree_hal_executable_plugin_sanitizer_kind_e = 2;
pub const IREE_HAL_EXECUTABLE_PLUGIN_SANITIZER_THREAD: iree_hal_executable_plugin_sanitizer_kind_e = 3;
pub const IREE_HAL_EXECUTABLE_PLUGIN_SANITIZER_UNDEFINED: iree_hal_executable_plugin_sanitizer_kind_e = 4;
pub const IREE_HAL_EXECUTABLE_PLUGIN_SANITIZER_MAX_ENUM: iree_hal_executable_plugin_sanitizer_kind_e = 2147483647;
pub type iree_hal_executable_plugin_sanitizer_kind_e = ::core::ffi::c_uint;
pub use self::iree_hal_executable_plugin_sanitizer_kind_e as iree_hal_executable_plugin_sanitizer_kind_t;
pub type iree_hal_executable_plugin_version_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_plugin_header_t {
    pub version: iree_hal_executable_plugin_version_t,
    pub name: *const ::core::ffi::c_char,
    pub description: *const ::core::ffi::c_char,
    pub features: iree_hal_executable_plugin_features_t,
    pub sanitizer: iree_hal_executable_plugin_sanitizer_kind_t,
    pub reserved: [u64; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_header_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_header_t>() - 88usize];
    [
        "Alignment of iree_hal_executable_plugin_header_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_header_t>() - 8usize];
    [
        "Offset of field: iree_hal_executable_plugin_header_t::version",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_header_t, version) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_header_t::name",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_header_t, name) - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_header_t::description",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_header_t, description)
        - 8usize];
    [
        "Offset of field: iree_hal_executable_plugin_header_t::features",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_header_t, features) - 12usize];
    [
        "Offset of field: iree_hal_executable_plugin_header_t::sanitizer",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_header_t, sanitizer) - 16usize];
    [
        "Offset of field: iree_hal_executable_plugin_header_t::reserved",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_header_t, reserved) - 24usize];
};
impl Default for iree_hal_executable_plugin_header_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_hal_executable_plugin_query_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        max_version: iree_hal_executable_plugin_version_t,
        reserved: *mut ::core::ffi::c_void,
    ) -> *mut *const iree_hal_executable_plugin_header_t,
>;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_OK: iree_hal_executable_plugin_status_code_e = 0;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_CANCELLED: iree_hal_executable_plugin_status_code_e = 1;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_UNKNOWN: iree_hal_executable_plugin_status_code_e = 2;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_INVALID_ARGUMENT: iree_hal_executable_plugin_status_code_e = 3;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_DEADLINE_EXCEEDED: iree_hal_executable_plugin_status_code_e = 4;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_NOT_FOUND: iree_hal_executable_plugin_status_code_e = 5;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_ALREADY_EXISTS: iree_hal_executable_plugin_status_code_e = 6;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_PERMISSION_DENIED: iree_hal_executable_plugin_status_code_e = 7;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_RESOURCE_EXHAUSTED: iree_hal_executable_plugin_status_code_e = 8;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_FAILED_PRECONDITION: iree_hal_executable_plugin_status_code_e = 9;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_ABORTED: iree_hal_executable_plugin_status_code_e = 10;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_OUT_OF_RANGE: iree_hal_executable_plugin_status_code_e = 11;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_UNIMPLEMENTED: iree_hal_executable_plugin_status_code_e = 12;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_INTERNAL: iree_hal_executable_plugin_status_code_e = 13;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_UNAVAILABLE: iree_hal_executable_plugin_status_code_e = 14;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_DATA_LOSS: iree_hal_executable_plugin_status_code_e = 15;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_UNAUTHENTICATED: iree_hal_executable_plugin_status_code_e = 16;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_DEFERRED: iree_hal_executable_plugin_status_code_e = 17;
pub const IREE_HAL_EXECUTABLE_PLUGIN_STATUS_CODE_MASK: iree_hal_executable_plugin_status_code_e = 31;
pub type iree_hal_executable_plugin_status_code_e = ::core::ffi::c_uint;
pub use self::iree_hal_executable_plugin_status_code_e as iree_hal_executable_plugin_status_code_t;
pub type iree_hal_executable_plugin_status_t = *mut iree_status_handle_t;
pub const IREE_HAL_EXECUTABLE_PLUGIN_ALLOCATOR_COMMAND_MALLOC: iree_hal_executable_plugin_allocator_command_e = 0;
pub const IREE_HAL_EXECUTABLE_PLUGIN_ALLOCATOR_COMMAND_CALLOC: iree_hal_executable_plugin_allocator_command_e = 1;
pub const IREE_HAL_EXECUTABLE_PLUGIN_ALLOCATOR_COMMAND_REALLOC: iree_hal_executable_plugin_allocator_command_e = 2;
pub const IREE_HAL_EXECUTABLE_PLUGIN_ALLOCATOR_COMMAND_FREE: iree_hal_executable_plugin_allocator_command_e = 3;
pub type iree_hal_executable_plugin_allocator_command_e = ::core::ffi::c_uint;
pub use self::iree_hal_executable_plugin_allocator_command_e as iree_hal_executable_plugin_allocator_command_t;
#[repr(C)]
pub struct iree_hal_executable_plugin_allocator_alloc_params_t {
    pub byte_length: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_allocator_alloc_params_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_allocator_alloc_params_t>()
        - 4usize];
    [
        "Alignment of iree_hal_executable_plugin_allocator_alloc_params_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_allocator_alloc_params_t>()
        - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_allocator_alloc_params_t::byte_length",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_allocator_alloc_params_t, byte_length
    ) - 0usize];
};
impl Default for iree_hal_executable_plugin_allocator_alloc_params_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_hal_executable_plugin_allocator_ctl_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        self_: *mut ::core::ffi::c_void,
        command: iree_hal_executable_plugin_allocator_command_t,
        params: *const ::core::ffi::c_void,
        inout_ptr: *mut *mut ::core::ffi::c_void,
    ) -> iree_hal_executable_plugin_status_t,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_plugin_allocator_t {
    pub self_: *mut ::core::ffi::c_void,
    pub ctl: iree_hal_executable_plugin_allocator_ctl_fn_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_allocator_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_allocator_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_plugin_allocator_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_allocator_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_allocator_t::self_",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_allocator_t, self_) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_allocator_t::ctl",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_allocator_t, ctl) - 4usize];
};
impl Default for iree_hal_executable_plugin_allocator_t {
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
    #[link_name = "iree_hal_executable_plugin_allocator_malloc__extern"]
    pub fn iree_hal_executable_plugin_allocator_malloc(
        allocator: iree_hal_executable_plugin_allocator_t,
        byte_length: iree_host_size_t,
        inout_ptr: *mut *mut ::core::ffi::c_void,
    ) -> iree_hal_executable_plugin_status_t;
}
unsafe extern "C" {
    #[link_name = "iree_hal_executable_plugin_allocator_free__extern"]
    pub fn iree_hal_executable_plugin_allocator_free(
        allocator: iree_hal_executable_plugin_allocator_t,
        ptr: *mut ::core::ffi::c_void,
    ) -> iree_hal_executable_plugin_status_t;
}
#[repr(C)]
pub struct iree_hal_executable_plugin_string_view_t {
    pub data: *const ::core::ffi::c_char,
    pub size: iree_host_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_string_view_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_string_view_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_plugin_string_view_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_string_view_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_string_view_t::data",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_string_view_t, data) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_string_view_t::size",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_string_view_t, size) - 4usize];
};
impl Default for iree_hal_executable_plugin_string_view_t {
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
pub struct iree_hal_executable_plugin_string_pair_t {
    pub __bindgen_anon_1: iree_hal_executable_plugin_string_pair_t__bindgen_ty_1,
    pub __bindgen_anon_2: iree_hal_executable_plugin_string_pair_t__bindgen_ty_2,
}
#[repr(C)]
pub struct iree_hal_executable_plugin_string_pair_t__bindgen_ty_1 {
    pub first: __BindgenUnionField<iree_hal_executable_plugin_string_view_t>,
    pub key: __BindgenUnionField<iree_hal_executable_plugin_string_view_t>,
    pub bindgen_union_field: [u32; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_string_pair_t__bindgen_ty_1",
    ][::core::mem::size_of::<iree_hal_executable_plugin_string_pair_t__bindgen_ty_1>()
        - 8usize];
    [
        "Alignment of iree_hal_executable_plugin_string_pair_t__bindgen_ty_1",
    ][::core::mem::align_of::<iree_hal_executable_plugin_string_pair_t__bindgen_ty_1>()
        - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_string_pair_t__bindgen_ty_1::first",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_string_pair_t__bindgen_ty_1, first
    ) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_string_pair_t__bindgen_ty_1::key",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_string_pair_t__bindgen_ty_1, key
    ) - 0usize];
};
impl Default for iree_hal_executable_plugin_string_pair_t__bindgen_ty_1 {
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
pub struct iree_hal_executable_plugin_string_pair_t__bindgen_ty_2 {
    pub second: __BindgenUnionField<iree_hal_executable_plugin_string_view_t>,
    pub value: __BindgenUnionField<iree_hal_executable_plugin_string_view_t>,
    pub bindgen_union_field: [u32; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_string_pair_t__bindgen_ty_2",
    ][::core::mem::size_of::<iree_hal_executable_plugin_string_pair_t__bindgen_ty_2>()
        - 8usize];
    [
        "Alignment of iree_hal_executable_plugin_string_pair_t__bindgen_ty_2",
    ][::core::mem::align_of::<iree_hal_executable_plugin_string_pair_t__bindgen_ty_2>()
        - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_string_pair_t__bindgen_ty_2::second",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_string_pair_t__bindgen_ty_2, second
    ) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_string_pair_t__bindgen_ty_2::value",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_string_pair_t__bindgen_ty_2, value
    ) - 0usize];
};
impl Default for iree_hal_executable_plugin_string_pair_t__bindgen_ty_2 {
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
        "Size of iree_hal_executable_plugin_string_pair_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_string_pair_t>() - 16usize];
    [
        "Alignment of iree_hal_executable_plugin_string_pair_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_string_pair_t>() - 4usize];
};
impl Default for iree_hal_executable_plugin_string_pair_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const IREE_HAL_EXECUTABLE_PLUGIN_RESOLUTION_MISSING_OPTIONAL: iree_hal_executable_plugin_resolution_bits_e = 1;
pub type iree_hal_executable_plugin_resolution_bits_e = ::core::ffi::c_uint;
pub type iree_hal_executable_plugin_resolution_t = u32;
unsafe extern "C" {
    #[link_name = "iree_hal_executable_plugin_import_is_optional__extern"]
    pub fn iree_hal_executable_plugin_import_is_optional(
        symbol_name: *const ::core::ffi::c_char,
    ) -> bool;
}
unsafe extern "C" {
    #[link_name = "iree_hal_executable_plugin_strcmp__extern"]
    pub fn iree_hal_executable_plugin_strcmp(
        lhs: *const ::core::ffi::c_char,
        rhs: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_plugin_environment_v0_t {
    pub host_allocator: iree_hal_executable_plugin_allocator_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_environment_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_environment_v0_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_plugin_environment_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_environment_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_environment_v0_t::host_allocator",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_environment_v0_t, host_allocator
    ) - 0usize];
};
impl Default for iree_hal_executable_plugin_environment_v0_t {
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
pub struct iree_hal_executable_plugin_resolve_params_v0_t {
    pub count: usize,
    pub symbol_names: *const *const ::core::ffi::c_char,
    pub out_fn_ptrs: *mut *mut ::core::ffi::c_void,
    pub out_fn_contexts: *mut *mut ::core::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_resolve_params_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_resolve_params_v0_t>()
        - 16usize];
    [
        "Alignment of iree_hal_executable_plugin_resolve_params_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_resolve_params_v0_t>()
        - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_resolve_params_v0_t::count",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_resolve_params_v0_t, count)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_resolve_params_v0_t::symbol_names",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_resolve_params_v0_t, symbol_names
    ) - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_resolve_params_v0_t::out_fn_ptrs",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_resolve_params_v0_t, out_fn_ptrs
    ) - 8usize];
    [
        "Offset of field: iree_hal_executable_plugin_resolve_params_v0_t::out_fn_contexts",
    ][::core::mem::offset_of!(
        iree_hal_executable_plugin_resolve_params_v0_t, out_fn_contexts
    ) - 12usize];
};
impl Default for iree_hal_executable_plugin_resolve_params_v0_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type iree_hal_executable_plugin_resolve_fn_v0_t = ::core::option::Option<
    unsafe extern "C" fn(
        self_: *mut ::core::ffi::c_void,
        params: *const iree_hal_executable_plugin_resolve_params_v0_t,
        out_resolution: *mut iree_hal_executable_plugin_resolution_t,
    ) -> iree_hal_executable_plugin_status_t,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_plugin_v0_t {
    pub header: *const iree_hal_executable_plugin_header_t,
    pub load: ::core::option::Option<
        unsafe extern "C" fn(
            environment: *const iree_hal_executable_plugin_environment_v0_t,
            param_count: usize,
            params: *const iree_hal_executable_plugin_string_pair_t,
            out_self: *mut *mut ::core::ffi::c_void,
        ) -> iree_hal_executable_plugin_status_t,
    >,
    pub unload: ::core::option::Option<
        unsafe extern "C" fn(self_: *mut ::core::ffi::c_void),
    >,
    pub resolve: iree_hal_executable_plugin_resolve_fn_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_v0_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_v0_t>() - 16usize];
    [
        "Alignment of iree_hal_executable_plugin_v0_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_v0_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_v0_t::header",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_v0_t, header) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_v0_t::load",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_v0_t, load) - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_v0_t::unload",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_v0_t, unload) - 8usize];
    [
        "Offset of field: iree_hal_executable_plugin_v0_t::resolve",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_v0_t, resolve) - 12usize];
};
impl Default for iree_hal_executable_plugin_v0_t {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        // SAFETY: The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const IREE_HAL_EXECUTABLE_IMPORT_RESOLUTION_MISSING_OPTIONAL: iree_hal_executable_import_resolution_bits_e = 1;
pub type iree_hal_executable_import_resolution_bits_e = ::core::ffi::c_uint;
pub type iree_hal_executable_import_resolution_t = u32;
pub type iree_hal_executable_import_provider_resolve_fn_t = ::core::option::Option<
    unsafe extern "C" fn(
        self_: *mut ::core::ffi::c_void,
        count: iree_host_size_t,
        symbol_names: *const *const ::core::ffi::c_char,
        out_fn_ptrs: *mut *mut ::core::ffi::c_void,
        out_fn_contexts: *mut *mut ::core::ffi::c_void,
        out_resolution: *mut iree_hal_executable_import_resolution_t,
    ) -> iree_status_t,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_import_provider_t {
    pub self_: *mut ::core::ffi::c_void,
    pub resolve: iree_hal_executable_import_provider_resolve_fn_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_import_provider_t",
    ][::core::mem::size_of::<iree_hal_executable_import_provider_t>() - 8usize];
    [
        "Alignment of iree_hal_executable_import_provider_t",
    ][::core::mem::align_of::<iree_hal_executable_import_provider_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_import_provider_t::self_",
    ][::core::mem::offset_of!(iree_hal_executable_import_provider_t, self_) - 0usize];
    [
        "Offset of field: iree_hal_executable_import_provider_t::resolve",
    ][::core::mem::offset_of!(iree_hal_executable_import_provider_t, resolve) - 4usize];
};
impl Default for iree_hal_executable_import_provider_t {
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
    #[link_name = "iree_hal_executable_import_provider_null__extern"]
    pub fn iree_hal_executable_import_provider_null() -> iree_hal_executable_import_provider_t;
}
unsafe extern "C" {
    #[link_name = "iree_hal_executable_import_is_optional__extern"]
    pub fn iree_hal_executable_import_is_optional(
        symbol_name: *const ::core::ffi::c_char,
    ) -> bool;
}
#[repr(C)]
pub struct iree_hal_executable_loader_t {
    pub ref_count: iree_atomic_ref_count_t,
    pub vtable: *const iree_hal_executable_loader_vtable_t,
    pub import_provider: iree_hal_executable_import_provider_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_loader_t",
    ][::core::mem::size_of::<iree_hal_executable_loader_t>() - 16usize];
    [
        "Alignment of iree_hal_executable_loader_t",
    ][::core::mem::align_of::<iree_hal_executable_loader_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_loader_t::ref_count",
    ][::core::mem::offset_of!(iree_hal_executable_loader_t, ref_count) - 0usize];
    [
        "Offset of field: iree_hal_executable_loader_t::vtable",
    ][::core::mem::offset_of!(iree_hal_executable_loader_t, vtable) - 4usize];
    [
        "Offset of field: iree_hal_executable_loader_t::import_provider",
    ][::core::mem::offset_of!(iree_hal_executable_loader_t, import_provider) - 8usize];
};
impl Default for iree_hal_executable_loader_t {
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
pub struct iree_hal_executable_loader_vtable_t {
    pub destroy: ::core::option::Option<
        unsafe extern "C" fn(executable_loader: *mut iree_hal_executable_loader_t),
    >,
    pub infer_format: ::core::option::Option<
        unsafe extern "C" fn(
            executable_loader: *mut iree_hal_executable_loader_t,
            caching_mode: iree_hal_executable_caching_mode_t,
            executable_data: iree_const_byte_span_t,
            executable_format_capacity: iree_host_size_t,
            executable_format: *mut ::core::ffi::c_char,
            out_inferred_size: *mut iree_host_size_t,
        ) -> iree_status_t,
    >,
    pub query_support: ::core::option::Option<
        unsafe extern "C" fn(
            executable_loader: *mut iree_hal_executable_loader_t,
            caching_mode: iree_hal_executable_caching_mode_t,
            executable_format: iree_string_view_t,
        ) -> bool,
    >,
    pub try_load: ::core::option::Option<
        unsafe extern "C" fn(
            executable_loader: *mut iree_hal_executable_loader_t,
            executable_params: *const iree_hal_executable_params_t,
            worker_capacity: iree_host_size_t,
            out_executable: *mut *mut iree_hal_executable_t,
        ) -> iree_status_t,
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_loader_vtable_t",
    ][::core::mem::size_of::<iree_hal_executable_loader_vtable_t>() - 16usize];
    [
        "Alignment of iree_hal_executable_loader_vtable_t",
    ][::core::mem::align_of::<iree_hal_executable_loader_vtable_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_loader_vtable_t::destroy",
    ][::core::mem::offset_of!(iree_hal_executable_loader_vtable_t, destroy) - 0usize];
    [
        "Offset of field: iree_hal_executable_loader_vtable_t::infer_format",
    ][::core::mem::offset_of!(iree_hal_executable_loader_vtable_t, infer_format)
        - 4usize];
    [
        "Offset of field: iree_hal_executable_loader_vtable_t::query_support",
    ][::core::mem::offset_of!(iree_hal_executable_loader_vtable_t, query_support)
        - 8usize];
    [
        "Offset of field: iree_hal_executable_loader_vtable_t::try_load",
    ][::core::mem::offset_of!(iree_hal_executable_loader_vtable_t, try_load) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iree_hal_executable_plugin_manager_t {
    _unused: [u8; 0],
}
pub type iree_hal_executable_plugin_resolve_thunk_t = ::core::option::Option<
    unsafe extern "C" fn(
        fn_ptr: *mut ::core::ffi::c_void,
        self_: *mut ::core::ffi::c_void,
        params: *const ::core::ffi::c_void,
        out_resolution: *mut u32,
    ) -> iree_status_t,
>;
#[repr(C)]
pub struct iree_hal_executable_plugin_t {
    pub ref_count: iree_atomic_ref_count_t,
    pub vtable: *const iree_hal_executable_plugin_vtable_t,
    pub library: iree_hal_executable_plugin_t__bindgen_ty_1,
    pub self_: *mut ::core::ffi::c_void,
    pub identifier: iree_string_view_t,
    pub resolve_thunk: iree_hal_executable_plugin_resolve_thunk_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union iree_hal_executable_plugin_t__bindgen_ty_1 {
    pub header: *mut *const iree_hal_executable_plugin_header_t,
    pub v0: *const iree_hal_executable_plugin_v0_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_t__bindgen_ty_1",
    ][::core::mem::size_of::<iree_hal_executable_plugin_t__bindgen_ty_1>() - 4usize];
    [
        "Alignment of iree_hal_executable_plugin_t__bindgen_ty_1",
    ][::core::mem::align_of::<iree_hal_executable_plugin_t__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_t__bindgen_ty_1::header",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t__bindgen_ty_1, header)
        - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_t__bindgen_ty_1::v0",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t__bindgen_ty_1, v0) - 0usize];
};
impl Default for iree_hal_executable_plugin_t__bindgen_ty_1 {
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
        "Size of iree_hal_executable_plugin_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_t>() - 28usize];
    [
        "Alignment of iree_hal_executable_plugin_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_t::ref_count",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t, ref_count) - 0usize];
    [
        "Offset of field: iree_hal_executable_plugin_t::vtable",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t, vtable) - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_t::library",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t, library) - 8usize];
    [
        "Offset of field: iree_hal_executable_plugin_t::self_",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t, self_) - 12usize];
    [
        "Offset of field: iree_hal_executable_plugin_t::identifier",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t, identifier) - 16usize];
    [
        "Offset of field: iree_hal_executable_plugin_t::resolve_thunk",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_t, resolve_thunk) - 24usize];
};
impl Default for iree_hal_executable_plugin_t {
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
pub struct iree_hal_executable_plugin_vtable_t {
    pub destroy: ::core::option::Option<
        unsafe extern "C" fn(plugin: *mut iree_hal_executable_plugin_t),
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of iree_hal_executable_plugin_vtable_t",
    ][::core::mem::size_of::<iree_hal_executable_plugin_vtable_t>() - 4usize];
    [
        "Alignment of iree_hal_executable_plugin_vtable_t",
    ][::core::mem::align_of::<iree_hal_executable_plugin_vtable_t>() - 4usize];
    [
        "Offset of field: iree_hal_executable_plugin_vtable_t::destroy",
    ][::core::mem::offset_of!(iree_hal_executable_plugin_vtable_t, destroy) - 0usize];
};
pub const IREE_HAL_LOADER_MODULE_FLAG_NONE: iree_hal_loader_module_flag_bits_t = 0;
pub type iree_hal_loader_module_flag_bits_t = ::core::ffi::c_uint;
pub type iree_hal_loader_module_flags_t = u32;
