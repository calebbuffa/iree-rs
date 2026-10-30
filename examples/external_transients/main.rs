//! Rust port of IREE's externally allocated transient-buffer sample.

mod run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run::run(None)
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_with_an_external_transient_buffer() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
