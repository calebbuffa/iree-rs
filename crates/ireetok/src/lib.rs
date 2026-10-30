//! Safe Rust interfaces for IREE's tokenizer API.
//!
//! The crate covers tokenizer models, normalization, segmentation, decoding,
//! post-processing, streaming, vocabularies, and tiktoken formats. Custom
//! components use the safe traits in [`extension`]; [`native`] exposes the
//! lower-level C API for advanced interoperability.
//!
//! ```
//! use ireetok::Tokenizer;
//!
//! let tokenizer = Tokenizer::from_huggingface_json(r###"{
//!   "model": {
//!     "type": "WordPiece", "unk_token": "[UNK]",
//!     "continuing_subword_prefix": "##", "max_input_chars_per_word": 100,
//!     "vocab": {"[UNK]": 0, "hello": 1, "world": 2, "##s": 3}
//!   },
//!   "pre_tokenizer": {"type": "Whitespace"},
//!   "decoder": {"type": "WordPiece", "prefix": "##", "cleanup": true}
//! }"###)?;
//! let ids = tokenizer.encode("hello worlds")?;
//! assert_eq!(ids, [1, 2, 3]);
//! assert_eq!(tokenizer.decode(&ids)?, "hello worlds");
//! # Ok::<(), ireetok::Error>(())
//! ```
#![deny(missing_docs, clippy::all)]

pub(crate) use iree_native as n;
pub use iree_native::{Code, Error, Result};

mod batch;
mod decoder;
pub mod extension;
mod indexes;
mod model;
mod normalizer;
mod postprocessor;
mod regex;
mod segmenter;
mod special;
mod state;
mod status;
mod tiktoken;
mod tokenizer;
mod vocab;

pub use batch::*;
pub use decoder::*;
pub use indexes::*;
pub use model::*;
pub use normalizer::{BertNormalizerFlags, Normalizer, NormalizerFlags};
pub use postprocessor::*;
pub use regex::*;
pub use segmenter::*;
pub use special::*;
pub use state::*;
pub(crate) use status::{
    bytes_view, capacity_for, check, const_bytes_view, copy_string_view, grow_capacity, span, view,
    word_storage,
};
pub use tiktoken::*;
pub use tokenizer::*;
pub use vocab::*;

pub use iree_native as native;
