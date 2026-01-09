use crate::adapters::{ClockworkAdapter, ClockworkAdapters, context::ContextAdapter};

pub trait SurfaceAdapter: ClockworkAdapter + Sized {
    type ContextAdapter: ContextAdapter<Adapters = Self::Adapters> = <Self::Adapters as ClockworkAdapters>::ContextAdapter;

    fn new_from_target(
        context: &Self::ContextAdapter,
        target: wgpu::SurfaceTarget<'static>,
        dimensions: glam::UVec2,
    ) -> anyhow::Result<Self>;

    fn resize(&self, context: &Self::ContextAdapter, dimensions: glam::UVec2);

    fn dimensions(&self) -> glam::UVec2;
}
