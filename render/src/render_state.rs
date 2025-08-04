use crate::{array_buffer::ArrayBuffer, instance::Instance, uniforms::Uniforms};
use wgpu::util::DeviceExt;

pub(crate) struct RenderState {
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    uniform_bind_group: wgpu::BindGroup,
    instance_buffer: ArrayBuffer<Instance>,
}

impl RenderState {
    pub(crate) fn new(device: &wgpu::Device, uniforms: Uniforms, instances: Vec<Instance>) -> Self {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let instance_buffer = ArrayBuffer::new(
            device,
            Some("Instance Buffer"),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            &instances,
        );

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
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
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
            instance_buffer,
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
            self.instance_buffer.update_data(device, queue, &instances);
        }
    }

    pub(crate) fn get_bind_group_layouts(&self) -> [&wgpu::BindGroupLayout; 1] {
        [&self.uniform_bind_group_layout]
    }

    pub(crate) fn get_bind_groups(&self) -> [&wgpu::BindGroup; 1] {
        [&self.uniform_bind_group]
    }

    pub(crate) fn get_instance_buffer(&self) -> &ArrayBuffer<Instance> {
        &self.instance_buffer
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct UpdateRenderState {
    pub uniforms: Option<Uniforms>,
    pub instances: Option<Vec<Instance>>,
}
