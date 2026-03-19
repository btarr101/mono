use crate::adapters::{
    ClockworkRendererAdapters,
    wgpu::{context::WgpuContextAdapter, mesh::WgpuMeshAdapter, surface::WgpuSurfaceAdapter, texture::WgpuTextureAdapter},
};

mod buffer;
mod context;
mod instance;
mod mesh;
mod surface;
mod texture;

pub struct WgpuAdapters;

impl ClockworkRendererAdapters for WgpuAdapters {
    type ContextAdapter = WgpuContextAdapter;
    type SurfaceAdapter = WgpuSurfaceAdapter;
    type TextureAdapter = WgpuTextureAdapter;
    type MeshAdapter = WgpuMeshAdapter;
}
