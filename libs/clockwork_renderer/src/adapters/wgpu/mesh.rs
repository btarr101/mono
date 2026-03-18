use crate::{
    adapters::{
        ClockworkRendererAdapter,
        mesh::MeshAdapter,
        wgpu::{WgpuAdapters, buffer::Buffer},
    },
    mesh::{Index, Vertex},
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::NoUninit, Debug)]
pub struct WgpuVertex {
    pub position: [f32; 3],
    pub texture_coordinates: [f32; 2],
}

type WgpuIndex = Index;

pub enum WgpuMeshAdapter {
    Empty,
    NonEmpty {
        vertex_buffer: Buffer<WgpuVertex>,
        // If None, this mesh does not use an index buffer
        // If Some(None), this mesh no indices
        index_buffer: Option<Option<Buffer<WgpuIndex>>>,
    },
}

impl ClockworkRendererAdapter for WgpuMeshAdapter {
    type Adapters = WgpuAdapters;
}

impl MeshAdapter for WgpuMeshAdapter {
    fn new(
        context: &<Self::Adapters as crate::adapters::ClockworkRendererAdapters>::ContextAdapter,
        vertices: &[Vertex],
        indices: Option<&[Index]>,
    ) -> Self {
        let wgpu_vertices = vertices
            .iter()
            .map(
                |Vertex {
                     position,
                     texture_coordinates,
                 }| WgpuVertex {
                    position: position.to_array(),
                    texture_coordinates: texture_coordinates.to_array(),
                },
            )
            .collect::<Vec<_>>();

        let vertex_buffer = wgpu_vertices
            .as_slice()
            .try_into()
            .ok()
            .map(|data| Buffer::new_with_data(&context.device, wgpu::BufferUsages::VERTEX, data));

        match vertex_buffer {
            None => Self::Empty,
            Some(vertex_buffer) => Self::NonEmpty {
                vertex_buffer,
                index_buffer: indices.map(|indices| {
                    indices
                        .try_into()
                        .ok()
                        .map(|data| Buffer::new_with_data(&context.device, wgpu::BufferUsages::INDEX, data))
                }),
            },
        }
    }
}
