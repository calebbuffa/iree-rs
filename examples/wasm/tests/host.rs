//! Check the demo's numerical result on the host, not its Wasm deployment.

#[test]
fn simple_mul_runs_on_host() {
    let result = iree_wasm_example::run_demo().expect("host CPU device runs the VMVX module");
    let expected: Vec<f32> = iree_wasm_example::LHS
        .iter()
        .zip(iree_wasm_example::RHS.iter())
        .map(|(a, b)| a * b)
        .collect();
    assert_eq!(result, expected, "simple_mul must multiply elementwise");
    assert_eq!(result, vec![5.0, 12.0, 21.0, 32.0]);
}
