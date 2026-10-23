# iree-sys

Native build ownership and generated C bindings for IREE 3.13.0. The ergonomic
`iree` crate re-exports these APIs as `iree::sys`.

The build pipeline is:

1. Woodshed downloads verified pinned sources and caches the native CMake build.
2. CMake exports the shim's transitive include paths and preprocessor definitions.
3. Bindgen generates only the selected API families against that configuration
   and the Rust destination target. Generated static-inline forwarders are
   compiled with the same definitions and include paths.
4. Cargo links the native libraries and exports the deployment directory.

Generated files live only in `OUT_DIR`. No maintainer binding generation or
committed target snapshots are needed. Consumers need libclang at build time;
application users do not.

The raw APIs are organized into `base` and feature-selected `task`, `local`,
`local_sync`, `local_task`, `vulkan`, `cuda`, `hip`, `amdgpu`, `metal`, `null`,
`webgpu`, and `compiler` modules. Symbols are also re-exported at the root.
Compiler backend/input/LLVM architecture features configure native compilation,
not binding snapshots. Unavailable platform drivers fail explicitly.
