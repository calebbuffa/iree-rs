use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=memory.ld");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    println!(
        "cargo:rustc-link-arg=-T{}",
        root.join("memory.ld").display()
    );
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
}
