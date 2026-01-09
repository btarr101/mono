use crate::{
    adapters::{ClockworkAdapters, context::ContextAdapter},
    handle::UnlockedHandle,
    surface::Surface,
};

pub struct Context<A: ContextAdapter> {
    adapter: A,
}

impl<A: ContextAdapter> Context<A> {
    pub async fn new_handle_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> anyhow::Result<(
        UnlockedHandle<Self>,
        UnlockedHandle<Surface<<A::Adapters as ClockworkAdapters>::SurfaceAdapter>>,
    )> {
        let (context_adapter, surface_adapter) = A::new_with_surface(surface_target).await?;
        let context = UnlockedHandle::new(context_adapter);

        let surface = Surface::new(context, surface_adapter);

        todo!()
    }
}
