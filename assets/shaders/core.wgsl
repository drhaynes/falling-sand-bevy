
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

const AIR: u32 = 0u;
const SAND: u32 = 1u;
const STONE: u32 = 2u;

fn colour_for_particle_type(particle_type: u32) -> vec4<f32> {
    switch particle_type {
        case 0u: {
            return AIR_COLOUR;
        }
        case 1u: {
            return SAND_COLOUR;
        }
        case 2u: {
            return STONE_COLOUR;
        }
        default: {
            return AIR_COLOUR;
        }
    }
}

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

// Returns the index in 1D buffer of given width of the given 2D location.
fn index_of(location: vec2<i32>, width: u32) -> i32 {
    return (location.y * i32(width)) + location.x;
}
