//! Native feature selection must not be coupled to binding generation.
#[path = "../native/config.rs"]
mod config;

#[test]
fn runtime_and_compiler_backends_are_independent() {
    let cuda = config::Configuration::new(["cuda"]);
    assert!(cuda.validate_target("x86_64-pc-windows-msvc").is_ok());
    assert!(cuda.needs_source("iree"));
    assert!(!cuda.needs_source("llvm"));
    let options = cuda.options("aarch64");
    assert!(options.contains(&("IREE_HAL_DRIVER_CUDA".into(), "ON".into())));
    assert!(options.contains(&("IREE_TARGET_BACKEND_CUDA".into(), "OFF".into())));
    let metal = config::Configuration::new(["compiler", "target-metal", "llvm-riscv"]);
    assert!(metal.validate_target("x86_64-pc-windows-msvc").is_ok());
    assert!(metal.needs_source("spirv_cross"));
    let options = metal.options("x86_64");
    assert!(options.contains(&("IREE_HAL_DRIVER_METAL".into(), "OFF".into())));
    assert!(options.contains(&("IREE_TARGET_BACKEND_METAL_SPIRV".into(), "ON".into())));
    assert!(options.contains(&("IREE_DEFAULT_CPU_LLVM_TARGETS".into(), "RISCV;X86".into())));
}

#[test]
fn cpu_capabilities_and_local_sync_are_distinct() {
    let cpu = config::Configuration::new(["cpu"]);
    assert_eq!(
        cpu.native_capabilities(),
        ["cpu", "local-sync", "local-task", "task"]
    );
    let sync = config::Configuration::new(["local-sync"]);
    assert_eq!(sync.native_capabilities(), ["local-sync"]);
    assert!(
        sync.options("wasm32")
            .contains(&("IREE_RS_TASK".into(), "OFF".into()))
    );
}

#[test]
fn platform_incompatible_runtimes_fail_explicitly() {
    for (feature, supported) in [
        ("metal", "aarch64-apple-darwin"),
        ("amdgpu", "x86_64-unknown-linux-gnu"),
        ("webgpu", "wasm32-unknown-emscripten"),
    ] {
        let selection = config::Configuration::new([feature]);
        assert!(selection.validate_target("x86_64-pc-windows-msvc").is_err());
        assert!(selection.validate_target(supported).is_ok());
    }
}
