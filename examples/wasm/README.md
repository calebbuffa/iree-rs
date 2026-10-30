# WebAssembly prototype

This directory preserves an experimental Emscripten/Node.js integration
prototype. Its manifest and source still reference the removed `iree` facade
and legacy runtime APIs, so it is not a supported or validated workspace
example.

The JavaScript files demonstrate the former IREE proactor import arrangement;
they do not establish that the current runtime can create a CPU device or run
inference under WebAssembly. The current runtime API is [`ireert`](../../crates/ireert/README.md).
