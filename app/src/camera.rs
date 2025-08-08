use bevy_ecs::prelude::*;
use bevy_transform::components::Transform;
use render::wgpu;

/// A world-space camera. (Currently only one camera is supported)
#[derive(Clone, Copy, Component)]
#[require(Transform)]
pub struct Camera {
    pub vertical_scale: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub clear_color: wgpu::Color,
}
