use crate::{
    adapters::{ClockworkRendererAdapters, DefaultAdapters, camera::CameraAdapter},
    context::Context,
    handle::UnlockedHandle,
    surface::Surface,
};

pub struct Camera<A: ClockworkRendererAdapters = DefaultAdapters> {
    pub view_projection: glam::Mat4,
    context: UnlockedHandle<Context<A>>,
    pub(crate) adapter: <A as ClockworkRendererAdapters>::CameraAdapter,
}

impl<A: ClockworkRendererAdapters> Camera<A> {
    pub(crate) fn new(context: UnlockedHandle<Context<A>>, view_projection: glam::Mat4) -> Self {
        let adapter = <A as ClockworkRendererAdapters>::CameraAdapter::new(&context.adapter);
        Self {
            view_projection,
            context,
            adapter,
        }
    }

    pub fn render(&self, surface: &Surface<A>) {
        self.adapter.update(&self.context.adapter, self.view_projection);
        self.adapter.render(&self.context.adapter, &surface.adapter)
    }
}
