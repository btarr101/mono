use crate::adapters::{ClockworkRendererAdapter, ClockworkRendererAdapters};

pub trait TextureAdapter: ClockworkRendererAdapter {
    fn new(context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter, data: &[u8], dimensions: glam::UVec2)
    -> Self;
}
