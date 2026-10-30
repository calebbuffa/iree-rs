//! Rust port of IREE's Vulkan embedding sample.

#[path = "run.rs"]
mod run;

use iree_samples::vm_support::SampleDevice;
use std::error::Error;

const ARCHIVE: &[u8] = include_bytes!("../assets/simple_embedding_vulkan.vmfb");

fn main() -> Result<(), Box<dyn Error>> {
    let device = SampleDevice::from_uri("vulkan")?;
    run::run_with_device(device, ARCHIVE)
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_with_embedded_bytecode_on_vulkan() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
