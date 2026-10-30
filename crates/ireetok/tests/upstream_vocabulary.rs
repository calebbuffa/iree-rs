//! Public vocabulary and index cases adapted from IREE's upstream tests.

use ireetok::{
    MergeHash, SpecialTokenKind, TokenAttributes, VocabularyBuilder, VocabularyHash, VocabularyTrie,
};

#[test]
fn vocabulary_builder_preserves_lookup_scores_attributes_and_special_ids() -> ireetok::Result<()> {
    let mut builder = VocabularyBuilder::new(3)?;
    builder.add_token("[UNK]", -1.0, TokenAttributes::UNKNOWN)?;
    builder.add_token("ab", 0.5, TokenAttributes::NONE)?;
    builder.add_token("a", 0.25, TokenAttributes::SPECIAL)?;
    builder.set_special_token(SpecialTokenKind::Unknown, 0)?;
    builder.set_special_token(SpecialTokenKind::Bos, 2)?;

    let vocabulary = builder.build()?;
    let view = vocabulary.view();
    assert_eq!(view.capacity(), 3);
    assert_eq!(view.token_count(), 3);
    assert_eq!(view.lookup(b"ab"), Some(1));
    assert_eq!(view.lookup(b"AB"), None);
    assert_eq!(view.token_text(1), Some(&b"ab"[..]));
    assert_eq!(view.token_text(-1), None);
    assert_eq!(view.token_score(0), -1.0);
    assert!(view.token_attributes(0).contains(TokenAttributes::UNKNOWN));
    assert_eq!(view.special_ids().unk, Some(0));
    assert_eq!(view.special_ids().bos, Some(2));
    assert_eq!(
        view.tokens().map(|token| token.id).collect::<Vec<_>>(),
        [0, 1, 2]
    );
    Ok(())
}

#[test]
fn vocabulary_hash_supports_owned_and_caller_backing() -> ireetok::Result<()> {
    let mut builder = VocabularyBuilder::new(3)?;
    for token in ["alpha", "beta", "gamma"] {
        builder.add_token(token, 0.0, TokenAttributes::NONE)?;
    }
    let vocabulary = builder.build()?;
    let view = vocabulary.view();

    let allocated = VocabularyHash::new(view, 70)?;
    assert_eq!(allocated.lookup(b"alpha"), Some(0));
    assert_eq!(allocated.lookup(b"missing"), None);
    drop(allocated);

    let required = VocabularyHash::storage_size(&view, 70)?;
    let words = required.div_ceil(std::mem::size_of::<usize>());
    assert!(VocabularyHash::with_storage(view, 70, vec![0; words.saturating_sub(1)]).is_err());
    let stored = VocabularyHash::with_storage(view, 70, vec![0; words])?;
    assert_eq!(stored.lookup(b"gamma"), Some(2));
    Ok(())
}

#[test]
fn vocabulary_trie_and_merge_hash_match_upstream_lookups() -> ireetok::Result<()> {
    let mut builder = VocabularyBuilder::new(3)?;
    for token in ["a", "b", "ab"] {
        builder.add_token(token, 0.0, TokenAttributes::NONE)?;
    }
    builder.add_merge(0, 1)?;
    let vocabulary = builder.build()?;
    let view = vocabulary.view();

    let trie = VocabularyTrie::new(view)?;
    assert_eq!(trie.max_depth(), 2);
    let mut cursor = trie.cursor();
    assert!(cursor.advance(b'a'));
    assert_eq!(cursor.depth(), 1);
    assert_eq!(cursor.token_id(), Some(0));
    assert!(cursor.advance(b'b'));
    assert_eq!(cursor.depth(), 2);
    assert_eq!(cursor.token_id(), Some(2));
    cursor.reset();
    assert!(!cursor.advance(b'x'));
    assert_eq!(cursor.token_id(), None);

    let merges = MergeHash::new(&view)?;
    assert_eq!(
        merges.lookup(0, 1),
        Some(ireetok::MergeResult {
            rank: 0,
            token_id: 2,
        })
    );
    assert_eq!(merges.lookup(1, 0), None);
    Ok(())
}
