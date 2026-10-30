//! Rust port of the compiler-side upstream static-library sample.

use ireec::{Compiler, Output, Pipeline};
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

const MODULE: &str = include_str!("../../extern/iree/samples/static_library/simple_mul.mlir");
static NEXT_DIRECTORY_ID: AtomicUsize = AtomicUsize::new(0);

struct OutputDirectory {
    path: PathBuf,
    cleaned: bool,
}

impl OutputDirectory {
    fn new() -> io::Result<Self> {
        let output_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/iree-samples");
        fs::create_dir_all(&output_root)?;
        for _ in 0..128 {
            let id = NEXT_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
            let path = output_root.join(format!(
                "iree-samples-static-library-{}-{id}",
                std::process::id()
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    return Ok(Self {
                        path,
                        cleaned: false,
                    });
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "could not allocate a unique static-library sample directory",
        ))
    }

    fn object_path(&self) -> PathBuf {
        self.path.join("simple_mul.o")
    }

    fn header_path(&self) -> PathBuf {
        self.path.join("simple_mul.h")
    }

    fn cleanup(mut self) -> io::Result<()> {
        fs::remove_dir_all(&self.path)?;
        self.cleaned = true;
        Ok(())
    }
}

impl Drop for OutputDirectory {
    fn drop(&mut self) {
        if !self.cleaned {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let output_directory = OutputDirectory::new()?;
    let object_path = output_directory.object_path();
    let object_path = object_path.to_str().ok_or("object path is not UTF-8")?;
    let compiler = Compiler::new()?;
    let mut session = compiler.session()?;
    session.set_flags(&[
        "--iree-hal-target-device=local",
        "--iree-hal-local-target-device-backends=llvm-cpu",
        "--iree-llvmcpu-target-cpu=generic",
        "--iree-llvmcpu-link-embedded=false",
        "--iree-llvmcpu-link-static",
        &format!("--iree-llvmcpu-static-library-output-path={object_path}"),
        "--iree-vm-target-index-bits=32",
    ])?;
    let source = session.source_buffer("static_library.mlir", MODULE.as_bytes())?;
    let mut invocation = session.invocation()?;
    invocation.parse_source(source)?;
    invocation.pipeline(Pipeline::Std)?;
    let mut bytecode = Output::memory(&compiler)?;
    invocation.output_vm_bytecode(&mut bytecode)?;

    assert!(!bytecode.bytes()?.is_empty());
    assert!(Path::new(object_path).is_file(), "{object_path}");
    assert!(output_directory.header_path().is_file());
    output_directory.cleanup()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::OutputDirectory;

    #[test]
    fn output_directories_are_unique_and_cleaned() -> std::io::Result<()> {
        let first = OutputDirectory::new()?;
        let first_path = first.path.clone();
        let second_path;
        {
            let second = OutputDirectory::new()?;
            second_path = second.path.clone();
            assert_ne!(first_path, second_path);
            assert!(second_path.is_dir());
            std::fs::write(second.object_path(), b"sample object")?;
        }
        assert!(!second_path.exists());
        std::fs::write(first.header_path(), b"sample header")?;
        first.cleanup()?;
        assert!(!first_path.exists());
        Ok(())
    }

    #[test]
    fn emits_static_library_and_vm_bytecode() -> Result<(), Box<dyn std::error::Error>> {
        super::main()
    }
}
