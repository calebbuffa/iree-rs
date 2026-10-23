# iree

Rust bindings for [IREE](https://iree.dev).

## Compile and execute on CPU

```rust,no_run
use iree::{Compiler, Instance, Target};

let source = r#"
module {
  func.func @main(%a: tensor<4xf32>, %b: tensor<4xf32>) -> tensor<4xf32> {
    %sum = stablehlo.add %a, %b : tensor<4xf32>
    return %sum : tensor<4xf32>
  }
}"#;

let artifact = Compiler::new()?.compile(source, Target::Cpu)?;
let instance = Instance::new()?;
let device = instance.device("local-task")?;
let mut session = device.load(&artifact)?;
let a = device.tensor(&[4], &[1.0f32, 2.0, 3.0, 4.0])?;
let b = device.tensor(&[4], &[10.0f32, 20.0, 30.0, 40.0])?;
let result = session.call("module.main", &[a, b])?;
assert_eq!(result[0].read::<f32>()?, vec![11.0, 22.0, 33.0, 44.0]);
# Ok::<(), iree::Error>(())
```

## Reuse and compose modules

```rust,no_run
use iree::{Compiler, Instance, SessionSignal, Target};

# fn example(library_source: &str, application_source: &str) -> iree::Result<()> {
let mut compiler = Compiler::new()?;
let instance = Instance::new()?;
let library = instance.module(&compiler.compile(library_source, Target::Cpu)?)?;
let application = instance.module(&compiler.compile(application_source, Target::Cpu)?)?;
let device = instance.device("local-task")?;
let mut session = device.session_builder()
    .module(&library)
    .module(&application)
    .build()?;
let metadata = application.info()?;
for function in metadata.exports {
    println!("{}: {}", function.name, function.calling_convention);
}
session.notify(SessionSignal::LowMemory)?;
# Ok(())
# }
```

## Discover and select devices

```rust,no_run
use iree::Instance;

let instance = Instance::new()?;
for info in instance.drivers()? {
    println!("{}: {}", info.name, info.description);
}
let driver = instance.driver("local-task")?;
let available = driver.devices()?;
let device = available[0].builder().build()?;
// Alternatively: driver.device_builder().ordinal(0).parameter(key, value).build()
# Ok::<(), iree::Error>(())
```

An enumerated device retains its originating driver, so its opaque ID cannot
accidentally be used with another driver. Reusing a driver avoids repeated
native driver initialization. Native selection/configuration errors propagate.

Created devices expose their runtime identifier through `Device::id()`,
driver-specific integer properties through `query_i64(category, key)`, and
live-allocation-preserving cache/pool reclamation through `trim()`.
Unsupported queries remain native errors, distinct from supported queries
returning zero.

## Native completion futures

```rust,no_run
use iree::Timeline;

# async fn example() -> iree::Result<()> {
let timeline = Timeline::new(0)?;
let completion = timeline.wait(1)?;
let producer = timeline.clone();
std::thread::spawn(move || producer.signal(1)).join().unwrap()?;
completion.await?;
# Ok(())
# }
```

This example requires `task` for cross-thread ownership. A `Timeline` is a
native software timeline; `Device::semaphore` creates a device-compatible HAL
timeline. Both return futures driven by IREE timepoint callbacks, not blocking
worker threads or timer polling. Futures retain their source, replace executor
wakers safely, and unregister pending timepoints when dropped. Callbacks
already in flight retain their completion storage. Timeline cancellation is a
sticky native failure and wakes current and future waiters.

## Asynchronous function execution

```rust,no_run
use iree::{CompileOptions, Compiler, ExecutionModel, Input, Instance, Target, Value};

# async fn example(source: &str) -> iree::Result<()> {
let options = CompileOptions::new(Target::Cpu)
    .input(Input::StableHlo)
    .execution_model(ExecutionModel::AsyncExternal);
let artifact = Compiler::new()?.compile_with_options(source, &options)?;
let device = Instance::new()?.device("local-task")?;
let mut session = device.load(&artifact)?;
let input = Value::from(device.tensor(&[4], &[1_f32; 4])?);
let outputs = session.call_async("module.main", &[input])?.await?;
# let _ = outputs;
# Ok(())
# }
```

This uses IREE's reflected `coarse-fences` ABI. Submission performs native
host-side preparation, then returns a callback-driven completion future without
spawning a blocking Rust worker. Output objects remain inaccessible until the
completion fence is satisfied. `call_async_after` adds an explicit same-device
dependency; pending accesses to input and caller-provided output tensors are
chained automatically. Safe tensor reads, including clones/aliased native
views, wait for outstanding accesses.

Dropping an invocation future detaches work rather than canceling it. Its
session retains argument/result storage until completion. `Session::drain`
waits for detached calls and reports failures; dropping that session drains
remaining calls and can block. Submission can itself perform host work, so
this is not a promise that every native call returns immediately.

General staged/event-loop VM invocation
are separate interfaces still under development.

## Configure native task execution

```rust,no_run
use iree::{Instance, task::{ExecutorOptions, TaskDeviceOptions, Topology}};

let topology = Topology::workers(4)?;
let executor = ExecutorOptions::default()
    .local_memory_size(64 * 1024)
    .build(&topology)?;
let device = TaskDeviceOptions::default().build(&Instance::new()?, &[executor])?;
// Load artifacts and create tensors on this explicitly configured device.
# let _ = device;
# Ok::<(), iree::Error>(())
```

`task` enables topology/executor ownership; `local-task` additionally enables
configured devices. Each supplied executor defines a native queue. Executors
can be shared across queues or selected per NUMA domain, and native queues
retain their worker pools. Defaults are initialized through IREE, not copied
from hard-coded native implementation details. Zero-worker executors are
rejected because the safe interface does not expose thread donation.

## Parameter-backed weights

```rust,no_run
use iree::{Instance, ParameterIndex};

# fn example(artifact: &iree::Artifact) -> iree::Result<()> {
let mut weights = ParameterIndex::new()?;
weights.load_file("model.safetensors")?;
let provider = weights.provider("model", 4)?;
let device = Instance::new()?.device("local-task")?;
let session = device.load_with_parameters(artifact, &[provider])?;
drop(weights); // The loaded parameter module retains native backing handles.
# let _ = session;
# Ok(())
# }
```

Indexes support owned byte allocations, compact repeated patterns, and native
GGUF/IRPA/safetensors parsing from owned memory or read-only files. Scope names
must match the compiled model's named parameter references. Providers retain
indexes and backing storage; the loaded module retains its providers. Parsing
into an existing index is not atomic: malformed later entries can leave earlier
insertions present. General Rust-implemented custom providers are not yet
exposed.

## Ordered transfer commands

```rust,no_run
use iree::Instance;

# async fn example() -> iree::Result<()> {
let device = Instance::new()?.device("local-task")?;
let input = device.buffer(&[0; 64])?;
let output = device.buffer(&[0; 64])?;
let mut commands = device.transfer_commands()?;
commands.fill(&input, &[7])?.copy(&input, &output)?;
commands.submit()?.await?;
assert_eq!(output.read()?, [7; 64]);
# Ok(())
# }
```

`Buffer` exposes only initialized bytes and checked shared subranges. The
one-shot recorder retains resources, validates device identity, rejects
overlapping copies, and orders transfer operations. Consecutive submissions
automatically chain outstanding readers/writers for an allocation, including
its slices. Host reads/writes wait for those accesses. Dropping a submission
future does not cancel work; dropping the final buffer owner drains it.
`buffer.as_tensor::<f32>(&shape)` and `tensor.as_buffer()` create owned aliases
without copying or host staging. Byte length, shape overflow, and element
alignment are checked. All aliases share allocation-level dependency tracking,
including subranges created through different native buffer views. Transfer
submissions automatically wait on asynchronous model accesses, and model calls
automatically chain outstanding transfers on their tensor inputs.
`Submission::completion()` exposes a fence for explicit downstream work.

This interface currently records fill/update/copy transfers, not arbitrary
dispatches or collectives.

## Share hardware across worker threads

```rust,no_run
use iree::Instance;

let device = Instance::new()?.shared_device("local-task")?;
let worker = std::thread::spawn(move || -> iree::Result<()> {
    let local = device.local();
    let tensor = local.tensor(&[2], &[1_i32, 2])?;
    assert_eq!(tensor.read::<i32>()?, [1, 2]);
    // Load an Artifact here to create this worker's isolated VM context.
    Ok(())
});
worker.join().unwrap()?;
# Ok::<(), iree::Error>(())
```

This requires `task`. Workers share one native device, allocator, task runtime,
HAL module, and device group, but not an `Rc` ownership graph or mutable VM
context. This API deliberately cannot adopt an arbitrary externally owned
WGPU/native device as a thread-shareable handle.
