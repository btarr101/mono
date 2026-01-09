use crate::adapters::{context::ContextAdapter, surface::SurfaceAdapter};

pub mod context;
pub mod surface;
mod wgpu;

pub trait ClockworkAdapter {
    type Adapters: ClockworkAdapters;
}

pub trait ClockworkAdapters {
    type ContextAdapter: ContextAdapter<Adapters = Self>;
    type SurfaceAdapter: SurfaceAdapter<Adapters = Self>;
}
