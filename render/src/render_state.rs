use crate::{array_buffer::ArrayBuffer, instance::Instance, uniforms::Uniforms};

pub(crate) struct RenderState {
    uniform_buffer: ArrayBuffer<Uniforms>,
    instance_buffer: ArrayBuffer<Instance>,
}

impl RenderState {
    pub fn new(device: &wgpu::Device, uniforms: Uniforms, instances: Vec<Instance>) -> Self {
        Self {
            uniform_buffer: ArrayBuffer::new(
                device,
                Some("Uniform Buffer"),
                wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                &[uniforms],
            ),
            instance_buffer: ArrayBuffer::new(
                device,
                Some("Instance Buffer"),
                wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                &instances,
            ),
        }
    }

    pub fn update_render_state(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        update_render_state: UpdateRenderState,
    ) {
        if let Some(uniforms) = update_render_state.uniforms {
            self.uniform_buffer.update_data(device, queue, &[uniforms]);
        }

        if let Some(instances) = update_render_state.instances {
            self.instance_buffer.update_data(device, queue, &instances);
        }
    }

    pub fn get_uniform_buffer(&self) -> &ArrayBuffer<Uniforms> {
        &self.uniform_buffer
    }

    pub fn get_instance_buffer(&self) -> &ArrayBuffer<Instance> {
        &self.instance_buffer
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct UpdateRenderState {
    pub uniforms: Option<Uniforms>,
    pub instances: Option<Vec<Instance>>,
}
