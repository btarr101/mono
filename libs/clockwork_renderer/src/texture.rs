use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters, texture::TextureAdapter},
    context::Context,
    handle::UnlockedHandle,
    id::Id,
};
pub struct Texture<A: ClockworkRendererAdapters = DefaultAdapters> {
    pub(crate) id: Id<Self>,
    context: UnlockedHandle<Context<A>>,
    pub(crate) adapter: A::TextureAdapter,
}

impl<A: ClockworkRendererAdapters> Texture<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, data: &[u8], dimensions: glam::UVec2) -> Self {
        let adapter = A::TextureAdapter::new(&context.adapter, data, dimensions);

        Self {
            id: Id::new(context.generate_index()),
            context,
            adapter,
        }
    }
}
