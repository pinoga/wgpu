@vertex
fn vs_main(
    @location(0) position: vec3<f32>
) -> @builtin(position) {
    return vec4<f32>(position, 1.0)
}

@fragment
fn fs_main(
    @builtin(@location(0) bary: vec3<f32>) -> @location(0) vec4<f32> {
        let edge = min(bary.x, min(bary.y, bary.z));
        if edge < 0.02 {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
        return vec4<f32>(1.0, 1.0, 1.0, 1.0); 
    }
)