#import "shaders/core.wgsl"

@group(0) @binding(0)
var<uniform> size: vec2<u32>;
@group(0) @binding(1)
var<storage, read_write> simulation_source: array<Cell>;
@group(0) @binding(2)
var<storage, read_write> simulation_destination: array<Cell>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    var particle_type = AIR;

    // Add a stone barrier at the bottom of the screen
    let barrier_thickness = 40;
    if (location.y < barrier_thickness || location.y > i32(size.y) - barrier_thickness) {
        particle_type = STONE;
    }

    if (location.x < barrier_thickness || location.x > i32(size.x) - barrier_thickness) {
        particle_type = STONE;
    }

    simulation_source[index_of(location, size.x)] = Cell(particle_type);
}

fn get_cell(location: vec2<i32>) -> Cell {
    return simulation_source[index_of(location, size.x)];
}

fn write_cell(location: vec2<i32>, cell_type: u32) {
    simulation_destination[index_of(location, size.x)] = Cell(cell_type);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let current_cell = get_cell(location);

    let location_below = location + vec2<i32>(0, 1);
    let location_above = location + vec2<i32>(0, -1);

    let cell_above = get_cell(location_above);
    let cell_below = get_cell(location_below);

    if (current_cell.particle_type == AIR) {
        if (cell_above.particle_type == SAND) {
            write_cell(location, SAND);
            write_cell(location_above, AIR);
        } else {
            //write_cell(location, AIR);
        }
    } else if (current_cell.particle_type == SAND) {
        // check below and fall if we can

        if (cell_below.particle_type == AIR) {
            // fall straight down
            write_cell(location, AIR);
            write_cell(location_below, SAND);
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
                simulation_destination[index_of(location, size.x)] = Cell(AIR);
                simulation_destination[index_of(location + vec2<i32>(new_x, 1), size.x)] = Cell(SAND);
            } else {
                // can't go that way, try to go the other way
                let cell_below_other_way = get_cell(location + vec2<i32>(-new_x, 1));
                if (cell_below_other_way.particle_type == AIR) {
                simulation_destination[index_of(location, size.x)] = Cell(AIR);
                simulation_destination[index_of(location + vec2<i32>(-new_x, 1), size.x)] = Cell(SAND);
                } else {
                    // anything other than air, don't fall, stay in place
                    simulation_destination[index_of(location, size.x)] = Cell(SAND);
                }
            }
        }
    } else if (current_cell.particle_type == STONE) {
        write_cell(location, STONE);
    }
}
