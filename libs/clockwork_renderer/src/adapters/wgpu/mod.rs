use crate::adapters::{
    ClockworkAdapters,
    wgpu::{context::WgpuContextAdapter, surface::WgpuSurfaceAdapter},
};

mod context;
mod surface;

pub struct WgpuAdapters;

impl ClockworkAdapters for WgpuAdapters {
    type ContextAdapter = WgpuContextAdapter;
    type SurfaceAdapter = WgpuSurfaceAdapter;
}
