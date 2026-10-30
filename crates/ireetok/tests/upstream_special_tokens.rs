//! Special-token matcher cases adapted from IREE's upstream tests.

use ireetok::{Code, SpecialTokenFlags, SpecialTokenMatch, SpecialTokensBuilder};

#[test]
fn special_token_matcher_recognizes_exact_prefix_and_keeps_trailing_input() -> ireetok::Result<()> {
    let mut builder = SpecialTokensBuilder::new();
    builder.add("<|endoftext|>", 50256, SpecialTokenFlags::NONE)?;
    let tokens = builder.build()?;
    let mut matcher = tokens.matcher();

    assert_eq!(
        matcher.match_prefix(b"<|endoftext|>tail"),
        SpecialTokenMatch::Matched {
            length: 13,
            id: 50256,
            flags: SpecialTokenFlags::NONE
        }
    );
    Ok(())
}

#[test]
fn special_token_matcher_retains_partial_prefix_across_chunks() -> ireetok::Result<()> {
    let mut builder = SpecialTokensBuilder::new();
    builder.add("<|endoftext|>", 50256, SpecialTokenFlags::NONE)?;
    let tokens = builder.build()?;
    let mut matcher = tokens.matcher();

    assert_eq!(matcher.match_prefix(b"<|"), SpecialTokenMatch::NeedMore);
    assert!(matcher.has_partial());
    assert_eq!(matcher.partial_bytes(), b"<|");
    assert_eq!(
        matcher.match_prefix(b"endoftext"),
        SpecialTokenMatch::NeedMore
    );
    assert_eq!(
        matcher.match_prefix(b"|>"),
        SpecialTokenMatch::Matched {
            length: 2,
            id: 50256,
            flags: SpecialTokenFlags::NONE
        }
    );
    assert!(!matcher.has_partial());
    Ok(())
}

#[test]
fn special_token_matcher_prefers_longest_overlapping_spelling() -> ireetok::Result<()> {
    let mut builder = SpecialTokensBuilder::new();
    builder.add("<s>", 1, SpecialTokenFlags::NONE)?;
    builder.add("<special>", 2, SpecialTokenFlags::NONE)?;
    let tokens = builder.build()?;
    let mut matcher = tokens.matcher();

    assert_eq!(
        matcher.match_prefix(b"<special>tail"),
        SpecialTokenMatch::Matched {
            length: 9,
            id: 2,
            flags: SpecialTokenFlags::NONE
        }
    );
    Ok(())
}

#[test]
fn special_token_builder_rejects_empty_spelling() {
    let mut builder = SpecialTokensBuilder::new();
    let result = builder.add("", 1, SpecialTokenFlags::NONE);
    assert!(matches!(result, Err(error) if error.code() == Code::InvalidArgument));
}

#[test]
fn special_token_lstrip_requires_input_start_or_a_whitespace_boundary() -> ireetok::Result<()> {
    let mut builder = SpecialTokensBuilder::new();
    builder.add("<x>", 7, SpecialTokenFlags::LSTRIP)?;
    let tokens = builder.build()?;

    let mut at_start = tokens.matcher();
    assert!(matches!(
        at_start.match_prefix(b"<x>"),
        SpecialTokenMatch::Matched { id: 7, .. }
    ));

    let mut after_whitespace = tokens.matcher();
    after_whitespace.update_previous_byte(b' ');
    assert!(matches!(
        after_whitespace.match_prefix(b"<x>"),
        SpecialTokenMatch::Matched { id: 7, .. }
    ));

    let mut after_word = tokens.matcher();
    after_word.update_previous_byte(b'a');
    assert_eq!(after_word.match_prefix(b"<x>"), SpecialTokenMatch::NoMatch);
    Ok(())
}
