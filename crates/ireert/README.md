# ireert

`ireert` provides owned Rust interfaces for IREE's VM, HAL, and I/O APIs.
Instances, modules, devices, buffers, lists, and other native resources use
RAII; safe methods validate ranges and preserve native ownership.

The default `cpu` feature enables the local CPU drivers. Disable default
features and select `local-sync`, `local-task`, `vulkan`, `cuda`, `hip`,
`amdgpu`, `metal`, or other supported backends as needed. Runtime drivers select
execution; compiler targets are selected separately in `ireec`.
Parameter I/O and the IRPA, GGUF, and safetensors parsers are optional runtime
features.

VM bytecode and executable plugins can run native code. Load only trusted
artifacts. The [`iree-samples` index](../../examples/README.md) shows runtime
and HAL usage; native build requirements are in
[`iree-native`](../iree-native/README.md).
