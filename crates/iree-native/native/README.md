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
There is no static compiler-linking mode or hosted docs.rs build path.

Use CMake 3.26+, Ninja, Python 3, host libclang, and the destination C/C++
toolchain and SDK. `IREE_NATIVE_DIR`, `IREE_NATIVE_CACHE_DIR`,
`IREE_NATIVE_BUILD_JOBS`, `WOODSHED_OFFLINE`, `LIBCLANG_PATH`, and
`IREE_NATIVE_TOOLCHAIN_FILE` control installation reuse, caching, parallelism,
source access, and cross-compilation.
