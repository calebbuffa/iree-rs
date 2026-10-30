//! Golden cases adapted from IREE's upstream Hugging Face tokenizer tests.

use ireetok::{BatchEncodeInput, EncodeOptions, TokenId, TokenOutput, Tokenizer};

const BPE_BYTELEVEL: &str = include_str!("fixtures/bpe_bytelevel_minimal.json");
const WORDPIECE_BERT_PAIR: &str = include_str!("fixtures/wordpiece_bert_pair_minimal.json");

fn encode_streaming(
    tokenizer: &Tokenizer,
    text: &str,
    chunk_size: usize,
) -> ireetok::Result<Vec<TokenId>> {
    let mut state = tokenizer.encode_state(chunk_size, EncodeOptions::default())?;
    let mut actual = Vec::new();

    let mut chunk_start = 0;
    while chunk_start < text.len() {
        let mut chunk_end = chunk_start.saturating_add(chunk_size).min(text.len());
        while !text.is_char_boundary(chunk_end) {
            chunk_end += 1;
        }
        let chunk = &text[chunk_start..chunk_end];
        let mut consumed = 0;
        while consumed < chunk.len() {
            let mut ids = [0; 128];
            let mut output = TokenOutput::new(&mut ids, None, None)?;
            let progress = state.feed(&chunk[consumed..], &mut output)?;
            assert!(progress.consumed > 0 || progress.written > 0);
            consumed += progress.consumed;
            actual.extend_from_slice(&ids[..progress.written]);
        }
        chunk_start = chunk_end;
    }

    loop {
        let mut ids = vec![0; state.pending_token_bound().max(128)];
        let mut output = TokenOutput::new(&mut ids, None, None)?;
        let written = state.finalize(&mut output)?;
        actual.extend_from_slice(&ids[..written]);
        if !state.has_pending() {
            break;
        }
    }

    Ok(actual)
}

#[test]
fn byte_level_bpe_matches_upstream_huggingface_golden_ids() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(BPE_BYTELEVEL)?;
    let ids = tokenizer.encode("hello world")?;

    assert_eq!(ids, [98, 105, 110]);
    assert_eq!(tokenizer.decode(&ids)?, "hello world");
    Ok(())
}

#[test]
fn byte_level_bpe_can_disable_special_token_matching() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(BPE_BYTELEVEL)?;
    let ordinary = tokenizer.encode("<|endoftext|>")?;
    assert_eq!(ordinary, [111]);

    let unmatched = tokenizer.encode_with(
        "<|endoftext|>",
        EncodeOptions {
            no_special_token_matching: true,
            ..EncodeOptions::default()
        },
    )?;
    assert!(!unmatched.ids.contains(&111));
    assert_eq!(
        tokenizer
            .encode_with("<|endoftext|>", EncodeOptions::default())?
            .ids,
        [111]
    );
    Ok(())
}

#[test]
fn byte_level_bpe_matches_upstream_ground_truth_cases() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(BPE_BYTELEVEL)?;
    for (text, expected) in [
        (
            "The quick brown fox",
            &[
                51, 98, 94, 80, 84, 72, 66, 74, 94, 65, 81, 78, 86, 77, 94, 69, 78, 87,
            ][..],
        ),
        ("hello<|endoftext|>world", &[98, 105, 111, 86, 108][..]),
        (
            "  hello   world  ",
            &[94, 94, 98, 105, 94, 94, 110, 94, 94][..],
        ),
        ("a", &[64][..]),
        ("12345", &[16, 17, 18, 19, 20][..]),
        ("!@#$%^&*()", &[0, 31, 2, 3, 4, 61, 5, 9, 7, 8][..]),
        ("", &[][..]),
    ] {
        assert_eq!(tokenizer.encode(text)?, expected, "input {text:?}");
    }
    Ok(())
}

#[test]
fn byte_level_bpe_streaming_matches_golden_ids_across_chunk_boundaries() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(BPE_BYTELEVEL)?;
    for (text, expected) in [
        ("hello world", &[98, 105, 110][..]),
        ("hello<|endoftext|>world", &[98, 105, 111, 86, 108][..]),
        (
            "  hello   world  ",
            &[94, 94, 98, 105, 94, 94, 110, 94, 94][..],
        ),
    ] {
        for chunk_size in 1..=text.len() {
            assert_eq!(
                encode_streaming(&tokenizer, text, chunk_size)?,
                expected,
                "chunk size {chunk_size} for {text:?}"
            );
        }
    }
    Ok(())
}

#[test]
fn wordpiece_pair_matches_upstream_template_and_type_ids() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(WORDPIECE_BERT_PAIR)?;
    let encoded = tokenizer.encode_batch(
        &[BatchEncodeInput {
            text: "Hello".into(),
            text_pair: Some("worlds".into()),
        }],
        EncodeOptions {
            add_special_tokens: true,
            ..EncodeOptions::default()
        },
    )?;

    assert_eq!(encoded[0].ids, [101, 7592, 102, 2088, 2015, 102]);
    assert_eq!(
        encoded[0].type_ids.as_deref(),
        Some(&[0, 0, 0, 1, 1, 1][..])
    );
    Ok(())
}

#[test]
fn wordpiece_single_sequence_matches_upstream_template_and_type_ids() -> ireetok::Result<()> {
    let tokenizer = Tokenizer::from_huggingface_json(WORDPIECE_BERT_PAIR)?;
    let encoded = tokenizer.encode_with(
        "Hello",
        EncodeOptions {
            add_special_tokens: true,
            ..EncodeOptions::default()
        },
    )?;

    assert_eq!(encoded.ids, [101, 7592, 102]);
    assert_eq!(encoded.type_ids.as_deref(), Some(&[0, 0, 0][..]));
    Ok(())
}
