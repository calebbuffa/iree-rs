//! Rust port of IREE's local-sync external-transients sample.

#[path = "run.rs"]
mod run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run::run(Some("local-sync"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_with_an_external_transient_buffer_on_local_sync()
    -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
