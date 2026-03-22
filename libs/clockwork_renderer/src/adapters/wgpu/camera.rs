use std::num::NonZeroUsize;

use crate::adapters::{
    ClockworkRendererAdapter, ClockworkRendererAdapters,
    camera::CameraAdapter,
    wgpu::{WgpuAdapters, buffer::Buffer, context::WgpuContextAdapter, mesh::WgpuMeshAdapter},
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::NoUninit, Debug)]
pub struct WgpuCameraUniform {
    pub view_projection: [[f32; 4]; 4],
}

pub struct WgpuCameraAdapter {
    pub uniform_buffer: Buffer<WgpuCameraUniform>,
    pub bind_group: wgpu::BindGroup,
}

impl ClockworkRendererAdapter for WgpuCameraAdapter {
    type Adapters = WgpuAdapters;
}

impl CameraAdapter for WgpuCameraAdapter {
    fn new(context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter) -> Self {
        let uniform_buffer = Buffer::new(
            &context.device,
            wgpu::BufferUsages::UNIFORM,
            NonZeroUsize::new(1).expect("1 > 0"),
        );
        let bind_group = context.create_camera_bind_group(&uniform_buffer);

        Self {
            uniform_buffer,
            bind_group,
        }
    }

    fn update(&self, context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter, view_projection: glam::Mat4) {
        let uniform = WgpuCameraUniform {
            view_projection: view_projection.to_cols_array_2d(),
        };
        context
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    fn render(
        &self,
        context: &<Self::Adapters as ClockworkRendererAdapters>::ContextAdapter,
        surface: &<Self::Adapters as ClockworkRendererAdapters>::SurfaceAdapter,
    ) {
        let wgpu_surface = &surface.surface;
        let surface_texture = match wgpu_surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            _ => panic!("Failed to acquire surface texture"),
        };
        let surface_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let depth_texture = surface.depth_texture.borrow();
        {
            let mut render_pass = WgpuContextAdapter::begin_render_pass(&mut encoder, &surface_view, &depth_texture.view);
            render_pass.set_pipeline(&context.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            for (instance_key, instance_buffer) in context.instances.borrow().iter() {
                let mesh = instance_key.mesh.maybe_read();
                let WgpuMeshAdapter::NonEmpty {
                    vertex_buffer,
                    index_buffer,
                } = &mesh.adapter
                else {
                    // Empty mesh, we just don't render anything
                    continue;
                };

                let texture = instance_key.texture.maybe_read();

                render_pass.set_bind_group(1, Some(&texture.adapter.bind_group), &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
                match index_buffer {
                    None => render_pass.draw(0..vertex_buffer.len() as u32, 0..instance_buffer.len() as u32),
                    Some(index_buffer) => {
                        let Some(index_buffer) = index_buffer else {
                            continue;
                        };

                        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..index_buffer.len() as u32, 0, 0..instance_buffer.len() as u32)
                    }
                }
            }
        }

        context.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
