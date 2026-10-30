//! The VM: instances, contexts, modules, functions, lists, values, references,
//! invocation and event loops.

mod buffer;
mod bytecode_module;
mod context;
mod dynamic_module;
mod event_loop;
mod function;
mod hal_module;
mod instance;
#[cfg(feature = "cpu")]
pub(crate) use instance::IsolatedTypeRegistryGuard;
mod invocation;
#[cfg(feature = "io-parameters")]
mod io_parameters_module;
mod list;
mod module;
mod native_module;
mod ref_type;
mod reference;
mod value;

pub use buffer::{Buffer, BufferAccess};
pub use bytecode_module::{BytecodeModule, BytecodeModuleFlags, DisassemblyFormat, FeatureBits};
pub use context::{Context, ContextFlags, ContextSignal, ModuleState};
pub use event_loop::{Continuation, InlineLoop, Loop, LoopDriver, WaitMode, WaitRequest};
pub use function::{Function, FunctionLinkage};
pub use hal_module::{DebugSink, HalModuleDevicePolicy, HalModuleFlags, HalModuleState};
pub use instance::Instance;
pub use invocation::{Invocation, InvocationFlags, WaitFrame, WaitFrameKind};
pub use list::{Element, List};
pub use module::{Module, ModuleDependency, ModuleSignature};
#[doc(hidden)]
pub use native_module::{NativeArguments, NativeResults};
pub use native_module::{NativeCall, NativeCallPhase, NativeModule, NativeValue};
pub use ref_type::{RefType, TypeDef};
pub use reference::Ref;
pub use value::{Value, ValueType};
