#import "shaders/core.wgsl"

@group(0) @binding(0)
var<uniform> size: vec2<u32>;
@group(0) @binding(1)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    var colour = AIR_COLOUR;

    // Add a stone barrier at the bottom of the screen
    if(location.y > 640) {
        colour = STONE_COLOUR;
    }

    textureStore(texture, location, colour);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let current_particle_colour = textureLoad(texture, location);

    if (distance(current_particle_colour, AIR_COLOUR) < EPSILON) {
        // do nothing
    } else if (distance(current_particle_colour, SAND_COLOUR) < EPSILON) {
        // check below and fall if we can
        let colour_below = textureLoad(texture, location + vec2<i32>(0, 1));
        if (distance(colour_below, AIR_COLOUR) < EPSILON) {
            // fall straight down
            textureStore(texture, location, AIR_COLOUR);
            textureStore(texture, location + vec2<i32>(0, 1), SAND_COLOUR);
        } else {
            // there is something directly below
            // select a random direction diagonally down
            // and try to fall there
            let rand = randomFloat(invocation_id.x + invocation_id.y);
            var new_x = -1;
            if (rand > 0.5) {
                new_x = 1;
            }
            let colour_below_diagonally = textureLoad(texture, location + vec2<i32>(new_x, 1));
            if (distance(colour_below_diagonally, AIR_COLOUR) < EPSILON) {
                textureStore(texture, location, AIR_COLOUR);
                textureStore(texture, location + vec2<i32>(new_x, 1), SAND_COLOUR);
            }
        }
    }
}
