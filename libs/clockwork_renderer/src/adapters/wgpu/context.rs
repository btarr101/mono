use crate::adapters::{
    ClockworkAdapter,
    context::ContextAdapter,
    wgpu::{WgpuAdapters, surface::WgpuSurfaceAdapter},
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
}

impl ClockworkAdapter for WgpuContextAdapter {
    type Adapters = WgpuAdapters;
}
impl ContextAdapter for WgpuContextAdapter {
    async fn new_with_surface(
        surface_target: impl Into<wgpu::SurfaceTarget<'static>>,
    ) -> anyhow::Result<(Self, WgpuSurfaceAdapter)> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: BACKENDS,
            ..Default::default()
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
                label: Some("Device"),
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

        let context = Self {
            instance,
            adapter,
            device,
            queue,
        };

        Ok((context, surface))
    }
}
