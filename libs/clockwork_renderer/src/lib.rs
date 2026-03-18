#![feature(associated_type_defaults)]

use crate::{context::Context, handle::UnlockedHandle, surface::Surface};

mod adapters;
pub mod context;
pub mod handle;
pub mod maybe_locked;
pub mod mesh;
pub mod surface;
pub mod texture;

pub async fn create_context_with_surface_target(
    surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
) -> anyhow::Result<(UnlockedHandle<Context>, UnlockedHandle<Surface>)> {
    let (context, surface) = Context::new_handle_with_surface(surface_target).await?;
    Ok((context, surface))
}
