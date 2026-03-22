use crate::{
    adapters::{ClockworkRendererAdapter, ClockworkRendererAdapters},
    draw_params::DrawParams,
    handle::Handle,
    mesh::Mesh,
    texture::Texture,
};

pub trait ContextAdapter: ClockworkRendererAdapter + Sized {
    fn new_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> impl Future<Output = anyhow::Result<(Self, <Self::Adapters as ClockworkRendererAdapters>::SurfaceAdapter)>>;

    fn draw(&self, texture: Handle<'static, Texture>, mesh: Handle<'static, Mesh>, params: &DrawParams);

    fn clear(&self);
}
