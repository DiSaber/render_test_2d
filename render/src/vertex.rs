use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub(crate) struct GpuVertex {
    pub position: [f32; 3],
    pub texcoord: [f32; 2],
}

impl GpuVertex {
    pub const fn new(position: Vec3, texcoord: Vec2) -> Self {
        Self {
            position: position.to_array(),
            texcoord: texcoord.to_array(),
        }
    }

    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
