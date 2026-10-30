# Bare-metal prototype

This directory preserves an early Cortex-M4/QEMU VM experiment. It currently
depends on the removed `iree` facade and legacy runtime APIs, so it does not
build against the workspace crates and is not a supported example.

The files may still be useful as a record of the intended `no_std` setup,
board-support layout, and VM-only execution path. The current public runtime
crate is [`ireert`](../../crates/ireert/README.md); a maintained firmware port
requires a compatible `no_std` runtime API and is not yet available.
