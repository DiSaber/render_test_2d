use std::num::NonZeroU32;

use crate::{array_buffer::ArrayBuffer, instance::Instance, uniforms::Uniforms};
use wgpu::util::DeviceExt;

/// The maximum amount of textures allowed in the texture bind group.
pub(crate) const MAX_BINDING_ARRAY_TEXTURES: NonZeroU32 = NonZeroU32::new(100).unwrap();
/// The maximum amount of samplers allowed in the texture bind group.
pub(crate) const MAX_BINDING_ARRAY_SAMPLERS: NonZeroU32 = NonZeroU32::new(10).unwrap();

/// Manages the buffers and bind groups for a `RenderPipeline`.
pub(crate) struct RenderState {
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group: wgpu::BindGroup,
    // --- //
    instance_buffer: ArrayBuffer<Instance>,
    instance_bind_group_layout: wgpu::BindGroupLayout,
    instance_bind_group: wgpu::BindGroup,
    // --- //
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group: wgpu::BindGroup,
    // https://github.com/gfx-rs/wgpu/issues/3692
    dummy_instance: ArrayBuffer<Instance>,
    dummy_texture: wgpu::TextureView,
    dummy_sampler: wgpu::Sampler,
}

impl RenderState {
    /// Creates a new `RenderState` and initializes the buffers and bind groups with the provided
    /// data.
    pub(crate) fn new(
        device: &wgpu::Device,
        uniforms: Uniforms,
        instances: &[Instance],
        textures: &[wgpu::TextureView],
        samplers: &[wgpu::Sampler],
    ) -> Self {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
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
                }],
            });
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let instance_buffer = ArrayBuffer::new(
            device,
            Some("Instance Buffer"),
            instances,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        );
        let dummy_instance = ArrayBuffer::new(
            device,
            None,
            &[Instance::default()],
            wgpu::BufferUsages::STORAGE,
        );
        let instance_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Instance Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Instance Bind Group"),
            layout: &instance_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                // https://github.com/gfx-rs/wgpu/issues/3692
                resource: if instance_buffer.len() == 0 {
                    dummy_instance.get_buffer().as_entire_binding()
                } else {
                    instance_buffer.get_buffer().as_entire_binding()
                },
            }],
        });

        let dummy_texture = device
            .create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&wgpu::TextureViewDescriptor::default());
        let dummy_sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: Some(MAX_BINDING_ARRAY_TEXTURES),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: Some(MAX_BINDING_ARRAY_SAMPLERS),
                    },
                ],
            });
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(
                        // https://github.com/gfx-rs/wgpu/issues/3692
                        &(if textures.is_empty() {
                            vec![&dummy_texture]
                        } else {
                            textures.iter().collect()
                        }),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::SamplerArray(
                        // https://github.com/gfx-rs/wgpu/issues/3692
                        &(if samplers.is_empty() {
                            vec![&dummy_sampler]
                        } else {
                            samplers.iter().collect()
                        }),
                    ),
                },
            ],
            layout: &texture_bind_group_layout,
        });

        Self {
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
            instance_buffer,
            instance_bind_group_layout,
            instance_bind_group,
            texture_bind_group_layout,
            texture_bind_group,
            dummy_instance,
            dummy_texture,
            dummy_sampler,
        }
    }

    /// Updates the buffers and bind groups with the provided data.
    pub(crate) fn update_render_state(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        update_render_state: UpdateRenderState,
    ) {
        if let Some(uniforms) = update_render_state.uniforms {
            queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }

        if let Some(instances) = update_render_state.instances {
            if self.instance_buffer.write_buffer(device, queue, &instances) {
                // Buffer was resized, remake the bind group
                self.instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Instance Bind Group"),
                    layout: &self.instance_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        // https://github.com/gfx-rs/wgpu/issues/3692
                        resource: if self.instance_buffer.len() == 0 {
                            self.dummy_instance.get_buffer().as_entire_binding()
                        } else {
                            self.instance_buffer.get_buffer().as_entire_binding()
                        },
                    }],
                });
            }
        }

        if let Some((textures, samplers)) = update_render_state.textures {
            self.texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Texture Bind Group"),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureViewArray(
                            // https://github.com/gfx-rs/wgpu/issues/3692
                            &(if textures.is_empty() {
                                vec![&self.dummy_texture]
                            } else {
                                textures.iter().collect()
                            }),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::SamplerArray(
                            // https://github.com/gfx-rs/wgpu/issues/3692
                            &(if samplers.is_empty() {
                                vec![&self.dummy_sampler]
                            } else {
                                samplers.iter().collect()
                            }),
                        ),
                    },
                ],
                layout: &self.texture_bind_group_layout,
            });
        }
    }

    /// Gets the bind group layouts in order.
    pub(crate) fn get_bind_group_layouts(&self) -> [&wgpu::BindGroupLayout; 3] {
        [
            &self.uniform_bind_group_layout,
            &self.instance_bind_group_layout,
            &self.texture_bind_group_layout,
        ]
    }

    /// Gets the bind groups in order.
    pub(crate) fn get_bind_groups(&self) -> [&wgpu::BindGroup; 3] {
        [
            &self.uniform_bind_group,
            &self.instance_bind_group,
            &self.texture_bind_group,
        ]
    }

    /// Gets the amount of instances currently in the instance buffer.
    pub(crate) fn get_instance_count(&self) -> usize {
        self.instance_buffer.len()
    }
}

/// Used to update a `RenderState` with new data. Any `None` fields will be left untouched.
#[derive(Debug, Clone, Default)]
pub struct UpdateRenderState {
    pub uniforms: Option<Uniforms>,
    pub instances: Option<Vec<Instance>>,
    pub textures: Option<(Vec<wgpu::TextureView>, Vec<wgpu::Sampler>)>,
}
