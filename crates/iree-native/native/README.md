# Native build maintenance

This directory configures and builds the pinned IREE source used by
`iree-native`. Application developers normally need the
[crate build guide](../README.md), not this file.

## Build inputs

- `woodshed.toml` and `woodshed.lock` declare and pin native sources.
- `CMakeLists.txt`, `configuration.cmake`, `targets.cmake`, and
  `installation.cmake` configure and install selected runtime/compiler
  capabilities.
- `wrapper.h` selects public API headers for bindgen. Generated forwarders make
  C inline functions linkable.
- `config.rs`, `manifest.rs`, `workspace.rs`, and `bindings.rs` resolve Cargo
  features, validate the native installation, and generate target-specific
  declarations.

The runtime/HAL libraries are statically linked. The compiler API is built as a
shared library and its deployment resources are staged with Cargo outputs.
There is no static compiler-linking mode. Hosted docs.rs builds use packaged
documentation-only declarations without provisioning or linking native code.

Use CMake 3.26+, Ninja, Python 3, host libclang, and the destination C/C++
toolchain and SDK. `IREE_NATIVE_DIR`, `IREE_NATIVE_CACHE_DIR`,
`IREE_NATIVE_BUILD_JOBS`, `WOODSHED_OFFLINE`, `LIBCLANG_PATH`, and
`IREE_NATIVE_TOOLCHAIN_FILE` control installation reuse, caching, parallelism,
source access, and cross-compilation.

## Documentation snapshots

`DOCS_RS=1` selects `../docs/bindings/<TARGET>/snapshot.json` and verifies it against the pinned source
declarations and binding-generator inputs, verifies the declaration checksums,
and copies the declarations into `OUT_DIR`. Missing targets and stale snapshots
are errors; another target's declarations are never substituted. This path
bypasses native feature validation deliberately: docs.rs enables all Cargo
features but each snapshot contains its target's applicable APIs. It is not
suitable for executable builds.

To regenerate after changing the pin, wrapper headers, or generator, use a
normal native build with `compiler` and `compiler-tools`, host libclang, and an
absolute `IREE_NATIVE_DOCS_SNAPSHOT_DIR` outside the crate source. For example,
in PowerShell from the repository root:

```powershell
$env:IREE_NATIVE_DOCS_SNAPSHOT_DIR = 'C:\temp\iree-native-docs'
$env:IREE_NATIVE_DOCS_TARGET = 'x86_64-pc-windows-msvc'
cargo check -p iree-native --features compiler,compiler-tools
Remove-Item Env:IREE_NATIVE_DOCS_SNAPSHOT_DIR
Remove-Item Env:IREE_NATIVE_DOCS_TARGET
```

The generator reuses the native compiler installation and provisions the locked
HIP and HSA headers to parse every public driver family. It parses declarations
only for the additional drivers; it does not build their native libraries.
Repeat for every target in the crate's docs.rs metadata. Cross-generation
requires the real destination toolchain headers: provide a JSON string array
of Clang SDK/sysroot arguments in `IREE_NATIVE_DOCS_CLANG_ARGS`. Generation
fails with an actionable diagnostic if those headers are missing; it does not
fabricate platform declarations. In particular, WebAssembly uses Emscripten's
sysroot, and Apple targets need Apple-compatible destination headers. The
native compiler installation supplies IREE/MLIR headers, while Clang parses
with the requested target triple and destination SDK.

Copy only `*.rs` and `snapshot.json` from each target output directory into
`../docs/bindings/<TARGET>`.
Do not copy generated C/header files or local API audit paths into the package.
The snapshot records its generation target. Its layouts and target-specific
declarations are illustrative only; real builds always regenerate bindings.

Check the hosted path with native source acquisition disabled:

```powershell
$env:DOCS_RS = '1'
$env:WOODSHED_OFFLINE = '1'
cargo +nightly rustdoc -p iree-native --all-features --target x86_64-pc-windows-msvc -- --cfg docsrs
Remove-Item Env:DOCS_RS
Remove-Item Env:WOODSHED_OFFLINE
```

Keep the snapshot in the crate archive and repeat this check against the
extracted package for every configured target before release (install each
target's nightly Rust standard library with `rustup target add --toolchain
nightly <TARGET>`). docs.rs performs hosted builds automatically after
publication; maintainers do not upload generated HTML. Regeneration is needed
when pinned native or binding-generator inputs change, not just when the
crate's Rust package version changes. Ordinary `cargo doc` without `DOCS_RS` still
uses the native build and the exact selected features.
