//! Provision pinned native sources with Woodshed and link CMake's installation.
//! Generate Rust declarations and inline forwarders against that configuration.
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};
use woodshed::{BuildKey, Cache, CmakeBuild, Manifest, MsvcRuntime, Toolchain};

#[path = "native/bindings.rs"]
mod bindings;
#[path = "native/config.rs"]
mod config;
#[path = "native/manifest.rs"]
mod manifest;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(iree_native_threading)");
    for path in ["build.rs", "native"] {
        println!("cargo:rerun-if-changed={path}");
    }
    for variable in [
        "DOCS_RS",
        "IREE_DIR",
        "IREE_RS_CACHE_DIR",
        "IREE_RS_BUILD_JOBS",
        "CC",
        "CXX",
        "CMAKE_GENERATOR",
        "IREE_RS_TOOLCHAIN_FILE",
        "IREE_RS_HOST_BIN_DIR",
        "CUDAToolkit_ROOT",
        "CUDA_PATH",
        "IREE_CUDA_DEPS_DIR",
        "IREE_RS_ROCM_BITCODE_DIR",
        "WOODSHED_CACHE",
        "WOODSHED_OFFLINE",
    ] {
        println!("cargo:rerun-if-env-changed={variable}");
    }
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let features = config::Configuration::new(config::FEATURES.iter().copied().filter(|feature| {
        env::var_os(format!(
            "CARGO_FEATURE_{}",
            feature.replace('-', "_").to_uppercase()
        ))
        .is_some()
    }));
    features
        .validate_target(&target)
        .unwrap_or_else(|error| panic!("IREE: {error}"));
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    if let Some(prefix) = env::var_os("IREE_DIR") {
        let installed = manifest::Installation::read(&PathBuf::from(prefix))
            .expect("IREE_DIR must contain a complete iree-native.json installation");
        installed
            .validate(&target, &features)
            .expect("IREE_DIR is incompatible with this build");
        bindings::generate(&out, &installed.build, &features, &target)
            .expect("generate IREE bindings (install libclang or set LIBCLANG_PATH)");
        installed.link(&out);
        return;
    }
    let pins = Manifest::load_default(&root).expect("load pinned native versions");
    let cache = env::var_os("IREE_RS_CACHE_DIR")
        .map(|path| {
            let path = PathBuf::from(path);
            Cache::at(if path.is_absolute() {
                path
            } else {
                root.join(path)
            })
            .offline(env::var("WOODSHED_OFFLINE").is_ok_and(|value| value == "1"))
        })
        .unwrap_or_else(|| Cache::discover().expect("discover Woodshed cache"));
    // Retain compatible native object caches across the build-script rewrite.
    let mut key = BuildKey::new("iree-rs-v1").mix(&target);
    let mut sources = Vec::new();
    for dep in pins.iter().filter(|dep| features.needs_source(&dep.name)) {
        key = key.dep(dep);
        sources.push((
            dep.name.clone(),
            cache
                .source(dep)
                .unwrap_or_else(|error| panic!("IREE: provision {}: {error}", dep.name)),
        ));
    }
    let cpu = features.has("cpu") || features.has("local-sync") || features.has("local-task");
    key = key.mix(format!(
        "cpu={cpu};vulkan={};compiler={}",
        features.has("vulkan"),
        features.has("compiler")
    ));
    let extra = config::FEATURES
        .iter()
        .copied()
        .filter(|name| !["cpu", "vulkan", "compiler", "input-stablehlo"].contains(name))
        .filter(|name| !features.has("cpu") || !["local-sync", "local-task", "task"].contains(name))
        .filter(|name| features.has(name))
        .collect::<Vec<_>>();
    if !extra.is_empty() {
        key = key.mix(extra.join(","));
    }
    if woodshed::crt_static() {
        key = key.mix("crt-static");
    }
    let toolchain_file = env::var_os("IREE_RS_TOOLCHAIN_FILE").map(PathBuf::from);
    assert!(
        target == env::var("HOST").unwrap() || toolchain_file.is_some(),
        "IREE: cross compilation requires IREE_RS_TOOLCHAIN_FILE and the destination SDK"
    );
    if let Some(path) = &toolchain_file {
        println!("cargo:rerun-if-changed={}", path.display());
        key = key.mix(format!(
            "toolchain={}",
            woodshed::sha256_file(path).expect("hash toolchain file")
        ));
    }
    for name in ["CC", "CXX", "IREE_RS_HOST_BIN_DIR"] {
        if let Ok(value) = env::var(name) {
            key = key.mix(format!("{name}={value}"));
        }
    }
    let mut sdk_options = Vec::new();
    if features.has("cuda") || features.has("target-cuda") {
        let sdk = env::var("IREE_CUDA_DEPS_DIR")
            .or_else(|_| env::var("CUDAToolkit_ROOT"))
            .or_else(|_| env::var("CUDA_PATH"))
            .expect("IREE: CUDA requires CUDAToolkit_ROOT or CUDA_PATH");
        key = key.mix(format!("cuda-sdk={sdk}"));
        sdk_options.push(("CUDAToolkit_ROOT", sdk));
    }
    if features.has("target-rocm") {
        let path = PathBuf::from(
            env::var_os("IREE_RS_ROCM_BITCODE_DIR")
                .expect("IREE: target-rocm requires IREE_RS_ROCM_BITCODE_DIR"),
        );
        key = key.mix(format!("rocm-bitcode={}", path.display()));
        for file in ["ocml.bc", "ockl.bc"] {
            let file_path = path.join(file);
            println!("cargo:rerun-if-changed={}", file_path.display());
            key = key.mix(format!(
                "{file}={}",
                woodshed::sha256_file(&file_path).expect("hash ROCm device library")
            ));
        }
        sdk_options.push((
            "IREE_TARGET_BACKEND_ROCM_DEVICE_BC_PATH",
            path.canonicalize()
                .expect("resolve ROCm directory")
                .to_string_lossy()
                .into_owned(),
        ));
    }
    let mut dirs = cache.build_dir(&key);
    if target.contains("windows") && features.has("compiler") {
        dirs.build = cache
            .root()
            .join("b")
            .join(key.as_str().rsplit('-').next().unwrap());
        dirs.prefix = dirs.build.join("prefix");
    }
    let _lock = dirs.lock().expect("lock native build");
    let hash = [
        "CMakeLists.txt",
        "config.rs",
        "shim.c",
        "wrapper.h",
        "windows-genrule.cmake",
        "versions.toml",
    ]
    .into_iter()
    .fold(Sha256::new(), |mut hash, name| {
        hash.update(name);
        hash.update(fs::read(root.join("native").join(name)).expect("read native build input"));
        hash
    });
    let fingerprint = hash.finalize();
    let fingerprint = format!("{fingerprint:x}");
    let complete = manifest::Installation::read(&dirs.prefix).is_ok_and(|installed| {
        installed.fingerprint == fingerprint && installed.validate(&target, &features).is_ok()
    });
    if !complete {
        let root_source = &sources
            .iter()
            .find(|(name, _)| name == "iree")
            .expect("IREE source")
            .1;
        let mounts = sources
            .iter()
            .filter_map(|(name, path)| {
                let location = match name.as_str() {
                    "iree" => return None,
                    "llvm" => "llvm-project",
                    "torch_mlir" => "torch-mlir",
                    "hip_build_deps" => "hip-build-deps",
                    "hsa_runtime_headers" => "hsa-runtime-headers",
                    "abseil" => "abseil-cpp",
                    "spirv_headers" => "spirv-headers",
                    "spirv_tools" => "spirv-tools",
                    name => name,
                };
                Some((PathBuf::from("third_party").join(location), path.clone()))
            })
            .collect::<Vec<_>>();
        let prepared = cache
            .prepare(&key, root_source, &mounts)
            .expect("assemble native dependencies");
        if target.contains("msvc")
            && let Some(tool) = cc::windows_registry::find_tool(&target, "cl.exe")
        {
            for (name, value) in tool.get_envs() {
                // This build script has not created worker threads.
                unsafe { env::set_var(name, value) };
            }
        }
        let jobs = env::var("IREE_RS_BUILD_JOBS")
            .map(|value| {
                value
                    .parse::<usize>()
                    .expect("IREE_RS_BUILD_JOBS must be a positive integer")
            })
            .unwrap_or(4);
        assert!(jobs > 0, "IREE_RS_BUILD_JOBS must be positive");
        let source = cache
            .mirror(&root.join("native"), &key)
            .expect("mirror native superbuild");
        let mut build = CmakeBuild::new(&source, &dirs)
            .target(Some("iree_rs_install"))
            .msvc_runtime(if features.has("compiler") {
                MsvcRuntime::AlwaysDynamic
            } else {
                MsvcRuntime::MatchRust
            })
            .toolchain(Toolchain::detect().jobs(jobs))
            .define("IREE_RS_SOURCE", prepared.to_string_lossy())
            .define("IREE_RS_FINGERPRINT", &fingerprint)
            .define("IREE_RS_RUST_TARGET", &target)
            .define(
                "IREE_RS_CAPABILITIES",
                features.native_capabilities().join(";"),
            )
            .define("IREE_RS_CPU", if cpu { "ON" } else { "OFF" })
            .define(
                "IREE_RS_COMPILER",
                if features.has("compiler") {
                    "ON"
                } else {
                    "OFF"
                },
            )
            .define(
                "IREE_RS_VULKAN",
                if features.has("vulkan") { "ON" } else { "OFF" },
            );
        let iree = pins.get("iree").expect("IREE source pin");
        build = build.define("IREE_RELEASE_VERSION", &iree.version).define(
            "IREE_RELEASE_REVISION",
            iree.url
                .rsplit('/')
                .next()
                .unwrap()
                .trim_end_matches(".tar.gz"),
        );
        for (name, value) in features.options(&env::var("CARGO_CFG_TARGET_ARCH").unwrap()) {
            build = build.define(name, value);
        }
        for (name, value) in sdk_options {
            build = build.define(name, value);
        }
        if let Some(path) = toolchain_file {
            build = build.define(
                "CMAKE_TOOLCHAIN_FILE",
                path.canonicalize()
                    .expect("resolve toolchain")
                    .to_string_lossy(),
            );
        }
        if let Some(path) = env::var_os("IREE_RS_HOST_BIN_DIR") {
            build = build.define(
                "IREE_HOST_BIN_DIR",
                PathBuf::from(path)
                    .canonicalize()
                    .expect("resolve host tools")
                    .to_string_lossy(),
            );
        }
        build.install().expect("IREE native superbuild failed");
    }
    let installed =
        manifest::Installation::read(&dirs.prefix).expect("read completed native manifest");
    installed
        .validate(&target, &features)
        .expect("native installation is incomplete");
    bindings::generate(&out, &installed.build, &features, &target)
        .expect("generate IREE bindings (install libclang or set LIBCLANG_PATH)");
    installed.link(&out);
}
