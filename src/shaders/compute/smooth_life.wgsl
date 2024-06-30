
@group(0) @binding(0)
var read_texture: texture_storage_2d<r32float, read_write>;

@group(1) @binding(0)
var write_texture: texture_storage_2d<r32float, read_write>;


@compute @workgroup_size(16, 16, 1)
fn cs_main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let uv = vec2<i32>(i32(global_id.x), i32(global_id.y));


}

fn pull(pos: vec2<i32>) -> f32 {
    return textureLoad(read_texture, pos).r;
}

fn put(pos: vec2<i32>, val: f32) {
    textureStore(write_texture, pos, vec4<f32>(val, 0.0, 0.0, 1.0));
}

