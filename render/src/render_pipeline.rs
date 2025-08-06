use std::sync::Arc;

use glam::{Vec2, Vec3};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    render_state::{
        MAX_BINDING_ARRAY_SAMPLERS, MAX_BINDING_ARRAY_TEXTURES, RenderState, UpdateRenderState,
    },
    vertex::Vertex,
};

const QUAD_VERTICES: [Vertex; 4] = [
    Vertex::new(Vec3::new(0.5, 0.5, 0.0), Vec2::new(1.0, 1.0)),
    Vertex::new(Vec3::new(-0.5, 0.5, 0.0), Vec2::new(0.0, 1.0)),
    Vertex::new(Vec3::new(0.5, -0.5, 0.0), Vec2::new(1.0, 0.0)),
    Vertex::new(Vec3::new(-0.5, -0.5, 0.0), Vec2::new(0.0, 0.0)),
];
const QUAD_INDICES: [u16; 6] = [0, 3, 2, 3, 0, 1];

/// Holds the necessary state to draw frames to a window.
pub struct RenderPipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    quad_vertex_buffer: wgpu::Buffer,
    quad_index_buffer: wgpu::Buffer,
    depth_texture: wgpu::TextureView,
    pipeline: wgpu::RenderPipeline,
    render_state: RenderState,
}

impl RenderPipeline {
    /// Creates a new render pipeline that renders to the specified window.
    pub(crate) async fn new(window: Arc<Window>) -> Option<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok()?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::TEXTURE_BINDING_ARRAY
                    | wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                    | wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY,
                required_limits: wgpu::Limits {
                    max_binding_array_elements_per_shader_stage: MAX_BINDING_ARRAY_TEXTURES.get()
                        + MAX_BINDING_ARRAY_SAMPLERS.get(),
                    max_binding_array_sampler_elements_per_shader_stage: MAX_BINDING_ARRAY_SAMPLERS
                        .get(),
                    ..wgpu::Limits::defaults()
                },
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
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &render_state.get_bind_group_layouts(),
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("main_shader.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[Vertex::layout()],
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

    /// Call this whenever the window size changes to update the surface and internal textures.
    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);

        self.surface.configure(&self.device, &self.surface_config);

        self.depth_texture = Self::create_depth_texture(&self.device, &self.surface_config)
    }

    /// Uses the current `RenderState` to draw a frame to the window.
    pub(crate) fn render(&mut self, update_render_state: UpdateRenderState) {
        let inner_size = self.window.inner_size();
        if inner_size != PhysicalSize::new(self.surface_config.width, self.surface_config.height) {
            self.resize(inner_size);
        }

        self.render_state
            .update_render_state(&self.device, &self.queue, update_render_state);

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
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            let instance_count = self.render_state.get_instance_count();
            if instance_count > 0 {
                render_pass.set_pipeline(&self.pipeline);
                for (i, bind_group) in self.render_state.get_bind_groups().into_iter().enumerate() {
                    render_pass.set_bind_group(i as u32, bind_group, &[]);
                }

                render_pass
                    .set_index_buffer(self.quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));

                render_pass.draw_indexed(0..QUAD_INDICES.len() as u32, 0, 0..instance_count as u32);
            }
        }

        self.queue.submit(Some(encoder.finish()));

        self.window.pre_present_notify();
        surface_texture.present();
    }

    /// Gets the window `inner_size()`
    pub fn get_window_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    /// Wrapper around `wgpu::Device::create_texture()`
    pub fn create_texture(&self, texture_descriptor: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.device.create_texture(texture_descriptor)
    }

    /// Wrapper around `wgpu::Queue::write_texture()`
    pub fn write_texture(
        &self,
        texture: wgpu::TexelCopyTextureInfoBase<&wgpu::Texture>,
        data: &[u8],
        data_layout: wgpu::TexelCopyBufferLayout,
        size: wgpu::Extent3d,
    ) {
        self.queue.write_texture(texture, data, data_layout, size);
    }

    /// Wrapper around `wgpu::Device::create_sampler()`
    pub fn create_sampler(&self, sampler_descriptor: &wgpu::SamplerDescriptor) -> wgpu::Sampler {
        self.device.create_sampler(sampler_descriptor)
    }
}
