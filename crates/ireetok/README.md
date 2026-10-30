# ireetok

`ireetok` provides safe Rust wrappers for IREE's tokenizer API, including
models, vocabularies, normalizers, segmenters, decoders, post-processors,
streaming state, and tiktoken formats.

Custom model, normalizer, segmenter, and decoder components implement the safe
extension traits in `ireetok::extension`. The `native` re-export is the
lower-level C API escape hatch.

```toml
[dependencies]
ireetok = "0.1.0"
```

Load a Hugging Face `tokenizer.json` and use its vocabulary and transforms:

```rust,no_run
let json = std::fs::read_to_string("tokenizer.json")?;
let tokenizer = ireetok::Tokenizer::from_huggingface_json(&json)?;
let ids = tokenizer.encode("hello world")?;
let text = tokenizer.decode(&ids)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Encoding options control special tokens and byte-offset tracking. Tokenizer
instances are immutable and can be shared across threads; each streaming state
borrows its tokenizer and owns its mutable state.

Standalone segmenter states emit offsets relative to their current input.
Pending segments can refer to an earlier chunk using native unsigned
wraparound; use `Segment::absolute_range` with the current input's absolute
position, then `str::get` on retained text to check the resolved range.
