use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub(crate) struct Instance {
    /// Transposed affine matrix (last row is 0,0,0,1)
    pub transform: [[f32; 4]; 3],
    pub texture_index: u32,
    pub sampler_index: u32,
    _padding: [u32; 2],
}

impl Instance {
    pub(crate) fn new(transform: Mat4, texture_index: u32, sampler_index: u32) -> Self {
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
