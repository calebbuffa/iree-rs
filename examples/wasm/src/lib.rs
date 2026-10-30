//! Multiply two vectors using a VMVX artifact compiled by the host build script.
//!
//! The JavaScript loader targets `wasm32-unknown-emscripten`. The same runtime
//! function is used by the desktop test; device-creation errors propagate.

use iree::runtime::domain::RuntimeDomainBuilder;
use iree::runtime::engine::RuntimeInstance;
use iree::runtime::Value;

/// The portable VMVX module, compiled from `model/simple_mul.mlir` at build time
/// by `build.rs` using `iree-compiler`.
pub const SIMPLE_MUL_VMFB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/simple_mul.vmfb"));

/// The two constant input vectors of the demo.
pub const LHS: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
pub const RHS: [f32; 4] = [5.0, 6.0, 7.0, 8.0];

/// Load the embedded module on the synchronous CPU device and multiply
/// `LHS * RHS` elementwise, returning the device result.
///
/// Device-creation and execution errors propagate to the caller.
pub fn run_demo() -> iree::Result<Vec<f32>> {
    // Native registration is process-wide; run the demo once per process.
    let mut startup = RuntimeDomainBuilder::new()?;
    let instance_id = startup.add_instance(RuntimeInstance::builder().all_available_drivers())?;

    let device_id = startup.add_runtime_device(instance_id, "local-sync")?;
    let mut domain = startup.seal();

    let instance = domain.runtime_instance(instance_id)?;
    let device = domain.take_runtime_device(device_id)?;

    // SAFETY: these bytes were produced by the matching pinned compiler in
    // build.rs; they are a trusted, version-locked VMVX artifact.
    let module = unsafe { instance.module_from_bytes(SIMPLE_MUL_VMFB) }?;
    let mut session = instance.session_builder(device).build()?;
    session.append_module(&module)?;

    let lhs = session.device().tensor(&[LHS.len()], &LHS)?;
    let rhs = session.device().tensor(&[RHS.len()], &RHS)?;
    let results = session.call("module.simple_mul", &[Value::from(lhs), Value::from(rhs)])?;

    match results.into_iter().next() {
        Some(Value::Tensor(tensor)) => tensor.read::<f32>(),
        _ => Err(iree::Error::new(
            iree::StatusCode::INVALID_ARGUMENT,
            "simple_mul did not return a tensor",
        )),
    }
}

/// Exported entry point invoked from JavaScript (`Module.ccall`).
///
/// Returns zero on success and one on failure, with the native error on stderr.
#[no_mangle]
pub extern "C" fn iree_wasm_demo_run() -> i32 {
    match run_demo() {
        Ok(result) => {
            println!("iree-wasm-example: simple_mul({LHS:?} * {RHS:?}) = {result:?}");
            0
        }
        Err(error) => {
            eprintln!("iree-wasm-example: {error}");
            1
        }
    }
}
