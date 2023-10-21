const SAND_COLOUR = vec4<f32>(0.8, 0.8, 0.0, 1.0);

struct PushConstants {
    drawing_start: vec2<f32>,
    drawing_end: vec2<f32>,
    brush_radius: f32,
}
var<push_constant> drawing_constants: PushConstants;

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8, 8, 1)
fn draw(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let pixel = vec2<u32>(invocation_id.xy);
    let size = vec2<u32>(textureDimensions(texture));
    if (pixel.x >= size.x && pixel.y >= size.y) {
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
            textureStore(texture, vec2<i32>(current_pixel), SAND_COLOUR);
        }
    }
}
