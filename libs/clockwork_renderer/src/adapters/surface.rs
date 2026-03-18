use crate::adapters::{ClockworkRendererAdapter, ClockworkRendererAdapters};

pub trait SurfaceAdapter: ClockworkRendererAdapter + Sized {
    fn new_from_target(
        context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter,
        target: wgpu::SurfaceTarget<'static>,
        dimensions: glam::UVec2,
    ) -> anyhow::Result<Self>;

    fn resize(&self, context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter, dimensions: glam::UVec2);

    fn dimensions(&self) -> glam::UVec2;
}
