use bytemuck::{Pod, Zeroable};
use glam::Mat4;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub(crate) struct Uniforms {
    pub camera_view: [f32; 16],
}

impl Uniforms {
    pub(crate) fn new(camera_view: Mat4) -> Self {
        Self {
            camera_view: camera_view.to_cols_array(),
        }
    }
}
