use crate::adapters::{ClockworkAdapter, ClockworkAdapters};

pub trait ContextAdapter: ClockworkAdapter + Sized {
    fn new_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> impl Future<Output = anyhow::Result<(Self, <Self::Adapters as ClockworkAdapters>::SurfaceAdapter)>>;
}
