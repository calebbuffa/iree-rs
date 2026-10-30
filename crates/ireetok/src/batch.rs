//! Native tokenizer batch operations.
use crate::state::OffsetRun;
use crate::{
    Code, DecodeOptions, EncodeOptions, Encoded, Error, Offset, Result, TokenId, Tokenizer,
    capacity_for, check, n, span, view, word_storage,
};
use std::ptr;
/// Options for one item in a batch encode call.
pub struct BatchEncodeInput {
    /// First sequence to tokenize.
    pub text: String,
    /// Optional second sequence for pair encoding.
    pub text_pair: Option<String>,
}

impl Tokenizer {
    /// Encodes multiple inputs in one native batch operation.
    pub fn encode_batch(
        &self,
        inputs: &[BatchEncodeInput],
        options: EncodeOptions,
    ) -> Result<Vec<Encoded>> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        let max_len = inputs
            .iter()
            .map(|x| {
                x.text
                    .len()
                    .max(x.text_pair.as_ref().map_or(0, String::len))
            })
            .max()
            .unwrap_or(0);
        let mut state_size = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_encode_state_calculate_size(self.0.as_ptr(), &mut state_size)
        })?;
        let mut state = word_storage(state_size);
        // SAFETY: All pointer arguments refer to live storage for this call.
        let transform_size = unsafe { n::iree_tokenizer_transform_buffer_oneshot_size(max_len) };
        let mut transform = word_storage(transform_size);
        let mut offset_runs = vec![OffsetRun::default(); max_len.saturating_add(16)];
        let run_list = n::iree_tokenizer_offset_run_list_t {
            capacity: offset_runs.len(),
            values: offset_runs.as_mut_ptr().cast(),
        };
        let mut id_buffers = Vec::with_capacity(inputs.len());
        let mut offset_buffers = Vec::with_capacity(inputs.len());
        let include_type_ids = options.add_special_tokens && self.max_special_token_count() > 0;
        let mut type_buffers = Vec::with_capacity(inputs.len());
        for input in inputs {
            let cap = capacity_for(
                input
                    .text
                    .len()
                    .saturating_add(input.text_pair.as_ref().map_or(0, String::len)),
            )?;
            id_buffers.push(vec![0; cap]);
            offset_buffers.push(if options.track_offsets {
                vec![Offset::default(); cap]
            } else {
                Vec::new()
            });
            type_buffers.push(if include_type_ids {
                vec![0; cap]
            } else {
                Vec::new()
            });
        }
        let mut native = Vec::with_capacity(inputs.len());
        for (i, input) in inputs.iter().enumerate() {
            let flags = if input.text_pair.is_some() {
                n::IREE_TOKENIZER_ENCODE_BATCH_ITEM_FLAG_HAS_TEXT_PAIR as u32
            } else {
                0
            };
            native.push(n::iree_tokenizer_encode_batch_item_t {
                text: view(&input.text),
                text_pair: input.text_pair.as_deref().map_or_else(|| view(""), view),
                flags,
                output: n::iree_tokenizer_token_output_t {
                    capacity: id_buffers[i].len(),
                    token_ids: id_buffers[i].as_mut_ptr(),
                    token_offsets: if options.track_offsets {
                        offset_buffers[i].as_mut_ptr().cast()
                    } else {
                        ptr::null_mut()
                    },
                    type_ids: if include_type_ids {
                        type_buffers[i].as_mut_ptr()
                    } else {
                        ptr::null_mut()
                    },
                },
                out_token_count: 0,
            });
        }
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_encode_batch(
                self.0.as_ptr(),
                native.as_mut_ptr(),
                native.len(),
                options.flags(),
                span(&mut state),
                span(&mut transform),
                run_list,
            )
        })?;
        let mut result = Vec::with_capacity(inputs.len());
        for (i, item) in native.iter().enumerate() {
            let count = item.out_token_count;
            id_buffers[i].truncate(count);
            offset_buffers[i].truncate(count);
            type_buffers[i].truncate(count);
            result.push(Encoded {
                ids: std::mem::take(&mut id_buffers[i]),
                offsets: options
                    .track_offsets
                    .then(|| std::mem::take(&mut offset_buffers[i])),
                type_ids: include_type_ids.then(|| std::mem::take(&mut type_buffers[i])),
            });
        }
        Ok(result)
    }

    /// Decodes multiple token sequences to bytes in one native batch operation.
    pub fn decode_batch_bytes(
        &self,
        inputs: &[Vec<TokenId>],
        options: DecodeOptions,
    ) -> Result<Vec<Vec<u8>>> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        let max_tokens = inputs.iter().map(Vec::len).max().unwrap_or(0);
        let cap = capacity_for(max_tokens.saturating_mul(self.vocab().max_token_length()))?;
        let mut state_size = 0;
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decode_state_calculate_size(self.0.as_ptr(), &mut state_size)
        })?;
        let mut storage = word_storage(state_size);
        let mut output_buffers = vec![vec![0u8; cap]; inputs.len()];
        let mut native = inputs
            .iter()
            .zip(output_buffers.iter_mut())
            .map(|(tokens, output)| n::iree_tokenizer_decode_batch_item_t {
                tokens: n::iree_tokenizer_token_id_list_t {
                    count: tokens.len(),
                    values: tokens.as_ptr(),
                },
                text_output: n::iree_mutable_string_view_t {
                    data: output.as_mut_ptr().cast(),
                    size: output.len(),
                },
                out_text_length: 0,
            })
            .collect::<Vec<_>>();
        // SAFETY: All pointer arguments refer to live storage for this call.
        check(unsafe {
            n::iree_tokenizer_decode_batch(
                self.0.as_ptr(),
                native.as_mut_ptr(),
                native.len(),
                options.flags(),
                span(&mut storage),
            )
        })?;
        Ok(native
            .iter()
            .zip(output_buffers.iter_mut())
            .map(|(item, text)| {
                text.truncate(item.out_text_length);
                std::mem::take(text)
            })
            .collect())
    }

    /// Decodes multiple token sequences and validates each output as UTF-8.
    pub fn decode_batch(
        &self,
        inputs: &[Vec<TokenId>],
        options: DecodeOptions,
    ) -> Result<Vec<String>> {
        self.decode_batch_bytes(inputs, options)?
            .into_iter()
            .map(|bytes| {
                String::from_utf8(bytes).map_err(|e| Error::new(Code::DataLoss, e.to_string()))
            })
            .collect()
    }
}
