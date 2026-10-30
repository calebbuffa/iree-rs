//! Link checks for the embedded IREE compiler tool entry points.
#![cfg(all(feature = "compiler", feature = "compiler-tools"))]

use iree_native as native;

#[test]
fn compiler_tool_entry_points_are_generated_and_linkable() {
    macro_rules! check_export {
        ($($name:ident),+ $(,)?) => {
            $(
                assert_eq!(
                    std::hint::black_box(native::raw_compiler::$name as *const ()),
                    std::hint::black_box(native::$name as *const ())
                );
            )+
        };
    }

    check_export!(
        ireeCompilerRunMain,
        ireeOptRunMain,
        ireeMlirLspServerRunMain,
        ireeCompilerRunLldMain,
        ireeLinkRunMain,
        ireeReduceRunMain,
    );
}

#[test]
fn matched_mlir_registry_lifecycle_is_generated_and_linkable() {
    assert_eq!(
        native::raw_compiler::mlirDialectRegistryCreate as *const (),
        native::mlirDialectRegistryCreate as *const ()
    );
    assert_eq!(
        native::raw_compiler::mlirDialectRegistryDestroy as *const (),
        native::mlirDialectRegistryDestroy as *const ()
    );
    assert_eq!(
        native::raw_compiler::mlirContextAppendDialectRegistry as *const (),
        native::mlirContextAppendDialectRegistry as *const ()
    );
    let registry = unsafe { native::mlirDialectRegistryCreate() };
    assert!(!registry.ptr.is_null());
    unsafe { native::mlirDialectRegistryDestroy(registry) };
}
