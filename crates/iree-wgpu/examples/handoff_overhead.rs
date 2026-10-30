//! Host-side latency of context submissions; not a GPU execution benchmark.
//!
//! Measures `Context::submit` for a tiny WGPU command buffer, and the
//! cross-API handoff of a following IREE operation (`Context::copy_to_shared`
//! after WGPU work, which marks the WGPU queue and orders IREE after it).
//! Prints CSV of median and 95th-percentile host latencies.
//!
//! ```text
//! cargo run -p iree-wgpu --release --example handoff_overhead [-- --iterations COUNT]
//! ```

use iree_wgpu::{Context, ContextOptions};
use std::{error::Error, time::Instant};

fn report(label: &str, samples: &mut [u128]) {
    samples.sort_unstable();
    let median = samples[samples.len() / 2];
    let p95 = samples[(samples.len() - 1) * 95 / 100];
    println!("{label},{},{median},{p95}", samples.len());
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let iterations = match args.next().as_deref() {
        None => 256,
        Some("--iterations") => args
            .next()
            .ok_or("--iterations needs a count")?
            .parse::<usize>()?,
        Some(_) => return Err("usage: handoff_overhead [--iterations COUNT]".into()),
    };
    if args.next().is_some() || !(1..=100_000).contains(&iterations) {
        return Err("iteration count must be between 1 and 100000".into());
    }
    let context = pollster::block_on(Context::new(ContextOptions::default()))?;
    println!("adapter: {}", context.adapter_name());
    println!("queue topology: {:?}", context.queue_topology());
    println!(
        "profile: {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
    let shared = context.allocate_shared(256)?;
    let view = context.upload(&[0u32; 64], &[64])?;
    let clear = || {
        let mut encoder = context.device().create_command_encoder(&Default::default());
        encoder.clear_buffer(shared.wgpu(), 0, None);
        encoder.finish()
    };
    for _ in 0..16 {
        context.submit([clear()])?;
        drop(context.copy_to_shared(&view)?);
    }
    context.finish()?;

    let mut submit_samples = Vec::with_capacity(iterations);
    let mut handoff_samples = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let commands = clear();
        let start = Instant::now();
        context.submit([commands])?;
        submit_samples.push(start.elapsed().as_nanos());

        // Includes the shared allocation and WGPU adoption, so this bounds the
        // handoff cost from above.
        let start = Instant::now();
        let copy = context.copy_to_shared(&view)?;
        handoff_samples.push(start.elapsed().as_nanos());
        context.finish()?;
        drop(copy);
    }
    println!("path,samples,median_host_ns,p95_host_ns");
    report("context_submit", &mut submit_samples);
    report("wgpu_to_iree_copy_to_shared", &mut handoff_samples);
    println!("Host latency only; not IREE kernel, GPU latency, or end-to-end frame timing.");
    Ok(())
}
