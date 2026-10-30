use crate::{Postprocessor, TokenOutput, VocabularyRef, n};

/// Mutable encode state for a postprocessor template.
pub struct PostprocessorState<'a> {
    pub(crate) _parent: &'a Postprocessor,
    pub(crate) raw: n::iree_tokenizer_postprocessor_encode_state_t,
}
impl PostprocessorState<'_> {
    /// Whether a special-token phase still has pending entries.
    pub fn has_pending(&self) -> bool {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_encode_state_has_pending(&self.raw) }
    }
    /// Emits pending prefix token IDs and returns the number written.
    pub fn emit_prefix(&mut self, output: &mut TokenOutput<'_>, offset: usize) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_emit_prefix(&mut self.raw, output.raw(), offset) }
    }
    /// Emits pending infix token IDs and returns the number written.
    pub fn emit_infix(&mut self, output: &mut TokenOutput<'_>, offset: usize) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_emit_infix(&mut self.raw, output.raw(), offset) }
    }
    /// Emits pending suffix token IDs and returns the number written.
    pub fn emit_suffix(&mut self, output: &mut TokenOutput<'_>, offset: usize) -> usize {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_emit_suffix(&mut self.raw, output.raw(), offset) }
    }
    /// Assigns the active sequence's type ID to model-produced tokens.
    pub fn assign_type_ids(&self, output: &mut TokenOutput<'_>, offset: usize, count: usize) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe {
            n::iree_tokenizer_postprocessor_assign_type_ids(&self.raw, output.raw(), offset, count)
        }
    }
    /// Begins the infix/second-sequence phase after sequence A finalization.
    pub fn begin_infix(&mut self) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_begin_infix(&mut self.raw) }
    }
    /// Begins suffix emission after model finalization.
    pub fn begin_suffix(&mut self) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe { n::iree_tokenizer_postprocessor_begin_suffix(&mut self.raw) }
    }
    /// Trims output offsets according to this template and vocabulary.
    pub fn trim_offsets(
        &mut self,
        vocab: &VocabularyRef<'_>,
        output: &mut TokenOutput<'_>,
        start: usize,
        count: usize,
    ) {
        // SAFETY: All pointer arguments refer to live storage for this call.
        unsafe {
            n::iree_tokenizer_postprocessor_trim_token_offsets(
                &mut self.raw,
                vocab.raw(),
                output.raw(),
                start,
                count,
            )
        }
    }
}
