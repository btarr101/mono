use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters, texture::TextureAdapter},
    context::Context,
    handle::UnlockedHandle,
};
pub struct Texture<A: ClockworkRendererAdapters = DefaultAdapters> {
    context: UnlockedHandle<Context<A>>,
    adapter: A::TextureAdapter,
}

impl<A: ClockworkRendererAdapters> Texture<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, data: &[u8], dimensions: glam::UVec2) -> Self {
        let adapter = A::TextureAdapter::new(&context.adapter, data, dimensions);

        Self { context, adapter }
    }
}
