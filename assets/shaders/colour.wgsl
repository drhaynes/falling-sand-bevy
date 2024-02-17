#import "shaders/core.wgsl"

@group(0) @binding(0)
var<uniform> size: vec2<u32>;
@group(0) @binding(1)
var<storage, read_write> simulation_destination: array<Cell>;
@group(0) @binding(2)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8, 8, 1)
fn colour(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let cell = simulation_destination[index_of(location, size.x)];
    let colour = colour_for_particle_type(cell.particle_type);
    textureStore(texture, location, colour);
}
