use std::cell::RefCell;

use crate::adapters::{
    ClockworkRendererAdapter,
    surface::SurfaceAdapter,
    wgpu::{WgpuAdapters, context::WgpuContextAdapter},
};

pub struct WgpuSurfaceAdapter {
    pub(crate) configuration: RefCell<wgpu::SurfaceConfiguration>,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) depth_texture: RefCell<DepthTexture>,
}

impl ClockworkRendererAdapter for WgpuSurfaceAdapter {
    type Adapters = WgpuAdapters;
}

impl SurfaceAdapter for WgpuSurfaceAdapter {
    fn new_from_target(
        context: &WgpuContextAdapter,
        target: wgpu::SurfaceTarget<'static>,
        dimensions: glam::UVec2,
    ) -> anyhow::Result<Self> {
        let surface = context.instance.create_surface(target)?;
        Self::new_from_surface(surface, &context.adapter, &context.device, dimensions)
    }

    fn resize(&self, context: &WgpuContextAdapter, dimensions: glam::UVec2) {
        let glam::UVec2 { x: width, y: height } = dimensions.max(glam::UVec2::ONE);

        let mut configuration = self.configuration.borrow_mut();
        configuration.width = width;
        configuration.height = height;

        self.surface.configure(&context.device, &configuration);
        self.depth_texture.replace(DepthTexture::new(&context.device, width, height));
    }

    fn dimensions(&self) -> glam::UVec2 {
        let configuration = self.configuration.borrow();
        (configuration.width, configuration.height).into()
    }
}

impl WgpuSurfaceAdapter {
    pub fn new_from_surface(
        surface: wgpu::Surface<'static>,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        dimensions: glam::UVec2,
    ) -> anyhow::Result<Self> {
        let capabilities = surface.get_capabilities(adapter);
        let format = capabilities.formats.iter().find(|f| f.is_srgb()).unwrap_or(
            capabilities
                .formats
                .first()
                .ok_or(anyhow::anyhow!("No color formats available for surface"))?,
        );

        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *format,
            width: dimensions.x.max(1),
            height: dimensions.y.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            // present_mode: *capabilities
            //     .present_modes
            //     .first()
            //     .ok_or(anyhow!("No present mode part of surface capabilities"))?,
            desired_maximum_frame_latency: 3,
            alpha_mode: *capabilities
                .alpha_modes
                .first()
                .ok_or(anyhow::anyhow!("No alpha mode part of surface capabilities"))?,
            view_formats: vec![],
        };
        surface.configure(device, &configuration);

        let depth_texture = RefCell::new(DepthTexture::new(device, configuration.width, configuration.height));

        Ok(Self {
            configuration: configuration.into(),
            surface,
            depth_texture,
        })
    }
}

#[derive(Debug)]
pub struct DepthTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl DepthTexture {
    pub fn view(&self) -> &wgpu::TextureView { &self.view }

    fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        Self { texture, view, sampler }
    }
}
