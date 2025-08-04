use std::num::NonZeroU32;

use crate::{array_buffer::ArrayBuffer, instance::Instance, uniforms::Uniforms};
use wgpu::util::DeviceExt;

pub(crate) const MAX_BINDING_ARRAY_TEXTURES: NonZeroU32 = NonZeroU32::new(100).unwrap();
pub(crate) const MAX_BINDING_ARRAY_SAMPLERS: NonZeroU32 = NonZeroU32::new(10).unwrap();

pub(crate) struct RenderState {
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group: wgpu::BindGroup,
    instance_buffer: ArrayBuffer<Instance>,
    instance_bind_group_layout: wgpu::BindGroupLayout,
    instance_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group: wgpu::BindGroup,
}

impl RenderState {
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
        let instance_buffer = ArrayBuffer::new(
            device,
            Some("Instance Buffer"),
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            instances,
        );

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
                resource: instance_buffer.get_buffer().as_entire_binding(),
            }],
        });

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
                        &textures.iter().collect::<Vec<_>>(),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::SamplerArray(
                        &samplers.iter().collect::<Vec<_>>(),
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
        }
    }

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
            if self.instance_buffer.update_data(device, queue, &instances) {
                // Buffer was resized, remake the bind group
                self.instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Instance Bind Group"),
                    layout: &self.instance_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.instance_buffer.get_buffer().as_entire_binding(),
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
                            &textures.iter().collect::<Vec<_>>(),
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::SamplerArray(
                            &samplers.iter().collect::<Vec<_>>(),
                        ),
                    },
                ],
                layout: &self.texture_bind_group_layout,
            });
        }
    }

    pub(crate) fn get_bind_group_layouts(&self) -> [&wgpu::BindGroupLayout; 3] {
        [
            &self.uniform_bind_group_layout,
            &self.instance_bind_group_layout,
            &self.texture_bind_group_layout,
        ]
    }

    pub(crate) fn get_bind_groups(&self) -> [&wgpu::BindGroup; 3] {
        [
            &self.uniform_bind_group,
            &self.instance_bind_group,
            &self.texture_bind_group,
        ]
    }

    pub(crate) fn get_instance_buffer(&self) -> &ArrayBuffer<Instance> {
        &self.instance_buffer
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct UpdateRenderState {
    pub uniforms: Option<Uniforms>,
    pub instances: Option<Vec<Instance>>,
    pub textures: Option<(Vec<wgpu::TextureView>, Vec<wgpu::Sampler>)>,
}
