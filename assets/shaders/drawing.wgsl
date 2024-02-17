#import "shaders/core.wgsl"

var<push_constant> drawing_constants: PushConstants;
@group(0) @binding(0)
var<uniform> simulation_size: vec2<u32>;
@group(0) @binding(1)
var<storage, read_write> simulation_destination: array<Cell>;

@compute @workgroup_size(8, 8, 1)
fn draw(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let pixel = vec2<u32>(invocation_id.xy);

    if (pixel.x >= simulation_size.x && pixel.y >= simulation_size.y) {
        return;
    }

    if (drawing_constants.brush_radius > 0.0) {
        let current_pixel = vec2<f32>(pixel);
        let drawing_position = drawing_constants.drawing_end;
        draw_circle(current_pixel, drawing_position, drawing_constants.brush_radius);
    }
}

fn draw_circle(current_pixel: vec2<f32>, centre: vec2<f32>, radius: f32) {
    let y_min = centre.y - radius;
    let y_max = centre.y + radius;
    let x_min = centre.x - radius;
    let x_max = centre.x + radius;

    if (current_pixel.x >= x_min && current_pixel.x <= x_max && current_pixel.y >= y_min && current_pixel.y <= y_max) {
        let distance = length(current_pixel - centre);
        if (round(distance) <= radius) {
            simulation_destination[index_of(vec2<i32>(current_pixel), simulation_size.x)] = Cell(SAND);
        }
    }
}
