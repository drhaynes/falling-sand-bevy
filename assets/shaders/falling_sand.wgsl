const AIR_COLOUR = vec4<f32>(0.02, 0.02, 0.02, 1.0);
const SAND_COLOUR = vec4<f32>(0.8, 0.8, 0.2, 1.0);
const STONE_COLOUR = vec4<f32>(0.4, 0.4, 0.4, 1.0);
const EPSILON = 0.001;

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    var colour = AIR_COLOUR;

    // Add sand sprinled around randomly
    let random_number = randomFloat(invocation_id.y * num_workgroups.x + invocation_id.x);
    let is_sand = random_number > 0.9;

    if(is_sand) {
        colour = SAND_COLOUR;
    }

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
        if(distance(colour_below, AIR_COLOUR) < EPSILON) {
            // fall straight down
            textureStore(texture, location, AIR_COLOUR);
            textureStore(texture, location + vec2<i32>(0, 1), SAND_COLOUR);
        } else {
            // there is something below
            // select a random direction diagonally down
            // and try to fall there
        }
    }
}
