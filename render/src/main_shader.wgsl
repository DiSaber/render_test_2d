@group(0)
@binding(0)
var<uniform> view: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
}

struct InstanceInput {
    @location(2) mat_0: vec4<f32>,
    @location(3) mat_1: vec4<f32>,
    @location(4) mat_2: vec4<f32>,
    @location(5) mat_3: vec4<f32>,
};

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let instance_matrix = mat4x4<f32>(
        instance.mat_0,
        instance.mat_1,
        instance.mat_2,
        instance.mat_3
    );

    var result: VertexOutput;
    result.tex_coord = vertex.tex_coord;
    result.position = view * instance_matrix * vec4<f32>(vertex.position, 1.0);

    return result;
}

@group(0)
@binding(1)
var r_color: texture_2d<u32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureLoad(r_color, vec2<i32>(vertex.tex_coord * 256.0), 0);
    let v = f32(tex.x) / 255.0;

    return vec4<f32>(1.0 - (v * 5.0), 1.0 - (v * 15.0), 1.0 - (v * 50.0), 1.0);
}
