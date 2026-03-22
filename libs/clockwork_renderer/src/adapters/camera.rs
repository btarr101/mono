use crate::adapters::{ClockworkRendererAdapter, ClockworkRendererAdapters};

pub trait CameraAdapter: ClockworkRendererAdapter {
    fn new(context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter) -> Self;
    fn update(&self, context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter, view_projection: glam::Mat4);
    fn render(
        &self,
        context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter,
        surface: &<Self::Adapters as ClockworkRendererAdapters>::SurfaceAdapter,
    );
}
