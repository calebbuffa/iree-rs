//! Unsafe native-vtable access for advanced custom components.
//!
//! Prefer the safe component traits in [`super`] unless the native ABI requires
//! behavior they do not expose.

pub use iree_native::{
    iree_string_view_t, iree_tokenizer_decoder_capability_t, iree_tokenizer_decoder_t,
    iree_tokenizer_decoder_vtable_t, iree_tokenizer_model_t, iree_tokenizer_model_vtable_t,
    iree_tokenizer_normalizer_t, iree_tokenizer_normalizer_vtable_t, iree_tokenizer_segmenter_t,
    iree_tokenizer_segmenter_vtable_t,
};

/// Initializes the base fields of a custom normalizer.
///
/// # Safety
/// `normalizer` must point to writable storage for a concrete normalizer,
/// `vtable` must point to a valid static normalizer vtable, and `state_size`
/// must match the storage required by its state callbacks. The callbacks must
/// obey IREE's contracts and must not unwind across the C ABI.
pub unsafe fn initialize_normalizer(
    normalizer: *mut iree_tokenizer_normalizer_t,
    vtable: *const iree_tokenizer_normalizer_vtable_t,
    state_size: usize,
) {
    // SAFETY: upheld by the caller's custom-component contract.
    unsafe { iree_native::iree_tokenizer_normalizer_initialize(normalizer, vtable, state_size) }
}

/// Initializes the base fields of a custom segmenter.
///
/// # Safety
/// `segmenter` must point to writable storage for a concrete segmenter,
/// `vtable` must point to a valid static segmenter vtable, and `state_size`
/// must match the storage required by its state callbacks. The callbacks must
/// obey IREE's contracts and must not unwind across the C ABI.
pub unsafe fn initialize_segmenter(
    segmenter: *mut iree_tokenizer_segmenter_t,
    vtable: *const iree_tokenizer_segmenter_vtable_t,
    state_size: usize,
) {
    // SAFETY: upheld by the caller's custom-component contract.
    unsafe { iree_native::iree_tokenizer_segmenter_initialize(segmenter, vtable, state_size) }
}

/// Initializes the base fields of a custom model.
///
/// # Safety
/// `model` must point to writable storage for a concrete model, `vtable` must
/// point to a valid static model vtable, `state_size` must match the storage
/// required by its state callbacks, and `type_name` must reference static
/// storage. The callbacks must obey IREE's contracts and must not unwind
/// across the C ABI.
pub unsafe fn initialize_model(
    model: *mut iree_tokenizer_model_t,
    vtable: *const iree_tokenizer_model_vtable_t,
    state_size: usize,
    type_name: iree_string_view_t,
) {
    // SAFETY: upheld by the caller's custom-component contract.
    unsafe { iree_native::iree_tokenizer_model_initialize(model, vtable, state_size, type_name) }
}

/// Initializes the base fields of a custom decoder.
///
/// # Safety
/// `decoder` must point to writable storage for a concrete decoder,
/// `vtable` must point to a valid static decoder vtable, and `state_size`
/// must match the storage required by its state callbacks. The callbacks must
/// obey IREE's contracts and must not unwind across the C ABI.
pub unsafe fn initialize_decoder(
    decoder: *mut iree_tokenizer_decoder_t,
    vtable: *const iree_tokenizer_decoder_vtable_t,
    state_size: usize,
    capabilities: iree_tokenizer_decoder_capability_t,
) {
    // SAFETY: upheld by the caller's custom-component contract.
    unsafe {
        iree_native::iree_tokenizer_decoder_initialize(decoder, vtable, state_size, capabilities)
    }
}
