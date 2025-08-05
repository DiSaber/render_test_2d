use bytemuck::Pod;
use std::marker::PhantomData;
use wgpu::util::DeviceExt;

/// Manages a buffer containing an array of `T` that can be resized.
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
    /// Creates a new `ArrayBuffer<T>` with the provided data and options.
    pub(crate) fn new(
        device: &wgpu::Device,
        label: Option<&str>,
        data: &[T],
        usage: wgpu::BufferUsages,
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

    /// Writes the buffer with new data or makes a new buffer if the new data doesn't fit. Returns if a
    /// new buffer was made.
    pub(crate) fn write_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[T],
    ) -> bool {
        let resize = data.len() > self.capacity;
        if resize {
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

        resize
    }
}

impl<T> ArrayBuffer<T> {
    /// Gets the internal `wgpu::Buffer` holding the data.
    pub(crate) fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    /// Gets the buffer length.
    pub(crate) fn len(&self) -> usize {
        self.length
    }
}
