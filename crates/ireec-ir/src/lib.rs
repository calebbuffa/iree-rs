//! RAII bindings for the MLIR C API shipped with the IREE compiler.
//!
//! Every handle borrows its [`Context`]; owned handles destroy their native
//! object on drop unless ownership is transferred into a parent.
//!
//! This crate is the in-process route for building and transforming IR in the
//! same MLIR instance used by libIREECompiler. Independently linked MLIR tools
//! such as pliron or melior cannot share these native handles; exchange MLIR
//! text or bytecode through [`ireec::Source`] instead.
//!
//! A Rust external pass can be inserted into IREE's standard compilation
//! pipeline by stopping at a phase, exporting the module, running a pass
//! manager in the same context, then importing the module and resuming:
//!
//! ```no_run
//! use ireec::{Compiler, Output, Phase, Pipeline};
//! use ireec_ir::{
//!     Borrowed, Context, ExternalPass, InvocationExt, Operation, Pass, PassManager,
//! };
//!
//! struct RustPass;
//! impl ExternalPass for RustPass {
//!     fn clone_box(&self) -> Box<dyn ExternalPass> { Box::new(Self) }
//!     fn run(&mut self, _: &mut Operation<'_, Borrowed>) -> ireec_ir::Result<()> { Ok(()) }
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let compiler = Compiler::new()?;
//!     let session = compiler.session()?;
//!     let context = Context::borrow_session(&session)?;
//!     let mut invocation = session.invocation()?;
//!     invocation.parse_source(session.source_buffer("input.mlir", b"module {}")?)?;
//!     invocation.set_compile_to_phase(Phase::Flow)?;
//!     invocation.pipeline(Pipeline::Std)?;
//!
//!     let mut module = invocation.export_steal()?;
//!     let mut manager = PassManager::new_on_operation(&context, "builtin.module")?;
//!     manager.add_pass(Pass::external(
//!         &context, "rust-noop", "rust-noop", "", "builtin.module", RustPass,
//!     )?)?;
//!     let mut operation = module.operation();
//!     manager.run(&mut operation)?;
//!     drop(operation);
//!
//!     invocation = invocation.import_steal(module)?;
//!     invocation.set_compile_from_phase(Phase::Flow)?;
//!     invocation.pipeline(Pipeline::Std)?;
//!     let mut output = Output::memory(&compiler)?;
//!     invocation.output_vm_bytecode(&mut output)?;
//!     let _vmfb = output.bytes()?;
//!     Ok(())
//! }
//! ```
#![deny(missing_docs, clippy::all)]

pub mod affine;
pub mod attribute;
pub mod block;
mod callback;
pub mod compiler_bridge;
pub mod context;
pub mod diagnostics;
pub mod dialect;
pub mod dialect_registry;
pub mod error;
pub mod interfaces;
pub mod irdl;
pub mod location;
pub mod module;
pub mod operation;
pub mod operation_builder;
pub mod pass;
pub mod printing;
pub mod region;
mod string_ref;
pub mod symbols;
pub mod types;
pub mod value;

pub use affine::{AffineExpr, AffineMap, IntegerSet};
pub use attribute::Attribute;
pub use block::Block;
pub use compiler_bridge::{BorrowedInvocation, InvocationExt};
pub use context::{Borrowed, Context, Owned};
pub use diagnostics::{Diagnostic, DiagnosticHandler, DiagnosticSeverity};
pub use dialect::{Dialect, DialectHandle};
pub use dialect_registry::DialectRegistry;
pub use error::{Error, Result};
pub use interfaces::{
    MemoryEffect, MemoryEffectKind, MemoryEffectResource, OperationInterface, ShapedTypeComponents,
    Speculatability, TypeId,
};
pub use location::Location;
pub use module::Module;
pub use operation::Operation;
pub use operation_builder::OperationBuilder;
pub use pass::{ExternalPass, IrPrinting, OpPassManager, Pass, PassManager, StatisticsDisplay};
pub use printing::{OpPrintingFlags, PrintingOptions, write_bytecode, write_operation};
pub use region::Region;
pub use symbols::{SymbolTable, SymbolTableScope, replace_all_symbol_uses, walk_symbol_tables};
pub use types::Type;
pub use value::Value;
