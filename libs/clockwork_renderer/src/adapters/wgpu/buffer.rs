use std::{
    cell::Cell,
    cmp::max,
    marker::PhantomData,
    num::{NonZero, NonZeroUsize},
    ops::{Deref, RangeBounds},
};

use nunny::NonEmpty;
use thiserror::Error;
use wgpu::util::DeviceExt;

pub struct Buffer<T: bytemuck::NoUninit> {
    buffer: wgpu::Buffer,
    length: Cell<usize>,
    capacity: NonZeroUsize,
    _phantom: PhantomData<T>,
}

#[derive(Error, Debug)]
pub enum BufferWriteError {
    #[error("Buffer does not have enough capacity to write")]
    NotEnoughCapacity,
}

impl<T: bytemuck::NoUninit> Deref for Buffer<T> {
    type Target = wgpu::Buffer;
    fn deref(&self) -> &Self::Target { &self.buffer }
}

impl<T: bytemuck::NoUninit> Buffer<T> {
    pub fn len(&self) -> usize { self.length.get() }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn capacity(&self) -> NonZeroUsize { self.capacity }

    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages, capacity: impl Into<NonZeroUsize>) -> Self {
        let capacity = capacity.into();

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage,
            size: (usize::from(capacity) * std::mem::size_of::<T>()) as u64,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            length: Cell::new(0),
            capacity,
            _phantom: PhantomData::<T>,
        }
    }

    pub fn new_with_data(device: &wgpu::Device, usage: wgpu::BufferUsages, data: &NonEmpty<[T]>) -> Self {
        let length = data.len_ne();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage,
        });

        Self {
            buffer,
            length: Cell::new(length.into()),
            capacity: length,
            _phantom: PhantomData::<T>,
        }
    }

    pub fn write(&self, queue: &wgpu::Queue, index_offset: usize, data: &[T]) -> Result<(), BufferWriteError> {
        let new_length = max(index_offset + data.len(), self.len());
        if new_length > self.capacity.into() {
            return Err(BufferWriteError::NotEnoughCapacity);
        }

        let offset = (std::mem::size_of::<T>() * index_offset) as u64;
        queue.write_buffer(self, offset, bytemuck::cast_slice(data));

        self.length.set(new_length);

        Ok(())
    }

    pub fn push(&self, queue: &wgpu::Queue, data: &[T]) -> Result<(), BufferWriteError> {
        let index_offset = self.len();
        self.write(queue, index_offset, data)
    }

    pub fn push_and_reallocate(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T]) {
        while self.push(queue, data).is_err() {
            let new_capacity = self
                .capacity
                .checked_mul(NonZeroUsize::new(2).expect("2 > 0"))
                .max(NonZero::new(self.len() + data.len()))
                .unwrap_or(NonZeroUsize::MAX);

            let new_buffer = Self::new(device, self.usage(), new_capacity);
            new_buffer.copy_from(device, queue, 0, self, 0..self.len()).unwrap();
            *self = new_buffer;
        }
    }

    pub fn pop(&self) { self.length.update(|length| length.saturating_sub(1)); }

    pub fn clear(&self) { self.length.set(0); }

    pub fn copy_from(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        index_offset: usize,
        source: &Buffer<T>,
        source_range: impl RangeBounds<usize>,
    ) -> Result<(), BufferWriteError> {
        let source_index_offset = match source_range.start_bound() {
            std::ops::Bound::Included(index) => *index,
            std::ops::Bound::Excluded(index) => *index + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end_index_offset = match source_range.end_bound() {
            std::ops::Bound::Included(index) => *index + 1,
            std::ops::Bound::Excluded(index) => *index,
            std::ops::Bound::Unbounded => source.len(),
        };

        let source_range_length = end_index_offset - source_index_offset;

        let new_length = max(index_offset + source_range_length, self.len());
        if new_length > self.capacity.into() {
            return Err(BufferWriteError::NotEnoughCapacity);
        }

        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_from command encoder"),
        });

        let source_offset = source_index_offset * std::mem::size_of::<T>();

        command_encoder.copy_buffer_to_buffer(
            source,
            source_offset as u64,
            self,
            (index_offset * std::mem::size_of::<T>()) as u64,
            (source_range_length * std::mem::size_of::<T>()) as u64,
        );

        queue.submit(Some(command_encoder.finish()));

        self.length.set(new_length);

        Ok(())
    }
}
