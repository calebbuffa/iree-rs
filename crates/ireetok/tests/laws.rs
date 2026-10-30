//! Property laws for the safe tokenizer API.
// Test fixtures outside `#[test]` functions assert by panicking.

use ireetok::{DecodeOptions, EncodeOptions, Tokenizer};
use proptest::prelude::*;

const WORDPIECE: &str = r###"{
  "version": "1.0",
  "model": {
    "type": "WordPiece",
    "unk_token": "[UNK]",
    "continuing_subword_prefix": "##",
    "max_input_chars_per_word": 100,
    "vocab": {"[UNK]": 0, "hello": 1, "world": 2, "##s": 3}
  },
  "pre_tokenizer": {"type": "Whitespace"},
  "decoder": {"type": "WordPiece", "prefix": "##", "cleanup": true}
}"###;

const BPE: &str = r###"{
  "version": "1.0",
  "model": {
    "type": "BPE",
    "vocab": {"hello": 0, "world": 1, " ": 2},
    "merges": [],
    "unk_token": null,
    "ignore_merges": true
  },
  "decoder": {"type": "Replace", "pattern": {"String": "▁"}, "content": " "}
}"###;

fn tokenizer() -> ireetok::Result<Tokenizer> {
    Tokenizer::from_huggingface_json(WORDPIECE)
}
fn bpe_tokenizer() -> ireetok::Result<Tokenizer> {
    Tokenizer::from_huggingface_json(BPE)
}

fn known_text() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop_oneof![Just("hello"), Just("world"), Just("worlds")],
        0..8,
    )
    .prop_map(|parts| parts.join(" "))
}

#[test]
fn huggingface_parser_accepts_small_wordpiece_json() -> ireetok::Result<()> {
    let t = tokenizer()?;
    assert_eq!(t.model_type_name(), "WordPiece");
    assert_eq!(
        t.encode_with("hello", EncodeOptions::default())?.type_ids,
        None
    );
    Ok(())
}

#[test]
fn known_words_round_trip() -> ireetok::Result<()> {
    let t = tokenizer()?;
    let ids = t.encode("hello worlds")?;
    assert_eq!(ids, vec![1, 2, 3]);
    assert_eq!(t.decode(&ids)?, "hello worlds");
    assert_eq!(t.decode_bytes(&ids)?, b"hello worlds");
    Ok(())
}

#[test]
fn tiny_bpe_round_trip() -> ireetok::Result<()> {
    let t = bpe_tokenizer()?;
    let ids = t.encode("hello world")?;
    assert_eq!(t.decode(&ids)?, "hello world");
    Ok(())
}

#[test]
fn programmatic_model_keeps_its_vocabulary_alive() {
    let mut vocab = ireetok::VocabularyBuilder::new(2).unwrap();
    vocab
        .add_token("[UNK]", 0.0, ireetok::TokenAttributes::UNKNOWN)
        .unwrap();
    vocab
        .add_token("hello", 0.0, ireetok::TokenAttributes::NONE)
        .unwrap();
    vocab
        .set_special_token(ireetok::SpecialTokenKind::Unknown, 0)
        .unwrap();
    let model = ireetok::Model::wordpiece(
        vocab.build().unwrap(),
        "##",
        100,
        ireetok::WordPieceFlags::NONE,
    )
    .unwrap();
    let mut builder = ireetok::TokenizerBuilder::new();
    builder.set_model(model);
    let tokenizer = builder.build().unwrap();
    assert_eq!(tokenizer.encode("hello").unwrap(), vec![1]);
}

#[test]
fn regex_dfa_binary_and_streaming_matches_agree() {
    let dfa = ireetok::RegexDfa::compile(r"\d+", ireetok::RegexCompileFlags::NONE).unwrap();
    let expected = vec![
        ireetok::Offset { start: 1, end: 3 },
        ireetok::Offset { start: 5, end: 6 },
    ];
    assert_eq!(dfa.matches("a12 b3").unwrap(), expected);
    assert_eq!(dfa.count_matches("a12 b3").unwrap(), expected.len());
    assert!(dfa.has_match("a12").unwrap());

    let mut state = dfa.exec_state();
    let mut actual = state.feed("a1").unwrap();
    actual.extend(state.feed("2 b3").unwrap());
    actual.extend(state.finalize().unwrap());
    assert_eq!(actual, expected);

    let stride = dfa.accelerate().unwrap();
    let mut state = dfa.exec_state();
    let mut actual = state.feed_with_stride("a12 b3", &stride).unwrap();
    actual.extend(state.finalize().unwrap());
    assert_eq!(actual, expected);
}

#[test]
fn empty_vocabulary_tokens_remain_distinguishable_from_missing_ids() {
    let mut builder = ireetok::VocabularyBuilder::new(1).unwrap();
    builder
        .add_token("", 0.0, ireetok::TokenAttributes::NONE)
        .unwrap();
    let vocabulary = builder.build().unwrap();
    assert_eq!(vocabulary.view().token_text(0), Some(&b""[..]));
    assert_eq!(vocabulary.view().token_text(1), None);
}

#[test]
fn invalid_json_is_an_error() {
    assert!(Tokenizer::from_huggingface_json("not json").is_err());
}

#[test]
fn decode_without_a_decoder_is_a_bounded_error() {
    let json = WORDPIECE.replace(
        r###",
  "decoder": {"type": "WordPiece", "prefix": "##", "cleanup": true}"###,
        "",
    );
    assert_ne!(json, WORDPIECE);
    let t = Tokenizer::from_huggingface_json(&json).unwrap();
    assert!(t.decode(&[1]).is_err());
    assert!(t.decode(&[1, 2, 3]).is_err());
}

#[test]
fn offsets_are_within_text_and_monotone() -> ireetok::Result<()> {
    let t = tokenizer()?;
    let text = "hello worlds";
    let encoded = t.encode_with(
        text,
        EncodeOptions {
            track_offsets: true,
            ..Default::default()
        },
    )?;
    let offsets = encoded.offsets.unwrap();
    assert_eq!(offsets.len(), encoded.ids.len());
    let mut previous = 0;
    for offset in offsets {
        assert!(offset.start <= offset.end && offset.end <= text.len());
        assert!(offset.start >= previous);
        previous = offset.start;
    }
    Ok(())
}

#[test]
fn decode_options_skip_special_tokens() -> ireetok::Result<()> {
    let t = tokenizer()?;
    assert_eq!(
        t.decode_with(
            &[1, 2, 3],
            DecodeOptions {
                skip_special_tokens: true
            }
        )?,
        "hello worlds"
    );
    Ok(())
}

proptest! {
    #[test]
    fn encode_is_deterministic(text in known_text()) {
        let t = tokenizer().map_err(|error| TestCaseError::fail(error.to_string()))?;
        prop_assert_eq!(t.encode(&text).unwrap(), t.encode(&text).unwrap());
    }

    #[test]
    fn wordpiece_round_trips(text in known_text()) {
        let t = tokenizer().map_err(|error| TestCaseError::fail(error.to_string()))?;
        let ids = t.encode(&text).unwrap();
        prop_assert_eq!(t.decode(&ids).unwrap(), text);
    }

    #[test]
    fn bpe_round_trips(text in prop::collection::vec(prop_oneof![Just("hello"), Just("world")], 0..8).prop_map(|parts| parts.join(" "))) {
        let t = bpe_tokenizer().map_err(|error| TestCaseError::fail(error.to_string()))?;
        let ids = t.encode(&text).unwrap();
        prop_assert_eq!(t.decode(&ids).unwrap(), text);
    }

    #[test]
    fn streaming_encode_matches_one_shot(
        parts in prop::collection::vec(prop_oneof![Just("hello "), Just("worlds "), Just("world ")], 0..8),
        chunk_sizes in prop::collection::vec(1usize..9, 0..32)
    ) {
        let text = parts.concat();
        let t = tokenizer().map_err(|error| TestCaseError::fail(error.to_string()))?;
        let expected = t.encode(&text).unwrap();
        let mut state = t.encode_state(8, EncodeOptions::default()).unwrap();
        let mut actual = Vec::new();
        let mut position = 0;
        let mut size_index = 0;
        while position < text.len() {
            let size = chunk_sizes.get(size_index).copied().unwrap_or(1);
            let end = (position + size).min(text.len());
            let chunk = &text[position..end];
            let mut consumed = 0;
            while consumed < chunk.len() {
                let mut ids = vec![0; 128];
                let mut out = ireetok::TokenOutput::new(&mut ids, None, None).unwrap();
                let progress = state.feed(&chunk[consumed..], &mut out).unwrap();
                consumed += progress.consumed;
                actual.extend_from_slice(&ids[..progress.written]);
                prop_assert!(progress.consumed > 0 || progress.written > 0);
            }
            position = end;
            size_index += 1;
        }
        loop {
            let mut ids = vec![0; state.pending_token_bound().max(128)];
            let mut out = ireetok::TokenOutput::new(&mut ids, None, None).unwrap();
            let written = state.finalize(&mut out).unwrap();
            actual.extend_from_slice(&ids[..written]);
            if !state.has_pending() { break; }
        }
        prop_assert_eq!(actual, expected);
    }
}

proptest! {
    #[test]
    fn offsets_stay_monotone_and_in_bounds(text in known_text()) {
        let tokenizer = tokenizer().map_err(|error| TestCaseError::fail(error.to_string()))?;
        let result = tokenizer.encode_with(&text, EncodeOptions { track_offsets: true, ..Default::default() }).unwrap();
        let offsets = result.offsets.unwrap();
        prop_assert_eq!(offsets.len(), result.ids.len());
        let mut previous = 0;
        for offset in offsets {
            prop_assert!(offset.start <= offset.end);
            prop_assert!(offset.end <= text.len());
            prop_assert!(offset.start >= previous);
            previous = offset.start;
        }
    }
}

#[test]
fn batch_operations_match_individual_operations() -> ireetok::Result<()> {
    let t = tokenizer()?;
    let inputs = vec![
        ireetok::BatchEncodeInput {
            text: "hello".into(),
            text_pair: None,
        },
        ireetok::BatchEncodeInput {
            text: "worlds".into(),
            text_pair: None,
        },
    ];
    let encoded = t.encode_batch(&inputs, EncodeOptions::default()).unwrap();
    assert_eq!(encoded[0].ids, t.encode("hello").unwrap());
    assert_eq!(encoded[1].ids, t.encode("worlds").unwrap());
    assert!(encoded.iter().all(|item| item.type_ids.is_none()));
    let decoded_bytes = t
        .decode_batch_bytes(
            &encoded.iter().map(|x| x.ids.clone()).collect::<Vec<_>>(),
            DecodeOptions::default(),
        )
        .unwrap();
    assert_eq!(decoded_bytes, vec![b"hello".to_vec(), b"worlds".to_vec()]);
    let decoded = t
        .decode_batch(
            &encoded.iter().map(|x| x.ids.clone()).collect::<Vec<_>>(),
            DecodeOptions::default(),
        )
        .unwrap();
    assert_eq!(decoded, vec!["hello", "worlds"]);
    Ok(())
}

#[test]
fn null_huggingface_components_remain_absent() {
    assert!(
        ireetok::Normalizer::from_huggingface_json("null")
            .unwrap()
            .is_none()
    );
    assert!(
        ireetok::Decoder::from_huggingface_json("null")
            .unwrap()
            .is_none()
    );
    let (segmenter, flags) = ireetok::Segmenter::from_huggingface_json("null").unwrap();
    assert!(segmenter.is_none());
    assert_eq!(flags, ireetok::PreTokenizerFlags::NONE);
}

#[test]
fn vocabulary_indexes_borrow_their_source() {
    let mut builder = ireetok::VocabularyBuilder::new(3).unwrap();
    builder
        .add_token("a", 0.0, ireetok::TokenAttributes::NONE)
        .unwrap();
    builder
        .add_token("b", 0.0, ireetok::TokenAttributes::NONE)
        .unwrap();
    builder
        .add_token("ab", 0.0, ireetok::TokenAttributes::NONE)
        .unwrap();
    builder.add_merge(0, 1).unwrap();
    let vocab = builder.build().unwrap();
    let view = vocab.view();

    let hash = ireetok::VocabularyHash::new(view, 75).unwrap();
    assert_eq!(hash.lookup(b"ab"), Some(2));
    let required = ireetok::VocabularyHash::storage_size(&vocab.view(), 75).unwrap();
    let storage = vec![0; required.div_ceil(std::mem::size_of::<usize>())];
    let stored_hash = ireetok::VocabularyHash::with_storage(vocab.view(), 75, storage).unwrap();
    assert_eq!(stored_hash.lookup(b"a"), Some(0));

    let trie = ireetok::VocabularyTrie::new(vocab.view()).unwrap();
    let mut cursor = trie.cursor();
    for byte in b"ab" {
        assert!(cursor.advance(*byte));
    }
    assert_eq!(cursor.token_id(), Some(2));
    cursor.reset();
    assert_eq!(cursor.depth(), 0);

    let merge_hash = ireetok::MergeHash::new(&vocab.view()).unwrap();
    let merge = merge_hash.lookup(0, 1).unwrap();
    assert_eq!(merge.rank, 0);
    assert_eq!(merge.token_id, 2);
}

#[test]
fn tiktoken_presets_can_be_inspected() {
    let config = ireetok::TiktokenConfig::by_name("cl100k_base").unwrap();
    assert!(!config.pattern.is_empty());
    assert!(!config.special_tokens.is_empty());
}
