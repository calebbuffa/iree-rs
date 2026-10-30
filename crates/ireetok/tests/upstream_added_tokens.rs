//! Added-token JSON cases adapted from IREE's upstream parser tests.

use ireetok::{AddedTokenFlags, AddedTokens, Code};

#[test]
fn added_tokens_accept_absent_null_and_empty_lists() -> ireetok::Result<()> {
    for json in [
        r#"{"model": {}}"#,
        r#"{"added_tokens": null, "model": {}}"#,
        r#"{"added_tokens": [], "model": {}}"#,
    ] {
        let tokens = AddedTokens::from_huggingface_json(json)?;
        assert!(tokens.is_empty());
        assert_eq!(tokens.len(), 0);
    }
    Ok(())
}

#[test]
fn added_tokens_preserve_all_behavior_flags_and_content() -> ireetok::Result<()> {
    let tokens = AddedTokens::from_huggingface_json(
        r#"{
          "added_tokens": [
            {
              "id": 0,
              "content": "\u2581",
              "single_word": false,
              "lstrip": false,
              "rstrip": false,
              "normalized": true,
              "special": false
            },
            {
              "id": 1,
              "content": "[CLS]",
              "single_word": true,
              "lstrip": true,
              "rstrip": true,
              "normalized": false,
              "special": true
            }
          ],
          "model": {}
        }"#,
    )?;

    assert_eq!(tokens.len(), 2);
    let ordinary = tokens.get(0).ok_or_else(|| {
        ireetok::Error::new(Code::DataLoss, "parsed added token at index 0 is missing")
    })?;
    assert_eq!(ordinary.id, 0);
    assert_eq!(ordinary.content, "▁");
    assert_eq!(ordinary.flags, AddedTokenFlags::NORMALIZED);

    let special = tokens.get(1).ok_or_else(|| {
        ireetok::Error::new(Code::DataLoss, "parsed added token at index 1 is missing")
    })?;
    assert_eq!(special.id, 1);
    assert_eq!(special.content, "[CLS]");
    assert!(special.flags.contains(AddedTokenFlags::SINGLE_WORD));
    assert!(special.flags.contains(AddedTokenFlags::LSTRIP));
    assert!(special.flags.contains(AddedTokenFlags::RSTRIP));
    assert!(special.flags.contains(AddedTokenFlags::SPECIAL));
    assert!(!special.flags.contains(AddedTokenFlags::NORMALIZED));
    assert!(tokens.get(2).is_none());
    Ok(())
}

#[test]
fn added_tokens_reject_missing_required_boolean_fields() {
    let result = AddedTokens::from_huggingface_json(
        r#"{
          "added_tokens": [{
            "id": 0,
            "content": "token",
            "special": false
          }],
          "model": {}
        }"#,
    );
    assert!(matches!(result, Err(error) if error.code() == Code::NotFound));
}

#[test]
fn added_tokens_unescape_content_and_validate_required_fields() -> ireetok::Result<()> {
    let tokens = AddedTokens::from_huggingface_json(
        r#"{
          "added_tokens": [{
            "id": 7,
            "content": "tab\there\n\"quoted\"",
            "single_word": false,
            "lstrip": false,
            "rstrip": false,
            "normalized": true,
            "special": false
          }],
          "model": {}
        }"#,
    )?;
    let token = tokens
        .get(0)
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "parsed token is missing"))?;
    assert_eq!(token.content, "tab\there\n\"quoted\"");
    assert_eq!(token.id, 7);

    for (json, expected) in [
        (
            r#"{"added_tokens":[{"content":"x","single_word":false,"lstrip":false,"rstrip":false,"normalized":true,"special":false}],"model":{}}"#,
            Code::NotFound,
        ),
        (
            r#"{"added_tokens":[{"id":0,"single_word":false,"lstrip":false,"rstrip":false,"normalized":true,"special":false}],"model":{}}"#,
            Code::NotFound,
        ),
        (
            r#"{"added_tokens":[{"id":0,"content":"x","single_word":false,"lstrip":false,"rstrip":false,"normalized":true,"special":false,"extra":true}],"model":{}}"#,
            Code::InvalidArgument,
        ),
    ] {
        assert!(
            matches!(AddedTokens::from_huggingface_json(json), Err(error) if error.code() == expected),
            "unexpected parser result for {json}"
        );
    }
    Ok(())
}
