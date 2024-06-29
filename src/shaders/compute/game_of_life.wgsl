
@group(0) @binding(0)
var read_texture: texture_storage_2d<r32float, read_write>;

@group(1) @binding(0)
var write_texture: texture_storage_2d<r32float, read_write>;


@compute @workgroup_size(16, 16, 1)
fn cs_main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let uv = vec2<i32>(i32(global_id.x), i32(global_id.y));

    game_of_life(uv);
}

fn within_bounds(pos: vec2<i32>) -> bool {
    return true;
}

fn pull(pos: vec2<i32>) -> f32 {
    if !within_bounds(pos) { return 0.0; }
    return textureLoad(read_texture, pos).r;
}

fn put(pos: vec2<i32>, val: f32) {
    if !within_bounds(pos) { return; }
    textureStore(write_texture, pos, vec4<f32>(val, 0.0, 0.0, 1.0));
}


// game of life
fn game_of_life(uv: vec2<i32>) {
    let nearby = count_directly_nearby(uv);

    let alive = 1.0;
    let dead = 0.0;

    // dead alive threshold
    let dat = 0.5;

    // enshore all conditions are accounted for
    if pull(uv) > dat {
        if nearby < 2 || nearby > 3 { put(uv, dead); }
        else { put(uv, alive); }
    }
    else {
        if nearby == 3 { put(uv, alive); }
        else { put(uv, dead); }
    }
}

fn count_directly_nearby(uv: vec2<i32>) -> i32 {
    var count = 0;

    let dat = 0.5;

    // Top-left neighbor
    if pull(vec2(uv.x - 1, uv.y + 1)) > dat { count += 1; };
    // Top neighbor
    if pull(vec2(uv.x, uv.y + 1)) > dat { count += 1; };
    // Top-right neighbor
    if pull(vec2(uv.x + 1, uv.y + 1)) > dat { count += 1; };

    // Left neighbor
    if pull(vec2(uv.x - 1, uv.y)) > dat { count += 1; };
    // Right neighbor
    if pull(vec2(uv.x + 1, uv.y)) > dat { count += 1; };

    // Bottom-left neighbor
    if pull(vec2(uv.x - 1, uv.y - 1)) > dat { count += 1; };
    // Bottom neighbor
    if pull(vec2(uv.x, uv.y - 1)) > dat { count += 1; };
    // Bottom-right neighbor
    if pull(vec2(uv.x + 1, uv.y - 1)) > dat { count += 1; };


    return count;
}

