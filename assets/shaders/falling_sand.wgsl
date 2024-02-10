#import "shaders/core.wgsl"

@group(0) @binding(0)
var<uniform> size: vec2<u32>;
@group(0) @binding(1)
var<storage, read_write> simulation_source: array<Cell>;
@group(0) @binding(2)
var<storage, read_write> simulation_destination: array<Cell>;
@group(0) @binding(3)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    var particle_type = AIR;

    // Add a stone barrier at the bottom of the screen
    if(location.y > 640) {
        particle_type = STONE;
    }

    simulation_source[index_of(location, size.x)] = Cell(particle_type);
    let colour = colour_for_particle_type(particle_type);
    textureStore(texture, location, colour);
}

fn get_cell(location: vec2<i32>) -> Cell {
    return simulation_source[index_of(location, size.x)];
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let current_cell = get_cell(location);

    if (current_cell.particle_type == AIR) {
        // do nothing
    } else if (current_cell.particle_type == SAND) {
        // check below and fall if we can
        let cell_below = get_cell(location + vec2<i32>(0, 1));
        if (cell_below.particle_type == AIR) {
            // fall straight down
            simulation_source[index_of(location, size.x)] = Cell(AIR);
            let air_colour = colour_for_particle_type(AIR);
            textureStore(texture, location, air_colour);

            simulation_source[index_of(location + vec2<i32>(0, 1), size.x)] = Cell(SAND);
            let sand_colour = colour_for_particle_type(SAND);
            textureStore(texture, location + vec2<i32>(0, 1), sand_colour);
        } else {
            // there is something directly below
            // select a random direction diagonally down
            // and try to fall there
            let rand = randomFloat(invocation_id.x + invocation_id.y);
            var new_x = -1;
            if (rand > 0.5) {
                new_x = 1;
            }
            let cell_below_diagonally = get_cell(location + vec2<i32>(new_x, 1));
            if (cell_below_diagonally.particle_type == AIR) {
                simulation_source[index_of(location, size.x)] = Cell(AIR);
                let air_colour = colour_for_particle_type(AIR);
                textureStore(texture, location, air_colour);

            simulation_source[index_of(location + vec2<i32>(new_x, 1), size.x)] = Cell(SAND);
            let sand_colour = colour_for_particle_type(SAND);
                textureStore(texture, location + vec2<i32>(new_x, 1), sand_colour);
            }
        }
    }
}
