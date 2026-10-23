module {
  func.func @main(
      %x: tensor<2x4xf32>,
      %w1: tensor<4x8xf32>, %b1: tensor<8xf32>,
      %w2: tensor<8x3xf32>, %b2: tensor<3xf32>) -> tensor<2x3xf32> {
    %dot1 = "stablehlo.dot_general"(%x, %w1) {
      dot_dimension_numbers = #stablehlo.dot<
        lhs_contracting_dimensions = [1], rhs_contracting_dimensions = [0]>
    } : (tensor<2x4xf32>, tensor<4x8xf32>) -> tensor<2x8xf32>
    %bias1 = "stablehlo.broadcast_in_dim"(%b1) {
      broadcast_dimensions = array<i64: 1>
    } : (tensor<8xf32>) -> tensor<2x8xf32>
    %hidden = stablehlo.add %dot1, %bias1 : tensor<2x8xf32>
    %zero = stablehlo.constant dense<0.0> : tensor<2x8xf32>
    %relu = stablehlo.maximum %hidden, %zero : tensor<2x8xf32>
    %dot2 = "stablehlo.dot_general"(%relu, %w2) {
      dot_dimension_numbers = #stablehlo.dot<
        lhs_contracting_dimensions = [1], rhs_contracting_dimensions = [0]>
    } : (tensor<2x8xf32>, tensor<8x3xf32>) -> tensor<2x3xf32>
    %bias2 = "stablehlo.broadcast_in_dim"(%b2) {
      broadcast_dimensions = array<i64: 1>
    } : (tensor<3xf32>) -> tensor<2x3xf32>
    %output = stablehlo.add %dot2, %bias2 : tensor<2x3xf32>
    return %output : tensor<2x3xf32>
  }
}
