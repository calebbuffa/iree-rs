//! Tests safe Rust tokenizer extension adapters.
use ireetok::{
    Code, Decoder, Error, Model, Normalizer, NormalizerFlags, Progress, Result, Segment, Segmenter,
    TokenAttributes, TokenOutput, TokenizerBuilder, VocabularyBuilder,
    extension::{DecoderExtension, ModelExtension, NormalizerExtension, SegmenterExtension},
};

struct Lowercase;

impl NormalizerExtension for Lowercase {
    type State = ();

    fn process(
        &self,
        _: &mut Self::State,
        input: &str,
        output: &mut [u8],
        _: NormalizerFlags,
    ) -> Result<Progress> {
        let written = input.len().min(output.len());
        for (destination, source) in output.iter_mut().zip(input.bytes()).take(written) {
            *destination = source.to_ascii_lowercase();
        }
        Ok(Progress {
            consumed: written,
            written,
        })
    }
}

struct InvalidProgress;

impl NormalizerExtension for InvalidProgress {
    type State = ();

    fn process(
        &self,
        _: &mut Self::State,
        _: &str,
        _: &mut [u8],
        _: NormalizerFlags,
    ) -> Result<Progress> {
        Ok(Progress {
            consumed: 1,
            written: 1,
        })
    }
}

struct WholeInput;

impl SegmenterExtension for WholeInput {
    type State = ();

    fn process(
        &self,
        _: &mut Self::State,
        input: &str,
        output: &mut [Segment],
    ) -> Result<Progress> {
        if output.is_empty() || input.is_empty() {
            return Ok(Progress::default());
        }
        output[0] = Segment {
            start: 0,
            end: input.len(),
        };
        Ok(Progress {
            consumed: input.len(),
            written: 1,
        })
    }
}

struct Concatenate;

impl DecoderExtension for Concatenate {
    type State = ();

    fn process(
        &self,
        _: &mut Self::State,
        tokens: &[&[u8]],
        output: &mut [u8],
    ) -> Result<Progress> {
        let mut consumed = 0;
        let mut written = 0usize;
        for token in tokens {
            let Some(end) = written.checked_add(token.len()) else {
                break;
            };
            if end > output.len() {
                break;
            }
            output[written..end].copy_from_slice(token);
            consumed += 1;
            written = end;
        }
        Ok(Progress { consumed, written })
    }
}

struct SingleToken;

impl ModelExtension for SingleToken {
    const TYPE_NAME: &'static str = "single-token-test";

    type State = ();

    fn encode(
        &self,
        _: &mut Self::State,
        vocab: ireetok::VocabularyRef<'_>,
        transformed_text: &[u8],
        segments: &[Segment],
        _: bool,
        output: &mut TokenOutput<'_>,
    ) -> Result<Progress> {
        if segments.is_empty() || output.ids_mut().is_empty() {
            return Ok(Progress::default());
        }
        let segment = &segments[0];
        let token = &transformed_text[segment.start..segment.end];
        let Some(token_id) = vocab.lookup(token) else {
            return Err(Error::new(
                Code::InvalidArgument,
                "test model token is missing from vocabulary",
            ));
        };
        output.ids_mut()[0] = token_id;
        Ok(Progress {
            consumed: segments.len(),
            written: 1,
        })
    }
}

#[test]
fn custom_normalizer_runs_through_native_stream_state() -> Result<()> {
    let normalizer = Normalizer::from_extension(Lowercase);
    let mut state = normalizer.state()?;
    let mut output = [0; 5];
    let progress = state.process("HELLO", &mut output, NormalizerFlags::NONE)?;

    assert_eq!(
        progress,
        Progress {
            consumed: 5,
            written: 5
        }
    );
    assert_eq!(&output, b"hello");
    assert_eq!(state.finalize(&mut output)?, 0);
    assert!(!state.has_pending());
    Ok(())
}

#[test]
fn custom_normalizer_rejects_invalid_progress() -> Result<()> {
    let normalizer = Normalizer::from_extension(InvalidProgress);
    let mut state = normalizer.state()?;
    assert!(matches!(
        state.process("input", &mut [], NormalizerFlags::NONE),
        Err(error) if error.code() == ireetok::Code::InvalidArgument
    ));
    Ok(())
}

#[test]
fn custom_segmenter_runs_through_native_stream_state() -> Result<()> {
    let segmenter = Segmenter::from_extension(WholeInput);
    let mut state = segmenter.state()?;
    let mut output = [Segment::default()];
    let progress = state.process("whole input", &mut output)?;

    assert_eq!(
        progress,
        Progress {
            consumed: 11,
            written: 1
        }
    );
    assert_eq!(output[0], Segment { start: 0, end: 11 });
    Ok(())
}

#[test]
fn custom_decoder_handles_arbitrary_token_bytes() -> Result<()> {
    let decoder = Decoder::from_extension(Concatenate);
    let mut state = decoder.state()?;
    let token = [0xff, 0x80];
    let mut output = [0; 2];
    let progress = state.process(&[&token[..]], &mut output)?;

    assert_eq!(
        progress,
        Progress {
            consumed: 1,
            written: 2
        }
    );
    assert_eq!(output, token);
    Ok(())
}

#[test]
fn custom_model_owns_vocab_and_encodes_through_native_state() -> Result<()> {
    let mut vocab = VocabularyBuilder::new(1)?;
    vocab.add_token("x", 0.0, TokenAttributes::NONE)?;
    let model = Model::from_extension(vocab.build()?, SingleToken)?;
    assert_eq!(model.vocab().token_text(0), Some(&b"x"[..]));

    let mut state = model.state()?;
    let mut ids = [-1];
    let mut output = TokenOutput::new(&mut ids, None, None)?;
    let progress = state.encode(b"x", &[Segment { start: 0, end: 1 }], false, &mut output)?;
    assert_eq!(
        progress,
        Progress {
            consumed: 1,
            written: 1
        }
    );
    assert_eq!(ids, [0]);
    Ok(())
}

#[test]
fn custom_model_transfers_model_and_vocabulary_to_tokenizer() -> Result<()> {
    let mut vocab = VocabularyBuilder::new(1)?;
    vocab.add_token("x", 0.0, TokenAttributes::NONE)?;
    let model = Model::from_extension(vocab.build()?, SingleToken)?;
    let mut builder = TokenizerBuilder::new();
    builder.set_normalizer(Normalizer::from_extension(Lowercase));
    builder.set_segmenter(Segmenter::from_extension(WholeInput));
    builder.set_model(model);
    builder.set_decoder(Decoder::from_extension(Concatenate));
    let tokenizer = builder.build()?;

    assert_eq!(tokenizer.encode("X")?, [0]);
    assert_eq!(tokenizer.decode(&[0])?, "x");
    assert_eq!(tokenizer.vocab().token_text(0), Some(&b"x"[..]));
    Ok(())
}

struct SequenceChild {
    _owner: std::sync::Arc<()>,
}

impl NormalizerExtension for SequenceChild {
    type State = ();

    fn process(&self, _: &mut (), _: &str, _: &mut [u8], _: NormalizerFlags) -> Result<Progress> {
        Ok(Progress::default())
    }
}

impl SegmenterExtension for SequenceChild {
    type State = ();

    fn process(&self, _: &mut (), _: &str, _: &mut [Segment]) -> Result<Progress> {
        Ok(Progress::default())
    }
}

impl DecoderExtension for SequenceChild {
    type State = ();

    fn flags(&self) -> ireetok::DecoderFlags {
        ireetok::DecoderFlags::SHRINKING
    }

    fn process(&self, _: &mut (), _: &[&[u8]], _: &mut [u8]) -> Result<Progress> {
        Ok(Progress::default())
    }
}

#[test]
fn sequences_retain_children_until_drop_and_release_rejected_children() -> Result<()> {
    let owner = std::sync::Arc::new(());
    let child = || SequenceChild {
        _owner: owner.clone(),
    };
    let normalizer = Normalizer::sequence(vec![
        Normalizer::from_extension(child()),
        Normalizer::from_extension(child()),
    ])?;
    let segmenter = Segmenter::sequence(vec![
        Segmenter::from_extension(child()),
        Segmenter::from_extension(child()),
    ])?;
    let decoder = Decoder::sequence(vec![
        Decoder::from_extension(child()),
        Decoder::from_extension(child()),
    ])?;
    assert_eq!(std::sync::Arc::strong_count(&owner), 7);
    drop((normalizer, segmenter, decoder));
    assert_eq!(std::sync::Arc::strong_count(&owner), 1);

    assert!(Normalizer::sequence(vec![Normalizer::from_extension(child())]).is_err());
    assert!(Segmenter::sequence(vec![Segmenter::from_extension(child())]).is_err());
    assert!(Decoder::sequence(vec![Decoder::from_extension(child())]).is_err());
    assert_eq!(std::sync::Arc::strong_count(&owner), 1);
    Ok(())
}
