use bytemuck::{Pod, Zeroable};
use glam::Mat4;

/// Holds instance data that will be passed to the shader.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct Instance {
    /// Transposed affine matrix (last row is 0,0,0,1)
    pub transform: [[f32; 4]; 3],
    /// The texture to use when drawing this instance.
    pub texture_index: u32,
    /// The sampler to use when sampling the texture.
    pub sampler_index: u32,
    _padding: [u32; 2],
}

impl Instance {
    /// Creates a new `Instance` with the provided options. (The transformation matrix will be
    /// packed to save space)
    pub fn new(transform: Mat4, texture_index: u32, sampler_index: u32) -> Self {
        Self {
            transform: pack_transform(transform),
            texture_index,
            sampler_index,
            ..Default::default()
        }
    }
}

fn pack_transform(mut transform: Mat4) -> [[f32; 4]; 3] {
    transform = transform.transpose();

    [
        transform.x_axis.to_array(),
        transform.y_axis.to_array(),
        transform.z_axis.to_array(),
    ]
}
