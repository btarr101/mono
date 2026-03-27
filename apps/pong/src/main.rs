use std::sync::Arc;

use clockwork_renderer::{
    camera::Camera,
    context::{Context, ContextHandleExt},
    draw_params::DrawParams,
    handle::UnlockedHandle,
    mesh::{self, Mesh},
    surface::Surface,
    texture::Texture,
};
use winit::dpi::LogicalSize;

pub enum Pong {
    New,
    Running {
        window: Arc<winit::window::Window>,
        context: UnlockedHandle<Context>,
        surface: UnlockedHandle<Surface>,
        camera: UnlockedHandle<Camera>,
        texture: UnlockedHandle<Texture>,
        mesh: UnlockedHandle<Mesh>,
    },
}

impl Pong {
    pub fn surface(&self) -> Option<&UnlockedHandle<Surface>> {
        match self {
            Self::New => None,
            Self::Running { surface, .. } => Some(surface),
        }
    }

    pub fn window(&self) -> Option<&winit::window::Window> {
        match self {
            Self::New => None,
            Self::Running { window, .. } => Some(window),
        }
    }
}

impl winit::application::ApplicationHandler for Pong {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        match self {
            Self::New => {
                let window_attributes =
                    winit::window::WindowAttributes::default()
                        .with_title("Pong")
                        .with_inner_size(winit::dpi::Size::Logical(LogicalSize {
                            width: 640.,
                            height: 480.,
                        }));
                let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
                let (context, surface) = clockwork_renderer::create_context_with_surface_target(window.clone()).unwrap();
                let camera = context.create_camera(glam::Mat4::IDENTITY);
                let texture = context.create_texture(&[255, 255, 255, 255], (1, 1).into());
                let mesh = context.create_mesh(&mesh::MeshData {
                    vertices: vec![
                        mesh::Vertex {
                            position: (-0.5, -0.5, 0.).into(),
                            texture_coordinates: (0., 0.).into(),
                        },
                        mesh::Vertex {
                            position: (0.5, -0.5, 0.).into(),
                            texture_coordinates: (1., 0.).into(),
                        },
                        mesh::Vertex {
                            position: (0.5, 0.5, 0.).into(),
                            texture_coordinates: (1., 1.).into(),
                        },
                        mesh::Vertex {
                            position: (-0.5, 0.5, 0.).into(),
                            texture_coordinates: (0., 1.).into(),
                        },
                    ],
                    indices: Some(vec![0, 1, 2, 2, 3, 0]),
                });

                window.request_redraw();
                *self = Pong::Running {
                    window,
                    context,
                    surface,
                    camera,
                    texture,
                    mesh,
                };
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
            winit::event::WindowEvent::RedrawRequested => {
                if let Pong::Running {
                    window,
                    context,
                    surface,
                    camera,
                    texture,
                    mesh,
                } = self
                {
                    context.draw(
                        texture,
                        mesh,
                        &DrawParams {
                            transform: glam::Mat4::IDENTITY,
                            color: glam::U8Vec4::splat(u8::MAX),
                            ..Default::default()
                        },
                    );
                    camera.render(surface);
                    context.clear();

                    window.request_redraw();
                }
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
