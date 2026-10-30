//! Top-level Hugging Face tokenizer JSON cases adapted from IREE's tests.

use ireetok::{Code, Tokenizer};

const MINIMAL_BPE: &str = r#"{"model":{"type":"BPE","vocab":{"a":0},"merges":[]}}"#;

#[test]
fn tokenizer_json_validates_required_sections_and_version() {
    for (json, expected) in [
        ("", Code::InvalidArgument),
        (r#"{"version":"1.0"}"#, Code::NotFound),
        (
            r#"{"model":{"type":"BPE","vocab":{"a":0},"merges":[]},"unknown_field":true}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"version":"2.0","model":{"type":"BPE","vocab":{"a":0},"merges":[]}}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"model":{"type":"Unknown","vocab":{}}}"#,
            Code::Unimplemented,
        ),
    ] {
        assert!(
            matches!(Tokenizer::from_huggingface_json(json), Err(error) if error.code() == expected),
            "unexpected parser result for {json:?}"
        );
    }
}

#[test]
fn tokenizer_json_accepts_optional_sections_and_composes_normalization() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(MINIMAL_BPE)?;
    assert_eq!(tokenizer.encode("a")?, [0]);

    let lowercase = Tokenizer::from_huggingface_json(
        r#"{
          "model":{"type":"BPE","vocab":{"hello":0},"merges":[]},
          "normalizer":{"type":"Lowercase"},
          "pre_tokenizer":null,
          "decoder":null,
          "post_processor":null,
          "added_tokens":[]
        }"#,
    )?;
    assert_eq!(lowercase.encode("HELLO")?, [0]);
    Ok(())
}
