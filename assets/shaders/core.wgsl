
struct Cell {
    particle_type: u32
}

struct PushConstants {
    drawing_start: vec2<f32>,
    drawing_end: vec2<f32>,
    brush_radius: f32,
}

const AIR_COLOUR = vec4<f32>(0.02, 0.02, 0.02, 1.0);
const SAND_COLOUR = vec4<f32>(0.7, 0.58, 0.44, 1.0);
const STONE_COLOUR = vec4<f32>(0.4, 0.4, 0.4, 1.0);
const EPSILON = 0.01;

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
