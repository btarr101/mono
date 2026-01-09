use crate::adapters::{ClockworkAdapter, ClockworkAdapters};

pub trait SurfaceAdapter: ClockworkAdapter + Sized {
    fn new_from_target(
        context: &<Self::Adapters as ClockworkAdapters>::ContextAdapter,
        target: wgpu::SurfaceTarget<'static>,
        dimensions: glam::UVec2,
    ) -> anyhow::Result<Self>;

    fn resize(&self, context: &<Self::Adapters as ClockworkAdapters>::ContextAdapter, dimensions: glam::UVec2);

    fn dimensions(&self) -> glam::UVec2;
}
