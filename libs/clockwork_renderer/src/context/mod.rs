use crate::{
    adapters::{ClockworkAdapters, context::ContextAdapter},
    handle::UnlockedHandle,
    surface::Surface,
};

pub struct Context<A: ClockworkAdapters> {
    adapter: A::ContextAdapter,
}

impl<A: ClockworkAdapters> Context<A> {
    pub async fn new_handle_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> anyhow::Result<(UnlockedHandle<Self>, UnlockedHandle<Surface<A>>)> {
        let (adapter, surface_adapter) = A::ContextAdapter::new_with_surface(surface_target).await?;
        let context_handle = UnlockedHandle::new(Self { adapter });

        let surface = Surface::new(context_handle.clone(), surface_adapter);
        let surface_handle = UnlockedHandle::new(surface);

        Ok((context_handle, surface_handle))
    }
}
