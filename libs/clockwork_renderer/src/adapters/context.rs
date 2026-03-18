use crate::adapters::{ClockworkRendererAdapter, ClockworkRendererAdapters};

pub trait ContextAdapter: ClockworkRendererAdapter + Sized {
    fn new_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> impl Future<Output = anyhow::Result<(Self, <Self::Adapters as ClockworkRendererAdapters>::SurfaceAdapter)>>;
}
