# ireec-ir

Internal, unpublished RAII bindings for the MLIR C API linked into IREE's
compiler. Handles borrow their `Context`; owned operations, regions, types,
attributes, and passes release native resources on drop.

Use this crate when implementing in-process IR transformations against the same
MLIR instance as `ireec`. For independently linked MLIR tools, exchange MLIR
text or bytecode instead. See the [compiler API](../ireec/README.md).
