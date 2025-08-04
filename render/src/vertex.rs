use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

/// Holds vertex data that will be passed to the shader.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) texcoord: [f32; 2],
}

impl Vertex {
    /// Creates a new `Vertex` with the position and texcoord.
    pub(crate) const fn new(position: Vec3, texcoord: Vec2) -> Self {
        Self {
            position: position.to_array(),
            texcoord: texcoord.to_array(),
        }
    }

    /// The vertex attributes used by `wgpu`.
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    /// Gets the `VertexBufferLayout` of `Vertex`.
    pub(crate) const fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
