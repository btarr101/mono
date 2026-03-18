use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters},
    context::Context,
    handle::UnlockedHandle,
};

pub struct Surface<A: ClockworkRendererAdapters = DefaultAdapters> {
    context: UnlockedHandle<Context<A>>,
    adapter: A::SurfaceAdapter,
}

impl<A: ClockworkRendererAdapters> Surface<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, adapter: A::SurfaceAdapter) -> Self { Self { context, adapter } }
}

pub trait SurfaceExt {
    fn resize(&self, dimensions: glam::UVec2);
    fn dimensions(&self) -> glam::UVec2;
}
