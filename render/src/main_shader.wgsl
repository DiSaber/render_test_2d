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
    @location(5) texture_index: u32,
    @location(6) sampler_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) @interpolate(flat) texture_index: u32,
    @location(2) @interpolate(flat) sampler_index: u32,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let instance_matrix = transpose(mat4x4<f32>(
        instance.mat_0,
        instance.mat_1,
        instance.mat_2,
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));

    var result: VertexOutput;
    result.position = view * instance_matrix * vec4<f32>(vertex.position, 1.0);
    result.tex_coord = vertex.tex_coord;
    result.texture_index = instance.texture_index;
    result.sampler_index = instance.sampler_index;

    return result;
}

@group(1) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(1) @binding(1)
var sampler_array: binding_array<sampler>;

struct FragmentInput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) @interpolate(flat) texture_index: u32,
    @location(2) @interpolate(flat) sampler_index: u32,
}

@fragment
fn fs_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    let tex_coord_dpdx = dpdx(fragment.tex_coord);
    let tex_coord_dpdy = dpdy(fragment.tex_coord);

    let out = textureSampleGrad(
        texture_array[fragment.texture_index],
        sampler_array[fragment.sampler_index],
        fragment.tex_coord,
        tex_coord_dpdx,
        tex_coord_dpdy,
    );

    if out.w < 0.5 { discard; } // Discard pixel if the texture alpha is transparent

    return out;
}
