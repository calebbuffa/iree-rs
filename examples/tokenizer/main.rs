//! Rust port of IREE's tokenizer Hugging Face smoke test.

use ireetok::Tokenizer;
use std::error::Error;

const TOKENIZER_JSON: &str = r###"{
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

fn main() -> Result<(), Box<dyn Error>> {
    let tokenizer = Tokenizer::from_huggingface_json(TOKENIZER_JSON)?;
    let tokens = tokenizer.encode("hello worlds")?;
    assert_eq!(tokens, [1, 2, 3]);
    assert_eq!(tokenizer.decode(&tokens)?, "hello worlds");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn encodes_and_decodes_huggingface_wordpiece() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
