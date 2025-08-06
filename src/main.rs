use std::time::Duration;

use render::{
    glam::{self, Mat4, Quat, Vec3},
    prelude::*,
    wgpu,
    winit::window::WindowAttributes,
};

const VERTICAL_SCALE: f32 = 5.0;

fn main() {
    let mut first_run = false;
    let before_render = |_delta_time: Duration, render_pipeline: &mut RenderPipeline| {
        let window_size = render_pipeline.get_window_size();
        let aspect_ratio = window_size.width as f32 / window_size.height as f32;
        let uniforms = Uniforms::new(glam::Mat4::orthographic_rh(
            -aspect_ratio * VERTICAL_SCALE * 0.5,
            aspect_ratio * VERTICAL_SCALE * 0.5,
            -VERTICAL_SCALE * 0.5,
            VERTICAL_SCALE * 0.5,
            -10.0,
            10.0,
        ));

        let mut instances = None;
        let mut textures = None;
        if !first_run {
            let size = 1;
            let texture_extent = wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            };
            let texture1 = render_pipeline.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let texture_view1 = texture1.create_view(&wgpu::TextureViewDescriptor::default());
            render_pipeline.write_texture(
                texture1.as_image_copy(),
                &[255, 0, 0, 255],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: None,
                },
                texture_extent,
            );
            let texture2 = render_pipeline.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let texture_view2 = texture2.create_view(&wgpu::TextureViewDescriptor::default());
            render_pipeline.write_texture(
                texture2.as_image_copy(),
                &[0, 0, 255, 255],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: None,
                },
                texture_extent,
            );
            let sampler = render_pipeline.create_sampler(&wgpu::SamplerDescriptor::default());

            instances = Some(vec![
                Instance::new(Mat4::IDENTITY, 0, 0),
                Instance::new(
                    Mat4::from_scale_rotation_translation(
                        Vec3::splat(2.0),
                        Quat::IDENTITY,
                        Vec3::new(1.0, 0.0, -1.0),
                    ),
                    1,
                    0,
                ),
            ]);
            textures = Some((vec![texture_view1, texture_view2], vec![sampler]));
        }

        first_run = true;

        UpdateRenderState {
            uniforms: Some(uniforms),
            instances,
            textures,
        }
    };

    let mut render_app = RenderApp::new(before_render, |_| {}).with_window_attributes(
        WindowAttributes::default()
            .with_title("Render Test 2d")
            .with_resizable(true),
    );

    render_app.run_app().unwrap();
}
