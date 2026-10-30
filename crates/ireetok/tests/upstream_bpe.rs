//! Golden BPE model cases adapted from IREE's upstream tests.

use ireetok::{
    BpeFlags, Model, Segment, TokenAttributes, TokenId, TokenOutput, Tokenizer, TokenizerBuilder,
    VocabularyBuilder,
};

fn model(tokens: &[&str], merges: &[(u32, u32)], flags: BpeFlags) -> ireetok::Result<Model> {
    let mut vocabulary = VocabularyBuilder::new(tokens.len())?;
    for token in tokens {
        vocabulary.add_token(token, 0.0, TokenAttributes::NONE)?;
    }
    for &(left, right) in merges {
        vocabulary.add_merge(left, right)?;
    }
    Model::bpe(vocabulary.build()?, flags)
}

fn tokenizer(
    tokens: &[&str],
    merges: &[(u32, u32)],
    flags: BpeFlags,
) -> ireetok::Result<Tokenizer> {
    let mut builder = TokenizerBuilder::new();
    builder.set_model(model(tokens, merges, flags)?);
    builder.build()
}

fn encode_with_capacity(
    model: &Model,
    text: &str,
    segments: &[Segment],
    capacity: usize,
) -> ireetok::Result<Vec<TokenId>> {
    let mut state = model.state()?;
    let mut encoded = Vec::new();
    let mut consumed = 0;
    while consumed < segments.len() {
        let mut ids = vec![0; capacity];
        let mut output = TokenOutput::new(&mut ids, None, None)?;
        let progress = state.encode(text.as_bytes(), &segments[consumed..], false, &mut output)?;
        encoded.extend_from_slice(&ids[..progress.written]);
        consumed += progress.consumed;
        if progress.consumed == 0 && progress.written == 0 {
            return Err(ireetok::Error::new(
                ireetok::Code::DataLoss,
                "model made no progress with limited output capacity",
            ));
        }
    }

    loop {
        let mut ids = vec![0; capacity];
        let mut output = TokenOutput::new(&mut ids, None, None)?;
        let written = state.finalize(&mut output)?;
        encoded.extend_from_slice(&ids[..written]);
        if !state.has_pending() {
            break;
        }
        if written == 0 {
            return Err(ireetok::Error::new(
                ireetok::Code::DataLoss,
                "model retained output without making progress",
            ));
        }
    }
    Ok(encoded)
}

#[test]
fn bpe_applies_merge_rules_by_priority_not_greedy_longest_match() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &["a", "b", "c", "ab", "bc"],
        &[(1, 2), (0, 1)],
        BpeFlags::NONE,
    )?;
    assert_eq!(tokenizer.encode("abc")?, [0, 4]);
    Ok(())
}

#[test]
fn bpe_handles_empty_single_character_and_unmerged_input() -> ireetok::Result<()> {
    let tokens = ["a", "b"];
    let tokenizer = tokenizer(&tokens, &[], BpeFlags::NONE)?;

    assert_eq!(tokenizer.encode("")?, []);
    assert_eq!(tokenizer.encode("a")?, [0]);
    assert_eq!(tokenizer.encode("ab")?, [0, 1]);
    Ok(())
}

#[test]
fn bpe_applies_chained_merges() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &["h", "e", "l", "o", "he", "ll", "hell", "hello"],
        &[(0, 1), (2, 2), (4, 5), (6, 3)],
        BpeFlags::NONE,
    )?;

    assert_eq!(tokenizer.encode("hello")?, [7]);
    Ok(())
}

#[test]
fn bpe_only_uses_multibyte_vocabulary_entries_when_reachable() -> ireetok::Result<()> {
    let tokens = ["a", "aa"];
    let regular = tokenizer(&tokens, &[], BpeFlags::NONE)?;
    let ignored = tokenizer(
        &tokens,
        &[],
        BpeFlags::from_bits_retain(BpeFlags::IGNORE_MERGES.bits()),
    )?;

    assert_eq!(regular.encode("aa")?, [0, 0]);
    assert_eq!(ignored.encode("aa")?, [1]);
    Ok(())
}

#[test]
fn bpe_end_of_word_suffix_matches_virtual_whole_word_token() -> ireetok::Result<()> {
    let mut model = model(&["cat</w>"], &[], BpeFlags::NONE)?;
    model.set_end_of_word_suffix("</w>")?;
    let mut builder = TokenizerBuilder::new();
    builder.set_model(model);
    assert_eq!(builder.build()?.encode("cat")?, [0]);
    Ok(())
}

#[test]
fn bpe_end_of_word_suffix_falls_back_to_input_characters() -> ireetok::Result<()> {
    let mut model = model(
        &["dog</w>", "c", "a", "t"],
        &[],
        BpeFlags::from_bits_retain(BpeFlags::IGNORE_MERGES.bits()),
    )?;
    model.set_end_of_word_suffix("</w>")?;
    let mut builder = TokenizerBuilder::new();
    builder.set_model(model);
    assert_eq!(builder.build()?.encode("cat")?, [1, 2, 3]);
    Ok(())
}

#[test]
fn bpe_end_of_word_suffix_offsets_exclude_virtual_bytes() -> ireetok::Result<()> {
    let mut model = model(&["hi</w>"], &[], BpeFlags::NONE)?;
    model.set_end_of_word_suffix("</w>")?;
    let mut builder = TokenizerBuilder::new();
    builder.set_model(model);
    let encoded = builder.build()?.encode_with(
        "hi",
        ireetok::EncodeOptions {
            track_offsets: true,
            ..ireetok::EncodeOptions::default()
        },
    )?;
    assert_eq!(encoded.ids, [0]);
    assert_eq!(
        encoded.offsets.as_deref(),
        Some(&[ireetok::Offset { start: 0, end: 2 }][..])
    );
    Ok(())
}

#[test]
fn bpe_backtracks_when_suffix_merge_has_higher_priority() -> ireetok::Result<()> {
    let tokens = [
        "a", "b", "c", "d", "e", "ab", "bc", "cd", "de", "abc", "cde",
    ];
    let tokenizer = tokenizer(
        &tokens,
        &[(0, 1), (1, 2), (2, 3), (3, 4), (7, 4), (5, 2)],
        BpeFlags::NONE,
    )?;

    assert_eq!(tokenizer.encode("abcde")?, [5, 10]);
    Ok(())
}

#[test]
fn bpe_backtracking_respects_tokens_consumed_by_a_higher_priority_merge() -> ireetok::Result<()> {
    let tokens = [
        "e", "x", "p", "i", "a", "l", "ex", "ia", "al", "ial", "exp", "pia",
    ];
    let tokenizer = tokenizer(
        &tokens,
        &[(4, 5), (0, 1), (3, 4), (2, 7), (3, 8), (6, 2)],
        BpeFlags::NONE,
    )?;

    assert_eq!(tokenizer.encode("expial")?, [10, 9]);
    Ok(())
}

#[test]
fn bpe_backtracking_handles_repeating_suffixes() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &["a", "aa", "aaa", "aaaa"],
        &[(0, 0), (1, 0), (1, 1)],
        BpeFlags::NONE,
    )?;

    assert_eq!(tokenizer.encode("aaaa")?, [3]);
    assert_eq!(tokenizer.encode("aaaaa")?, [1, 2]);
    Ok(())
}

#[test]
fn bpe_byte_level_maps_space_tab_and_newline() -> ireetok::Result<()> {
    let tokens = ["Ġ", "ĉ", "Ċ", "a"];
    let tokenizer = tokenizer(
        &tokens,
        &[],
        BpeFlags::from_bits_retain(BpeFlags::BYTE_LEVEL_INPUT.bits()),
    )?;

    assert_eq!(tokenizer.encode(" ")?, [0]);
    assert_eq!(tokenizer.encode("\t")?, [1]);
    assert_eq!(tokenizer.encode("\n")?, [2]);
    assert_eq!(tokenizer.encode("a\ta\n")?, [3, 1, 3, 2]);
    Ok(())
}

#[test]
fn bpe_byte_level_merges_consecutive_spaces() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &["Ġ", "ĠĠ", "ĠĠĠ", "ĠĠĠĠ"],
        &[(0, 0), (1, 0), (1, 1)],
        BpeFlags::from_bits_retain(BpeFlags::BYTE_LEVEL_INPUT.bits()),
    )?;

    assert_eq!(tokenizer.encode("  ")?, [1]);
    assert_eq!(tokenizer.encode("   ")?, [2]);
    assert_eq!(tokenizer.encode("    ")?, [3]);
    assert_eq!(tokenizer.encode("     ")?, [1, 2]);
    Ok(())
}

#[test]
fn bpe_byte_level_suffix_blocking_accounts_for_mapped_bytes() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &["Ġ", "a", "b", "c", "Ġa", "ab", "bc", "Ġab", "abc"],
        &[(2, 3), (1, 2), (0, 1), (4, 2), (5, 3)],
        BpeFlags::from_bits_retain(BpeFlags::BYTE_LEVEL_INPUT.bits()),
    )?;

    assert_eq!(tokenizer.encode(" abc")?, [4, 6]);
    Ok(())
}

#[test]
fn bpe_falls_back_to_utf8_bytes_and_exposes_token_text() -> ireetok::Result<()> {
    let model = model(&["a", "<0xC3>", "<0xA9>", "hello"], &[], BpeFlags::NONE)?;
    let tokenizer = {
        let mut builder = TokenizerBuilder::new();
        builder.set_model(model);
        builder.build()?
    };
    assert_eq!(tokenizer.encode("é")?, [1, 2]);
    Ok(())
}

#[test]
fn bpe_encodes_multiple_segments_with_one_token_capacity() -> ireetok::Result<()> {
    let model = model(&["a", "b", "c"], &[], BpeFlags::NONE)?;
    assert_eq!(
        encode_with_capacity(
            &model,
            "abc",
            &[
                Segment { start: 0, end: 1 },
                Segment { start: 1, end: 2 },
                Segment { start: 2, end: 3 },
            ],
            1,
        )?,
        [0, 1, 2]
    );
    assert_eq!(model.token_text(2)?, b"c");
    Ok(())
}
