//! Link-level smoke tests for public API families beyond runtime sessions.

use iree_sys as sys;

#[test]
fn generated_modules_preserve_root_symbol_access() {
    assert_eq!(
        sys::base::iree_allocator_system as *const (),
        sys::iree_allocator_system as *const ()
    );
    #[cfg(feature = "task")]
    assert_eq!(
        sys::task::iree_task_executor_create as *const (),
        sys::iree_task_executor_create as *const ()
    );
    #[cfg(feature = "vulkan")]
    assert_eq!(
        sys::vulkan::iree_hal_vulkan_driver_create as *const (),
        sys::iree_hal_vulkan_driver_create as *const ()
    );
    #[cfg(feature = "compiler")]
    assert_eq!(
        sys::compiler::ireeCompilerSessionCreate as *const (),
        sys::ireeCompilerSessionCreate as *const ()
    );
}

#[test]
fn inline_functions_and_parameter_io_are_linkable() {
    unsafe {
        let view = sys::iree_make_cstring_view(c"native-api".as_ptr());
        assert_eq!(view.size, 10);
        let mut index = std::ptr::null_mut();
        let status = sys::iree_io_parameter_index_create(sys::iree_allocator_system(), &mut index);
        assert!(status.is_null(), "parameter index creation failed");
        assert!(!index.is_null());
        assert_eq!(sys::iree_io_parameter_index_count(index), 0);
        sys::iree_io_parameter_index_release(index);
    }
}

#[test]
#[cfg(feature = "compiler")]
fn compiler_mlir_interop_is_linkable() {
    unsafe {
        sys::ireeCompilerGlobalInitialize();
        let session = sys::ireeCompilerSessionCreate();
        assert!(!session.is_null());
        let context = sys::ireeCompilerSessionBorrowContext(session);
        assert!(!context.ptr.is_null());
        assert!(!sys::ireeCodegenRootOpAttrGetTypeID().ptr.is_null());
        assert!(!sys::ireeCodegenVMVXPipelineAttrGetTypeID().ptr.is_null());
        sys::ireeCompilerSessionDestroy(session);
    }
}
