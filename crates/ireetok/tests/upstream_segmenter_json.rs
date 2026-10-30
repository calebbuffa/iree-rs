//! Hugging Face pre-tokenizer JSON cases adapted from IREE's upstream tests.

use ireetok::{
    Code, PreTokenizerFlags, RegexCompileFlags, RegexDfa, Result, Segment, Segmenter, SplitBehavior,
};

fn shift_segment(segment: Segment, offset: usize) -> Result<Segment> {
    let start = segment
        .start
        .checked_add(offset)
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "segment start offset overflowed"))?;
    let end = segment
        .end
        .checked_add(offset)
        .ok_or_else(|| ireetok::Error::new(Code::DataLoss, "segment end offset overflowed"))?;
    Ok(Segment { start, end })
}

fn segment_all(segmenter: &Segmenter, input: &str) -> Result<Vec<Segment>> {
    let mut state = segmenter.state()?;
    let mut output = vec![Segment::default(); input.len().saturating_add(1)];
    let progress = state.process(input, &mut output)?;
    let remaining_input = input.get(progress.consumed..).ok_or_else(|| {
        ireetok::Error::new(Code::DataLoss, "segmenter consumed beyond the test input")
    })?;
    let mut segments = output[..progress.written].to_vec();
    let mut finalized = vec![Segment::default(); output.len().saturating_sub(progress.written)];
    let finalized_count = state.finalize(remaining_input, &mut finalized)?;
    segments.extend(
        finalized[..finalized_count]
            .iter()
            .map(|&segment| shift_segment(segment, progress.consumed))
            .collect::<Result<Vec<_>>>()?,
    );
    Ok(segments)
}

fn segment_text<'a>(segmenter: &Segmenter, input: &'a str) -> Result<Vec<&'a str>> {
    let segments = segment_all(segmenter, input)?;
    segments
        .into_iter()
        .map(|segment| {
            input.get(segment.start..segment.end).ok_or_else(|| {
                ireetok::Error::new(Code::DataLoss, "segment offsets are not UTF-8 boundaries")
            })
        })
        .collect()
}

#[test]
fn punctuation_split_behaviors_match_upstream_fixtures() -> Result<()> {
    for (behavior, consecutive, separated) in [
        (
            SplitBehavior::Isolated,
            &["a", ".", ".", ".", "b"][..],
            &["a", ".", "b", ".", "c"][..],
        ),
        (
            SplitBehavior::Removed,
            &["a", "b"][..],
            &["a", "b", "c"][..],
        ),
        (
            SplitBehavior::MergedWithPrevious,
            &["a.", ".", ".", "b"][..],
            &["a.", "b.", "c"][..],
        ),
        (
            SplitBehavior::MergedWithNext,
            &["a", ".", ".", ".b"][..],
            &["a", ".b", ".c"][..],
        ),
        (
            SplitBehavior::Contiguous,
            &["a", "...", "b"][..],
            &["a", ".", "b", ".", "c"][..],
        ),
    ] {
        let segmenter = Segmenter::punctuation(behavior)?;
        assert_eq!(
            segment_text(&segmenter, "a...b")?,
            consecutive,
            "{behavior:?} on consecutive matches"
        );
        assert_eq!(
            segment_text(&segmenter, "a.b.c")?,
            separated,
            "{behavior:?} on separated matches"
        );
    }

    let unicode_punctuation = Segmenter::punctuation(SplitBehavior::Isolated)?;
    assert_eq!(
        segment_text(&unicode_punctuation, "café—日本語!")?,
        ["café", "—", "日本語", "!"]
    );
    assert_eq!(
        segment_text(&unicode_punctuation, "hello world")?,
        ["hello world"]
    );
    assert_eq!(
        segment_text(&unicode_punctuation, "a\u{00a0}b")?,
        ["a\u{00a0}b"]
    );
    Ok(())
}

#[test]
fn digit_segmenter_supports_individual_and_grouped_runs() -> Result<()> {
    let individual = Segmenter::digits(true)?;
    assert_eq!(
        segment_text(&individual, "abc123def34")?,
        ["abc", "1", "2", "3", "def", "3", "4"]
    );

    let grouped = Segmenter::digits(false)?;
    assert_eq!(
        segment_text(&grouped, "abc123def34")?,
        ["abc", "123", "def", "34"]
    );
    assert_eq!(
        segment_text(&grouped, "a1b2c3")?,
        ["a", "1", "b", "2", "c", "3"]
    );
    Ok(())
}

#[test]
fn whitespace_and_metaspace_segmenters_preserve_source_byte_offsets() -> Result<()> {
    let whitespace = Segmenter::whitespace()?;
    assert_eq!(
        segment_text(&whitespace, "  café\t日本語 \n")?,
        ["café", "日本語"]
    );
    for (input, expected) in [
        ("hello\nworld\n", &["hello", "world"][..]),
        ("hello\rworld", &["hello", "world"][..]),
        ("a\t\n\r b", &["a", "b"][..]),
        ("  hello    world  ", &["hello", "world"][..]),
        ("こんにちは 世界", &["こんにちは", "世界"][..]),
    ] {
        assert_eq!(segment_text(&whitespace, input)?, expected, "{input:?}");
    }

    let metaspace = Segmenter::metaspace('▁', true)?;
    assert_eq!(
        segment_text(&metaspace, "hello▁world")?,
        ["hello", "▁world"]
    );
    let unsplit_metaspace = Segmenter::metaspace('▁', false)?;
    assert_eq!(
        segment_text(&unsplit_metaspace, "hello▁world")?,
        ["hello▁world"]
    );
    Ok(())
}

#[test]
fn bert_segmenter_matches_upstream_word_and_punctuation_boundaries() -> Result<()> {
    let bert = Segmenter::bert()?;
    for (input, expected) in [
        (
            "Hey friend!     How are you?!?",
            &["Hey", "friend", "!", "How", "are", "you", "?", "!", "?"][..],
        ),
        ("don't stop", &["don", "'", "t", "stop"][..]),
        ("price: $100.00", &["price", ":", "$", "100", ".", "00"][..]),
        ("hello\tworld\nnewline", &["hello", "world", "newline"][..]),
        (
            "caf\u{00e9} r\u{00e9}sum\u{00e9}",
            &["caf\u{00e9}", "r\u{00e9}sum\u{00e9}"][..],
        ),
        ("a\u{00a0}b", &["a", "b"][..]),
        (
            "hello\u{ff08}world\u{ff09}",
            &["hello", "\u{ff08}", "world", "\u{ff09}"][..],
        ),
        ("test\u{2014}dash", &["test", "\u{2014}", "dash"][..]),
        ("x\u{00b7}y", &["x", "\u{00b7}", "y"][..]),
        ("こんにちは世界", &["こんにちは世界"][..]),
    ] {
        assert_eq!(segment_text(&bert, input)?, expected, "{input:?}");
    }

    let only_whitespace = Segmenter::bert()?;
    assert!(segment_text(&only_whitespace, "  \t\n  ")?.is_empty());
    Ok(())
}

#[test]
fn whitespace_streaming_tracks_pending_segments_and_offsets() -> Result<()> {
    let segmenter = Segmenter::whitespace()?;
    let input = "hello world";
    let mut state = segmenter.state()?;
    let mut output = [Segment::default(); 4];

    let progress = state.process(input, &mut output)?;
    assert!(progress.consumed <= input.len());
    assert!(state.has_pending());
    let mut ranges = output[..progress.written].to_vec();

    let remaining = input.get(progress.consumed..).ok_or_else(|| {
        ireetok::Error::new(Code::DataLoss, "segmenter consumed beyond the test input")
    })?;
    let mut finalized = [Segment::default(); 4];
    let finalized_count = state.finalize(remaining, &mut finalized)?;
    ranges.extend(
        finalized[..finalized_count]
            .iter()
            .map(|&segment| shift_segment(segment, progress.consumed))
            .collect::<Result<Vec<_>>>()?,
    );

    let text = ranges
        .into_iter()
        .map(|segment| {
            input.get(segment.start..segment.end).ok_or_else(|| {
                ireetok::Error::new(Code::DataLoss, "segment offsets are not UTF-8 boundaries")
            })
        })
        .collect::<Result<Vec<_>>>()?;
    assert_eq!(text, ["hello", "world"]);
    assert!(!state.has_pending());
    Ok(())
}

#[test]
fn split_segmenter_matches_upstream_empty_and_zero_capacity_cases() -> Result<()> {
    let regex = RegexDfa::compile(r"\s+", RegexCompileFlags::NONE)?;
    let segmenter = Segmenter::split(regex, SplitBehavior::Isolated, false)?;
    let mut state = segmenter.state()?;
    let mut output = [];

    let empty = state.process("", &mut output)?;
    assert_eq!(empty.consumed, 0);
    assert_eq!(empty.written, 0);
    assert!(!state.has_pending());

    let zero_capacity = state.process("hello world", &mut output)?;
    assert_eq!(zero_capacity.consumed, 0);
    assert_eq!(zero_capacity.written, 0);
    Ok(())
}

#[test]
fn split_segmenter_reports_unconsumed_suffix_without_pending_state() -> Result<()> {
    let regex = RegexDfa::compile(r"\s+", RegexCompileFlags::NONE)?;
    let segmenter = Segmenter::split(regex, SplitBehavior::Isolated, false)?;
    let input = "hello";
    let mut state = segmenter.state()?;
    let mut output = [Segment::default(); 2];

    let progress = state.process(input, &mut output)?;
    assert_eq!(progress.consumed, 0);
    assert_eq!(progress.written, 0);
    assert!(!state.has_pending());

    let remaining = input.get(progress.consumed..).ok_or_else(|| {
        ireetok::Error::new(Code::DataLoss, "segmenter consumed beyond the test input")
    })?;
    let finalized = state.finalize(remaining, &mut output)?;
    assert_eq!(finalized, 1);
    assert_eq!(
        output[0],
        Segment {
            start: 0,
            end: input.len()
        }
    );
    assert!(!state.has_pending());
    Ok(())
}

#[test]
fn split_segmenter_preserves_large_match_sets() -> Result<()> {
    let segmenter = Segmenter::split_literal(" ", SplitBehavior::Removed, false)?;
    let input = (0..300).map(|_| "w").collect::<Vec<_>>().join(" ");
    let segments = segment_text(&segmenter, &input)?;
    assert_eq!(segments.len(), 300);
    assert!(segments.iter().all(|segment| *segment == "w"));
    Ok(())
}

#[test]
fn literal_split_segmenter_matches_upstream_modes_and_inversion() -> Result<()> {
    let isolated = Segmenter::split_literal(",", SplitBehavior::Isolated, false)?;
    assert_eq!(
        segment_text(&isolated, "a,b,,c")?,
        ["a", ",", "b", ",", ",", "c"]
    );

    let removed = Segmenter::split_literal(",", SplitBehavior::Removed, false)?;
    assert_eq!(segment_text(&removed, "a,b,,c")?, ["a", "b", "c"]);

    let merged_previous = Segmenter::split_literal(",", SplitBehavior::MergedWithPrevious, false)?;
    assert_eq!(segment_text(&merged_previous, "a,b,c")?, ["a,", "b,", "c"]);

    let regex = RegexDfa::compile(r"\s+", RegexCompileFlags::NONE)?;
    let regex_split = Segmenter::split(regex, SplitBehavior::Isolated, false)?;
    assert_eq!(segment_text(&regex_split, "a  b")?, ["a", "  ", "b"]);

    let unicode_letters = RegexDfa::compile(r"\p{L}+", RegexCompileFlags::NONE)?;
    let unicode_split = Segmenter::split(unicode_letters, SplitBehavior::Isolated, false)?;
    assert_eq!(
        segment_text(&unicode_split, "hello 世界")?,
        ["hello", " ", "世界"]
    );

    let gpt2_pattern = RegexDfa::compile(r"\p{L}+|\p{N}+|\s+|[.,!?]", RegexCompileFlags::NONE)?;
    let gpt2_split = Segmenter::split(gpt2_pattern, SplitBehavior::Isolated, false)?;
    assert_eq!(
        segment_text(&gpt2_split, "Hello, world!")?,
        ["Hello", ",", " ", "world", "!"]
    );

    let inverted_removed = Segmenter::split_literal("abc", SplitBehavior::Removed, true)?;
    assert_eq!(
        segment_text(&inverted_removed, "XXXabcYYYabcZZZ")?,
        ["abc", "abc"]
    );

    Ok(())
}

#[test]
fn huggingface_segmenter_json_parses_common_components_and_flags() -> Result<()> {
    let (segmenter, flags) = Segmenter::from_huggingface_json("null")?;
    assert!(segmenter.is_none());
    assert_eq!(flags, PreTokenizerFlags::NONE);

    let (segmenter, flags) = Segmenter::from_huggingface_json(r#"{"type":"BertPreTokenizer"}"#)?;
    assert!(flags.contains(PreTokenizerFlags::WORD_LEVEL_SPLIT));
    let segments = segment_all(&segmenter.unwrap(), "Hello, world!")?;
    assert_eq!(
        segments,
        [
            Segment { start: 0, end: 5 },
            Segment { start: 5, end: 6 },
            Segment { start: 7, end: 12 },
            Segment { start: 12, end: 13 },
        ]
    );

    let (segmenter, flags) = Segmenter::from_huggingface_json(
        r#"{"type":"Metaspace","replacement":"▁","prepend_scheme":"first","split":false}"#,
    )?;
    assert!(segmenter.is_some());
    assert!(flags.contains(PreTokenizerFlags::METASPACE_PREPEND));
    assert!(flags.contains(PreTokenizerFlags::METASPACE_REPLACE));

    let (segmenter, flags) = Segmenter::from_huggingface_json(r#"{"type":"Punctuation"}"#)?;
    assert!(segmenter.is_some());
    assert!(flags.contains(PreTokenizerFlags::WORD_LEVEL_SPLIT));

    let (segmenter, flags) = Segmenter::from_huggingface_json(r#"{"type":"Whitespace"}"#)?;
    assert!(segmenter.is_some());
    assert!(flags.contains(PreTokenizerFlags::HAS_WHITESPACE_SPLIT));
    Ok(())
}

#[test]
fn huggingface_segmenter_json_sequences_compact_null_children() -> Result<()> {
    let (empty, _) = Segmenter::from_huggingface_json(r#"{"type":"Sequence","pretokenizers":[]}"#)?;
    assert!(empty.is_none());

    let (single, _) = Segmenter::from_huggingface_json(
        r#"{"type":"Sequence","pretokenizers":[{"type":"Metaspace","replacement":"▁","split":true}]}"#,
    )?;
    assert!(single.is_some());

    let (sequence, flags) = Segmenter::from_huggingface_json(
        r#"{"type":"Sequence","pretokenizers":[{"type":"Metaspace","replacement":"▁","split":true},{"type":"Punctuation","behavior":"Isolated"}]}"#,
    )?;
    assert!(sequence.is_some());
    assert!(flags.contains(PreTokenizerFlags::METASPACE_REPLACE));
    assert!(flags.contains(PreTokenizerFlags::WORD_LEVEL_SPLIT));

    let (only_null, _) = Segmenter::from_huggingface_json(
        r#"{"type":"Sequence","pretokenizers":[{"type":"ByteLevel","add_prefix_space":false,"trim_offsets":false,"use_regex":false}]}"#,
    )?;
    assert!(only_null.is_none());
    Ok(())
}

#[test]
fn huggingface_segmenter_json_rejects_invalid_configuration() {
    for (json, expected) in [
        (r#"{"type":"Metaspace"}"#, Code::NotFound),
        (
            r#"{"type":"Punctuation","behavior":"Unknown"}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"type":"Split","pattern":{"String":""},"behavior":"Isolated","invert":false}"#,
            Code::InvalidArgument,
        ),
        (
            r#"{"type":"Split","pattern":{},"behavior":"Isolated","invert":false}"#,
            Code::InvalidArgument,
        ),
        (r#"{"type":"Sequence"}"#, Code::NotFound),
        (
            r#"{"type":"Sequence","pretokenizers":[{"type":"Unsupported"}]}"#,
            Code::Unimplemented,
        ),
        (r#"{"type":"Unsupported"}"#, Code::Unimplemented),
        (r#"{"not_type":"Whitespace"}"#, Code::NotFound),
    ] {
        assert!(
            matches!(Segmenter::from_huggingface_json(json), Err(error) if error.code() == expected),
            "unexpected parser result for {json}"
        );
    }
}

#[test]
fn huggingface_segmenter_json_validates_sequence_child_limit() -> Result<()> {
    let child = r#"{"type":"Metaspace","replacement":"▁"}"#;
    for (count, should_succeed) in [(8, true), (9, false)] {
        let children = vec![child; count].join(",");
        let json = format!(r#"{{"type":"Sequence","pretokenizers":[{children}]}}"#);
        let result = Segmenter::from_huggingface_json(&json);
        let valid = if should_succeed {
            matches!(result, Ok((Some(_), _)))
        } else {
            matches!(result, Err(error) if error.code() == Code::InvalidArgument)
        };
        if !valid {
            return Err(ireetok::Error::new(
                Code::DataLoss,
                format!("unexpected result for {count} sequence children"),
            ));
        }
    }
    Ok(())
}

#[test]
fn huggingface_segmenter_json_accepts_llama_three_pattern() -> Result<()> {
    let json = r#"{"type":"Split","pattern":{"Regex":"(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?\\p{L}+|\\p{N}{1,3}| ?[^\\s\\p{L}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+"},"behavior":"Isolated","invert":false}"#;
    let (segmenter, _) = Segmenter::from_huggingface_json(json)?;
    assert!(segmenter.is_some());
    Ok(())
}
