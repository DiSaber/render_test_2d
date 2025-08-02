use crate::instance::Instance;
use wgpu::util::DeviceExt;

pub(crate) struct RenderState {
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

impl RenderState {
    pub(crate) fn new(device: &wgpu::Device, instances: Vec<Instance>) -> Self {
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&instances),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            instances,
            instance_buffer,
        }
    }

    pub(crate) fn get_instances(&self) -> &[Instance] {
        &self.instances
    }

    pub(crate) fn get_instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }
}
