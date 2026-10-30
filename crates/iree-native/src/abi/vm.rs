//! Value-only operations for public IREE VM type definitions.

use crate as n;

/// Extract the public VM value-type bitfield.
#[inline]
pub fn type_def_as_value(value: n::iree_vm_type_def_t) -> n::iree_vm_value_type_t {
    value.value_type_bits() as n::iree_vm_value_type_t
}

/// Reconstitute the public VM reference-type tag from its stored bitfield.
#[inline]
pub fn type_def_as_ref(value: n::iree_vm_type_def_t) -> n::iree_vm_ref_type_t {
    value.ref_type_bits() << n::IREE_VM_REF_TYPE_TAG_BITS
}

/// Whether the type definition carries a primitive value type.
#[inline]
pub fn type_def_is_value(value: n::iree_vm_type_def_t) -> bool {
    type_def_as_value(value) != n::IREE_VM_VALUE_TYPE_NONE
}

/// Whether the type definition carries a reference type.
#[inline]
pub fn type_def_is_ref(value: n::iree_vm_type_def_t) -> bool {
    type_def_as_ref(value) != n::IREE_VM_REF_TYPE_NULL as n::iree_vm_ref_type_t
}

/// Whether neither public type bitfield is set.
#[inline]
pub fn type_def_is_undefined(value: n::iree_vm_type_def_t) -> bool {
    !type_def_is_value(value) && !type_def_is_ref(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_queries_distinguish_values_refs_and_undefined() {
        let mut value = n::iree_vm_type_def_t::default();
        assert!(type_def_is_undefined(value));
        value.set_value_type_bits(n::IREE_VM_VALUE_TYPE_I32 as usize);
        assert_eq!(type_def_as_value(value), n::IREE_VM_VALUE_TYPE_I32);
        assert!(type_def_is_value(value));
        assert!(!type_def_is_ref(value));
        assert!(!type_def_is_undefined(value));
        value.set_value_type_bits(0);
        value.set_ref_type_bits(17);
        assert_eq!(type_def_as_ref(value), 17 << n::IREE_VM_REF_TYPE_TAG_BITS);
        assert!(type_def_is_ref(value));
        assert!(!type_def_is_value(value));
        assert!(!type_def_is_undefined(value));
    }
}
