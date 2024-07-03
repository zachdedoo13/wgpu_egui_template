
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
    let nearby = count_directly_nearby(uv, 1);

    let alive = 1.0;
    let dead = 0.0;

    let lower = 2;
    let upper = 3;

    let live_lower = 3;
    let live_upper = 3;

    // dead alive threshold
    let dat = 0.5;

    // enshore all conditions are accounted for
    if pull(uv) > dat {
        if nearby < lower || nearby > upper { put(uv, dead); }
        else { put(uv, alive); }
    }
    else {
        if nearby >= live_lower && nearby <= live_upper { put(uv, alive); }
        else { put(uv, dead); }
    }
}

fn count_directly_nearby(uv: vec2<i32>, range: i32) -> i32 {
    var count = 0;
    let dat = 0.5;

    for (var dx: i32 = -range; dx <= range; dx++) {
        for (var dy: i32 = -range; dy <= range; dy++) {
            if dx == 0 && dy == 0 { continue; } // Skip the cell itself
            let value = pull(vec2(uv.x + dx, uv.y + dy));
            if value > dat { count += 1; };
        }
    }

    return count;
}

