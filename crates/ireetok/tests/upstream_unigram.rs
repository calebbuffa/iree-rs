//! Golden Unigram model cases adapted from IREE's upstream tests.

use ireetok::{
    Code, Error, Model, Segment, TokenAttributes, TokenId, TokenOutput, Tokenizer,
    TokenizerBuilder, UnigramFlags, VocabularyBuilder,
};

fn model(vocab: &[(&str, f32)], flags: UnigramFlags) -> ireetok::Result<Model> {
    let mut builder = VocabularyBuilder::new(vocab.len())?;
    for (text, score) in vocab {
        let attributes = if *text == "[UNK]" {
            TokenAttributes::UNKNOWN
        } else {
            TokenAttributes::NONE
        };
        builder.add_token(text, *score, attributes)?;
    }
    let unk_id = i32::try_from(vocab.len() - 1)
        .map_err(|_| Error::new(Code::OutOfRange, "fixture size exceeds TokenId"))?;
    builder.set_special_token(ireetok::SpecialTokenKind::Unknown, unk_id)?;
    Model::unigram(builder.build()?, unk_id, -10.0, flags)
}

fn tokenizer(vocab: &[(&str, f32)], flags: UnigramFlags) -> ireetok::Result<Tokenizer> {
    let mut tokenizer = TokenizerBuilder::new();
    tokenizer.set_model(model(vocab, flags)?);
    tokenizer.build()
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
            return Err(Error::new(
                Code::DataLoss,
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
            return Err(Error::new(
                Code::DataLoss,
                "model retained output without making progress",
            ));
        }
    }
    Ok(encoded)
}

#[test]
fn unigram_uses_the_highest_scoring_segmentation() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[
            ("abc", -3.0),
            ("a", -1.0),
            ("bc", -1.5),
            ("ab", -2.0),
            ("c", -1.0),
            ("[UNK]", -10.0),
        ],
        UnigramFlags::NO_BYTE_FALLBACK,
    )?;
    assert_eq!(tokenizer.encode("abc")?, [1, 2]);
    Ok(())
}

#[test]
fn unigram_prefers_a_higher_scoring_whole_token() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("abc", -1.0), ("a", -2.0), ("bc", -2.0), ("[UNK]", -10.0)],
        UnigramFlags::NO_BYTE_FALLBACK,
    )?;
    assert_eq!(tokenizer.encode("abc")?, [0]);
    Ok(())
}

#[test]
fn unigram_fuses_adjacent_unknown_characters_when_configured() -> ireetok::Result<()> {
    let vocab = [("a", -1.0), ("[UNK]", -10.0)];
    assert_eq!(
        tokenizer(&vocab, UnigramFlags::NO_BYTE_FALLBACK)?.encode("xyz")?,
        [1]
    );
    assert_eq!(
        tokenizer(
            &vocab,
            UnigramFlags::from_bits_retain(
                UnigramFlags::NO_BYTE_FALLBACK.bits() | UnigramFlags::NO_FUSE_UNK.bits(),
            )
        )?
        .encode("xyz")?,
        [1, 1, 1]
    );
    Ok(())
}

#[test]
fn unigram_uses_byte_tokens_for_unknown_utf8() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[
            ("a", -1.0),
            ("<0xC3>", -5.0),
            ("<0xA9>", -5.0),
            ("[UNK]", -10.0),
        ],
        UnigramFlags::NONE,
    )?;
    assert_eq!(tokenizer.encode("aé")?, [0, 1, 2]);
    Ok(())
}

#[test]
fn unigram_preserves_known_tokens_around_an_unknown_character() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("a", -1.0), ("b", -1.0), ("[UNK]", -10.0)],
        UnigramFlags::NO_BYTE_FALLBACK,
    )?;

    assert_eq!(tokenizer.encode("axb")?, [0, 2, 1]);
    Ok(())
}

#[test]
fn unigram_uses_byte_fallback_for_unknown_ascii() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("a", -1.0), ("<0x78>", -5.0), ("[UNK]", -10.0)],
        UnigramFlags::NONE,
    )?;

    assert_eq!(tokenizer.encode("ax")?, [0, 1]);
    Ok(())
}

#[test]
fn unigram_no_byte_fallback_uses_unknown_even_when_byte_token_exists() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("a", -1.0), ("<0x78>", -5.0), ("[UNK]", -10.0)],
        UnigramFlags::NO_BYTE_FALLBACK,
    )?;

    assert_eq!(tokenizer.encode("ax")?, [0, 2]);
    Ok(())
}

#[test]
fn unigram_matches_multibyte_and_four_byte_tokens() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("é", -1.0), ("😀", -1.0), ("[UNK]", -10.0)],
        UnigramFlags::NONE,
    )?;

    assert_eq!(tokenizer.encode("é😀")?, [0, 1]);
    Ok(())
}

#[test]
fn unigram_fuses_only_adjacent_unreachable_regions() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("a", -1.0), ("b", -1.0), ("c", -1.0), ("[UNK]", -10.0)],
        UnigramFlags::NO_BYTE_FALLBACK,
    )?;
    assert_eq!(tokenizer.encode("axbyc")?, [0, 3, 1, 3, 2]);
    assert_eq!(tokenizer.encode("a😀b🌍")?, [0, 3, 1, 3]);
    Ok(())
}

#[test]
fn unigram_handles_one_token_capacity_across_segments() -> ireetok::Result<()> {
    let model = model(
        &[
            ("a", -1.0),
            ("b", -1.0),
            ("c", -1.0),
            ("d", -1.0),
            ("[UNK]", -10.0),
        ],
        UnigramFlags::NONE,
    )?;
    assert_eq!(
        encode_with_capacity(
            &model,
            "abcd",
            &[Segment { start: 0, end: 2 }, Segment { start: 2, end: 4 },],
            1,
        )?,
        [0, 1, 2, 3]
    );
    Ok(())
}

#[test]
fn unigram_tokenizes_long_segments_beyond_a_single_viterbi_chunk() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        &[("a", -1.0), ("[UNK]", -10.0)],
        UnigramFlags::NO_BYTE_FALLBACK,
    )?;
    let input = "a".repeat(4096);
    assert_eq!(tokenizer.encode(&input)?, vec![0; input.len()]);
    Ok(())
}
