//! Hugging Face model JSON cases adapted from IREE's upstream parser tests.

use ireetok::{AddedTokens, BpeFlags, Code, Model, PreTokenizerFlags, Tokenizer, TokenizerBuilder};

fn tokenizer(model: Model) -> ireetok::Result<Tokenizer> {
    let mut builder = TokenizerBuilder::new();
    builder.set_model(model);
    builder.build()
}

#[test]
fn bpe_parser_accepts_string_and_pair_merge_formats() -> ireetok::Result<()> {
    for (json, expected) in [
        (
            r#"{"type":"BPE","vocab":{"a":0,"b":1,"ab":2},"merges":["a b"]}"#,
            [2],
        ),
        (
            r#"{"type":"BPE","vocab":{"a":0,"b":1,"ab":2},"merges":[["a","b"]]}"#,
            [2],
        ),
    ] {
        let model = Model::from_huggingface_bpe_json(json, None, BpeFlags::NONE)?;
        assert_eq!(tokenizer(model)?.encode("ab")?, expected);
    }
    Ok(())
}

#[test]
fn wordpiece_parser_applies_hugging_face_defaults() -> ireetok::Result<()> {
    let model = Model::from_huggingface_wordpiece_json(
        r###"{"type":"WordPiece","vocab":{"[UNK]":0,"un":1,"##able":2}}"###,
        None,
    )?;
    assert_eq!(tokenizer(model)?.encode("unable")?, [1, 2]);
    Ok(())
}

#[test]
fn unigram_parser_reads_scored_vocabulary_and_unknown_id() -> ireetok::Result<()> {
    let model = Model::from_huggingface_json(
        r#"{
          "type":"Unigram",
          "vocab":[["<unk>",-10.0],["a",-1.0],["b",-1.5],["ab",-0.1]],
          "unk_id":0
        }"#,
        None,
        PreTokenizerFlags::NONE,
    )?;
    assert_eq!(tokenizer(model)?.encode("ab")?, [3]);
    Ok(())
}

#[test]
fn model_parser_resolves_added_unknown_token_and_owns_vocab() -> ireetok::Result<()> {
    let added_tokens = AddedTokens::from_huggingface_json(r#"{"added_tokens":[],"model":{}}"#)?;
    let model = Model::from_huggingface_json(
        r#"{"type":"BPE","vocab":{"a":0,"<unk>":1},"merges":[],"unk_token":"<unk>"}"#,
        Some(&added_tokens),
        PreTokenizerFlags::NONE,
    )?;
    let tokenizer = tokenizer(model)?;
    assert_eq!(tokenizer.vocab().special_ids().unk, Some(1));
    Ok(())
}

#[test]
fn bpe_parser_rejects_unknown_unk_token() {
    let result = Model::from_huggingface_bpe_json(
        r#"{"type":"BPE","vocab":{"a":0},"merges":[],"unk_token":"<unk>"}"#,
        None,
        BpeFlags::NONE,
    );
    assert!(matches!(result, Err(error) if error.code() == Code::NotFound));
}

#[test]
fn bpe_parser_applies_optional_fields_and_preserves_escaped_tokens() -> ireetok::Result<()> {
    let model = Model::from_huggingface_bpe_json(
        r#"{
          "type":"BPE",
          "vocab":{"hello\"world":0,"a":1,"b":2,"ab":3},
          "merges":[["a","b"]],
          "unk_token":null,
          "dropout":null,
          "continuing_subword_prefix":null,
          "end_of_word_suffix":null,
          "byte_fallback":true,
          "fuse_unk":true,
          "ignore_merges":false
        }"#,
        None,
        BpeFlags::NONE,
    )?;
    assert_eq!(model.vocab().token_count(), 4);
    assert_eq!(model.vocab().lookup(b"hello\"world"), Some(0));
    assert_eq!(model.vocab().merge_count(), 1);
    assert_eq!(model.vocab().special_ids().unk, None);
    Ok(())
}

#[test]
fn wordpiece_parser_resolves_defaults_and_rejects_invalid_values() -> ireetok::Result<()> {
    let model = Model::from_huggingface_wordpiece_json(
        r###"{"type":"WordPiece","vocab":{"[UNK]":5,"a":0,"##b":1}}"###,
        None,
    )?;
    assert_eq!(model.vocab().special_ids().unk, Some(5));

    for (json, expected) in [
        (
            r#"{"type":"WordPiece","vocab":{"a":0},"unk_token":"[UNK]"}"#,
            Code::NotFound,
        ),
        (
            r#"{"type":"WordPiece","vocab":{"[UNK]":0},"max_input_chars_per_word":0}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"type":"WordPiece","vocab":{"[UNK]":0},"extra":"field"}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"type":"WordPiece","vocab":{"a":0},"unk_token":null}"#,
            Code::InvalidArgument,
        ),
    ] {
        assert!(
            matches!(Model::from_huggingface_wordpiece_json(json, None), Err(error) if error.code() == expected),
            "unexpected parser result for {json}"
        );
    }
    Ok(())
}

#[test]
fn bpe_parser_rejects_missing_merge_data_and_unknown_merge_tokens() {
    for json in [
        r#"{"type":"BPE","merges":[]}"#,
        r#"{"type":"BPE","vocab":{"a":0}}"#,
    ] {
        assert!(
            matches!(
                Model::from_huggingface_bpe_json(json, None, BpeFlags::NONE),
                Err(error) if error.code() == Code::NotFound
            ),
            "unexpected parser result for {json}"
        );
    }

    let unknown_merge = Model::from_huggingface_bpe_json(
        r#"{"type":"BPE","vocab":{"a":0},"merges":["a b"]}"#,
        None,
        BpeFlags::NONE,
    );
    assert!(matches!(unknown_merge, Err(error) if error.code() == Code::NotFound));
}

#[test]
fn bpe_parser_covers_optional_training_and_affix_fields() -> ireetok::Result<()> {
    for json in [
        r#"{"type":"BPE","vocab":{"a":0},"merges":[],"byte_fallback":true,"fuse_unk":true,"ignore_merges":true}"#,
        r#"{"type":"BPE","vocab":{"a":0},"merges":[],"dropout":0.1}"#,
        r#"{"type":"BPE","vocab":{"a":0},"merges":[],"dropout":null,"unk_token":null}"#,
        r#"{"type":"BPE","vocab":{"a":0,"b</w>":1},"merges":[],"end_of_word_suffix":"</w>"}"#,
        r#"{"type":"BPE","vocab":{"a":0},"merges":[],"continuing_subword_prefix":null,"end_of_word_suffix":null}"#,
    ] {
        let model = Model::from_huggingface_bpe_json(json, None, BpeFlags::NONE)?;
        assert!(
            model.vocab().token_count() > 0,
            "model did not retain vocab: {json}"
        );
    }

    let unsupported_prefix = Model::from_huggingface_bpe_json(
        r###"{"type":"BPE","vocab":{"a":0},"merges":[],"continuing_subword_prefix":"##"}"###,
        None,
        BpeFlags::NONE,
    );
    assert!(matches!(
        unsupported_prefix,
        Err(error) if error.code() == Code::Unimplemented
    ));

    let unknown_model =
        Model::from_huggingface_json(r#"{"type":"Unknown"}"#, None, PreTokenizerFlags::NONE);
    assert!(matches!(
        unknown_model,
        Err(error) if error.code() == Code::Unimplemented
    ));
    Ok(())
}
