# iree-native

Bindings for IREE's public C APIs. Prefer the safe RAII wrappers in `ireec`, `ireert`, and
`ireetok` for application code.

## Build

The first build requires Rust 1.95+, CMake 3.26+, Ninja, Python 3, host libclang, and the target C/C++ toolchain. Compiler features also build LLVM/MLIR.

```toml
[dependencies]
iree-native = "0.1.0"
```

The default `cpu` feature enables the synchronous and task-based CPU runtimes.
Runtime libraries are statically linked; the optional compiler API is dynamically
linked.

Common settings:

| Variable                                    | Purpose                                                      |
| ------------------------------------------- | ------------------------------------------------------------ |
| `IREE_NATIVE_DIR`                           | Reuse a compatible native installation                       |
| `IREE_NATIVE_CACHE_DIR` or `WOODSHED_CACHE` | Select the source/build cache                                |
| `IREE_NATIVE_BUILD_JOBS`                    | Limit native build parallelism (positive integer)            |
| `WOODSHED_OFFLINE=1`                        | Disable native source acquisition; requires cached inputs    |
| `LIBCLANG_PATH`                             | Select host libclang                                         |
| `IREE_NATIVE_TOOLCHAIN_FILE`                | Select the destination CMake toolchain for cross-compilation |
| `IREE_NATIVE_HOST_BIN_DIR`                  | Supply host tools for a cross build                          |

`IREE_NATIVE_DIR` must point to an installation produced by this crate's native
build, including `iree-native.json`, matching native-input fingerprint, IREE
version, Rust target, and requested capabilities. A generic upstream IREE SDK
is not sufficient. The recorded source/header and build-metadata paths must
remain available; copying only the libraries is not a portable SDK. Even when
reusing an installation, bindings regenerate with host libclang and support C
sources may be compiled with the target toolchain.

## Features and targets

Select features for the destination rather than enabling `--all-features`;
some combinations are intentionally incompatible.

| Features                                            | Constraints                                                                                                                                                                                              |
| --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cpu` (default), `local-sync`, `local-task`, `task` | `local-task` requires native threading. For a nonthreaded runtime, disable defaults and select `local-sync`.                                                                                             |
| `bare-metal`                                        | Disable defaults and use a freestanding `*-none` Rust target with a Generic CMake toolchain, C library, and platform clock. Compiler, tokenizer, task, and GPU runtime features are incompatible.        |
| `compiler`, `target-*`, `llvm-*`, `input-*`         | Compiler backend selection is separate from runtime drivers. `compiler` enables StableHLO input; `input-stablehlo` alone does not enable the compiler. Other input and target features imply `compiler`. |
| `compiler-tools`                                    | Exposes tool entry points when combined with `compiler`; does not enable `compiler` or install CLI executables by itself.                                                                                |
| `cuda`, `target-cuda`                               | Native builds need `CUDAToolkit_ROOT` or `CUDA_PATH` (also accepted: `IREE_CUDA_DEPS_DIR`).                                                                                                              |
| `target-rocm`                                       | Native builds need `IREE_NATIVE_ROCM_BITCODE_DIR` containing `ocml.bc` and `ockl.bc`.                                                                                                                    |
| `metal`                                             | Apple runtime targets only; `target-metal` is a separate compiler backend.                                                                                                                               |
| `amdgpu`                                            | Linux runtime targets only.                                                                                                                                                                              |
| `webgpu`                                            | Requires a wasm32 JavaScript embedding. The Emscripten configuration disables threading; disable defaults and avoid `local-task`/`task`. `target-webgpu` is a separate compiler backend.                 |
| `vulkan`, `hip`, `mpi`                              | Need the corresponding platform libraries/devices at deployment; enabling bindings is not a portability guarantee.                                                                                       |
| `tokenizer`, `null`, `io-*`                         | Optional runtime APIs; `io-parameters` and the format features imply `io-index`.                                                                                                                         |

Target checks reject known-invalid combinations, but are not a promise that
every remaining Rust target or feature combination is supported. Validate the
exact destination toolchain and feature set before deployment.

## Documentation

docs.rs builds `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`,
`aarch64-apple-darwin`, and `wasm32-unknown-emscripten`, selecting the exact
snapshot for each target without a cross-target fallback. Use its target
selector to find platform-specific drivers such as AMDGPU, Metal, and WebGPU.
Snapshots document optional APIs for their target, not a buildable combination
of every Cargo feature. Bare-metal configuration requirements remain described
above; this hosted matrix does not include freestanding targets.
Normal builds always regenerate bindings for the actual target and features.
`DOCS_RS=1` is for documentation only: it does not build or link IREE libraries.

With the build prerequisites installed and `DOCS_RS` unset, generate API
documentation locally or in native CI:

```text
cargo doc -p iree-native --no-deps
cargo doc -p iree-native --no-deps --features compiler,target-llvm-cpu
```

Local documentation generation without `DOCS_RS` runs the normal native build
script and can download and compile native sources just like a normal build.
See [native build maintenance](native/README.md#documentation-snapshots) for
snapshot regeneration and hosted-build checks.

## Deploy compiler applications

The compiler API is dynamically linked. Distribute the matching staged
`iree-native` directory, which contains the compiler library and resources,
alongside the application. Windows builds stage dependent DLLs beside Cargo
profile executables. `IREE_COMPILER_RUNTIME_DIR` overrides the compiler
resource path.

## License

The packaged bindings and generated upstream API documentation use
`Apache-2.0 WITH LLVM-exception`; the Rust wrapper is Apache-2.0. See
[LICENSE](LICENSE), [LICENSE-LLVM-EXCEPTION](LICENSE-LLVM-EXCEPTION), and
[NOTICE](NOTICE). Redistributed native libraries and compiler resources also
require their respective third-party license notices.

## Regenerating release documentation

After bumping versions, run this from the repository root in PowerShell 7:

```powershell
.\crates\iree-native\regenerate-docs.ps1 -InstallToolchains -AllowDirty
```

The command regenerates all four target snapshots, runs their integrity tests,
verifies the crate archive with `cargo package`, and builds the extracted
package's documentation for every docs.rs target without native SDK/cache access.
`-InstallToolchains` allows downloading Zig's cross-platform libc headers,
Emscripten, and missing nightly Rust components; subsequent runs reuse them.
The ordinary native build prerequisites above must already be installed.
You can instead provide `-ZigLib` and `-EmsdkRoot`, or
`-ClangArgumentsFile` with target-to-argument-array JSON for custom SDKs.

The command stops on any error, restores environment variables, and never
publishes. Commit the regenerated snapshots, then publish explicitly:

```text
cargo publish -p iree-native
```

docs.rs builds the hosted HTML automatically after publication. Snapshot
regeneration is required when the IREE pin or binding-generator inputs change;
Rust-only package version bumps can reuse unchanged snapshots.
