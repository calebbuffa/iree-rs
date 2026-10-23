# Native build and deployment

Woodshed verifies and provisions `versions.toml` sources, prepares the upstream
submodule layout, and caches the CMake build. `config.rs` maps Cargo features to
IREE options. `CMakeLists.txt` builds the runtime closure, shim, and optional
shared compiler, and exports link and binding configuration.

`bindings.rs` runs bindgen during ordinary Cargo builds against the selected
target and CMake's transitive definitions/include paths. Generated declarations,
feature-family modules, link tests, and inline forwarders live in `OUT_DIR`.
There is no maintainer generation command and no checked-in platform snapshot.

## Prerequisites

Rust, a C/C++ toolchain and destination SDK, CMake 3.26+, Ninja, Python 3, and
libclang. Python is required by upstream IREE native generators, not by bindgen.
Windows requires Visual Studio C++ tools and the Windows SDK. Apple targets
require the appropriate Xcode SDK. Tools are build dependencies, not deployment
dependencies.

Runtime-only builds do not compile LLVM/MLIR. First compiler builds are large
and use four native jobs by default.

| Environment variable | Purpose |
| --- | --- |
| `LIBCLANG_PATH` | Directory containing host libclang |
| `BINDGEN_EXTRA_CLANG_ARGS` | Additional destination SDK/sysroot parsing arguments |
| `WOODSHED_CACHE` | General native cache root |
| `IREE_RS_CACHE_DIR` | IREE-specific native cache override |
| `WOODSHED_OFFLINE=1` | Require selected archives to be cached |
| `IREE_RS_BUILD_JOBS` | Positive native build parallelism |
| `IREE_DIR` | Compatible complete native installation, including its binding configuration |
| `CC`, `CXX` | Native C/C++ compiler selection |
| `CMAKE_GENERATOR` | Native build generator |
| `IREE_RS_TOOLCHAIN_FILE` | Destination CMake toolchain; required when host differs from target |
| `IREE_RS_HOST_BIN_DIR` | Host IREE code-generation tools for cross builds |
| `CUDAToolkit_ROOT`, `CUDA_PATH`, `IREE_CUDA_DEPS_DIR` | Explicit CUDA SDK location |
| `IREE_RS_ROCM_BITCODE_DIR` | ROCm directory containing `ocml.bc` and `ockl.bc` |

Cross-compilation needs a matching Rust target, native compiler/linker, SDK, and
host-executable IREE generators. CMake's sysroot is passed to binding generation.
Additional toolchain-specific parsing flags can be supplied through bindgen's
environment variables. Arbitrary compiler flags that alter ABI must be applied
consistently to both native compilation and Clang parsing. Ordinary native
builds do not need cross-toolchain configuration.

Selecting `llvm-*` compiler backends does not cross-compile the runtime.
The upstream WebGPU runtime is a wasm32 JavaScript embedding and still requires
browser glue. It is distinct from native WGPU/Vulkan interoperability.

## Deployment contract

The runtime is statically linked. Compiler support uses upstream's shared
`IREECompiler` library with selected resources and, for backends that build it,
`iree-lld`. A GPU-only compiler does not require the CPU linker helper. The native install
produces a dedicated `deploy` directory containing runtime files only, including
MSVC redistributables when dynamic CRT linking is selected.

`iree-sys` exposes it as `DEP_IREE_RUNTIME_DEPLOYMENT_DIR` to immediate dependent
build scripts and emits `WOODSHED_DEPLOYMENT_DIR` for Woodshed's Cargo bundler.
The bundler reads Cargo build messages, so the application need not directly
depend on `iree-sys` or forward transitive build metadata.

From the workspace:

```powershell
cargo xtask package dist -- -p iree --example compile --features compiler --release
```

For another application:

```powershell
cargo xtask package dist -- --manifest-path C:\path\app\Cargo.toml --bin app --release
```

The generic alternative is
`woodshed bundle --output dist -- --manifest-path <manifest> --bin app --release`.
Install the published CLI with
`cargo install woodshed --version 0.2.3 --features cli`.

Packaging uses `cargo rustc` to add a final executable-relative loader path on
Linux/macOS, then copies the executable and declared runtime trees. Select
exactly one executable. The output directory must not already exist. Identical
resource files are shared; conflicting contents fail rather than silently
overwrite. Files are staged in a temporary sibling directory and renamed into
place only after successful assembly.

Target selection respects `--target`, `CARGO_BUILD_TARGET`, and Cargo config
from the invocation directory. Multi-target builds and CLI `--config` overrides
are rejected explicitly. Cross-built bundles must be run on their destination
platform.

On Windows, Rust `-C target-feature=+crt-static` selects matching static CRT for
runtime-only native builds. The optional shared compiler uses dynamic CRT and
ships the redistributable files. On Linux, native C++ runtime support is linked
statically; the application still requires a compatible system libc baseline.
Apple's system libraries, GPU drivers, and backend-specific vendor runtimes
(such as HIP) remain platform prerequisites. CUDA selections also require the
CUDA SDK on the build machine; it is not bundled as an end-user dependency.
Redistribution must include applicable licences/notices.

An ahead-of-time example:

```powershell
cargo run -p iree --features compiler --example compile -- crates\iree\examples\models\add.mlir add.vmfb
cargo run -p iree --example runtime -- add.vmfb
```

Only trusted VMFB should be loaded: it may contain native machine code.
No end-user build tools or native cache are required by a correctly packaged
application.

## Qualification

```powershell
cargo test -p iree-sys --no-default-features
cargo test --workspace --features iree/compiler,iree/vulkan,iree-gfx/compiler
cargo test -p iree-gfx --features compiler --test wgpu_interop -- --include-ignored
```

Deployment must also be exercised after relocation outside Cargo's build tree,
with cache/loader environment variables removed. Cargo-run success alone is
not a deployment check. Platform CI is provided, but only completed runs establish
qualification.

`cargo doc` has the same native prerequisites as a normal build. There is no
`DOCS_RS` shortcut using another platform's declarations; network-isolated
documentation builders require a preprovisioned native cache.
