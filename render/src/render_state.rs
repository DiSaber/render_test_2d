use crate::{instance::Instance, uniforms::Uniforms};
use wgpu::util::DeviceExt;

pub(crate) struct RenderState {
    uniform_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_count: usize,
}

impl RenderState {
    pub fn new(device: &wgpu::Device, uniforms: Uniforms, instances: Vec<Instance>) -> Self {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            uniform_buffer,
            instance_buffer,
            instance_count: instances.len(),
        }
    }

    pub fn update_render_state(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        update_render_state: UpdateRenderState,
    ) {
        if let Some(uniforms) = update_render_state.uniforms {
            queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        }

        if let Some(instances) = update_render_state.instances {
            if instances.len() > self.instance_count {
                self.instance_buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Instance Buffer"),
                        contents: bytemuck::cast_slice(&instances),
                        usage: wgpu::BufferUsages::VERTEX,
                    });
            } else {
                queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
            }

            self.instance_count = instances.len()
        }
    }

    pub fn get_uniform_buffer(&self) -> &wgpu::Buffer {
        &self.uniform_buffer
    }

    pub fn get_instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn get_instance_count(&self) -> usize {
        self.instance_count
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct UpdateRenderState {
    pub uniforms: Option<Uniforms>,
    pub instances: Option<Vec<Instance>>,
}
