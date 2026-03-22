use clockwork_renderer::{
    camera::Camera,
    context::{Context, ContextHandleExt},
    handle::UnlockedHandle,
    surface::Surface,
};
use winit::dpi::LogicalSize;

pub enum Pong {
    New,
    Running {
        context: UnlockedHandle<Context>,
        surface: UnlockedHandle<Surface>,
        camera: UnlockedHandle<Camera>,
    },
}

impl Pong {
    pub fn surface(&self) -> Option<&UnlockedHandle<Surface>> {
        match self {
            Self::New => None,
            Self::Running { surface, .. } => Some(surface),
        }
    }
}

impl winit::application::ApplicationHandler for Pong {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            Self::New => {
                let window_attributes =
                    winit::window::WindowAttributes::default()
                        .with_title("Pong")
                        .with_inner_size(winit::dpi::Size::Logical(LogicalSize {
                            width: 640.,
                            height: 480.,
                        }));
                let window = event_loop.create_window(window_attributes).unwrap();
                let (context, surface) = clockwork_renderer::create_context_with_surface_target(window).unwrap();
                let camera = context.create_camera(glam::Mat4::IDENTITY);
                *self = Pong::Running {
                    context,
                    surface,
                    camera,
                }
            }
            Self::Running { .. } => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                self.surface().inspect(|&surface| surface.resize((width, height).into()));
            }
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut Pong::New)?;

    Ok(())
}
