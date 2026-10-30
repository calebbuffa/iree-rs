//! Portable cases adapted from IREE's tokenizer normalizer, regex, and tiktoken tests.

use ireetok::{
    BertNormalizerFlags, Code, Normalizer, NormalizerFlags, RegexCompileFlags, RegexDfa,
    TiktokenConfig, Tokenizer,
};

fn minimal_tiktoken_data() -> String {
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut data = String::new();
    for byte in 0u8..=u8::MAX {
        data.push(alphabet[usize::from(byte >> 2)] as char);
        data.push(alphabet[usize::from((byte & 0b11) << 4)] as char);
        data.push(' ');
        data.push_str(&byte.to_string());
        data.push('\n');
    }
    data
}

fn normalize(component: &Normalizer, input: &str) -> ireetok::Result<Vec<u8>> {
    let mut state = component.state()?;
    let capacity = input.len().saturating_mul(16).saturating_add(64);
    let mut output = vec![0; capacity];
    let progress = state.process(input, &mut output, NormalizerFlags::SEGMENT_END)?;
    if progress.consumed != input.len() {
        return Err(ireetok::Error::new(
            Code::DataLoss,
            "normalizer did not consume the complete test input",
        ));
    }

    let mut written = progress.written;
    loop {
        let count = state.finalize(&mut output[written..])?;
        written += count;
        if !state.has_pending() {
            break;
        }
        if count == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "normalizer retained output without making progress",
            ));
        }
    }
    output.truncate(written);
    Ok(output)
}

fn normalize_chunks(component: &Normalizer, chunks: &[&str]) -> ireetok::Result<Vec<u8>> {
    let mut state = component.state()?;
    let capacity = chunks.iter().map(|chunk| chunk.len()).sum::<usize>() * 4 + 64;
    let mut output = vec![0; capacity];
    let mut written = 0;
    for (index, chunk) in chunks.iter().enumerate() {
        let flags = if index + 1 == chunks.len() {
            NormalizerFlags::SEGMENT_END
        } else {
            NormalizerFlags::NONE
        };
        let progress = state.process(chunk, &mut output[written..], flags)?;
        if progress.consumed != chunk.len() {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "normalizer did not consume the complete test chunk",
            ));
        }
        written += progress.written;
    }

    loop {
        let count = state.finalize(&mut output[written..])?;
        written += count;
        if !state.has_pending() {
            break;
        }
        if count == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "normalizer retained output without making progress",
            ));
        }
    }
    output.truncate(written);
    Ok(output)
}

fn normalize_with_byte_output(component: &Normalizer, input: &str) -> ireetok::Result<Vec<u8>> {
    let mut state = component.state()?;
    let mut position = 0;
    let mut output = Vec::new();
    while position < input.len() || state.has_pending() {
        let remaining = input.get(position..).ok_or_else(|| {
            ireetok::Error::new(Code::DataLoss, "normalizer consumed beyond the test input")
        })?;
        let mut byte = [0];
        let progress = state.process(remaining, &mut byte, NormalizerFlags::NONE)?;
        position += progress.consumed;
        output.extend_from_slice(&byte[..progress.written]);
        if progress.consumed == 0 && progress.written == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "normalizer made no progress with one-byte output",
            ));
        }
    }

    loop {
        let mut byte = [0];
        let written = state.finalize(&mut byte)?;
        output.extend_from_slice(&byte[..written]);
        if !state.has_pending() {
            break;
        }
        if written == 0 {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                "normalizer retained output during finalization",
            ));
        }
    }
    Ok(output)
}

#[test]
fn upstream_bert_default_flags_transform_text() -> ireetok::Result<()> {
    let normalizer = Normalizer::bert(BertNormalizerFlags::DEFAULT)?;
    assert_eq!(
        normalize(&normalizer, "Hello 中Café\tWorld")?,
        "hello  中 cafe world".as_bytes()
    );
    Ok(())
}

#[test]
fn upstream_bert_clean_text_replaces_whitespace_and_removes_controls() -> ireetok::Result<()> {
    let normalizer = Normalizer::bert(BertNormalizerFlags::CLEAN_TEXT)?;
    assert_eq!(normalize(&normalizer, "a\u{1}\tb\nc\rd")?, b"a b c d");
    assert_eq!(normalize(&normalizer, "a\0b")?, b"ab");
    Ok(())
}

#[test]
fn upstream_lowercase_handles_unicode_expansions_and_streaming_capacity() -> ireetok::Result<()> {
    let normalizer = Normalizer::lowercase()?;
    for (input, expected) in [
        ("HELLO", "hello"),
        ("CAFÉ", "café"),
        ("ΩMEGA", "ωmega"),
        ("ПРИВЕТ", "привет"),
        ("İSTANBUL", "i\u{307}stanbul"),
        ("🎉你好", "🎉你好"),
    ] {
        assert_eq!(
            normalize(&normalizer, input)?,
            expected.as_bytes(),
            "{input}"
        );
    }

    let mut state = normalizer.state()?;
    let input = "İ";
    let mut position = 0;
    let mut result = Vec::new();
    while position < input.len() || state.has_pending() {
        let remaining = input.get(position..).ok_or_else(|| {
            ireetok::Error::new(Code::DataLoss, "normalizer consumed beyond the test input")
        })?;
        let mut byte = [0; 1];
        let progress = state.process(remaining, &mut byte, NormalizerFlags::NONE)?;
        position += progress.consumed;
        result.extend_from_slice(&byte[..progress.written]);
        assert!(
            progress.consumed > 0 || progress.written > 0,
            "normalizer made no progress"
        );
    }
    loop {
        let mut byte = [0; 1];
        let written = state.finalize(&mut byte)?;
        result.extend_from_slice(&byte[..written]);
        if written == 0 {
            break;
        }
    }
    assert_eq!(result, "i\u{307}".as_bytes());
    Ok(())
}

#[test]
fn upstream_nfc_composes_combining_marks() -> ireetok::Result<()> {
    let normalizer = Normalizer::nfc()?;
    assert_eq!(normalize(&normalizer, "e\u{301}")?, "é".as_bytes());
    Ok(())
}

#[test]
fn upstream_nfc_composes_hangul_across_chunks() -> ireetok::Result<()> {
    let normalizer = Normalizer::nfc()?;
    let mut state = normalizer.state()?;
    let mut output = [0; 32];
    let first = state.process("\u{1100}", &mut output, NormalizerFlags::NONE)?;
    let second = state.process(
        "\u{1161}",
        &mut output[first.written..],
        NormalizerFlags::NONE,
    )?;
    let written = first.written + second.written;
    let third = state.process(
        "\u{11a8}",
        &mut output[written..],
        NormalizerFlags::SEGMENT_END,
    )?;
    let mut written = written + third.written;
    written += state.finalize(&mut output[written..])?;

    assert_eq!(third.consumed, "\u{11a8}".len());
    assert_eq!(&output[..written], "각".as_bytes());
    Ok(())
}

#[test]
fn upstream_nfkd_decomposes_canonical_and_compatibility_characters() -> ireetok::Result<()> {
    let normalizer = Normalizer::nfkd()?;
    assert_eq!(normalize(&normalizer, "é")?, "e\u{301}".as_bytes());
    assert_eq!(normalize(&normalizer, "ﬁ")?, b"fi");
    Ok(())
}

#[test]
fn upstream_strip_accents_removes_combining_marks_but_preserves_precomposed_text()
-> ireetok::Result<()> {
    let normalizer = Normalizer::strip_accents()?;
    assert_eq!(normalize(&normalizer, "cafe\u{301}")?, b"cafe");
    assert_eq!(normalize(&normalizer, "\u{301}\u{301}")?, b"");
    assert_eq!(normalize(&normalizer, "café")?, "café".as_bytes());
    assert_eq!(normalize(&normalizer, "A\u{20dd}")?, b"A");
    assert_eq!(normalize(&normalizer, "👍🏻")?, "👍🏻".as_bytes());
    Ok(())
}

#[test]
fn upstream_strip_accents_handles_marks_across_input_chunks() -> ireetok::Result<()> {
    let normalizer = Normalizer::strip_accents()?;
    let mut state = normalizer.state()?;
    let mut output = [0; 16];
    let first = state.process("cafe", &mut output, NormalizerFlags::NONE)?;
    let second = state.process(
        "\u{301}",
        &mut output[first.written..],
        NormalizerFlags::SEGMENT_END,
    )?;
    let written = first.written
        + second.written
        + state.finalize(&mut output[first.written + second.written..])?;

    assert_eq!(written, 4);
    assert_eq!(&output[..written], b"cafe");
    Ok(())
}

#[test]
fn upstream_regex_compilation_reports_invalid_patterns() {
    for pattern in ["", "(abc", r"\q"] {
        assert!(matches!(
            RegexDfa::compile(pattern, RegexCompileFlags::NONE),
            Err(error) if error.code() == Code::InvalidArgument
        ));
    }
}

#[test]
fn upstream_regex_execution_returns_digit_match_ranges() -> ireetok::Result<()> {
    let dfa = RegexDfa::compile(r"\d+", RegexCompileFlags::NONE)?;
    assert_eq!(
        dfa.matches("abc123xyz456")?,
        [
            ireetok::Offset { start: 3, end: 6 },
            ireetok::Offset { start: 9, end: 12 },
        ]
    );
    Ok(())
}

#[test]
fn upstream_regex_replacement_handles_deletion_and_overlapping_candidates() -> ireetok::Result<()> {
    let delete_digits = Normalizer::regex_replace(r"\d+", "")?;
    assert_eq!(normalize(&delete_digits, "a123b456c")?, b"abc");

    let collapse_runs = Normalizer::regex_replace(r"a+", "X")?;
    assert_eq!(normalize(&collapse_runs, "baaab")?, b"bXb");
    Ok(())
}

#[test]
fn upstream_regex_replacement_covers_whitespace_edges_unicode_and_expansion() -> ireetok::Result<()>
{
    let collapse_spaces = Normalizer::regex_replace(r" {2,}", " ")?;
    for (input, expected) in [
        ("a  b", "a b"),
        ("a   b", "a b"),
        ("a  b   c    d", "a b c d"),
        ("  hello", " hello"),
        ("hello  ", "hello "),
        ("    ", " "),
        ("hello ", "hello "),
        ("日本  語", "日本 語"),
        ("日本語", "日本語"),
    ] {
        assert_eq!(
            normalize(&collapse_spaces, input)?,
            expected.as_bytes(),
            "{input:?}"
        );
    }

    let collapse_whitespace = Normalizer::regex_replace(r"\s+", " ")?;
    for (input, expected) in [
        ("a b", "a b"),
        ("a\tb", "a b"),
        ("a\nb", "a b"),
        ("a \n\t b", "a b"),
        ("a\r\nb", "a b"),
        (" \t hello", " hello"),
        ("hello \n ", "hello "),
        ("a\u{a0}b", "a b"),
        (" \t\n", " "),
    ] {
        assert_eq!(
            normalize(&collapse_whitespace, input)?,
            expected.as_bytes(),
            "{input:?}"
        );
    }

    let expand_whitespace = Normalizer::regex_replace(r"\s", "___")?;
    assert_eq!(normalize(&expand_whitespace, "a b")?, b"a___b");
    assert_eq!(
        normalize_with_byte_output(&expand_whitespace, "a b c")?,
        b"a___b___c"
    );

    let adjacent_matches = Normalizer::regex_replace(r"[0-9]+", "X")?;
    assert_eq!(normalize(&adjacent_matches, "a1b23c456d")?, b"aXbXcXd");
    let word = Normalizer::regex_replace(r"\w+", "X")?;
    assert_eq!(normalize(&word, "hello world")?, b"X X");
    Ok(())
}

#[test]
fn upstream_regex_replacement_rejects_invalid_configuration() {
    for (pattern, replacement) in [("", " "), ("[unclosed", " "), (" +", &"Y".repeat(33))] {
        assert!(matches!(
            Normalizer::regex_replace(pattern, replacement),
            Err(error) if error.code() == Code::InvalidArgument
        ));
    }
}

#[test]
fn upstream_normalizer_options_preserve_prefix_and_strip_semantics() -> ireetok::Result<()> {
    let conditional_prepend = Normalizer::prepend("▁", true)?;
    assert_eq!(normalize(&conditional_prepend, "word")?, "▁word".as_bytes());
    assert_eq!(
        normalize(&conditional_prepend, "▁word")?,
        "▁word".as_bytes()
    );
    assert_eq!(normalize(&conditional_prepend, "▁")?, "▁".as_bytes());
    assert_eq!(normalize(&conditional_prepend, "")?, b"");

    let literal_replace = Normalizer::replace("ab", "X")?;
    assert_eq!(normalize(&literal_replace, "zababa")?, b"zXXa");

    let strip_left = Normalizer::strip(true, false)?;
    assert_eq!(normalize(&strip_left, " \t hello \n")?, b"hello \n");
    let strip_right = Normalizer::strip(false, true)?;
    assert_eq!(normalize(&strip_right, " \t hello \n")?, b" \t hello");
    let no_strip = Normalizer::strip(false, false)?;
    assert_eq!(normalize(&no_strip, " \t hello \n")?, b" \t hello \n");
    assert_eq!(normalize(&strip_left, "")?, b"");
    assert_eq!(normalize(&strip_right, " \t\n")?, b"");
    assert_eq!(normalize(&Normalizer::strip(true, true)?, " \t\n")?, b"");

    let mut state = conditional_prepend.state()?;
    let mut output = [0; 32];
    let progress = state.process(
        "word",
        &mut output,
        NormalizerFlags::from_bits_retain(
            NormalizerFlags::FIRST_CONSUMED.bits() | NormalizerFlags::SEGMENT_END.bits(),
        ),
    )?;
    let mut written = progress.written;
    written += state.finalize(&mut output[written..])?;
    assert_eq!(progress.consumed, "word".len());
    assert_eq!(&output[..written], b"word");
    Ok(())
}

#[test]
fn upstream_literal_replacement_handles_binary_bytes_unicode_and_tiny_output() -> ireetok::Result<()>
{
    let metaspace = Normalizer::replace(" ", "▁")?;
    for (input, expected) in [
        ("", ""),
        ("hello", "hello"),
        ("a  b", "a▁▁b"),
        ("日本 語", "日本▁語"),
    ] {
        assert_eq!(normalize(&metaspace, input)?, expected.as_bytes());
    }
    assert_eq!(
        normalize_with_byte_output(&metaspace, " x")?,
        "▁x".as_bytes()
    );

    let nul = Normalizer::replace("\0", "NULL")?;
    assert_eq!(normalize(&nul, "a\0b")?, b"aNULLb");
    let delete = Normalizer::replace("X", "")?;
    assert_eq!(normalize(&delete, "aXbXXc")?, b"abc");

    let cross_chunk_match = Normalizer::replace("abc", "X")?;
    assert_eq!(
        normalize_chunks(&cross_chunk_match, &["za", "bcq"])?,
        b"zXq"
    );
    assert_eq!(
        normalize_chunks(&cross_chunk_match, &["zab", "x"])?,
        b"zabx"
    );
    assert!(matches!(
        Normalizer::replace("", "x"),
        Err(error) if error.code() == Code::InvalidArgument
    ));
    Ok(())
}

#[test]
fn upstream_strip_handles_large_and_cross_chunk_trailing_whitespace() -> ireetok::Result<()> {
    let strip_right = Normalizer::strip(false, true)?;
    let long_input = format!("text{}", " ".repeat(10_000));
    assert_eq!(normalize(&strip_right, &long_input)?, b"text");

    let mut state = strip_right.state()?;
    let mut output = [0; 32];
    let first = state.process("text ", &mut output, NormalizerFlags::NONE)?;
    let second = state.process("\t\n", &mut output[first.written..], NormalizerFlags::NONE)?;
    let third = state.process(
        " ",
        &mut output[first.written + second.written..],
        NormalizerFlags::SEGMENT_END,
    )?;
    let mut written = first.written + second.written + third.written;
    written += state.finalize(&mut output[written..])?;
    assert_eq!(&output[..written], b"text");
    Ok(())
}

#[test]
fn upstream_tiktoken_parser_rejects_non_monotone_ranks() {
    let config = TiktokenConfig::by_name("cl100k_base");
    assert!(matches!(
        config.map(|config| Tokenizer::from_tiktoken_config("Ag 2\nAA 0\n", &config)),
        Some(Err(error)) if error.code() == Code::InvalidArgument
    ));
}

#[test]
fn upstream_tiktoken_parser_rejects_malformed_records_and_patterns() -> ireetok::Result<()> {
    let config = TiktokenConfig::by_name("cl100k_base")
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "missing cl100k_base config"))?;
    for data in ["", "Z!== 0\n", "AA\n"] {
        assert!(matches!(
            Tokenizer::from_tiktoken_config(data, &config),
            Err(error) if error.code() == Code::InvalidArgument
        ));
    }

    let mut empty_pattern = config.clone();
    empty_pattern.pattern.clear();
    assert!(matches!(
        Tokenizer::from_tiktoken_config("AA 0\n", &empty_pattern),
        Err(error) if error.code() == Code::InvalidArgument
    ));

    assert!(matches!(
        Tokenizer::from_tiktoken("AA 0\n", "unknown_encoding"),
        Err(error) if error.code() == Code::NotFound
    ));
    Ok(())
}

#[test]
fn upstream_tiktoken_parser_accepts_byte_vocab_with_blank_lines_and_final_newline()
-> ireetok::Result<()> {
    let config = TiktokenConfig::by_name("cl100k_base")
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "missing cl100k_base config"))?;
    let data = minimal_tiktoken_data();
    assert!(data.ends_with('\n'));

    for input in [data.clone(), format!("\n\n{data}")] {
        let tokenizer = Tokenizer::from_tiktoken_config(&input, &config)?;
        assert_eq!(
            tokenizer.vocab().token_count(),
            256 + config.special_tokens.len()
        );
        assert_eq!(tokenizer.encode("hi")?, [104, 105]);
    }
    Ok(())
}

#[test]
fn upstream_tiktoken_named_configs_expose_expected_special_tokens() -> ireetok::Result<()> {
    let names = [
        "cl100k_base",
        "o200k_base",
        "o200k_harmony",
        "r50k_base",
        "gpt2",
        "p50k_base",
        "p50k_edit",
    ];
    let configs = names
        .iter()
        .map(|&name| {
            TiktokenConfig::by_name(name)
                .ok_or_else(|| ireetok::Error::new(Code::DataLoss, format!("missing {name}")))
        })
        .collect::<ireetok::Result<Vec<_>>>()?;

    assert_eq!(configs[3], configs[4]);
    for (name, config) in names.into_iter().zip(&configs) {
        assert!(!config.pattern.is_empty(), "{name} has no regex pattern");
    }

    let p50k_edit = &configs[6];
    assert_eq!(p50k_edit.special_tokens.len(), 4);
    for (spelling, id) in [
        ("<|fim_prefix|>", 50281),
        ("<|fim_middle|>", 50282),
        ("<|fim_suffix|>", 50283),
    ] {
        assert!(
            p50k_edit
                .special_tokens
                .iter()
                .any(|(token, token_id)| token == spelling && *token_id == id),
            "p50k_edit missing {spelling}"
        );
    }

    let harmony = &configs[2];
    assert_eq!(harmony.special_tokens.len(), 10);
    for (spelling, id) in [("<|startoftext|>", 199998), ("<|endoftext|>", 199999)] {
        assert!(
            harmony
                .special_tokens
                .iter()
                .any(|(token, token_id)| token == spelling && *token_id == id),
            "o200k_harmony missing {spelling}"
        );
    }
    assert_eq!(harmony.pattern, configs[1].pattern);
    assert!(TiktokenConfig::by_name("nonexistent").is_none());
    assert!(TiktokenConfig::by_name("").is_none());
    Ok(())
}
