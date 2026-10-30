//! Compile the model on the host and configure the Emscripten final link.

use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=model/simple_mul.mlir");

    generate_vmfb();

    // Emscripten-only final-link contract.
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("emscripten") {
        emit_emscripten_link_args();
    }
}

fn generate_vmfb() {
    use iree_compiler::{CompileOptions, Compiler, Input, Target};

    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let source = fs::read_to_string(manifest.join("model").join("simple_mul.mlir"))
        .expect("read model/simple_mul.mlir");

    // VMVX can be loaded on the host or the Wasm target.
    let options = CompileOptions::new(Target::Vmvx).input(Input::Auto);
    let mut compiler = Compiler::new().expect(
        "the `compiler` build-dependency must provide the IREE compiler library; \
         see crates/iree-native/README.md",
    );
    let artifact = compiler
        .compile(source.as_str(), &options)
        .expect("compile simple_mul.mlir to a VMVX vmfb");
    fs::write(out.join("simple_mul.vmfb"), artifact.as_bytes()).expect("write simple_mul.vmfb");
}

fn emit_emscripten_link_args() {
    // The JS loader supplies these host imports at instantiation.
    for arg in [
        "-Wl,--allow-undefined",
        "-sERROR_ON_UNDEFINED_SYMBOLS=0",
        // Instantiated by our loader with custom imports, not auto-run.
        "-sMODULARIZE=1",
        "-sEXPORT_ES6=1",
        "-sEXPORT_NAME=createIreeModule",
        "-sALLOW_MEMORY_GROWTH=1",
        "-sENVIRONMENT=node,web",
        // Preserve the exported entry point and the allocator the loader needs.
        "-sEXPORTED_FUNCTIONS=_iree_wasm_demo_run,_malloc,_free",
        "-sEXPORTED_RUNTIME_METHODS=ccall,cwrap",
    ] {
        println!("cargo:rustc-link-arg={arg}");
    }
}
