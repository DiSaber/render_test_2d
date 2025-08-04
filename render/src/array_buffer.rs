use bytemuck::Pod;
use std::marker::PhantomData;
use wgpu::util::DeviceExt;

pub(crate) struct ArrayBuffer<T> {
    buffer: wgpu::Buffer,
    capacity: usize,
    length: usize,
    label: Option<String>,
    _marker: PhantomData<T>,
}

impl<T> ArrayBuffer<T>
where
    T: Pod,
{
    /// Creates a new buffer.
    pub(crate) fn new(
        device: &wgpu::Device,
        label: Option<&str>,
        usage: wgpu::BufferUsages,
        data: &[T],
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label,
            contents: bytemuck::cast_slice(data),
            usage,
        });

        Self {
            buffer,
            capacity: data.len(),
            length: data.len(),
            label: label.map(String::from),
            _marker: PhantomData::default(),
        }
    }

    /// Updates the buffer with new data and resizes it if the data doesn't fit.
    pub(crate) fn update_data(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T]) {
        if data.len() > self.capacity {
            self.buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: self.label.as_deref(),
                contents: bytemuck::cast_slice(data),
                usage: self.buffer.usage(),
            });
            self.capacity = data.len()
        } else {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
        }

        self.length = data.len();
    }
}

impl<T> ArrayBuffer<T> {
    pub(crate) fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub(crate) fn len(&self) -> usize {
        self.length
    }
}
