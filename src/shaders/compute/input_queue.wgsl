@group(0) @binding(0)
var read_texture: texture_storage_2d<r32float, read_write>;

@group(1) @binding(0)
var write_texture: texture_storage_2d<r32float, read_write>;

/// x = posx | y = posy | z = shape (1 = sq, 2 = sph) | w = range

struct Queue {
    data: array<vec4<i32>, 100>,
}
@group(2) @binding(0)
var<uniform> queue: Queue;

@compute @workgroup_size(8, 1, 1)
fn cs_main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let index = global_id.x;

    let action = queue.data[index];

    if action.z == 1 /* square */ {
        for (var i = 0; i < action.w; i++) {
            for (var j = 0; j < action.w; j++) {
                let x = (action.x - action.w / 2) + i;
                let y = (action.y - action.w / 2) + j;
                put(vec2(x, y), 1.0);
            }
        }
    }
}

fn pull(pos: vec2<i32>) -> f32 {
    return textureLoad(read_texture, pos).r;
}

fn put(pos: vec2<i32>, val: f32) {
    textureStore(write_texture, pos, vec4<f32>(val, 0.0, 0.0, 1.0));
}