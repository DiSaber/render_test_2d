use bytemuck::{Pod, Zeroable};
use glam::Mat4;

/// Holds uniform data that will be passed to the shader.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable, Default)]
pub struct Uniforms {
    pub camera_view: [f32; 16],
    pub camera_projection: [f32; 16],
}

impl Uniforms {
    /// Creates a new `Uniforms` with the camera view and projection.
    pub fn new(camera_view: Mat4, camera_projection: Mat4) -> Self {
        Self {
            camera_view: camera_view.inverse().to_cols_array(),
            camera_projection: camera_projection.to_cols_array(),
        }
    }
}
