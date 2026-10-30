//! Rust port of IREE's embedded-bytecode HAL sample.

mod run;

use iree_samples::vm_support::SampleDevice;

const ARCHIVE: &[u8] = include_bytes!("../assets/simple_embedding.vmfb");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = SampleDevice::from_environment()?;
    run::run_with_device(device, ARCHIVE)
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_with_embedded_bytecode() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
