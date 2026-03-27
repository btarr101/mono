use std::{cell::RefCell, num::NonZeroUsize};

use crate::{
    adapters::{
        ClockworkRendererAdapter,
        context::ContextAdapter,
        wgpu::{
            WgpuAdapters,
            buffer::Buffer,
            camera::{WgpuCameraAdapter, WgpuCameraUniform},
            instance::{WgpuInstance, WgpuInstanceKey},
            mesh::{WgpuMeshAdapter, WgpuVertex},
            surface::WgpuSurfaceAdapter,
        },
    },
    draw_params::DrawParams,
    handle::Handle,
    mesh::Mesh,
    texture::Texture,
};

#[cfg(not(target_arch = "wasm32"))]
const BACKENDS: wgpu::Backends = wgpu::Backends::PRIMARY;
#[cfg(target_arch = "wasm32")]
const BACKENDS: wgpu::Backends = wgpu::Backends::GL;

pub struct WgpuContextAdapter {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
    // globals_bind_group: wgpu::BindGroup,
    // globals_uniform_buffer: Buffer<WgpuGlobalsUniform>,
    pub(crate) camera_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    pub instances: RefCell<rustc_hash::FxHashMap<WgpuInstanceKey, Buffer<WgpuInstance>>>,
}

impl ClockworkRendererAdapter for WgpuContextAdapter {
    type Adapters = WgpuAdapters;
}
impl ContextAdapter for WgpuContextAdapter {
    async fn new_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> anyhow::Result<(Self, WgpuSurfaceAdapter)> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: BACKENDS,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });

        let surface = instance.create_surface(surface_target.into())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                #[cfg(not(target_arch = "wasm32"))]
                required_limits: wgpu::Limits::default(),
                #[cfg(target_arch = "wasm32")]
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                ..Default::default()
            })
            .await?;

        let surface = WgpuSurfaceAdapter::new_from_surface(surface, &adapter, &device, glam::uvec2(1, 1))?;
        let camera_bind_group_layout = Self::create_camera_bind_group_layout(&device);
        let texture_bind_group_layout = Self::create_texture_bind_group_layout(&device);
        let render_pipeline = Self::create_render_pipeline(
            &device,
            &camera_bind_group_layout,
            &texture_bind_group_layout,
            surface.configuration.borrow().format,
        );

        let context = Self {
            instance,
            adapter,
            device,
            queue,
            camera_bind_group_layout,
            texture_bind_group_layout,
            render_pipeline,
            instances: RefCell::new(rustc_hash::FxHashMap::default()),
        };

        Ok((context, surface))
    }

    fn draw(&self, texture: Handle<'static, Texture>, mesh: Handle<'static, Mesh>, params: &DrawParams) {
        self.instances
            .borrow_mut()
            .entry(WgpuInstanceKey { texture, mesh })
            .or_insert_with(|| {
                Buffer::new(
                    &self.device,
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    NonZeroUsize::new(1).expect("1 > 0"),
                )
            })
            .push_and_reallocate(
                &self.device,
                &self.queue,
                &[WgpuInstance {
                    transform: params.transform.to_cols_array_2d(),
                    color: params.color.to_array(),
                    uv_window: params.texture_window.to_array(),
                    // flip_bits: 0,
                }],
            )
    }

    fn clear(&self) {
        self.instances.borrow_mut().retain(|_, instance_buffer| {
            if instance_buffer.is_empty() {
                false
            } else {
                instance_buffer.clear();
                true
            }
        });
    }
}

impl WgpuContextAdapter {
    fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    pub fn create_camera_bind_group(&self, uniform_buffer: &Buffer<WgpuCameraUniform>) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &self.camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        })
    }

    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }

    pub fn create_texture_bind_group(&self, texture_view: &wgpu::TextureView, sampler: &wgpu::Sampler) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        texture_format: wgpu::TextureFormat,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(camera_bind_group_layout), Some(texture_bind_group_layout)],
            ..Default::default()
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: None,
                compilation_options: Default::default(),
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<WgpuVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            // Position
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            // Texture coordinates
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x2,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<WgpuInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // Transform 1
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Transform 2
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Transform 3
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Transform 4
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 3) as wgpu::BufferAddress,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Color
                            wgpu::VertexAttribute {
                                offset: (std::mem::size_of::<[f32; 4]>() * 4) as wgpu::BufferAddress,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Unorm8x4,
                            },
                            // Texture Coordinate Window
                            wgpu::VertexAttribute {
                                offset: ((std::mem::size_of::<[f32; 4]>() * 4) + std::mem::size_of::<[u8; 4]>())
                                    as wgpu::BufferAddress,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // FLIP BITS
                            // wgpu::VertexAttribute {
                            //     offset: ((std::mem::size_of::<[f32; 4]>() * 5) + std::mem::size_of::<[u8; 4]>())
                            //         as wgpu::BufferAddress,
                            //     shader_location: 8,
                            //     format: wgpu::VertexFormat::Uint32,
                            // },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: None,
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(wgpu::CompareFunction::Less),
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

            multiview_mask: None,
            cache: None,
        })
    }

    pub fn begin_render_pass<'a>(
        encoder: &'a mut wgpu::CommandEncoder,
        surface_view: &'a wgpu::TextureView,
        depth_texture_view: &'a wgpu::TextureView,
    ) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        })
    }
}
