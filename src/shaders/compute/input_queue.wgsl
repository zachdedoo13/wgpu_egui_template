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
    else if action.z == -1 /* remove square */ {
            for (var i = 0; i < action.w; i++) {
                for (var j = 0; j < action.w; j++) {
                    let x = (action.x - action.w / 2) + i;
                    let y = (action.y - action.w / 2) + j;
                    put(vec2(x, y), 0.0);
                }
            }
        }

    else if action.z == 2 /* circle */ {
        let pos: vec2<i32> = action.xy;
        let radius: i32 = action.w;
        let x0: i32 = pos.x;
        let y0: i32 = pos.y;

        var x: i32 = radius;
        var y: i32 = 0;
        var err: i32 = 0;

        while (x >= y) {
            for (var i = -x; i <= x; i++) {
                put(vec2(x0 + i, y0 + y), 1.0);
                put(vec2(x0 + i, y0 - y), 1.0);
            }

            for (var i = -y; i <= y; i++) {
                put(vec2(x0 + i, y0 + x), 1.0);
                put(vec2(x0 + i, y0 - x), 1.0);
            }

            if (err <= 0) {
                y += 1;
                err += 2*y + 1;
            }

            if (err > 0) {
                x -= 1;
                err -= 2*x + 1;
            }
        }
    }
    else if action.z == -2 /* remove circle */ {
        let pos: vec2<i32> = action.xy;
        let radius: i32 = action.w;
        let x0: i32 = pos.x;
        let y0: i32 = pos.y;

        var x: i32 = radius;
        var y: i32 = 0;
        var err: i32 = 0;

        while (x >= y) {
            for (var i = -x; i <= x; i++) {
                put(vec2(x0 + i, y0 + y), 0.0);
                put(vec2(x0 + i, y0 - y), 0.0);
            }

            for (var i = -y; i <= y; i++) {
                put(vec2(x0 + i, y0 + x), 0.0);
                put(vec2(x0 + i, y0 - x), 0.0);
            }

            if (err <= 0) {
                y += 1;
                err += 2*y + 1;
            }

            if (err > 0) {
                x -= 1;
                err -= 2*x + 1;
            }
        }
    }

    else if action.z == 3 /* square grid */ {
        var counter = 0;
        for (var i = 0; i < action.w; i++) {
            if action.w % 2 == 0 { counter += 1; }
            for (var j = 0; j < action.w; j++) {
                counter += 1;
                if counter % 2 == 0 { continue; }

                let x = (action.x - action.w / 2) + i;
                let y = (action.y - action.w / 2) + j;
                put(vec2(x, y), 1.0);
            }
        }
    }
    else if action.z == -3 /* remove square grid */ {
            var counter = 0;
            for (var i = 0; i < action.w; i++) {
                if action.w % 2 == 0 { counter += 1; }
                for (var j = 0; j < action.w; j++) {
                    counter += 1;
                    if counter % 2 == 0 { continue; }

                    let x = (action.x - action.w / 2) + i;
                    let y = (action.y - action.w / 2) + j;
                    put(vec2(x, y), 0.0);
                }
            }
        }

    else if action.z == 4 /* sparce square grid */ {
        var counter = 0;
        var flipper = false;
        for (var i = 0; i < action.w; i++) {
            if action.w % 2 == 0 { counter += 1; }
            for (var j = 0; j < action.w; j++) {
                counter += 1;
                if counter % 2 == 0 { continue; }

                flipper = !flipper;
                if flipper { continue; }

                let x = (action.x - action.w / 2) + i;
                let y = (action.y - action.w / 2) + j;
                put(vec2(x, y), 1.0);
            }
        }
    }
    else if action.z == -4 /* remove square */ {
        for (var i = 0; i < action.w; i++) {
            for (var j = 0; j < action.w; j++) {
                let x = (action.x - action.w / 2) + i;
                let y = (action.y - action.w / 2) + j;
                put(vec2(x, y), 0.0);
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