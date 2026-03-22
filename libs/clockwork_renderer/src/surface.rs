use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters, surface::SurfaceAdapter},
    context::Context,
    handle::UnlockedHandle,
};

pub struct Surface<A: ClockworkRendererAdapters = DefaultAdapters> {
    context: UnlockedHandle<Context<A>>,
    pub(crate) adapter: A::SurfaceAdapter,
}

impl<A: ClockworkRendererAdapters> Surface<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, adapter: A::SurfaceAdapter) -> Self { Self { context, adapter } }
}

impl<A: ClockworkRendererAdapters> Surface<A> {
    pub fn resize(&self, dimensions: glam::UVec2) { self.adapter.resize(&self.context.adapter, dimensions); }
    pub fn dimensions(&self) -> glam::UVec2 { self.adapter.dimensions() }
}
