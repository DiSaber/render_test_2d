use bytemuck::{Pod, Zeroable};
use glam::Mat4;

/// Holds uniform data that will be passed to the shader.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct Uniforms {
    pub camera_view: [f32; 16],
}

impl Uniforms {
    /// Creates a new `Uniforms` with the camera view.
    pub fn new(camera_view: Mat4) -> Self {
        Self {
            camera_view: camera_view.to_cols_array(),
        }
    }
}
