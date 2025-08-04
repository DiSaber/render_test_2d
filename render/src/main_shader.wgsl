@group(0) @binding(0)
var<uniform> view: mat4x4<f32>;

struct Instance {
    transform: mat3x4<f32>,
    texture_index: u32,
    sampler_index: u32,
};

@group(1) @binding(0)
var<storage, read> instances: array<Instance>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) @interpolate(flat) instance_index: u32,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    let instance = instances[instance_index];
    let instance_matrix = transpose(mat4x4<f32>(
        instance.transform[0],
        instance.transform[1],
        instance.transform[2],
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    ));

    var result: VertexOutput;
    result.position = view * instance_matrix * vec4<f32>(vertex.position, 1.0);
    result.tex_coord = vertex.tex_coord;
    result.instance_index = instance_index;

    return result;
}

@group(2) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(2) @binding(1)
var sampler_array: binding_array<sampler>;

struct FragmentInput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) @interpolate(flat) instance_index: u32,
}

@fragment
fn fs_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    let instance = instances[fragment.instance_index];
    let out = textureSample(
        texture_array[instance.texture_index],
        sampler_array[instance.sampler_index],
        fragment.tex_coord,
    );

    if out.w < 0.5 { discard; } // Discard pixel if the texture alpha is transparent

    return out;
}
