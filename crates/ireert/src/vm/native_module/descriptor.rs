//! Immutable descriptor tables retained by a native module.

use super::trampolines::call;
use super::{Callback, Instance, NativeModule};
use crate::{Code, Error, Result};
use iree_native as n;

pub(super) struct Export {
    pub(super) name: String,
    pub(super) cconv: String,
    pub(super) attributes: Vec<(String, String)>,
    pub(super) callback: Box<Callback>,
    pub(super) layout: Option<Layout>,
}

pub(super) struct Import {
    pub(super) name: String,
    pub(super) optional: bool,
}

pub(super) struct Dependency {
    pub(super) name: String,
    pub(super) version: u32,
    pub(super) optional: bool,
}

/// Rejects empty names and adjacent duplicates in a sorted table.
pub(super) fn validate_names<'a>(names: impl Iterator<Item = &'a str>) -> Result<()> {
    let mut previous = None;
    for name in names {
        if name.is_empty() || previous == Some(name) {
            return Err(Error::new(
                Code::InvalidArgument,
                "empty or duplicate function name",
            ));
        }
        previous = Some(name);
    }
    Ok(())
}

#[derive(Debug)]
pub(super) struct Fragment {
    pub(super) size: usize,
    pub(super) slots: Vec<usize>,
    pub(super) refs: Vec<usize>,
}

impl Fragment {
    pub(super) fn parse(fragment: &str) -> Result<Self> {
        let mut result = Self {
            size: 0,
            slots: Vec::new(),
            refs: Vec::new(),
        };
        if fragment == "v" {
            return Ok(result);
        }
        // Mirrors `iree_vm_function_call_compute_cconv_fragment_size`: slots
        // are naturally aligned and the total is padded to the widest slot.
        let overflow = || Error::new(Code::OutOfRange, "call storage size overflow");
        let mut max_alignment = std::mem::size_of::<i32>();
        for character in fragment.bytes() {
            let (size, alignment) = match character {
                b'i' | b'f' => (4, 4),
                b'I' | b'F' => (8, 8),
                b'r' => (
                    std::mem::size_of::<n::iree_vm_ref_t>(),
                    std::mem::align_of::<n::iree_vm_ref_t>(),
                ),
                b'C' | b'D' => {
                    return Err(Error::new(
                        Code::Unimplemented,
                        "variadic native calling conventions are not wrapped",
                    ));
                }
                _ => {
                    return Err(Error::new(
                        Code::InvalidArgument,
                        "invalid calling convention",
                    ));
                }
            };
            max_alignment = max_alignment.max(alignment);
            let offset = result
                .size
                .checked_next_multiple_of(alignment)
                .ok_or_else(overflow)?;
            result.slots.push(offset);
            if character == b'r' {
                result.refs.push(offset);
            }
            result.size = offset.checked_add(size).ok_or_else(overflow)?;
        }
        result.size = result
            .size
            .checked_next_multiple_of(max_alignment)
            .ok_or_else(overflow)?;
        Ok(result)
    }
}

#[derive(Debug)]
pub(super) struct Layout {
    pub(super) arguments: Fragment,
    pub(super) results: Fragment,
}

impl Layout {
    pub(super) fn parse(cconv: &str) -> Result<Self> {
        let cconv = if cconv.is_empty() { "0" } else { cconv };
        let cconv = cconv
            .strip_prefix('0')
            .ok_or_else(|| Error::new(Code::InvalidArgument, "expected VM ABI version 0"))?;
        let (arguments, results) = cconv.split_once('_').unwrap_or((cconv, ""));
        Ok(Self {
            arguments: Fragment::parse(arguments)?,
            results: Fragment::parse(results)?,
        })
    }
}

pub(super) struct ModuleData {
    pub(super) definition: NativeModule,
    pub(super) _instance: Instance,
    pub(super) _exports: Box<[n::iree_vm_native_export_descriptor_t]>,
    pub(super) _export_attributes: Vec<Box<[n::iree_string_pair_t]>>,
    pub(super) _imports: Box<[n::iree_vm_native_import_descriptor_t]>,
    pub(super) _dependencies: Box<[n::iree_vm_module_dependency_t]>,
    pub(super) _attributes: Box<[n::iree_string_pair_t]>,
    pub(super) _functions: Box<[n::iree_vm_native_function_ptr_t]>,
    pub(super) descriptor: n::iree_vm_native_module_descriptor_t,
}

// SAFETY: the raw pointers address immutable heap tables owned by this value,
// and every callback is Send + Sync.
unsafe impl Send for ModuleData {}
// SAFETY: the tables are never mutated after construction.
unsafe impl Sync for ModuleData {}

fn attributes(values: &[(String, String)]) -> Box<[n::iree_string_pair_t]> {
    values
        .iter()
        .map(|(key, value)| {
            let mut pair = n::iree_string_pair_t::default();
            pair.__bindgen_anon_1.key = crate::view(key);
            pair.__bindgen_anon_2.value = crate::view(value);
            pair
        })
        .collect()
}

impl ModuleData {
    pub(super) fn new(definition: NativeModule, instance: Instance) -> Self {
        let export_attributes: Vec<_> = definition
            .exports
            .iter()
            .map(|export| attributes(&export.attributes))
            .collect();
        let exports: Box<[_]> = definition
            .exports
            .iter()
            .zip(&export_attributes)
            .map(|(export, attrs)| n::iree_vm_native_export_descriptor_t {
                local_name: crate::view(&export.name),
                calling_convention: crate::view(&export.cconv),
                attr_count: attrs.len(),
                attrs: attrs.as_ptr(),
            })
            .collect();
        let imports: Box<[_]> = definition
            .imports
            .iter()
            .map(|import| n::iree_vm_native_import_descriptor_t {
                flags: if import.optional {
                    n::IREE_VM_NATIVE_IMPORT_OPTIONAL as _
                } else {
                    n::IREE_VM_NATIVE_IMPORT_REQUIRED as _
                },
                full_name: crate::view(&import.name),
            })
            .collect();
        let dependencies: Box<[_]> = definition
            .dependencies
            .iter()
            .map(|dependency| n::iree_vm_module_dependency_t {
                name: crate::view(&dependency.name),
                minimum_version: dependency.version,
                flags: if dependency.optional {
                    n::IREE_VM_MODULE_DEPENDENCY_FLAG_OPTIONAL as _
                } else {
                    n::IREE_VM_MODULE_DEPENDENCY_FLAG_REQUIRED as _
                },
            })
            .collect();
        let attributes = attributes(&definition.attributes);
        let functions: Box<[_]> = definition
            .exports
            .iter()
            .map(|_| n::iree_vm_native_function_ptr_t {
                shim: Some(call),
                target: None,
            })
            .collect();
        let descriptor = n::iree_vm_native_module_descriptor_t {
            name: crate::view(&definition.name),
            version: definition.version,
            attr_count: attributes.len(),
            attrs: attributes.as_ptr(),
            dependency_count: dependencies.len(),
            dependencies: dependencies.as_ptr(),
            import_count: imports.len(),
            imports: imports.as_ptr(),
            export_count: exports.len(),
            exports: exports.as_ptr(),
            function_count: functions.len(),
            functions: functions.as_ptr(),
        };
        Self {
            definition,
            _instance: instance,
            _exports: exports,
            _export_attributes: export_attributes,
            _imports: imports,
            _dependencies: dependencies,
            _attributes: attributes,
            _functions: functions,
            descriptor,
        }
    }
}
