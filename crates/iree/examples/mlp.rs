//! Compile and execute a two-layer MLP, comparing with a Rust reference.
use iree::{Compiler, Instance, Target};

const MODEL: &str = include_str!("models/mlp.mlir");

fn run(target: Target, driver: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let mut compiler = Compiler::new()?;
    let artifact = compiler.compile(MODEL, target)?;

    let device = Instance::new()?.device(driver)?;
    let mut session = device.load(&artifact)?;
    let x = [1.0f32, -2.0, 3.0, -4.0, -0.5, 0.25, 1.5, 2.0];
    let w1 = std::array::from_fn::<_, 32, _>(|i| (i as i32 % 9 - 4) as f32 / 8.0);
    let b1 = [-0.5f32, 0.25, 0.0, 0.75, -0.25, 0.5, -0.75, 1.0];
    let w2 = std::array::from_fn::<_, 24, _>(|i| (i as i32 % 7 - 3) as f32 / 4.0);
    let b2 = [0.125f32, -0.25, 0.5];
    let inputs = [
        device.tensor(&[2, 4], &x)?,
        device.tensor(&[4, 8], &w1)?,
        device.tensor(&[8], &b1)?,
        device.tensor(&[8, 3], &w2)?,
        device.tensor(&[3], &b2)?,
    ];
    let expected = reference(&x, &w1, &b1, &w2, &b2);
    for _ in 0..3 {
        let outputs = session.call("module.main", &inputs)?;
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].shape(), [2, 3]);
        let actual = outputs[0].read::<f32>()?;
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(&expected) {
            assert!(
                (actual - expected).abs() <= 1e-5,
                "MLP mismatch: {actual} != {expected}"
            );
        }
    }
    Ok(expected)
}

fn reference(
    x: &[f32; 8],
    w1: &[f32; 32],
    b1: &[f32; 8],
    w2: &[f32; 24],
    b2: &[f32; 3],
) -> Vec<f32> {
    let mut output = vec![0.0; 6];
    for batch in 0..2 {
        let hidden = std::array::from_fn::<_, 8, _>(|channel| {
            (b1[channel]
                + (0..4)
                    .map(|input| x[batch * 4 + input] * w1[input * 8 + channel])
                    .sum::<f32>())
            .max(0.0)
        });
        for channel in 0..3 {
            output[batch * 3 + channel] = b2[channel]
                + (0..8)
                    .map(|input| hidden[input] * w2[input * 3 + channel])
                    .sum::<f32>();
        }
    }
    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (target, driver) = match std::env::args().nth(1).as_deref() {
        None => (Target::Cpu, "local-task"),
        Some("--vulkan") => (Target::Vulkan, "vulkan"),
        Some(argument) => return Err(format!("unknown argument: {argument}; use --vulkan").into()),
    };
    let output = run(target, driver)?;
    println!("{driver}: 2 x (4 -> 8 -> 3), ReLU, three verified invocations");
    println!("output: {output:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "cpu")]
    #[test]
    fn mlp_cpu() -> Result<(), Box<dyn std::error::Error>> {
        run(Target::Cpu, "local-task").map(|_| ())
    }

    #[cfg(feature = "vulkan")]
    #[test]
    #[ignore = "requires a Vulkan compute device"]
    fn mlp_vulkan() -> Result<(), Box<dyn std::error::Error>> {
        run(Target::Vulkan, "vulkan").map(|_| ())
    }
}
