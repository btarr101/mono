use crate::adapters::{
    camera::CameraAdapter, context::ContextAdapter, mesh::MeshAdapter, surface::SurfaceAdapter, texture::TextureAdapter,
    wgpu::WgpuAdapters,
};

pub mod camera;
pub mod context;
pub mod mesh;
pub mod surface;
pub mod texture;
mod wgpu;

pub trait ClockworkRendererAdapter {
    type Adapters: ClockworkRendererAdapters;
}

pub trait ClockworkRendererAdapters {
    type ContextAdapter: ContextAdapter<Adapters = Self>;
    type SurfaceAdapter: SurfaceAdapter<Adapters = Self>;
    type CameraAdapter: CameraAdapter<Adapters = Self>;
    type TextureAdapter: TextureAdapter<Adapters = Self>;
    type MeshAdapter: MeshAdapter<Adapters = Self>;
}

pub type DefaultAdapters = WgpuAdapters;
