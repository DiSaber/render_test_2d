use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub(crate) struct Instance {
    /// Transposed affine matrix (last row is 0,0,0,1)
    pub transform: [[f32; 4]; 3],
}

impl Instance {
    pub(crate) fn new(transform: Mat4) -> Self {
        Self {
            transform: pack_transform(transform),
        }
    }

    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![2 => Float32x4, 3 => Float32x4, 4 => Float32x4];

    pub(crate) const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
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
