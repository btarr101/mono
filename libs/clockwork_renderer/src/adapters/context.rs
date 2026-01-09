use crate::adapters::{ClockworkAdapter, ClockworkAdapters, surface::SurfaceAdapter};

pub trait ContextAdapter: ClockworkAdapter + Sized {
    type SurfaceAdapter: SurfaceAdapter<Adapters = Self::Adapters> = <Self::Adapters as ClockworkAdapters>::SurfaceAdapter;

    fn new_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> impl Future<Output = anyhow::Result<(Self, Self::SurfaceAdapter)>>;
}
