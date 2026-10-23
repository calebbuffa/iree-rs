# IREE for Rust

[IREE](https://iree.dev).

| Crate      | Responsibility                                                                                        |
| ---------- | ----------------------------------------------------------------------------------------------------- |
| `iree-sys` | Woodshed native provisioning, target-local bindgen, inline C forwarders, linking, deployment metadata |
| `iree`     | Owned Rust compiler/runtime interfaces; raw APIs through `iree::sys`                                  |
| `iree-gfx` | Native Vulkan/WGPU sharing and synchronization                                                        |

## Build

Install Rust, a C/C++ toolchain and platform SDK, CMake, Ninja, Python 3 (for
upstream native source generation), and libclang. Windows requires Visual Studio
C++ tools and the Windows SDK. Set `LIBCLANG_PATH` if libclang is not discoverable.
Python does not generate the Rust bindings.

`iree-sys` provisions pinned, verified sources through Woodshed, builds the
selected native capabilities, then generates bindings against that target's
headers and native configuration. All generated Rust and C files stay in
Cargo's build directory. There are no committed target snapshots, maintainer
binding updates, or feature-combination bundles.

```powershell
cargo test -p iree-sys -p iree
cargo test --workspace --features iree/compiler,iree/vulkan,iree-gfx/compiler
cargo test -p iree-gfx --features compiler --test wgpu_interop -- --include-ignored
```

GPU tests require compatible hardware. Do not use `--all-features`: some
drivers require mutually incompatible target platforms or external SDKs.

## Deploy

The runtime is linked statically. Models and weights may be external or embedded.
On Windows, `-C target-feature=+crt-static` selects the matching static native CRT;
otherwise packaging includes the MSVC redistributable files.

The optional in-process compiler is shared. Package it with any selected linker
helper, resources, and required redistributables in one operation:

```powershell
cargo xtask package dist -- -p iree --example compile --features compiler --release
```

`dist` must be a new directory. The same generic packaging operation is available
as `woodshed bundle --output dist -- <Cargo options>` when Woodshed's CLI is
built. Pass `--manifest-path <application manifest>` and `--bin <application>`
to package a downstream application. Linux/macOS executables receive relative
loader paths. No end-user Rust, Clang, Python, CMake, or IREE installation is
needed. GPU execution still requires a compatible system driver and any
backend-specific vendor runtime, such as HIP.

See [native build and deployment](crates/iree-sys/native/README.md) for details.
