//! Execute embedded VM bytecode without linking the optional IREE compiler.
use iree::Instance;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let path = args.next().ok_or("usage: runtime <add.vmfb>")?;
    if args.next().is_some() {
        return Err("usage: runtime <add.vmfb>".into());
    }
    let bytes = std::fs::read(path)?;
    // This example accepts only trusted compiler output; VMFB may contain native code.
    let device = Instance::new()?.device("local-task")?;
    let mut session = unsafe { device.load_bytes(bytes) }?;
    let inputs = [
        device.tensor(&[4], &[1.0f32, 2.0, 3.0, 4.0])?,
        device.tensor(&[4], &[10.0f32, 20.0, 30.0, 40.0])?,
    ];
    let outputs = session.call("module.main", &inputs)?;
    let result = outputs
        .first()
        .ok_or("model returned no outputs")?
        .read::<f32>()?;
    if result != [11.0, 22.0, 33.0, 44.0] {
        return Err(format!("unexpected model result: {result:?}").into());
    }
    println!("CPU runtime: {result:?}");
    Ok(())
}
