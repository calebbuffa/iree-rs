//! Golden WordPiece model cases adapted from IREE's upstream tests.

use ireetok::{EncodeOptions, Model, Segment, TokenId, TokenOutput, Tokenizer};

fn tokenizer(vocab: &str, max_chars: usize) -> ireetok::Result<Tokenizer> {
    let json = format!(
        r###"{{
          "version": "1.0",
          "model": {{
            "type": "WordPiece",
            "unk_token": "[UNK]",
            "continuing_subword_prefix": "##",
            "max_input_chars_per_word": {max_chars},
            "vocab": {vocab}
          }}
        }}"###
    );
    Tokenizer::from_huggingface_json(&json)
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
fn wordpiece_uses_unknown_token_for_unmatched_words() -> ireetok::Result<()> {
    let tokenizer = tokenizer(r#"{"hello": 0, "[UNK]": 1}"#, 100)?;
    assert_eq!(tokenizer.encode("xyz")?, [1]);
    Ok(())
}

#[test]
fn wordpiece_encodes_whole_words_and_continuation_subwords() -> ireetok::Result<()> {
    let whole_word = tokenizer(r#"{"hello": 0, "[UNK]": 1}"#, 100)?;
    assert_eq!(whole_word.encode("hello")?, [0]);

    let subwords = tokenizer(r###"{"un": 0, "##aff": 1, "##able": 2, "[UNK]": 3}"###, 100)?;
    assert_eq!(subwords.encode("unaffable")?, [0, 1, 2]);
    Ok(())
}

#[test]
fn wordpiece_encodes_each_character_as_a_continuation() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        r###"{"h": 0, "##e": 1, "##l": 2, "##o": 3, "[UNK]": 4}"###,
        100,
    )?;

    assert_eq!(tokenizer.encode("hello")?, [0, 1, 2, 2, 3]);
    Ok(())
}

#[test]
fn wordpiece_replaces_the_whole_word_when_a_subword_is_missing() -> ireetok::Result<()> {
    let tokenizer = tokenizer(r#"{"hel": 0, "[UNK]": 1}"#, 100)?;
    assert_eq!(tokenizer.encode("hello")?, [1]);
    Ok(())
}

#[test]
fn wordpiece_enforces_maximum_characters_per_word() -> ireetok::Result<()> {
    let tokenizer = tokenizer(r###"{"a": 0, "##a": 1, "[UNK]": 2}"###, 3)?;
    assert_eq!(tokenizer.encode("aaaa")?, [2]);
    assert_eq!(tokenizer.encode("aaa")?, [0, 1, 1]);
    Ok(())
}

#[test]
fn wordpiece_greedily_selects_longest_subwords() -> ireetok::Result<()> {
    let tokenizer = tokenizer(
        r###"{"a": 0, "ab": 1, "##c": 2, "##cd": 3, "[UNK]": 4}"###,
        100,
    )?;
    assert_eq!(tokenizer.encode("abcd")?, [1, 3]);
    Ok(())
}

#[test]
fn wordpiece_counts_unicode_characters_and_tracks_byte_offsets() -> ireetok::Result<()> {
    let tokenizer = tokenizer(r###"{"é": 0, "##é": 1, "[UNK]": 2}"###, 2)?;
    let text = "éé";
    let encoded = tokenizer.encode_with(
        text,
        EncodeOptions {
            track_offsets: true,
            ..EncodeOptions::default()
        },
    )?;
    assert_eq!(encoded.ids, [0, 1]);
    assert_eq!(
        encoded.offsets.as_deref(),
        Some(
            &[
                ireetok::Offset { start: 0, end: 2 },
                ireetok::Offset { start: 2, end: 4 }
            ][..]
        )
    );
    assert_eq!(tokenizer.encode("ééé")?, [2]);
    Ok(())
}

#[test]
fn wordpiece_handles_one_token_capacity_across_multiple_segments() -> ireetok::Result<()> {
    let model = Model::from_huggingface_wordpiece_json(
        r###"{"type":"WordPiece","unk_token":"[UNK]","continuing_subword_prefix":"##","max_input_chars_per_word":100,"vocab":{"h":0,"##e":1,"##l":2,"##o":3,"a":4,"##b":5,"[UNK]":6}}"###,
        None,
    )?;
    assert_eq!(
        encode_with_capacity(
            &model,
            "helloabxyz",
            &[
                Segment { start: 0, end: 5 },
                Segment { start: 5, end: 7 },
                Segment { start: 7, end: 10 },
            ],
            1,
        )?,
        [0, 1, 2, 2, 3, 4, 5, 6]
    );
    Ok(())
}

#[test]
fn wordpiece_accepts_empty_and_multicharacter_continuation_prefixes() -> ireetok::Result<()> {
    for (prefix, vocabulary, expected) in [
        ("", r#"{"hel":0,"lo":1,"[UNK]":2}"#, [0, 1]),
        ("###", r####"{"hel":0,"###lo":1,"[UNK]":2}"####, [0, 1]),
    ] {
        let json = format!(
            r#"{{"type":"WordPiece","unk_token":"[UNK]","continuing_subword_prefix":"{prefix}","max_input_chars_per_word":100,"vocab":{vocabulary}}}"#
        );
        let model = Model::from_huggingface_wordpiece_json(&json, None)?;
        assert_eq!(
            encode_with_capacity(&model, "hello", &[Segment { start: 0, end: 5 }], 1)?,
            expected
        );
    }
    Ok(())
}
