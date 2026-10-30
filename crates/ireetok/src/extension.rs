//! Safe Rust interfaces for custom tokenizer components.
//!
//! Implement a component trait and construct the corresponding owned wrapper.
//! Safe Rust extension points cover models, normalizers, segmenters, and
//! decoders; [`raw`] retains the unsafe native-vtable escape hatch.

mod decoder;
mod model;
mod normalizer;
pub mod raw;
mod segmenter;
mod support;

pub use decoder::DecoderExtension;
pub use model::ModelExtension;
pub use normalizer::NormalizerExtension;
pub use segmenter::SegmenterExtension;
