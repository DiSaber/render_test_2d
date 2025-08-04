use std::sync::Arc;

use glam::{Mat4, Quat, Vec2, Vec3};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    instance::Instance,
    render_state::{RenderState, UpdateRenderState},
    uniforms::Uniforms,
    vertex::GpuVertex,
};

const QUAD_VERTICES: [GpuVertex; 4] = [
    GpuVertex::new(Vec3::new(0.5, 0.5, 0.0), Vec2::new(1.0, 1.0)),
    GpuVertex::new(Vec3::new(-0.5, 0.5, 0.0), Vec2::new(0.0, 1.0)),
    GpuVertex::new(Vec3::new(0.5, -0.5, 0.0), Vec2::new(1.0, 0.0)),
    GpuVertex::new(Vec3::new(-0.5, -0.5, 0.0), Vec2::new(0.0, 0.0)),
];
const QUAD_INDICES: [u16; 6] = [0, 3, 2, 3, 0, 1];

const VERTICAL_SCALE: f32 = 5.0;

pub(crate) struct RenderPipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    quad_vertex_buffer: wgpu::Buffer,
    quad_index_buffer: wgpu::Buffer,
    depth_texture: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    pub render_state: RenderState,
}

impl RenderPipeline {
    /// Creates a new render pipeline
    pub(crate) async fn new(window: Arc<Window>) -> Option<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok()?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::TEXTURE_BINDING_ARRAY,
                ..Default::default()
            })
            .await
            .ok()?;

        let window_size = window.inner_size();
        let width = window_size.width.max(1);
        let height = window_size.height.max(1);

        let surface = instance.create_surface(window.clone()).ok()?;
        let mut surface_config = surface.get_default_config(&adapter, width, height)?;

        surface_config
            .view_formats
            .push(surface_config.format.add_srgb_suffix());
        surface_config.present_mode = wgpu::PresentMode::AutoVsync;

        surface.configure(&device, &surface_config);

        let render_state = RenderState::new(
            &device,
            Self::create_uniforms(&surface_config),
            vec![
                Instance::new(Mat4::IDENTITY),
                Instance::new(Mat4::from_scale_rotation_translation(
                    Vec3::splat(2.0),
                    Quat::IDENTITY,
                    Vec3::new(1.0, 0.0, -1.0),
                )),
            ],
        );

        let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let quad_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let depth_texture = Self::create_depth_texture(&device, &surface_config);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<Uniforms>() as u64
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Mandelbrot set placeholder
        fn create_texels(size: usize) -> Vec<u8> {
            (0..size * size)
                .map(|id| {
                    // get high five for recognizing this ;)
                    let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
                    let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
                    let (mut x, mut y, mut count) = (cx, cy, 0);
                    while count < 0xFF && x * x + y * y < 4.0 {
                        let old_x = x;
                        x = x * x - y * y + cx;
                        y = 2.0 * old_x * y + cy;
                        count += 1;
                    }
                    count
                })
                .collect()
        }

        let size = 256u32;
        let texels = create_texels(size as usize);
        let texture_extent = wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            texture.as_image_copy(),
            &texels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(size),
                rows_per_image: None,
            },
            texture_extent,
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: render_state
                        .get_uniform_buffer()
                        .get_buffer()
                        .as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
            ],
            label: None,
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("main_shader.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[GpuVertex::layout(), Instance::layout()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(surface_config.view_formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Some(Self {
            device,
            queue,
            window,
            surface,
            surface_config,
            quad_vertex_buffer,
            quad_index_buffer,
            depth_texture,
            bind_group,
            pipeline,
            render_state,
        })
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: surface_config.width,
                height: surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    // Temporary
    fn create_uniforms(surface_config: &wgpu::SurfaceConfiguration) -> Uniforms {
        let aspect_ratio = surface_config.width as f32 / surface_config.height as f32;
        Uniforms::new(glam::Mat4::orthographic_rh(
            -aspect_ratio * VERTICAL_SCALE * 0.5,
            aspect_ratio * VERTICAL_SCALE * 0.5,
            -VERTICAL_SCALE * 0.5,
            VERTICAL_SCALE * 0.5,
            -10.0,
            10.0,
        ))
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);

        self.depth_texture = Self::create_depth_texture(&self.device, &self.surface_config)
    }

    pub(crate) fn render(&mut self) {
        // Temporary
        self.render_state.update_render_state(
            &self.device,
            &self.queue,
            UpdateRenderState {
                uniforms: Some(Self::create_uniforms(&self.surface_config)),
                ..Default::default()
            },
        );

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_config.view_formats[0]),
                ..Default::default()
            });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let instance_buffer = self.render_state.get_instance_buffer();
            if instance_buffer.len() > 0 {
                rpass.set_pipeline(&self.pipeline);
                rpass.set_bind_group(0, &self.bind_group, &[]);

                rpass.set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                rpass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
                rpass.set_vertex_buffer(1, instance_buffer.get_buffer().slice(..));

                rpass.draw_indexed(
                    0..QUAD_INDICES.len() as u32,
                    0,
                    0..instance_buffer.len() as u32,
                );
            }
        }

        self.queue.submit(Some(encoder.finish()));

        self.window.pre_present_notify();
        surface_texture.present();
    }
}
