// Vertex shader
struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>, // Add this line
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;


@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) instance_index : u32,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * vec4<f32>((model.position), 1.0);
    out.uv = model.position.xy * 0.5 + 0.5;
    return out;
}



// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_storage_2d<r32float, read_write>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = vec2<i32>(in.uv * vec2<f32>(textureDimensions(t_diffuse)));
    let sampled_color = textureLoad(t_diffuse, uv).r;

    return vec4<f32>(sampled_color, 0.0, 0.0, 1.0);
}