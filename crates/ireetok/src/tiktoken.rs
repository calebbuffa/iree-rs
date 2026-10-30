//! tiktoken and HuggingFace format entry points backed by IREE's public parser API.
use crate::{
    Code, Error, Result, SpecialTokens, TokenId, Tokenizer, TokenizerBuilder, check,
    copy_string_view, n, view,
};
use std::ptr;
/// Configuration for loading a custom tiktoken encoding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TiktokenConfig {
    /// Regex pattern that defines pre-tokenization boundaries.
    pub pattern: String,
    /// Special spellings and their vocabulary IDs.
    pub special_tokens: Vec<(String, TokenId)>,
}

impl TiktokenConfig {
    /// Copies one of IREE's built-in tiktoken configurations by encoding name.
    pub fn by_name(name: &str) -> Option<Self> {
        // SAFETY: The lookup reads the borrowed name and returns static configuration storage.
        let raw = unsafe { n::iree_tokenizer_tiktoken_config_by_name(view(name)) };
        if raw.is_null() {
            return None;
        }
        // SAFETY: A non-null result points to a static native configuration.
        let raw = unsafe { &*raw };
        let special_tokens = (0..raw.special_token_count)
            .map(|index| {
                // SAFETY: The C config promises arrays of special_token_count elements.
                let text = unsafe { *raw.special_token_strings.add(index) };
                // SAFETY: The parallel ID array has the same documented count.
                let id = unsafe { *raw.special_token_ids.add(index) };
                (copy_string_view(text), id)
            })
            .collect();
        Some(Self {
            pattern: copy_string_view(raw.pattern),
            special_tokens,
        })
    }
}

/// Resolves one of IREE's built-in tiktoken configurations.
fn named_config(encoding: &str) -> Result<*const n::iree_tokenizer_tiktoken_config_t> {
    // SAFETY: The lookup reads the borrowed name and returns static configuration storage.
    let config = unsafe { n::iree_tokenizer_tiktoken_config_by_name(view(encoding)) };
    if config.is_null() {
        return Err(Error::new(
            Code::NotFound,
            format!("unknown tiktoken encoding {encoding:?}"),
        ));
    }
    Ok(config)
}

/// Calls `f` with a native view of `config` that borrows its strings.
fn with_native_config<R>(
    config: &TiktokenConfig,
    f: impl FnOnce(&n::iree_tokenizer_tiktoken_config_t) -> R,
) -> R {
    let (strings, ids): (Vec<_>, Vec<_>) = config
        .special_tokens
        .iter()
        .map(|(text, id)| (view(text), *id))
        .unzip();
    f(&n::iree_tokenizer_tiktoken_config_t {
        pattern: view(&config.pattern),
        special_token_count: strings.len(),
        special_token_strings: strings.as_ptr(),
        special_token_ids: ids.as_ptr(),
    })
}

fn tokenizer_from_tiktoken(
    data: &str,
    config: *const n::iree_tokenizer_tiktoken_config_t,
) -> Result<Tokenizer> {
    let mut raw = ptr::null_mut();
    // SAFETY: data and config remain valid throughout the call.
    check(unsafe {
        n::iree_tokenizer_from_tiktoken(view(data), config, crate::status::system(), &mut raw)
    })?;
    Ok(Tokenizer(crate::status::nonnull(
        raw,
        "tokenizer creation succeeded with a null output",
    )?))
}

fn builder_from_tiktoken(
    data: &str,
    config: *const n::iree_tokenizer_tiktoken_config_t,
) -> Result<TokenizerBuilder> {
    let mut builder = TokenizerBuilder::new();
    // SAFETY: Input/configuration storage and the builder outlive the native call.
    check(unsafe { n::iree_tokenizer_parse_tiktoken(view(data), config, builder.raw()) })?;
    Ok(builder)
}

impl Tokenizer {
    /// Builds from tiktoken vocabulary data and a built-in encoding name.
    pub fn from_tiktoken(data: &str, encoding: &str) -> Result<Self> {
        tokenizer_from_tiktoken(data, named_config(encoding)?)
    }

    /// Builds from a custom tiktoken configuration.
    pub fn from_tiktoken_config(data: &str, config: &TiktokenConfig) -> Result<Self> {
        with_native_config(config, |config| tokenizer_from_tiktoken(data, config))
    }
}

impl TokenizerBuilder {
    /// Parses a complete HuggingFace tokenizer document into this builder.
    pub fn from_huggingface_json(json: &str) -> Result<Self> {
        let mut builder = Self::new();
        // SAFETY: Input storage and the builder outlive the native call.
        check(unsafe { n::iree_tokenizer_parse_huggingface_json(view(json), builder.raw()) })?;
        Ok(builder)
    }

    /// Parses tiktoken data and a built-in encoding into this builder.
    pub fn from_tiktoken(data: &str, encoding: &str) -> Result<Self> {
        builder_from_tiktoken(data, named_config(encoding)?)
    }

    /// Parses custom tiktoken data and configuration into this builder.
    pub fn from_tiktoken_config(data: &str, config: &TiktokenConfig) -> Result<Self> {
        with_native_config(config, |config| builder_from_tiktoken(data, config))
    }

    /// Transfers and consumes a pre-normalization special-token collection.
    pub fn set_special_tokens(&mut self, tokens: SpecialTokens) {
        let mut tokens = tokens;
        // SAFETY: The collection is unique; this C API moves its slab into the live builder.
        unsafe { n::iree_tokenizer_builder_set_special_tokens(self.raw(), &mut tokens.0) };
        std::mem::forget(tokens);
    }

    /// Transfers and consumes a post-normalization special-token collection.
    pub fn set_special_tokens_post_norm(&mut self, tokens: SpecialTokens) {
        let mut tokens = tokens;
        // SAFETY: The collection is unique; this C API moves its slab into the live builder.
        unsafe {
            n::iree_tokenizer_builder_set_special_tokens_post_norm(self.raw(), &mut tokens.0)
        };
        std::mem::forget(tokens);
    }
}
