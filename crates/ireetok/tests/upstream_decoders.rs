//! Portable decoder cases adapted from IREE's upstream tokenizer tests.

use ireetok::{Code, Decoder, MetaspacePrependScheme, Result, WordPieceDecoderConfig};

fn decode_with_capacity(
    decoder: &Decoder,
    tokens: &[&[u8]],
    output_capacity: usize,
) -> Result<String> {
    let mut state = decoder.state()?;
    let mut output = Vec::new();
    let mut consumed = 0;

    while consumed < tokens.len() {
        let mut bytes = vec![0; output_capacity];
        let progress = state.process(&tokens[consumed..], &mut bytes)?;
        if progress.consumed == 0 && progress.written == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "decoder did not make progress",
            ));
        }
        consumed += progress.consumed;
        output.extend_from_slice(&bytes[..progress.written]);
    }

    loop {
        let mut bytes = vec![0; output_capacity];
        let written = state.finalize(&mut bytes)?;
        output.extend_from_slice(&bytes[..written]);
        if !state.has_pending() {
            break;
        }
        if written == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "decoder retained output without making progress",
            ));
        }
    }

    String::from_utf8(output)
        .map_err(|error| ireetok::Error::new(Code::DataLoss, error.to_string()))
}

fn decode(decoder: &Decoder, tokens: &[&[u8]]) -> Result<String> {
    decode_with_capacity(decoder, tokens, 128)
}

fn decode_batches(decoder: &Decoder, batches: &[&[&[u8]]]) -> Result<String> {
    let mut state = decoder.state()?;
    let mut output = Vec::new();
    let mut buffer = [0; 128];

    for batch in batches {
        let progress = state.process(batch, &mut buffer)?;
        if progress.consumed != batch.len() {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "decoder did not consume the complete token batch",
            ));
        }
        output.extend_from_slice(&buffer[..progress.written]);
    }

    loop {
        let written = state.finalize(&mut buffer)?;
        output.extend_from_slice(&buffer[..written]);
        if !state.has_pending() {
            break;
        }
        if written == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "decoder retained output without making progress",
            ));
        }
    }

    String::from_utf8(output)
        .map_err(|error| ireetok::Error::new(Code::DataLoss, error.to_string()))
}

fn decode_huggingface_json(json: &str, tokens: &[&[u8]]) -> Result<Option<String>> {
    let Some(decoder) = Decoder::from_huggingface_json(json)? else {
        return Ok(None);
    };
    decode(&decoder, tokens).map(Some)
}

#[test]
fn huggingface_decoder_json_parses_supported_components() -> Result<()> {
    assert_eq!(
        decode_huggingface_json(r#"{"type":"ByteFallback"}"#, &[b"<0xC3>", b"<0xA9>"])?,
        Some("é".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"ByteLevel","add_prefix_space":true,"trim_offsets":true}"#,
            &["Ġhello".as_bytes()],
        )?,
        Some(" hello".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"Metaspace","replacement":"▁","prepend_scheme":"first","split":true}"#,
            &["▁hello".as_bytes(), "▁world".as_bytes()],
        )?,
        Some("hello world".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"Replace","pattern":{"String":"▁"},"content":" "}"#,
            &["▁hello▁world".as_bytes()],
        )?,
        Some(" hello world".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"Strip","content":" ","start":1,"stop":0}"#,
            &[b" hello"],
        )?,
        Some("hello".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r###"{"type":"WordPiece","prefix":"##","cleanup":true}"###,
            &[b"hello", b"##s", b"."],
        )?,
        Some("hellos.".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"CTC","pad_token":"<pad>","word_delimiter_token":"|","cleanup":false}"#,
            &[b"<pad>", b"h", b"i", b"|", b"t", b"h", b"e", b"r", b"e"],
        )?,
        Some("hi|there".into())
    );
    Ok(())
}

#[test]
fn huggingface_decoder_json_compacts_noop_and_sequence_children() -> Result<()> {
    assert!(Decoder::from_huggingface_json("null")?.is_none());
    assert!(Decoder::from_huggingface_json(r#"{"type":"Fuse"}"#)?.is_none());
    assert_eq!(
        decode_huggingface_json(r#"{"type":"Sequence","decoders":[]}"#, &[b"hello"])?,
        None
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"Sequence","decoders":[{"type":"Fuse"}]}"#,
            &[b"hello"],
        )?,
        None
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"Sequence","decoders":[{"type":"Replace","pattern":{"String":"▁"},"content":" "},{"type":"Fuse"}]}"#,
            &["▁hello".as_bytes()],
        )?,
        Some(" hello".into())
    );
    assert_eq!(
        decode_huggingface_json(
            r#"{"type":"Sequence","decoders":[{"type":"ByteFallback"},{"type":"Metaspace","replacement":"▁","prepend_scheme":"always"}]}"#,
            &[
                b"<0xE2>", b"<0x96>", b"<0x81>", b"hello", b"<0xE2>", b"<0x96>", b"<0x81>",
                b"world"
            ],
        )?,
        Some("hello world".into())
    );
    Ok(())
}

#[test]
fn huggingface_decoder_json_rejects_missing_required_fields() {
    for (json, expected) in [
        (r#"{"type":"ByteLevel"}"#, Code::NotFound),
        (r#"{"type":"Metaspace"}"#, Code::NotFound),
        (r#"{"type":"WordPiece"}"#, Code::NotFound),
        (
            r#"{"type":"CTC","word_delimiter_token":"|","cleanup":true}"#,
            Code::NotFound,
        ),
        (
            r#"{"type":"CTC","pad_token":"<pad>","cleanup":true}"#,
            Code::NotFound,
        ),
        (
            r#"{"type":"CTC","pad_token":"<pad>","word_delimiter_token":"|"}"#,
            Code::NotFound,
        ),
        (
            r#"{"type":"CTC","pad_token":"<pad>","word_delimiter_token":"|","cleanup":"yes"}"#,
            Code::InvalidArgument,
        ),
        (r#"{"type":"UnknownDecoder"}"#, Code::Unimplemented),
        (r#"{"not_type":"ByteFallback"}"#, Code::NotFound),
    ] {
        assert!(
            matches!(Decoder::from_huggingface_json(json), Err(error) if error.code() == expected),
            "unexpected parser result for {json}"
        );
    }
}

#[test]
fn passthrough_decoder_preserves_token_bytes_and_stream_order() -> Result<()> {
    let decoder = Decoder::passthrough()?;
    assert_eq!(
        decode(
            &decoder,
            &[b"", b"Hello", b" ", "日本語".as_bytes(), b"\t\n", b"world"]
        )?,
        "Hello 日本語\t\nworld"
    );
    assert_eq!(
        decode_with_capacity(&decoder, &[b"ab", b"cd", b"ef"], 4)?,
        "abcdef"
    );
    Ok(())
}

#[test]
fn wordpiece_decodes_continuations_punctuation_and_contractions() -> Result<()> {
    let decoder = Decoder::wordpiece(&WordPieceDecoderConfig {
        prefix: "##".into(),
        cleanup: true,
    })?;

    assert_eq!(
        decode(&decoder, &[b"un", b"##believ", b"##able", b"."])?,
        "unbelievable."
    );
    assert_eq!(decode(&decoder, &[b"do", b"n't"])?, "don't");
    assert_eq!(
        decode(&decoder, &[b"Hello", b",", b"world", b"!"])?,
        "Hello, world!"
    );
    Ok(())
}

#[test]
fn wordpiece_cleanup_can_be_disabled() -> Result<()> {
    let decoder = Decoder::wordpiece(&WordPieceDecoderConfig {
        prefix: "##".into(),
        cleanup: false,
    })?;

    assert_eq!(decode(&decoder, &[b"Hello", b"."])?, "Hello .");
    assert_eq!(decode(&decoder, &[b"do", b"n't"])?, "do n't");
    Ok(())
}

#[test]
fn wordpiece_decoder_resumes_with_single_byte_output() -> Result<()> {
    let decoder = Decoder::wordpiece(&WordPieceDecoderConfig {
        prefix: "##".into(),
        cleanup: true,
    })?;

    assert_eq!(
        decode_with_capacity(&decoder, &[b"Hi", b"there"], 1)?,
        "Hi there"
    );
    Ok(())
}

#[test]
fn metaspace_decoder_honors_each_prepend_policy() -> Result<()> {
    let tokens = ["▁Hello".as_bytes(), "▁World".as_bytes()];
    let always = Decoder::metaspace('▁', MetaspacePrependScheme::Always)?;
    let never = Decoder::metaspace('▁', MetaspacePrependScheme::Never)?;
    let first = Decoder::metaspace('▁', MetaspacePrependScheme::First)?;

    assert_eq!(decode(&always, &tokens)?, "Hello World");
    assert_eq!(decode(&never, &tokens)?, " Hello World");
    assert_eq!(decode(&first, &tokens)?, "Hello World");
    Ok(())
}

#[test]
fn byte_level_decoder_reconstructs_utf8_across_tokens() -> Result<()> {
    let decoder = Decoder::byte_level()?;
    assert_eq!(decode(&decoder, &["Ã".as_bytes(), "©".as_bytes()])?, "é");
    assert_eq!(decode(&decoder, &["Ġhello".as_bytes()])?, " hello");
    Ok(())
}

#[test]
fn byte_fallback_decoder_reconstructs_multibyte_utf8() -> Result<()> {
    let decoder = Decoder::byte_fallback()?;
    assert_eq!(decode(&decoder, &[b"caf", b"<0xC3>", b"<0xA9>"])?, "café");
    assert_eq!(
        decode(&decoder, &[b"<0xF0>", b"<0x9F>", b"<0x98>", b"<0x80>"])?,
        "😀"
    );
    Ok(())
}

#[test]
fn byte_fallback_decoder_replaces_invalid_and_incomplete_sequences() -> Result<()> {
    let decoder = Decoder::byte_fallback()?;
    assert_eq!(decode(&decoder, &[b"<0x80>"])?, "�");
    assert_eq!(decode(&decoder, &[b"<0xC3>"])?, "�");
    assert_eq!(decode(&decoder, &[b"<0xC3>", b"<0x41>"])?, "�A");
    Ok(())
}

#[test]
fn ctc_decoder_collapses_repeats_and_replaces_word_delimiters() -> Result<()> {
    let decoder = Decoder::ctc("<pad>", "|", true)?;
    assert_eq!(
        decode(
            &decoder,
            &[
                b"<pad>", b"<pad>", b"h", b"e", b"e", b"l", b"l", b"<pad>", b"l", b"o", b"o", b"o",
                b"<pad>", b"<pad>", b"|", b"<pad>", b"w", b"o", b"o", b"o", b"r", b"<pad>",
                b"<pad>", b"l", b"l", b"d", b"<pad>", b"<pad>", b"<pad>", b"<pad>"
            ]
        )?,
        "hello world"
    );
    Ok(())
}

#[test]
fn ctc_decoder_can_preserve_the_word_delimiter() -> Result<()> {
    let decoder = Decoder::ctc("<pad>", "|", false)?;
    assert_eq!(
        decode(&decoder, &[b"h", b"i", b"|", b"t", b"h", b"e", b"r", b"e"])?,
        "hi|there"
    );
    Ok(())
}

#[test]
fn ctc_decoder_supports_custom_pad_and_word_delimiter_tokens() -> Result<()> {
    let decoder = Decoder::ctc("[PAD]", "<space>", true)?;
    assert_eq!(
        decode(
            &decoder,
            &[
                b"[PAD]", b"h", b"i", b"<space>", b"t", b"h", b"e", b"r", b"e"
            ]
        )?,
        "hi there"
    );
    Ok(())
}

#[test]
fn ctc_decoder_applies_wordpiece_cleanup_rules() -> Result<()> {
    let decoder = Decoder::ctc("<pad>", "|", true)?;
    for (tokens, expected) in [
        (&[b"hello .".as_slice()][..], "hello."),
        (&[b"what ?".as_slice()], "what?"),
        (&[b"wow !".as_slice()], "wow!"),
        (&[b"hi ,".as_slice()], "hi,"),
        (&[b"it ' s".as_slice()], "it's"),
        (&[b"do n't".as_slice()], "don't"),
        (&[b"I 'm".as_slice()], "I'm"),
        (&[b"I do not".as_slice()], "I don't"),
        (&[b"it 's".as_slice()], "it's"),
        (&[b"I 've".as_slice()], "I've"),
        (&[b"you 're".as_slice()], "you're"),
    ] {
        assert_eq!(decode(&decoder, tokens)?, expected, "{tokens:?}");
    }

    let without_cleanup = Decoder::ctc("<pad>", "|", false)?;
    assert_eq!(decode(&without_cleanup, &[b"hello ."])?, "hello .");
    Ok(())
}

#[test]
fn ctc_decoder_preserves_deduplication_across_batches() -> Result<()> {
    let decoder = Decoder::ctc("<pad>", "|", false)?;
    assert_eq!(
        decode_batches(&decoder, &[&[b"h", b"h"], &[b"h", b"e"]])?,
        "he"
    );
    assert_eq!(
        decode_batches(&decoder, &[&[b"l", b"<pad>"], &[b"l", b"o"]])?,
        "llo"
    );
    Ok(())
}

#[test]
fn replace_decoder_matches_in_multiple_tokens_and_resumes_with_small_output() -> Result<()> {
    let decoder = Decoder::replace("abc", "X")?;
    assert_eq!(decode(&decoder, &[b"1abc2", b"abc3"])?, "1X2X3");
    assert_eq!(decode_with_capacity(&decoder, &[b"1abc2abc3"], 1)?, "1X2X3");
    Ok(())
}

#[test]
fn replace_decoder_can_delete_matches() -> Result<()> {
    let decoder = Decoder::replace("X", "")?;
    assert_eq!(decode(&decoder, &[b"aXbXc"])?, "abc");
    Ok(())
}

#[test]
fn replace_decoder_rejects_empty_or_expanding_patterns() -> Result<()> {
    let empty_pattern = match Decoder::replace("", "x") {
        Err(error) => error,
        Ok(_) => {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "replace decoder accepted an empty pattern",
            ));
        }
    };
    assert_eq!(empty_pattern.code(), Code::InvalidArgument);

    let expanding_pattern = match Decoder::replace("a", "abc") {
        Err(error) => error,
        Ok(_) => {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "replace decoder accepted an expanding replacement",
            ));
        }
    };
    assert_eq!(expanding_pattern.code(), Code::InvalidArgument);
    Ok(())
}

#[test]
fn replace_decoder_covers_upstream_boundary_cases() -> Result<()> {
    let decoder = Decoder::replace("XX", "Y")?;
    let cases: &[(&[&[u8]], &str)] = &[
        (&[], ""),
        (&[b""], ""),
        (&[b"Hello"], "Hello"),
        (&[b"aXXbXXc"], "aYbYc"),
        (&[b"XXhello"], "Yhello"),
        (&[b"helloXX"], "helloY"),
        (&[b"XXXXXX"], "YYY"),
        (&[b"aaa"], "aaa"),
        (&[b"a", b"XX", b"b"], "aYb"),
    ];
    for (tokens, expected) in cases {
        assert_eq!(decode(&decoder, tokens)?, *expected);
    }

    let overlapping = Decoder::replace("aa", "X")?;
    assert_eq!(decode(&overlapping, &[b"aaa"])?, "Xa");

    let metaspace = Decoder::replace("▁", " ")?;
    assert_eq!(
        decode(&metaspace, &["▁Hello".as_bytes(), "▁World".as_bytes()])?,
        " Hello World"
    );
    Ok(())
}

#[test]
fn strip_decoder_removes_prefixes_per_token() -> Result<()> {
    let decoder = Decoder::strip(" ", 1, 0)?;
    assert_eq!(decode(&decoder, &[b" Hello", b" World"])?, "HelloWorld");
    assert_eq!(decode(&decoder, &[b"  Hello"])?, " Hello");
    Ok(())
}

#[test]
fn strip_decoder_supports_multibyte_prefixes_and_rejects_unsupported_options() -> Result<()> {
    let decoder = Decoder::strip("##", 1, 0)?;
    assert_eq!(decode(&decoder, &[b"##Hello"])?, "Hello");

    let empty_content = match Decoder::strip("", 1, 0) {
        Err(error) => error,
        Ok(_) => {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "strip decoder accepted empty content",
            ));
        }
    };
    assert_eq!(empty_content.code(), Code::InvalidArgument);

    let unsupported_stop_count = match Decoder::strip(" ", 1, 1) {
        Err(error) => error,
        Ok(_) => {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "strip decoder accepted an unsupported stop count",
            ));
        }
    };
    assert_eq!(unsupported_stop_count.code(), Code::Unimplemented);
    Ok(())
}

#[test]
fn strip_decoder_covers_upstream_boundary_cases() -> Result<()> {
    let decoder = Decoder::strip(" ", 1, 0)?;
    let cases: &[(&[&[u8]], &str)] = &[
        (&[], ""),
        (&[b"Hello"], "Hello"),
        (&[b" "], ""),
        (&[b"  "], " "),
        (&[b" Hello"], "Hello"),
        (&[b"  Hello"], " Hello"),
        (&[b"Hello World"], "Hello World"),
        (&[b" Hello "], "Hello "),
        (&[b" Hello", b" World"], "HelloWorld"),
        (&[b"Hello", b" World"], "HelloWorld"),
        (&[b" ", b"Hello"], "Hello"),
    ];
    for (tokens, expected) in cases {
        assert_eq!(decode(&decoder, tokens)?, *expected);
    }
    Ok(())
}
