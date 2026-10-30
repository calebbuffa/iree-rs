//! Rust port of IREE's local-sync VMVX embedding sample.

#[path = "run.rs"]
mod run;

use iree_samples::vm_support::SampleDevice;
use std::error::Error;

const ARCHIVE: &[u8] = include_bytes!("../assets/simple_embedding.vmfb");

fn main() -> Result<(), Box<dyn Error>> {
    let device = SampleDevice::from_uri("local-sync")?;
    run::run_with_device(device, ARCHIVE)
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs_with_local_sync_vmvx() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
