# iree-native

Bindings for IREE's public C APIs. Prefer the safe RAII wrappers in `ireec`, `ireert`, and
`ireetok` for application code.

## Build

The first build requires Rust 1.95+, CMake 3.26+, Ninja, Python 3, host libclang, and the target C/C++ toolchain. Compiler features also build LLVM/MLIR.

Common settings:

| Variable                                    | Purpose                                |
| ------------------------------------------- | -------------------------------------- |
| `IREE_NATIVE_DIR`                           | Reuse a compatible native installation |
| `IREE_NATIVE_CACHE_DIR` or `WOODSHED_CACHE` | Select the source/build cache          |
| `IREE_NATIVE_BUILD_JOBS`                    | Limit native build parallelism         |
| `WOODSHED_OFFLINE=1`                        | Disable source downloads               |
| `LIBCLANG_PATH`                             | Select host libclang                   |

## Deploy compiler applications

The compiler API is dynamically linked. Distribute the matching staged
`iree-native` directory, which contains the compiler library and resources,
alongside the application. Windows builds stage dependent DLLs beside Cargo
profile executables. `IREE_COMPILER_RUNTIME_DIR` overrides the compiler
resource path.
