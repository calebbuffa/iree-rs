# [IREE](https://iree.dev) for Rust

| Crate      | Responsibility                                     |
| ---------- | -------------------------------------------------- |
| `iree-sys` | C bindings                                         |
| `iree`     | Safe Rust interfaces; raw APIs through `iree::sys` |
| `iree-gfx` | Vulkan/WGPU interop                                |

## Build

`iree-sys` provisions pinned, verified sources through Woodshed, builds the
selected native capabilities, then generates bindings against that target's
headers and native configuration. All generated Rust and C files stay in
Cargo's build directory.

## Deploy

See [native build and deployment](crates/iree-sys/native/README.md).
