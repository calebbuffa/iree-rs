# ireetok

`ireetok` provides safe Rust wrappers for IREE's tokenizer API, including
models, vocabularies, normalizers, segmenters, decoders, post-processors,
streaming state, and tiktoken formats.

Custom model, normalizer, segmenter, and decoder components implement the safe
extension traits in `ireetok::extension`. The `native` re-export is the
lower-level C API escape hatch.
