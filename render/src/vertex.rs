use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub(crate) struct GpuVertex {
    pub(crate) position: Vec3,
    _p0: u32,
    pub(crate) texcoord: Vec2,
}

impl GpuVertex {
    pub(crate) const fn new(position: Vec3, texcoord: Vec2) -> Self {
        Self {
            position,
            texcoord,
            _p0: 0,
        }
    }
}
