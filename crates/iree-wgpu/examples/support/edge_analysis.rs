//! Shared definitions for the `render_analysis` example: image dimensions,
//! the StableHLO edge-detection program, and the WGSL render shader.

pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 192;
pub const EDGE_WIDTH: u32 = WIDTH - 2;
pub const EDGE_HEIGHT: u32 = HEIGHT - 2;

pub fn source() -> String {
    let (w, h, ew, eh) = (WIDTH, HEIGHT, EDGE_WIDTH, EDGE_HEIGHT);
    format!(
        r#"
module {{
  func.func @main(%rgba: tensor<{h}x{w}x4xf32>,
      %destination: tensor<{eh}x{ew}xf32> {{iree.abi.output = 0 : index}})
      -> tensor<{eh}x{ew}xf32> {{
    %red = stablehlo.slice %rgba [0:{h}, 0:{w}, 0:1] : (tensor<{h}x{w}x4xf32>) -> tensor<{h}x{w}x1xf32>
    %gray = stablehlo.reshape %red : (tensor<{h}x{w}x1xf32>) -> tensor<{h}x{w}xf32>
    %left = stablehlo.slice %gray [1:{hm}, 0:{ew}] : (tensor<{h}x{w}xf32>) -> tensor<{eh}x{ew}xf32>
    %right = stablehlo.slice %gray [1:{hm}, 2:{w}] : (tensor<{h}x{w}xf32>) -> tensor<{eh}x{ew}xf32>
    %top = stablehlo.slice %gray [0:{eh}, 1:{wm}] : (tensor<{h}x{w}xf32>) -> tensor<{eh}x{ew}xf32>
    %bottom = stablehlo.slice %gray [2:{h}, 1:{wm}] : (tensor<{h}x{w}xf32>) -> tensor<{eh}x{ew}xf32>
    %gx = stablehlo.subtract %right, %left : tensor<{eh}x{ew}xf32>
    %gy = stablehlo.subtract %bottom, %top : tensor<{eh}x{ew}xf32>
    %xx = stablehlo.multiply %gx, %gx : tensor<{eh}x{ew}xf32>
    %yy = stablehlo.multiply %gy, %gy : tensor<{eh}x{ew}xf32>
    %sum = stablehlo.add %xx, %yy : tensor<{eh}x{ew}xf32>
    %edge = stablehlo.sqrt %sum : tensor<{eh}x{ew}xf32>
    return %edge : tensor<{eh}x{ew}xf32>
  }}
}}
"#,
        hm = h - 1,
        wm = w - 1,
    )
}

pub const RENDER: &str = r#"
@group(0) @binding(0) var<uniform> frame: vec4<f32>;
@vertex fn vertex(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(3.0, -1.0), vec2<f32>(-1.0, 3.0));
    return vec4<f32>(positions[i], 0.0, 1.0);
}
@fragment fn fragment(@builtin(position) p: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = p.xy / vec2<f32>(256.0, 192.0);
    let center = vec2<f32>(0.5 + 0.2 * sin(frame.x), 0.5);
    let radius = length((uv - center) * vec2<f32>(1.3333333, 1.0));
    let disk = 1.0 - smoothstep(0.245, 0.255, radius);
    let stripes = 0.5 + 0.5 * sin(uv.x * 40.0 + frame.x);
    let gray = 0.08 + disk * (0.35 + stripes * 0.55);
    return vec4<f32>(gray, gray, gray, 1.0);
}
"#;
