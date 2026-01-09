use crate::{
    adapters::{ClockworkAdapters, context::ContextAdapter, surface::SurfaceAdapter},
    context::Context,
    handle::UnlockedHandle,
};

pub struct Surface<A: SurfaceAdapter> {
    context: UnlockedHandle<Context<<A::Adapters as ClockworkAdapters>::ContextAdapter>>,
    adapter: A,
}

impl<A: SurfaceAdapter> Surface<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<<A::Adapters as ClockworkAdapters>::ContextAdapter>>, adapter: A) -> Self {
        Self { context, adapter }
    }
}
