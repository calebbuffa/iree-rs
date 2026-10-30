//! Rust port of IREE's local-sync embedded-ELF sample.

#[cfg(target_os = "linux")]
#[path = "run.rs"]
mod run;

#[cfg(target_os = "linux")]
use iree_samples::vm_support::SampleDevice;
#[cfg(target_os = "linux")]
use ireert::hal::backends::local_loader::ExecutableLoader;
#[cfg(target_os = "linux")]
use std::error::Error;

#[cfg(target_os = "linux")]
const ARCHIVE: &[u8] = include_bytes!("../assets/simple_embedding_llvm_cpu.vmfb");

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let loader = ExecutableLoader::embedded_elf(None)?;
    let device = SampleDevice::local_sync_with_loaders(&[loader])?;
    run::run_with_device(device, ARCHIVE)
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("This upstream sample requires the Linux embedded-ELF loader; skipping.");
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    #[test]
    fn runs_with_embedded_elf_on_local_sync() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
