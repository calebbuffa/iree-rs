//! Hugging Face normalizer JSON cases adapted from IREE's upstream tests.

use ireetok::{Code, Normalizer, NormalizerFlags, Result};

fn normalize(component: &Normalizer, input: &str) -> Result<Vec<u8>> {
    let mut state = component.state()?;
    let mut output = vec![0; input.len().saturating_mul(16).saturating_add(64)];
    let progress = state.process(input, &mut output, NormalizerFlags::SEGMENT_END)?;
    assert_eq!(progress.consumed, input.len());

    let mut written = progress.written;
    loop {
        let count = state.finalize(&mut output[written..])?;
        written += count;
        if !state.has_pending() {
            break;
        }
        assert_ne!(count, 0, "normalizer retained output without progress");
    }

    output.truncate(written);
    Ok(output)
}

#[test]
fn huggingface_normalizer_json_parses_common_components() -> Result<()> {
    assert!(Normalizer::from_huggingface_json("null")?.is_none());
    assert_eq!(
        normalize(
            &Normalizer::from_huggingface_json(r#"{"type":"Lowercase"}"#)?.unwrap(),
            "HELLO",
        )?,
        b"hello"
    );
    assert_eq!(
        normalize(
            &Normalizer::from_huggingface_json(r#"{"type":"NFC"}"#)?.unwrap(),
            "e\u{301}",
        )?,
        "é".as_bytes()
    );
    assert_eq!(
        normalize(
            &Normalizer::from_huggingface_json(
                r#"{"type":"Strip","strip_left":true,"strip_right":true}"#,
            )?
            .unwrap(),
            "  hello \n",
        )?,
        b"hello"
    );
    assert_eq!(
        normalize(
            &Normalizer::from_huggingface_json(r#"{"type":"Prepend","prepend":"▁"}"#)?.unwrap(),
            "hello",
        )?,
        "▁hello".as_bytes()
    );
    assert_eq!(
        normalize(
            &Normalizer::from_huggingface_json(r#"{"type":"StripAccents"}"#)?.unwrap(),
            "Cafe\u{301}",
        )?,
        b"Cafe"
    );
    assert_eq!(
        normalize(
            &Normalizer::from_huggingface_json(
                r#"{"type":"BertNormalizer","clean_text":true,"handle_chinese_chars":true,"strip_accents":null,"lowercase":true}"#,
            )?
            .unwrap(),
            "Café",
        )?,
        b"cafe"
    );
    Ok(())
}

#[test]
fn huggingface_normalizer_json_sequences_compact_and_compose() -> Result<()> {
    assert!(
        Normalizer::from_huggingface_json(r#"{"type":"Sequence","normalizers":[]}"#)?.is_none()
    );

    let single = Normalizer::from_huggingface_json(
        r#"{"type":"Sequence","normalizers":[{"type":"Lowercase"}]}"#,
    )?
    .unwrap();
    assert_eq!(normalize(&single, "HELLO")?, b"hello");

    let sequence = Normalizer::from_huggingface_json(
        r#"{"type":"Sequence","normalizers":[{"type":"Lowercase"},{"type":"Strip","strip_left":true,"strip_right":true}]}"#,
    )?
    .unwrap();
    assert_eq!(normalize(&sequence, "  HELLO \n")?, b"hello");

    let regex_replace = Normalizer::from_huggingface_json(
        r#"{"type":"Replace","pattern":{"Regex":"\\s+"},"content":" "}"#,
    )?
    .unwrap();
    assert_eq!(normalize(&regex_replace, "hello \t world")?, b"hello world");
    Ok(())
}

#[test]
fn huggingface_normalizer_json_reports_invalid_and_unsupported_values() {
    for (json, expected) in [
        (r#"{"type":"Strip"}"#, Code::NotFound),
        (r#"{"type":"BertNormalizer"}"#, Code::NotFound),
        (r#"{"type":"Replace","content":"X"}"#, Code::NotFound),
        (
            r#"{"type":"Replace","pattern":{"String":"X"}}"#,
            Code::NotFound,
        ),
        (
            r#"{"type":"Replace","pattern":{"String":""},"content":"X"}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"type":"Replace","pattern":{},"content":"X"}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"type":"Replace","pattern":{"String":"X"},"content":"Y","extra":true}"#,
            Code::InvalidArgument,
        ),
        (r#"{"type":"UnknownNormalizer"}"#, Code::Unimplemented),
        (r#"{"not_type":"Lowercase"}"#, Code::NotFound),
    ] {
        assert!(
            matches!(Normalizer::from_huggingface_json(json), Err(error) if error.code() == expected),
            "unexpected parser result for {json}"
        );
    }
}

#[test]
fn huggingface_precompiled_normalizer_validates_charsmap_data() -> Result<()> {
    assert!(
        Normalizer::from_huggingface_json(
            r#"{"type":"Precompiled","precompiled_charsmap":"AAAAAA=="}"#
        )?
        .is_some()
    );
    assert!(
        Normalizer::from_huggingface_json(r#"{"type":"Precompiled","precompiled_charsmap":""}"#)?
            .is_none()
    );

    for json in [
        r#"{"type":"Precompiled"}"#,
        r#"{"type":"Precompiled","precompiled_charsmap":"!!!"}"#,
        r#"{"type":"Precompiled","precompiled_charsmap":"AQI="}"#,
        r#"{"type":"Precompiled","precompiled_charsmap":"BQAAAAAAAAAAA=="}"#,
    ] {
        let result = Normalizer::from_huggingface_json(json);
        assert!(
            result.is_err(),
            "invalid charsmap unexpectedly parsed: {json}"
        );
    }
    Ok(())
}
