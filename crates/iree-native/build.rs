//! Provision pinned native sources with Woodshed and link CMake's installation.
//! Generate Rust declarations and inline forwarders against that configuration.
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use woodshed::project::{Dependency, PlanOptions, Project};
use woodshed::{Cache, CmakeBuild, MsvcRuntime, Toolchain};

#[path = "native/bindings.rs"]
mod bindings;
#[path = "native/config.rs"]
mod config;
#[path = "native/documentation.rs"]
mod documentation;
#[path = "native/manifest.rs"]
mod manifest;
#[path = "native/workspace.rs"]
mod workspace;

type BuildResult<T> = Result<T, Box<dyn std::error::Error>>;

trait ResultContext<T> {
    fn context(self, operation: impl std::fmt::Display) -> BuildResult<T>;
}

impl<T, E: std::fmt::Display> ResultContext<T> for Result<T, E> {
    fn context(self, operation: impl std::fmt::Display) -> BuildResult<T> {
        self.map_err(|error| format!("{operation}: {error}").into())
    }
}

fn main() -> BuildResult<()> {
    for path in ["build.rs", "native"] {
        println!("cargo:rerun-if-changed={path}");
    }
    for variable in [
        "DOCS_RS",
        "IREE_NATIVE_DOCS_SNAPSHOT_DIR",
        "IREE_NATIVE_DOCS_TARGET",
        "IREE_NATIVE_DOCS_CLANG_ARGS",
        "IREE_NATIVE_INSPECT_DIR",
        "IREE_NATIVE_DIR",
        "IREE_NATIVE_CACHE_DIR",
        "IREE_NATIVE_BUILD_JOBS",
        "CC",
        "CXX",
        "CMAKE_GENERATOR",
        "CMAKE_TOOLCHAIN_FILE",
        "IREE_NATIVE_TOOLCHAIN_FILE",
        "IREE_NATIVE_HOST_BIN_DIR",
        "CUDAToolkit_ROOT",
        "CUDA_PATH",
        "IREE_CUDA_DEPS_DIR",
        "IREE_NATIVE_ROCM_BITCODE_DIR",
        "WOODSHED_CACHE",
        "WOODSHED_OFFLINE",
        "WOODSHED_LOG",
        "WOODSHED_PROGRESS",
        "CFLAGS",
        "CXXFLAGS",
        "CPPFLAGS",
        "LDFLAGS",
        "AR",
        "SDKROOT",
        "MACOSX_DEPLOYMENT_TARGET",
        "CMAKE_GENERATOR_PLATFORM",
        "CMAKE_GENERATOR_TOOLSET",
    ] {
        println!("cargo:rerun-if-env-changed={variable}");
    }
    println!("cargo:rustc-check-cfg=cfg(docsrs)");
    println!("cargo:rustc-check-cfg=cfg(iree_native_docs)");
    let root = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").ok_or("Cargo CARGO_MANIFEST_DIR is missing")?,
    );
    let target = env::var("TARGET").context("read Cargo TARGET")?;
    let features = config::Configuration::new(config::FEATURES.iter().copied().filter(|feature| {
        env::var_os(format!(
            "CARGO_FEATURE_{}",
            feature.replace('-', "_").to_uppercase()
        ))
        .is_some()
    }));
    let out = PathBuf::from(env::var_os("OUT_DIR").ok_or("Cargo OUT_DIR is missing")?);
    let project = Project::load(root.join("native").join("woodshed.toml")).context(format!(
        "load native dependency declarations in {}",
        root.display()
    ))?;
    let locked = project
        .locked()
        .context("validate locked native dependencies")?;
    let pins = &locked.sources;
    let iree_version = pins
        .get("iree")
        .ok_or("missing IREE source pin")?
        .version
        .as_str();
    fs::write(
        out.join("version.rs"),
        format!("/// IREE release used for the native libraries and generated bindings.\npub const IREE_VERSION: &str = {iree_version:?};\n"),
    )
    .context(format!("write generated IREE version in {}", out.display()))?;
    if env::var("DOCS_RS").is_ok_and(|value| value == "1") {
        documentation::stage(&root, &out, iree_version, &target)?;
        println!("cargo:rustc-cfg=iree_native_docs");
        println!("cargo:rustc-env=IREE_COMPILER_BIN_DIR=unavailable in documentation-only builds");
        println!("cargo:rustc-env=IREE_COMPILER_DEPLOYMENT_ID=documentation-only");
        println!(
            "cargo:warning=IREE: using documentation-only bindings; no native libraries are built or linked"
        );
        return Ok(());
    }
    if target.contains("msvc")
        && let Some(tool) = cc::windows_registry::find_tool(&target, "cl.exe")
    {
        for (name, value) in tool.get_envs() {
            // C forwarders need the SDK environment even when native artifacts
            // are cached or supplied through IREE_NATIVE_DIR. No workers exist yet.
            // SAFETY: the single-threaded build script has no concurrent environment access.
            unsafe { env::set_var(name, value) };
        }
    }
    features
        .validate_target(&target)
        .context("IREE target configuration")?;
    if let Some(prefix) = env::var_os("IREE_NATIVE_DIR") {
        let prefix = PathBuf::from(prefix);
        println!(
            "cargo:rerun-if-changed={}",
            prefix.join("iree-native.json").display()
        );
        let installed = manifest::Installation::read(&prefix).context(format!(
            "IREE_NATIVE_DIR {} must contain a complete iree-native.json installation",
            prefix.display()
        ))?;
        let fingerprint = native_fingerprint(&root.join("native"))?;
        if installed.fingerprint != fingerprint {
            return Err(
                "IREE_NATIVE_DIR was built with different iree-native native inputs; rebuild it with this crate version"
            .into());
        }
        installed
            .validate(&target, &features)
            .context("IREE_NATIVE_DIR is incompatible with this build")?;
        report_plan(&format!(
            "SDK: use explicit IREE_NATIVE_DIR {}; metadata/required-file checks passed; skip provisioning and CMake. Bindings regenerate; support C compilation may run.",
            prefix.display()
        ))?;
        return generate_and_link(&out, &installed, &features, &target);
    }
    let cache = native_cache(&root)?;
    // Windows verbatim paths from canonicalize() make emcc derive a sysroot
    // that Clang cannot search. CMake can resolve symlinks from an absolute path.
    let toolchain_file = env::var_os("IREE_NATIVE_TOOLCHAIN_FILE")
        .map(|path| {
            std::path::absolute(&path).context(format!(
                "resolve toolchain path {}",
                Path::new(&path).display()
            ))
        })
        .transpose()?;
    if env::var_os("CMAKE_TOOLCHAIN_FILE").is_some() {
        return Err("IREE: use IREE_NATIVE_TOOLCHAIN_FILE instead of CMAKE_TOOLCHAIN_FILE so toolchain inputs are tracked".into());
    }
    let host = env::var("HOST").context("read Cargo HOST")?;
    if target != host && toolchain_file.is_none() {
        return Err(
            "IREE: cross compilation requires IREE_NATIVE_TOOLCHAIN_FILE and the destination SDK"
                .into(),
        );
    }
    if let Some(path) = &toolchain_file {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    let arch = env::var("CARGO_CFG_TARGET_ARCH").context("read Cargo target architecture")?;
    let family = workspace::family_key(
        pins.values(),
        &features,
        &target,
        &host,
        toolchain_file.as_deref(),
    )?;
    let family_dirs = workspace::directories(&cache, &family, target.contains("windows"));
    let _family_lock = family_dirs.lock().context(format!(
        "lock native workspace family {}",
        family_dirs.build.display()
    ))?;
    let state_path = family_dirs.build.join("workspace.json");
    let recorded = match workspace::Workspace::load(&state_path) {
        Ok(recorded) => recorded,
        Err(error) => {
            report_plan(&format!(
                "Workspace: cannot read receipt {}: {error}; stopping without rebuilding",
                state_path.display()
            ))?;
            return Err(format!("read native workspace {}: {error}", state_path.display()).into());
        }
    };
    let had_receipt = recorded.is_some();
    let mut state = recorded.unwrap_or_else(|| {
        workspace::Workspace::new(
            family_dirs.build.join("work"),
            family_dirs.build.join("native"),
        )
    });
    let native_features = state.selection(&features);
    let mut selection = PlanOptions::new(host, target.clone());
    selection.default_features = false;
    selection.features = native_features.source_features();
    let source_plan = project
        .resolve(&selection)
        .context("resolve native source feature graph")?;
    eprintln!(
        "IREE: incremental native workspace {} (capabilities: {})",
        state.build.display(),
        native_features.native_capabilities().join(", ")
    );
    native_features
        .validate_target(&target)
        .context("validate accumulated native capabilities")?;
    let cpu = native_features.has("cpu")
        || native_features.has("local-sync")
        || native_features.has("local-task");
    let native_options = native_features.options(&arch);
    let mut artifact = family
        .clone()
        .mix(native_features.native_capabilities().join(","));
    for node in &source_plan.dependencies {
        artifact = artifact.mix(&node.id);
    }
    for (name, value) in &native_options {
        artifact = artifact.mix(format!("{name}={value}"));
    }
    let mut sdk_options = Vec::new();
    if native_features.has("cuda") || native_features.has("target-cuda") {
        let sdk = cuda_sdk().ok_or("IREE: CUDA requires CUDAToolkit_ROOT or CUDA_PATH")?;
        sdk_options.push(("CUDAToolkit_ROOT", sdk));
    }
    if native_features.has("target-rocm") {
        let path = PathBuf::from(
            env::var_os("IREE_NATIVE_ROCM_BITCODE_DIR")
                .ok_or("IREE: target-rocm requires IREE_NATIVE_ROCM_BITCODE_DIR")?,
        );
        for file in ["ocml.bc", "ockl.bc"] {
            let file_path = path.join(file);
            println!("cargo:rerun-if-changed={}", file_path.display());
            artifact = artifact.mix(format!(
                "{file}={}",
                woodshed::sha256_file(&file_path)
                    .context(format!("hash ROCm device library {}", file_path.display()))?
            ));
        }
        sdk_options.push((
            "IREE_TARGET_BACKEND_ROCM_DEVICE_BC_PATH",
            path.canonicalize()
                .context(format!("resolve ROCm directory {}", path.display()))?
                .to_string_lossy()
                .into_owned(),
        ));
    }
    let native = root.join("native");
    let fingerprint = native_fingerprint(&native)?;
    // Native edits create a new SDK, not a new compilation workspace.
    artifact = artifact.mix(&fingerprint);
    let prefix = cache
        .root()
        .join("i")
        .join(
            family
                .as_str()
                .rsplit('-')
                .next()
                .ok_or("native family key has no hash")?,
        )
        .join(
            artifact
                .as_str()
                .rsplit('-')
                .next()
                .ok_or("native artifact key has no hash")?,
        );
    let dirs = woodshed::BuildDirs {
        build: state.build.clone(),
        prefix,
    };
    let _lock = dirs
        .lock()
        .context(format!("lock native build {}", dirs.build.display()))?;
    let sdk = workspace::installation_status(&dirs.prefix, &fingerprint, &target, &features);
    let complete = sdk.is_ok();
    report_plan(&format!(
        "Native plan for {target}; SDK {}",
        dirs.prefix.display()
    ))?;
    for line in workspace::build_plan(&state, had_receipt, &features, &native_features, sdk) {
        report_plan(&line)?;
    }
    if !complete {
        let mut sources = Vec::new();
        for node in &source_plan.dependencies {
            if !matches!(&node.declaration, Dependency::Source(source) if source.path.is_none()) {
                return Err(
                    "IREE: native source mounts require locked remote source declarations".into(),
                );
            }
            let dep = locked
                .sources
                .get(&node.id)
                .ok_or_else(|| format!("missing selected source pin {}", node.id))?;
            woodshed::progress::report(&format!(
                "IREE source {}: acquire/verify locked tree",
                node.name
            ))
            .context("report native source progress")?;
            let source = cache
                .source(dep)
                .context(format!("IREE: provision {}", dep.name))?;
            sources.push((node.name.clone(), source));
        }
        let root_source = &sources
            .iter()
            .find(|(name, _)| name == "iree")
            .ok_or("selected sources do not contain IREE")?
            .1;
        let mut mounts = Vec::new();
        for (name, path) in &sources {
            if let Some(woodshed::Resolution::Gitlink { path: mount, .. }) = &pins
                .get(name)
                .ok_or_else(|| format!("missing pinned source {name}"))?
                .resolve
            {
                mounts.push((PathBuf::from(mount), path.clone()));
                continue;
            }
            let location = match name.as_str() {
                "iree" => continue,
                "abseil" => "abseil-cpp",
                "spirv_headers" => "spirv-headers",
                "spirv_tools" => "spirv-tools",
                name => name,
            };
            mounts.push((PathBuf::from("third_party").join(location), path.clone()));
        }
        let prepared = match &state.source {
            Some(prepared) => cache.prepare_incremental_to(root_source, prepared, &mounts),
            None => cache.prepare_incremental(&family, root_source, &mounts),
        }
        .context("prepare stable native sources and additive dependencies")?;
        state.source = Some(prepared.clone());
        state.save(&state_path).context(format!(
            "record native workspace paths in {}",
            state_path.display()
        ))?;
        let jobs = match env::var("IREE_NATIVE_BUILD_JOBS") {
            Ok(value) => value
                .parse::<usize>()
                .context("IREE_NATIVE_BUILD_JOBS must be a positive integer")?,
            Err(env::VarError::NotPresent) => std::thread::available_parallelism()
                .map(|count| count.get())
                .unwrap_or(4),
            Err(error) => return Err(format!("read IREE_NATIVE_BUILD_JOBS: {error}").into()),
        };
        if jobs == 0 {
            return Err("IREE_NATIVE_BUILD_JOBS must be positive".into());
        }
        cache.mirror_to(&native, &state.mirror).context(format!(
            "refresh native inputs {} to {}",
            native.display(),
            state.mirror.display()
        ))?;
        let source = &state.mirror;
        let mut build = CmakeBuild::new(source, &dirs)
            .target(Some("iree_native_install"))
            .msvc_runtime(if native_features.uses_compiler() {
                MsvcRuntime::AlwaysDynamic
            } else {
                MsvcRuntime::MatchRust
            })
            .toolchain(Toolchain::detect().jobs(jobs))
            .define("IREE_NATIVE_SOURCE", prepared.to_string_lossy())
            .define("IREE_NATIVE_FINGERPRINT", &fingerprint)
            .define("IREE_NATIVE_RUST_TARGET", &target)
            .define(
                "IREE_NATIVE_CAPABILITIES",
                native_features.native_capabilities().join(";"),
            )
            .define("IREE_NATIVE_CPU", if cpu { "ON" } else { "OFF" })
            .define(
                "IREE_NATIVE_COMPILER",
                if native_features.uses_compiler() {
                    "ON"
                } else {
                    "OFF"
                },
            )
            .define(
                "IREE_NATIVE_VULKAN",
                if native_features.has("vulkan") {
                    "ON"
                } else {
                    "OFF"
                },
            );
        let iree = pins.get("iree").ok_or("missing IREE source pin")?;
        let revision = iree
            .git
            .as_ref()
            .map(|source| source.revision.as_str())
            .or_else(|| {
                iree.upstream
                    .as_ref()
                    .map(|source| source.revision.as_str())
            })
            .ok_or("IREE pin lacks its immutable upstream revision")?;
        build = build
            .define("IREE_RELEASE_VERSION", &iree.version)
            .define("IREE_RELEASE_REVISION", revision);
        for (name, value) in native_options {
            build = build.define(name, value);
        }
        for (name, value) in sdk_options {
            build = build.define(name, value);
        }
        if let Some(path) = toolchain_file {
            build = build.define("CMAKE_TOOLCHAIN_FILE", path.to_string_lossy());
        }
        if let Some(path) = env::var_os("IREE_NATIVE_HOST_BIN_DIR") {
            // As with the toolchain file above, canonicalize() yields a Windows
            // \\?\ verbatim path that CMake/emcc cannot search; absolute() keeps
            // the path absolute without the verbatim prefix.
            build = build.define(
                "IREE_HOST_BIN_DIR",
                std::path::absolute(PathBuf::from(&path))
                    .context(format!("resolve host tools {}", Path::new(&path).display()))?
                    .to_string_lossy(),
            );
        }
        let configured = build.configure().context("IREE native configure failed")?;
        #[cfg(windows)]
        expose_python3(configured.build_directory())?;
        configured.build().context("IREE native install failed")?;
        let installed = manifest::Installation::read(&dirs.prefix).context(format!(
            "read newly built native installation {}",
            dirs.prefix.display()
        ))?;
        workspace::publish(&installed, &dirs.prefix, &state.mirror).context(format!(
            "publish immutable native installation {}",
            dirs.prefix.display()
        ))?;
    }
    state.capabilities = native_features
        .native_capabilities()
        .into_iter()
        .map(String::from)
        .collect();
    state.save(&state_path).context(format!(
        "record completed native workspace {}",
        state_path.display()
    ))?;

    let installed = manifest::Installation::read(&dirs.prefix).context(format!(
        "read completed native manifest {}",
        dirs.prefix.display()
    ))?;
    installed
        .validate(&target, &features)
        .context("native installation is incomplete")?;
    generate_and_link(&out, &installed, &features, &target)
}

fn generate_and_link(
    out: &Path,
    installed: &manifest::Installation,
    features: &config::Configuration,
    target: &str,
) -> BuildResult<()> {
    woodshed::progress::report(
        "IREE bindings: generate declarations and compile required support sources",
    )
    .context("report binding progress")?;
    bindings::generate(out, installed, features, target)
        .context("generate IREE bindings (install libclang or set LIBCLANG_PATH)")?;
    if let Some(destination) = env::var_os("IREE_NATIVE_DOCS_SNAPSHOT_DIR") {
        let destination = PathBuf::from(destination);
        if !destination.is_absolute() {
            return Err("IREE_NATIVE_DOCS_SNAPSHOT_DIR must be an absolute path".into());
        }
        if !features.uses_compiler() {
            return Err(
                "documentation snapshots require the compiler feature and its native installation"
                    .into(),
            );
        }
        let root = PathBuf::from(
            env::var_os("CARGO_MANIFEST_DIR").ok_or("Cargo CARGO_MANIFEST_DIR is missing")?,
        );
        let documentation_target = match env::var("IREE_NATIVE_DOCS_TARGET") {
            Ok(value) => value,
            Err(env::VarError::NotPresent) => target.into(),
            Err(error) => return Err(format!("read IREE_NATIVE_DOCS_TARGET: {error}").into()),
        };
        documentation::features(&documentation_target)?;
        let project = Project::load(root.join("native").join("woodshed.toml"))?;
        let locked = project.locked()?;
        let cache = native_cache(&root)?;
        let headers = destination.join("header-inputs");
        let mut includes = vec![format!("-I{}", headers.display())];
        let sdk_arguments = match env::var("IREE_NATIVE_DOCS_CLANG_ARGS") {
            Ok(arguments) => serde_json::from_str::<Vec<String>>(&arguments).context(
                "IREE_NATIVE_DOCS_CLANG_ARGS must be a JSON string array of target SDK arguments",
            )?,
            Err(env::VarError::NotPresent) => Vec::new(),
            Err(error) => return Err(format!("read IREE_NATIVE_DOCS_CLANG_ARGS: {error}").into()),
        };
        if documentation_target != target && sdk_arguments.is_empty() {
            return Err(format!("cross-target documentation for {documentation_target} requires real destination SDK arguments in IREE_NATIVE_DOCS_CLANG_ARGS").into());
        }
        includes.extend(sdk_arguments);
        for name in ["hip_build_deps", "hsa_runtime_headers"] {
            let source = cache.source(
                locked
                    .sources
                    .get(name)
                    .ok_or("missing documentation header pin")?,
            )?;
            cache.mirror_to(
                &source,
                &headers.join("third_party").join(name.replace('_', "-")),
            )?;
        }
        fs::create_dir_all(&destination).context("create documentation snapshot directory")?;
        let destination = destination.join(&documentation_target);
        fs::create_dir_all(&destination)
            .context("create target documentation snapshot directory")?;
        bindings::generate_documentation(&destination, installed, &documentation_target, &includes)
            .context(format!("generate documentation for {documentation_target}; provide its real SDK/sysroot via IREE_NATIVE_DOCS_CLANG_ARGS"))?;
        documentation::record(
            &root,
            &destination,
            &installed.version,
            &documentation_target,
        )?;
    }
    copy_bindings_for_inspection(out, target, features)?;
    installed.link(out, features)?;
    Ok(())
}

fn report_plan(message: &str) -> BuildResult<()> {
    println!("cargo:warning=IREE: {}", message.replace(['\r', '\n'], " "));
    woodshed::progress::report(&format!("IREE: {message}")).context("report native build plan")
}

fn native_cache(root: &Path) -> BuildResult<Cache> {
    Ok(match env::var_os("IREE_NATIVE_CACHE_DIR") {
        Some(path) => {
            let path = PathBuf::from(path);
            Cache::at(if path.is_absolute() {
                path
            } else {
                root.join(path)
            })
            .offline(env::var("WOODSHED_OFFLINE").is_ok_and(|value| value == "1"))
        }
        None => Cache::discover().context("discover Woodshed cache")?,
    })
}

fn cuda_sdk() -> Option<String> {
    env::var("IREE_CUDA_DEPS_DIR")
        .or_else(|_| env::var("CUDAToolkit_ROOT"))
        .or_else(|_| env::var("CUDA_PATH"))
        .ok()
}

fn native_fingerprint(native: &Path) -> BuildResult<String> {
    fn collect(root: &Path, directory: &Path, files: &mut Vec<PathBuf>) -> BuildResult<()> {
        let entries = fs::read_dir(directory).context(format!(
            "read native input directory {}",
            directory.display()
        ))?;
        for entry in entries {
            let entry = entry.context(format!(
                "read native input entry in {}",
                directory.display()
            ))?;
            let path = entry.path();
            let kind = entry
                .file_type()
                .context(format!("inspect native input {}", path.display()))?;
            if kind.is_dir() {
                collect(root, &path, files)?;
            } else if kind.is_file()
                && (path
                    .file_name()
                    .is_some_and(|name| name == "CMakeLists.txt")
                    || matches!(
                        path.extension().and_then(|extension| extension.to_str()),
                        Some(
                            "cmake"
                                | "c"
                                | "h"
                                | "cc"
                                | "cpp"
                                | "cxx"
                                | "m"
                                | "mm"
                                | "inc"
                                | "def"
                                | "s"
                                | "S"
                        )
                    ))
            {
                files.push(
                    path.strip_prefix(root)
                        .context(format!(
                            "resolve native input {} under {}",
                            path.display(),
                            root.display()
                        ))?
                        .into(),
                );
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    collect(native, native, &mut files)?;
    files.sort();
    let mut hash = Sha256::new();
    for relative in files {
        hash.update(relative.to_string_lossy().replace('\\', "/").as_bytes());
        hash.update([0]);
        hash.update(fs::read(native.join(&relative)).context(format!(
            "read native input {}",
            native.join(&relative).display()
        ))?);
        hash.update([0]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

#[cfg(windows)]
fn expose_python3(build: &Path) -> BuildResult<()> {
    let executable =
        fs::read_to_string(build.join("iree-python-executable.txt")).context(format!(
            "read CMake-selected Python executable from {}",
            build.join("iree-python-executable.txt").display()
        ))?;
    let executable = executable.trim();
    if !Path::new(executable).is_file() {
        return Err(format!("CMake selected a missing Python executable: {executable}").into());
    }
    let tools = build.join("host-tools");
    fs::create_dir_all(&tools)
        .context(format!("create host tool directory {}", tools.display()))?;
    let command = tools.join("python3.cmd");
    fs::write(&command, format!("@\"{executable}\" %*\r\n"))
        .context(format!("write Python command shim {}", command.display()))?;
    let path = env::var_os("PATH").unwrap_or_default();
    let mut paths = vec![tools];
    paths.extend(env::split_paths(&path));
    let path = env::join_paths(paths).context("compose native build PATH")?;
    // SAFETY: this build-script helper runs before any concurrent environment access.
    unsafe { env::set_var("PATH", path) };
    Ok(())
}

/// Opt-in maintainer copy of generated declarations. Ordinary builds write only to
/// OUT_DIR: Cargo forbids build scripts from modifying package or registry sources.
fn copy_bindings_for_inspection(
    out: &Path,
    target: &str,
    features: &config::Configuration,
) -> BuildResult<()> {
    let Some(base) = env::var_os("IREE_NATIVE_INSPECT_DIR").map(PathBuf::from) else {
        return Ok(());
    };
    if !base.is_absolute() {
        return Err(format!(
            "IREE: IREE_NATIVE_INSPECT_DIR must be an absolute path: {}",
            base.display()
        )
        .into());
    }
    let capabilities = features.native_capabilities();
    let configuration = if capabilities.is_empty() {
        "base".to_owned()
    } else {
        capabilities.join("+")
    };
    let destination = base.join(target).join(configuration);
    let result = (|| -> std::io::Result<()> {
        fs::create_dir_all(&destination)?;
        for entry in fs::read_dir(out)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|extension| extension == "rs")
                || path
                    .file_name()
                    .is_some_and(|name| name == "api-audit.json")
            {
                fs::copy(&path, destination.join(entry.file_name()))?;
            }
        }
        Ok(())
    })();
    result.context(format!(
        "IREE: copy bindings from {} to {}",
        out.display(),
        destination.display()
    ))
}
