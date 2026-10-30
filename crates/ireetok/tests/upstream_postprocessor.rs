//! Postprocessor JSON cases adapted from IREE's upstream parser tests.

use ireetok::{Code, Postprocessor, PostprocessorFlags};

#[test]
fn bert_processing_parses_single_and_pair_templates() -> ireetok::Result<()> {
    let postprocessor = Postprocessor::from_huggingface_json(
        r#"{"type":"BertProcessing","sep":["[SEP]",102],"cls":["[CLS]",101]}"#,
    )?;

    assert_eq!(postprocessor.single().prefix, [(101, 0)]);
    assert_eq!(postprocessor.single().suffix, [(102, 0)]);
    let pair = postprocessor
        .pair()
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "BERT parser omitted pair template"))?;
    assert_eq!(pair.prefix, [(101, 0)]);
    assert_eq!(pair.infix, [(102, 0)]);
    assert_eq!(pair.suffix, [(102, 1)]);
    assert_eq!(pair.sequence_a_type_id, 0);
    assert_eq!(pair.sequence_b_type_id, 1);
    assert!(postprocessor.supports_pair());
    Ok(())
}

#[test]
fn template_processing_parses_bert_single_and_pair_sequences() -> ireetok::Result<()> {
    let postprocessor = Postprocessor::from_huggingface_json(
        r#"{
          "type": "TemplateProcessing",
          "single": [
            {"SpecialToken": {"id": "[CLS]", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}},
            {"SpecialToken": {"id": "[SEP]", "type_id": 0}}
          ],
          "pair": [
            {"SpecialToken": {"id": "[CLS]", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}},
            {"SpecialToken": {"id": "[SEP]", "type_id": 0}},
            {"Sequence": {"id": "B", "type_id": 1}},
            {"SpecialToken": {"id": "[SEP]", "type_id": 1}}
          ],
          "special_tokens": {
            "[CLS]": {"id": "[CLS]", "ids": [101], "tokens": ["[CLS]"]},
            "[SEP]": {"id": "[SEP]", "ids": [102], "tokens": ["[SEP]"]}
          }
        }"#,
    )?;

    assert_eq!(postprocessor.single().prefix, [(101, 0)]);
    assert_eq!(postprocessor.single().suffix, [(102, 0)]);
    let pair = postprocessor
        .pair()
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "template parser omitted pair"))?;
    assert_eq!(pair.prefix, [(101, 0)]);
    assert_eq!(pair.infix, [(102, 0)]);
    assert_eq!(pair.suffix, [(102, 1)]);
    assert_eq!(pair.sequence_a_type_id, 0);
    assert_eq!(pair.sequence_b_type_id, 1);
    Ok(())
}

#[test]
fn template_processing_supports_prefix_only_and_multi_token_specials() -> ireetok::Result<()> {
    let llama = Postprocessor::from_huggingface_json(
        r#"{
          "type": "TemplateProcessing",
          "single": [
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}}
          ],
          "pair": [
            {"SpecialToken": {"id": "<s>", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}},
            {"SpecialToken": {"id": "<s>", "type_id": 1}},
            {"Sequence": {"id": "B", "type_id": 1}}
          ],
          "special_tokens": {
            "<s>": {"id": "<s>", "ids": [1], "tokens": ["<s>"]}
          }
        }"#,
    )?;
    assert_eq!(llama.single().prefix, [(1, 0)]);
    assert!(llama.single().suffix.is_empty());
    let pair = llama.pair().ok_or_else(|| {
        ireetok::Error::new(Code::DataLoss, "Llama template omitted pair sequence")
    })?;
    assert_eq!(pair.infix, [(1, 1)]);
    assert_eq!(pair.sequence_b_type_id, 1);

    let whisper = Postprocessor::from_huggingface_json(
        r#"{
          "type": "TemplateProcessing",
          "single": [
            {"SpecialToken": {"id": "PREFIX", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}}
          ],
          "pair": [
            {"SpecialToken": {"id": "PREFIX", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}},
            {"Sequence": {"id": "B", "type_id": 1}}
          ],
          "special_tokens": {
            "PREFIX": {
              "id": "PREFIX",
              "ids": [50258, 50259, 50360],
              "tokens": ["<|sot|>", "<|en|>", "<|transcribe|>"]
            }
          }
        }"#,
    )?;
    assert_eq!(
        whisper.single().prefix,
        [(50258, 0), (50259, 0), (50360, 0)]
    );
    Ok(())
}

#[test]
fn template_processing_rejects_missing_tokens_and_piece_overflow() {
    let missing_token = Postprocessor::from_huggingface_json(
        r#"{
          "type": "TemplateProcessing",
          "single": [
            {"SpecialToken": {"id": "MISSING", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}}
          ],
          "pair": [
            {"Sequence": {"id": "A", "type_id": 0}},
            {"Sequence": {"id": "B", "type_id": 0}}
          ],
          "special_tokens": {}
        }"#,
    );
    assert!(matches!(missing_token, Err(error) if error.code() == Code::NotFound));

    let too_many = Postprocessor::from_huggingface_json(
        r#"{
          "type": "TemplateProcessing",
          "single": [
            {"SpecialToken": {"id": "BIG", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}}
          ],
          "pair": [
            {"SpecialToken": {"id": "BIG", "type_id": 0}},
            {"Sequence": {"id": "A", "type_id": 0}},
            {"Sequence": {"id": "B", "type_id": 0}}
          ],
          "special_tokens": {
            "BIG": {
              "id": "BIG",
              "ids": [1,2,3,4,5,6,7,8],
              "tokens": ["a","b","c","d","e","f","g","h"]
            }
          }
        }"#,
    );
    assert!(matches!(too_many, Err(error) if error.code() == Code::InvalidArgument));
}

#[test]
fn postprocessor_json_handles_null_sequence_and_unsupported_types() -> ireetok::Result<()> {
    let null = Postprocessor::from_huggingface_json("null")?;
    assert_eq!(null.single().prefix, []);
    assert_eq!(null.flags(), PostprocessorFlags::NONE);

    let byte_level = Postprocessor::from_huggingface_json(
        r#"{
          "type": "Sequence",
          "processors": [
            {"type": "ByteLevel", "add_prefix_space": false, "trim_offsets": false}
          ]
        }"#,
    )?;
    assert!(byte_level.single().prefix.is_empty());
    assert_eq!(byte_level.flags(), PostprocessorFlags::NONE);

    let unsupported = Postprocessor::from_huggingface_json(r#"{"type":"FutureProcessing"}"#);
    assert!(matches!(unsupported, Err(error) if error.code() == Code::Unimplemented));
    Ok(())
}

#[test]
fn roberta_processing_parses_offset_options() -> ireetok::Result<()> {
    let postprocessor = Postprocessor::from_huggingface_json(
        r#"{"type":"RobertaProcessing","sep":["</s>",2],"cls":["<s>",0],"trim_offsets":true,"add_prefix_space":false}"#,
    )?;
    assert!(postprocessor.supports_pair());
    assert_eq!(postprocessor.flags(), PostprocessorFlags::TRIM_OFFSETS);
    Ok(())
}

#[test]
fn roberta_processing_requires_explicit_options() {
    let result = Postprocessor::from_huggingface_json(
        r#"{"type":"RobertaProcessing","sep":["</s>",2],"cls":["<s>",0]}"#,
    );
    assert!(matches!(result, Err(error) if error.code() == Code::NotFound));
}

#[test]
fn byte_level_postprocessor_has_no_special_tokens() -> ireetok::Result<()> {
    let postprocessor = Postprocessor::from_huggingface_json(
        r#"{"type":"ByteLevel","add_prefix_space":true,"trim_offsets":false,"use_regex":true}"#,
    )?;
    let single = postprocessor.single();
    assert!(single.prefix.is_empty());
    assert!(single.infix.is_empty());
    assert!(single.suffix.is_empty());
    assert!(!postprocessor.supports_pair());
    assert_eq!(postprocessor.flags(), PostprocessorFlags::ADD_PREFIX_SPACE);
    Ok(())
}

#[test]
fn byte_level_postprocessor_exposes_each_option_independently() -> ireetok::Result<()> {
    let trim_offsets = Postprocessor::from_huggingface_json(
        r#"{"type":"ByteLevel","add_prefix_space":false,"trim_offsets":true}"#,
    )?;
    assert_eq!(trim_offsets.flags(), PostprocessorFlags::TRIM_OFFSETS);

    let add_prefix_space = Postprocessor::from_huggingface_json(
        r#"{"type":"ByteLevel","add_prefix_space":true,"trim_offsets":false}"#,
    )?;
    assert_eq!(
        add_prefix_space.flags(),
        PostprocessorFlags::ADD_PREFIX_SPACE
    );
    Ok(())
}

#[test]
fn bert_postprocessor_rejects_unknown_fields() {
    let result = Postprocessor::from_huggingface_json(
        r#"{"type":"BertProcessing","sep":["[SEP]",102],"cls":["[CLS]",101],"extra":true}"#,
    );
    assert!(matches!(result, Err(error) if error.code() == Code::InvalidArgument));
}
