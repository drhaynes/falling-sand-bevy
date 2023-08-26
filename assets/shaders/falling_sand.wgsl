@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    var colour = vec4<f32>(0.2, 0.8, 0.2, 1.0);

    if(location.x > 500) {
        colour = vec4<f32>(0.2, 0.2, 0.8, 1.0);
    }

    textureStore(texture, location, colour);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {}
