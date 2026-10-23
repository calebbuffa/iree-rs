module {
  func.func @main(%lhs: tensor<4xf32>, %rhs: tensor<4xf32>) -> tensor<4xf32> {
    %result = stablehlo.add %lhs, %rhs : tensor<4xf32>
    return %result : tensor<4xf32>
  }
}
