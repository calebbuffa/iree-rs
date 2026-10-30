// The canonical IREE "simple_mul" sample: elementwise multiply of two 4xf32
// tensors. Compiled to a portable VMVX .vmfb by scripts/generate_module.*.
func.func @simple_mul(%arg0: tensor<4xf32>, %arg1: tensor<4xf32>) -> tensor<4xf32> {
  %0 = arith.mulf %arg0, %arg1 : tensor<4xf32>
  return %0 : tensor<4xf32>
}
