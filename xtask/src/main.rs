//! Workspace packaging convenience; native deployment lives in Woodshed.
use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args_os().skip(1);
    if arguments.next().as_deref() != Some(std::ffi::OsStr::new("package")) {
        return Err("usage: cargo xtask package <new-directory> -- <cargo options: -p iree --example compile --features compiler>".into());
    }
    let destination = PathBuf::from(arguments.next().ok_or("missing bundle destination")?);
    if arguments.next().as_deref() != Some(std::ffi::OsStr::new("--")) {
        return Err("expected -- before Cargo options".into());
    }
    let executable = woodshed::bundle_cargo(&destination, &arguments.collect::<Vec<_>>())?;
    println!("Bundled {}", executable.display());
    Ok(())
}
