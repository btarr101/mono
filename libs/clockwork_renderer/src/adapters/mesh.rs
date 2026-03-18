use crate::{
    adapters::{ClockworkRendererAdapter, ClockworkRendererAdapters},
    mesh::{Index, Vertex},
};

pub trait MeshAdapter: ClockworkRendererAdapter {
    fn new(
        context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter,
        vertices: &[Vertex],
        indices: Option<&[Index]>,
    ) -> Self;
}
