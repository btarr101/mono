use std::cell::Cell;

use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters, context::ContextAdapter},
    handle::UnlockedHandle,
    mesh::{Mesh, MeshData},
    surface::Surface,
    texture::Texture,
};

pub struct Context<A: ClockworkRendererAdapters = DefaultAdapters> {
    pub(crate) adapter: A::ContextAdapter,
    next_index: Cell<usize>,
}

impl<A: ClockworkRendererAdapters> Context<A> {
    pub async fn new_handle_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> anyhow::Result<(UnlockedHandle<Self>, UnlockedHandle<Surface<A>>)> {
        let (adapter, surface_adapter) = A::ContextAdapter::new_with_surface(surface_target).await?;
        let context_handle = UnlockedHandle::new(Self {
            adapter,
            next_index: Cell::new(0),
        });

        let surface = Surface::new(context_handle.clone(), surface_adapter);
        let surface_handle = UnlockedHandle::new(surface);

        Ok((context_handle, surface_handle))
    }

    pub(crate) fn generate_index(&self) -> usize {
        let index = self.next_index.get();
        self.next_index.set(index + 1);
        index
    }
}

pub trait ContextHandleExt<A: ClockworkRendererAdapters> {
    fn create_mesh(&self, data: &MeshData) -> UnlockedHandle<Mesh<A>>;
    fn create_texture(&self, data: &[u8], dimensions: glam::UVec2) -> UnlockedHandle<Texture<A>>;
}

impl<A: ClockworkRendererAdapters> ContextHandleExt<A> for UnlockedHandle<Context<A>> {
    fn create_mesh(&self, data: &MeshData) -> UnlockedHandle<Mesh<A>> {
        let mesh = Mesh::new(self.clone(), data);
        UnlockedHandle::new(mesh)
    }

    fn create_texture(&self, data: &[u8], dimensions: glam::UVec2) -> UnlockedHandle<Texture<A>> {
        let texture = Texture::new(self.clone(), data, dimensions);
        UnlockedHandle::new(texture)
    }
}
