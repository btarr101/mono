use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters, mesh::MeshAdapter},
    context::Context,
    handle::UnlockedHandle,
};

pub struct Vertex {
    pub position: glam::Vec3,
    pub texture_coordinates: glam::Vec2,
}

pub type Index = u16;

#[derive(Default)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<Index>>,
}

pub struct Mesh<A: ClockworkRendererAdapters = DefaultAdapters> {
    context: UnlockedHandle<Context<A>>,
    adapter: A::MeshAdapter,
}

impl<A: ClockworkRendererAdapters> Mesh<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, data: &MeshData) -> Self {
        let adapter = A::MeshAdapter::new(&context.adapter, &data.vertices, data.indices.as_deref());

        Self { context, adapter }
    }
}
