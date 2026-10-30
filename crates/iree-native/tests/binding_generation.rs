//! Generator policy tests independent of a particular native declaration set.
#[path = "../native/binding_support.rs"]
mod support;

use support::{Inventory, document_native_examples, normalize_bindings};

#[test]
fn integer_bitfield_helpers_use_casts_without_redundant_unsafe()
-> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
        pub struct Record { _bitfield_1: Bits }
        impl Record {
            pub fn get(&self) -> u32 {
                unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 3u8) as u32) }
            }
            pub fn set(&mut self, val: u32) {
                unsafe {
                    let val: u32 = ::core::mem::transmute(val);
                    self._bitfield_1.set(0usize, 3u8, val as u64)
                }
            }
            pub unsafe fn get_raw(this: *const Self) -> u32 {
                unsafe {
                    ::core::mem::transmute(
                        Bits::raw_get(::core::ptr::addr_of!((*this)._bitfield_1), 0usize, 3u8) as u32
                    )
                }
            }
        }
    "#;
    let output = normalize_bindings(source)?;
    assert!(!output.contains("transmute"));
    assert_eq!(output.matches("unsafe {").count(), 1);
    assert_eq!(output.matches("// SAFETY:").count(), 1);
    assert!(output.contains("# Safety"));
    assert!(output.contains("allocated record storage"));
    assert_eq!(Inventory::parse(source)?, Inventory::parse(&output)?);
    syn::parse_file(&output)?;
    Ok(())
}

#[test]
fn c_integer_bitfield_helpers_normalize_without_assuming_target_integer_widths()
-> Result<(), Box<dyn std::error::Error>> {
    let output = normalize_bindings(
        r#"
        pub struct Record { _bitfield_1: Bits }
        impl Record {
            pub fn flags(&self) -> ::core::ffi::c_int {
                unsafe { ::core::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
            }
            pub fn set_flags(&mut self, val: ::core::ffi::c_int) {
                unsafe {
                    let val: u32 = ::core::mem::transmute(val);
                    self._bitfield_1.set(0usize, 24u8, val as u64)
                }
            }
        }
        "#,
    )?;
    assert!(!output.contains("transmute"));
    assert!(!output.contains("unsafe"));
    assert!(output.contains("as ::core::ffi::c_int"));
    syn::parse_file(&output)?;
    Ok(())
}

#[test]
fn generated_defaults_keep_initialization_and_explain_its_safety()
-> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
        impl Default for CRecord {
            fn default() -> Self {
                let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
                unsafe {
                    ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
                    s.assume_init()
                }
            }
        }
    "#;
    let output = normalize_bindings(source)?;
    assert!(output.contains("// SAFETY: The entire storage is initialized to zero"));
    assert!(output.contains("write_bytes(s.as_mut_ptr(), 0, 1)"));
    assert!(output.contains("s.assume_init()"));
    assert!(!output.contains("allow("));
    syn::parse_file(&output)?;
    Ok(())
}

#[test]
fn unreviewed_unsafe_helpers_fail_generation() {
    assert!(
        normalize_bindings(
            "impl Unknown { pub unsafe fn new_operation() { unsafe { unknown(); } } }"
        )
        .is_err()
    );
}

#[test]
fn raw_bitfield_helpers_match_safe_access_across_byte_boundaries() {
    use iree_native::raw_base::__BindgenBitfieldUnit;
    for width in [1u8, 3, 8, 31, 64] {
        for offset in [0usize, 1, 7, 32] {
            for value in [0u64, 1, u64::MAX, 0x1234_5678_9abc_def0] {
                let mut field = __BindgenBitfieldUnit::new([0u8; 16]);
                field.set(offset, width, value);
                let expected = field.get(offset, width);
                // SAFETY: field is initialized, aligned, and live; the entire
                // tested range fits its 128 bits and there is no concurrent write.
                let raw = unsafe { __BindgenBitfieldUnit::raw_get(&field, offset, width) };
                assert_eq!(raw, expected);
                // SAFETY: field is exclusively borrowed, and the same in-bounds
                // range is writable throughout the operation.
                unsafe {
                    __BindgenBitfieldUnit::raw_set(&mut field, offset, width, !value);
                }
                let mask = u64::MAX >> (64 - width);
                assert_eq!(field.get(offset, width), !value & mask);
            }
        }
    }
}

#[test]
fn flexible_array_and_union_helpers_document_borrowing_contracts()
-> Result<(), Box<dyn std::error::Error>> {
    let output = normalize_bindings(
        r#"
        impl<T> __IncompleteArrayField<T> {
            pub unsafe fn as_slice(&self, len: usize) -> &[T] {
                unsafe { ::core::slice::from_raw_parts(self.as_ptr(), len) }
            }
        }
        impl<T> __BindgenUnionField<T> {
            pub unsafe fn as_mut(&mut self) -> &mut T {
                unsafe { ::core::mem::transmute(self) }
            }
        }
    "#,
    )?;
    assert_eq!(output.matches("# Safety").count(), 2);
    assert_eq!(output.matches("// SAFETY:").count(), 2);
    assert!(output.contains("isize::MAX"));
    assert!(output.contains("valid, initialized T"));
    assert!(output.contains("Mutable access requires exclusivity"));
    syn::parse_file(&output)?;
    Ok(())
}

#[test]
fn tooling_registry_is_linked_without_compiling_a_second_copy() {
    let path = std::path::Path::new(env!("OUT_DIR")).join("api-audit.json");
    let audit: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    for field in ["support_sources", "binding_support_sources"] {
        for source in audit[field].as_array().unwrap() {
            assert_ne!(
                std::path::Path::new(source.as_str().unwrap()).file_name(),
                Some(std::ffi::OsStr::new("flags.c")),
                "tooling must share the installed SDK registry"
            );
        }
    }
}

#[test]
fn syntax_inventory_ignores_formatting_and_comments() {
    let source = r#"
        // pub struct not_an_export;
        #[doc = "pub const also_not_an_export: u32 = 1;"]
        pub struct iree_record_t { pub member: u32 }
        impl iree_record_t { pub fn method(&self) {} }
        pub union iree_union_t { pub value: u32 }
        pub enum iree_enum_t { Value }
        pub type iree_alias_t = u32;
        pub const IREE_VALUE: u32 = 1;
        pub use self::{
            iree_record_t as iree_record_alias_t,
            iree_alias_t as r#type,
        };
        unsafe extern "C" { pub fn iree_function(
            value: iree_record_t,
        ); pub fn mlirFunction(); pub static mut iree_global: u32; }
        extern "C" { pub fn iree_legacy(); }
        struct private_record;
        pub(crate) struct restricted_record;
        unsafe extern "C" { fn iree_private(); pub fn unrelated(); }
    "#;
    let inventory = Inventory::parse(source).unwrap();
    assert_eq!(
        inventory.exports.into_iter().collect::<Vec<_>>(),
        [
            "IREE_VALUE",
            "iree_alias_t",
            "iree_enum_t",
            "iree_function",
            "iree_global",
            "iree_legacy",
            "iree_record_alias_t",
            "iree_record_t",
            "iree_union_t",
            "mlirFunction",
            "r#type",
        ]
    );
    assert_eq!(
        inventory.functions.into_iter().collect::<Vec<_>>(),
        ["iree_function", "iree_legacy", "mlirFunction"]
    );
}

#[test]
fn shared_bindgen_helpers_are_inventoried_without_methods() {
    let inventory = Inventory::parse(
        "pub struct __BindgenBitfieldUnit<Storage> { storage: Storage }
         impl<Storage> __BindgenBitfieldUnit<Storage> { pub fn new() {} }
         pub struct __IncompleteArrayField<T>(T);",
    )
    .unwrap();
    assert_eq!(
        inventory.exports.into_iter().collect::<Vec<_>>(),
        ["__BindgenBitfieldUnit", "__IncompleteArrayField"]
    );
    assert!(inventory.functions.is_empty());
}

#[test]
fn invalid_generated_rust_fails_closed() {
    assert!(Inventory::parse("pub struct").is_err());
}

/// Pool constructors are public HAL API; the rest of each pool stays internal.
const PUBLIC_POOL_CONSTRUCTORS: &[&str] = &[
    "iree_hal_tlsf_pool_create",
    "iree_hal_fixed_block_pool_create",
];

#[test]
fn implementation_utilities_are_not_generated_public_functions() {
    let source =
        std::fs::read_to_string(std::path::Path::new(env!("OUT_DIR")).join("base.rs")).unwrap();
    let inventory = Inventory::parse(&source).unwrap();
    for prefix in [
        "iree_sysfs_",
        "iree_prng_",
        "iree_json_",
        "iree_base64_",
        "iree_spsc_",
        "iree_mpsc_",
        "iree_atomic_freelist_",
        "iree_atomic_slist_",
        "iree_fpu_state_",
        "iree_vm_bytecode_dispatch_",
        "iree_hal_resource_set_",
        "iree_hal_deferred_command_buffer_",
        "iree_hal_deferred_work_queue_",
        "iree_hal_collective_batch_",
        "iree_hal_profile_event_ring_",
        "iree_hal_stream_tracing_",
        "iree_hal_memory_trace_",
        "iree_hal_tlsf_pool_",
        "iree_hal_pool_set_",
        "iree_hal_fixed_block_pool_",
        "iree_hal_slab_cache_",
        "iree_hal_memory_arena_",
        "iree_hal_memory_tlsf_",
        "iree_hal_memory_fixed_block_",
        "iree_hal_device_queue_emulated_",
        "iree_arena_",
        "iree_arena_block_pool_",
        "iree_hal_queue_emulated_",
    ] {
        assert!(
            inventory
                .functions
                .iter()
                .filter(|name| !PUBLIC_POOL_CONSTRUCTORS.contains(&name.as_str()))
                .all(|name| !name.starts_with(prefix)),
            "implementation function leaked: {prefix}"
        );
    }
    for name in [
        "iree_vm_bytecode_module_t",
        "iree_vm_bytecode_module_state_t",
        "iree_vm_bytecode_frame_storage_t",
        "iree_vm_registers_t",
        "iree_vm_bytecode_block_t",
        "iree_vm_bytecode_block_list_t",
    ] {
        assert!(
            !inventory.exports.contains(name),
            "private layout leaked: {name}"
        );
    }
}

#[test]
fn generated_families_have_complete_exports_and_link_tests() {
    let out = std::path::Path::new(env!("OUT_DIR"));
    let index = std::fs::read_to_string(out.join("bindings.rs")).unwrap();
    let link_tests = std::fs::read_to_string(out.join("link_tests.rs")).unwrap();
    let mut families = 0;
    let mut helpers = std::collections::BTreeSet::new();
    for item in syn::parse_file(&index).unwrap().items {
        let syn::Item::Mod(module) = item else {
            continue;
        };
        let Some(path) = module
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("path"))
        else {
            continue;
        };
        let syn::Meta::NameValue(path) = &path.meta else {
            panic!("generated family path must be a string");
        };
        let syn::Expr::Lit(path) = &path.value else {
            panic!("generated family path must be a string");
        };
        let syn::Lit::Str(path) = &path.lit else {
            panic!("generated family path must be a string");
        };
        let source = std::fs::read_to_string(out.join(path.value())).unwrap();
        let inventory = Inventory::parse(&source).unwrap();
        for name in inventory.exports {
            if name.starts_with("__") && !helpers.insert(name.clone()) {
                assert_eq!(
                    index.matches(&format!("::{name};")).count(),
                    1,
                    "shared bindgen helper must be imported only once"
                );
                continue;
            }
            let visibility = if name.starts_with("__") { "" } else { "pub " };
            assert!(
                index.contains(&format!("{visibility}use {}::{name};", module.ident)),
                "missing root import for {}::{name}",
                module.ident
            );
            if name.starts_with("__") {
                assert!(!index.contains(&format!("pub use {}::{name};", module.ident)));
            }
        }
        for name in inventory.functions {
            assert!(
                link_tests.contains(&format!("super::{name} as *const ()")),
                "missing executable link test for {name}"
            );
        }
        families += 1;
    }
    assert!(families > 0, "no generated API families were checked");
}

#[test]
fn native_examples_keep_closing_fences_and_typed_blocks() {
    let comment = "Example:\n```\niree_call();\n```\n\
                   ```c\nint x;\n```\n```rust\nlet x = 1;\n```\n";
    assert_eq!(
        document_native_examples(comment),
        comment.replacen("```\n", "```text\n", 1)
    );
}

#[test]
fn native_parameter_lists_separate_following_prose_without_changing_examples() {
    let comment = "Parameters:\n - `path`: output path.\n - `userData`: callback state\n   retained by the caller.\n On failure, returns null.\n";
    assert_eq!(
        document_native_examples(comment),
        comment.replace("\n On failure", "\n\n On failure")
    );
    let example = "```c\n- example\nnot a list paragraph\n```\n";
    assert_eq!(document_native_examples(example), example);
    let crlf = "- parameter\r\nFollowing paragraph.\r\n";
    assert_eq!(
        document_native_examples(crlf),
        "- parameter\r\n\r\nFollowing paragraph.\r\n"
    );
}

#[test]
fn native_examples_support_indentation_tildes_and_crlf() {
    assert_eq!(
        document_native_examples("  ~~~~ \r\ncode\r\n  ~~~~\r\n```\nx\n```"),
        "  ~~~~text \r\ncode\r\n  ~~~~\r\n```text\nx\n```"
    );
    assert_eq!(document_native_examples("No examples."), "No examples.");
}

#[test]
fn shorter_or_different_fences_do_not_close_an_example() {
    assert_eq!(
        document_native_examples("````\n```\n~~~\n````\n```\nx\n```\n"),
        "````text\n```\n~~~\n````\n```text\nx\n```\n"
    );
}

#[test]
fn upstream_inline_forwarders_remain_linkable() {
    let generated =
        std::fs::read_to_string(std::path::Path::new(env!("OUT_DIR")).join("base.rs")).unwrap();
    let inventory = Inventory::parse(&generated).unwrap();
    for name in [
        "iree_allocator_system",
        "iree_allocator_null",
        "iree_status_free",
        "iree_hal_buffer_retain",
        "iree_hal_buffer_release",
        "iree_hal_buffer_view_type",
        "iree_hal_device_create_params_default",
        "iree_vm_make_undefined_type_def",
    ] {
        assert!(
            inventory.functions.contains(name),
            "upstream forwarder missing: {name}"
        );
    }
}

#[test]
fn compiler_plugins_do_not_wrap_private_cpp_interfaces() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(!root.join("native").join("compiler_plugins").exists());
    for path in [
        "native/bindings.rs",
        "native/wrapper.h",
        "native/targets.cmake",
        "native/config.rs",
        "Cargo.toml",
    ] {
        let source = std::fs::read_to_string(root.join(path)).unwrap();
        for forbidden in [
            "experimental-plugins",
            "compiler_plugins/adapter",
            "iree_native_compiler_exports",
            "IreeCompilerPlugin",
            "PluginAPI/PluginEntryPoint",
        ] {
            assert!(!source.contains(forbidden), "{path}: {forbidden}");
        }
    }
}

#[test]
fn entry_headers_have_one_public_include_list_and_only_required_support_sources() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let native = root.join("native");
    let entries: Vec<_> = std::fs::read_dir(&native)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    let mut headers: Vec<_> = entries
        .iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "h"))
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    headers.sort();
    assert_eq!(headers, ["wrapper.h"]);
    let mut support_sources: Vec<_> = entries
        .iter()
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| matches!(ext.to_str(), Some("c" | "m" | "cpp")))
        })
        .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    support_sources.sort();
    assert_eq!(support_sources, ["compiler_runtime.c", "hal_interop.c"]);
    let generator = std::fs::read_to_string(native.join("bindings.rs")).unwrap();
    assert_eq!(generator.matches("#include").count(), 1);
    let targets = std::fs::read_to_string(native.join("targets.cmake")).unwrap();
    assert!(targets.contains("iree_runtime_unified,INTERFACE_INCLUDE_DIRECTORIES"));
    assert!(targets.contains("iree_runtime_unified,INTERFACE_COMPILE_DEFINITIONS"));
    let header = std::fs::read_to_string(native.join("wrapper.h")).unwrap();
    for gate in [
        "IREE_NATIVE_IO_INDEX",
        "IREE_NATIVE_IO_PARAMETERS",
        "IREE_NATIVE_IO_FORMAT_GGUF",
        "IREE_NATIVE_IO_FORMAT_IRPA",
        "IREE_NATIVE_IO_FORMAT_SAFETENSORS",
    ] {
        assert!(header.contains(gate));
    }
}
