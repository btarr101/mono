use crate::{adapters::ClockworkAdapters, context::Context, handle::UnlockedHandle};

pub struct Surface<A: ClockworkAdapters> {
    context: UnlockedHandle<Context<A>>,
    adapter: A::SurfaceAdapter,
}

impl<A: ClockworkAdapters> Surface<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, adapter: A::SurfaceAdapter) -> Self { Self { context, adapter } }
}
