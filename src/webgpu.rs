use std::sync::Arc;
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits,
    PresentMode, Queue, Surface, SurfaceConfiguration, TextureFormat, TextureUsages,
};
use winit::window::Window;

pub struct WebGpuState {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub format: TextureFormat,
    pub size: (u32, u32),
    #[allow(dead_code)]
    pub scale_factor: f64,
    /// Whether compute shaders are supported (required for deep zoom)
    pub has_compute_shaders: bool,
}

impl WebGpuState {
    pub async fn new(window: Arc<Window>) -> Result<Self, String> {
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        // Try WebGPU first (for compute shaders), fall back to WebGL
        #[cfg(target_arch = "wasm32")]
        let (instance, surface, adapter, has_compute_shaders) = {
            // First try WebGPU
            let webgpu_result = Self::try_create_adapter(
                window.clone(),
                Backends::BROWSER_WEBGPU,
            ).await;

            match webgpu_result {
                Ok((instance, surface, adapter)) => {
                    log::info!("Using WebGPU backend");
                    (instance, surface, adapter, true)
                }
                Err(e) => {
                    log::warn!("WebGPU failed ({}), falling back to WebGL", e);
                    // Fall back to WebGL
                    let (instance, surface, adapter) = Self::try_create_adapter(
                        window.clone(),
                        Backends::GL,
                    ).await.map_err(|e| format!("Both WebGPU and WebGL failed: {}", e))?;
                    log::info!("Using WebGL backend");
                    (instance, surface, adapter, false)
                }
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        let (instance, surface, adapter, has_compute_shaders) = {
            let (instance, surface, adapter) = Self::try_create_adapter(
                window.clone(),
                Backends::all(),
            ).await?;
            let has_compute = adapter.get_info().backend != wgpu::Backend::Gl;
            (instance, surface, adapter, has_compute)
        };

        let adapter_info = adapter.get_info();
        log::info!("Adapter: {:?}", adapter_info);
        log::info!("Compute shader support: {}", has_compute_shaders);

        // Request device - always use WebGL2 compatible limits for broad compatibility
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("fractal-device"),
                    required_features: Features::empty(),
                    required_limits: Limits::downlevel_webgl2_defaults(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create device: {e}"))?;

        // Get surface capabilities and choose format
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        log::info!("Surface format: {:?}", format);

        let width = size.width.max(1);
        let height = size.height.max(1);

        // Configure surface
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Ok(Self {
            device,
            queue,
            surface,
            config,
            format,
            size: (width, height),
            scale_factor,
            has_compute_shaders,
        })
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let new_width = new_width.max(1);
        let new_height = new_height.max(1);
        self.size = (new_width, new_height);
        self.config.width = new_width;
        self.config.height = new_height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Try to create an adapter with the specified backend
    async fn try_create_adapter(
        window: Arc<Window>,
        backends: Backends,
    ) -> Result<(Instance, Surface<'static>, wgpu::Adapter), String> {
        let instance = Instance::new(InstanceDescriptor {
            backends,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window)
            .map_err(|e| format!("Failed to create surface: {e}"))?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| "Failed to find a suitable GPU adapter".to_string())?;

        Ok((instance, surface, adapter))
    }
}
