use crate::adapters::{ClockworkRendererAdapter, texture::TextureAdapter, wgpu::WgpuAdapters};

pub struct WgpuTextureAdapter {
    texture: wgpu::Texture,
    pub(crate) bind_group: wgpu::BindGroup,
}

impl ClockworkRendererAdapter for WgpuTextureAdapter {
    type Adapters = WgpuAdapters;
}

impl TextureAdapter for WgpuTextureAdapter {
    fn new(
        context: &<Self::Adapters as crate::adapters::ClockworkRendererAdapters>::ContextAdapter,
        data: &[u8],
        dimensions: glam::UVec2,
    ) -> Self {
        let texture = create_wgpu_texture(&context.device, &context.queue, data, dimensions);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = context.create_texture_bind_group(&texture_view, &sampler);

        Self { texture, bind_group }
    }
}

fn create_wgpu_texture(device: &wgpu::Device, queue: &wgpu::Queue, data: &[u8], dimensions: glam::UVec2) -> wgpu::Texture {
    let size = wgpu::Extent3d {
        width: dimensions.x,
        height: dimensions.y,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    update_wgpu_texture(queue, data, size, &texture);

    texture
}

fn update_wgpu_texture(queue: &wgpu::Queue, data: &[u8], size: wgpu::Extent3d, texture: &wgpu::Texture) {
    queue.write_texture(
        wgpu::TexelCopyTextureInfoBase {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width), // 4 for RGBA
            rows_per_image: Some(size.height),
        },
        size,
    );
}
