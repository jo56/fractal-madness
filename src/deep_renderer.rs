//! Deep fractal renderer using WebGPU compute shaders.
//!
//! This renderer uses perturbation theory to enable extremely deep zooming
//! into the Mandelbrot set (10^100+ magnification).

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendState, Buffer, BufferBindingType,
    BufferDescriptor, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoder,
    ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, Extent3d,
    FilterMode, FragmentState, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor,
    PrimitiveState, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, StorageTextureAccess, StoreOp, Texture, TextureDescriptor,
    TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView,
    TextureViewDescriptor, TextureViewDimension, VertexState,
};

use crate::deep_zoom::{DeepZoomParams, ReferenceOrbit, ReferencePoint, SeriesApproximation};

/// Maximum reference orbit size (in iterations)
const MAX_REF_ORBIT_SIZE: u32 = 100_000;

/// Uniform parameters for the compute shader
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
struct DeepShaderParams {
    /// Resolution (width, height)
    resolution: [u32; 2],
    /// Maximum iterations
    max_iter: u32,
    /// Iterations to skip via SA
    sa_skip: u32,
    /// Escape radius squared
    escape_radius_sq: f32,
    /// Color scheme index
    color_scheme: u32,
    /// Flags (smooth coloring, etc.)
    flags: u32,
    /// Reference orbit length
    ref_orbit_len: u32,
    /// Top-left corner delta (double-double)
    corner_delta_re_hi: f32,
    corner_delta_re_lo: f32,
    corner_delta_im_hi: f32,
    corner_delta_im_lo: f32,
    /// Pixel step size (double-double)
    step_re_hi: f32,
    step_re_lo: f32,
    step_im_hi: f32,
    step_im_lo: f32,
}

/// Deep fractal renderer using compute shaders
pub struct DeepFractalRenderer {
    // Compute pipeline
    compute_pipeline: ComputePipeline,
    compute_bind_group_layout: BindGroupLayout,

    // Buffers
    params_buffer: Buffer,
    reference_orbit_buffer: Buffer,
    sa_coefficients_buffer: Buffer,

    // Output texture
    output_texture: Texture,
    output_view: TextureView,

    // Display pipeline (to show compute output)
    display_pipeline: RenderPipeline,
    display_bind_group_layout: BindGroupLayout,
    display_sampler: Sampler,

    // Cached data
    reference_orbit: Option<ReferenceOrbit>,
    series_approx: Option<SeriesApproximation>,

    // Size
    size: (u32, u32),
}

impl DeepFractalRenderer {
    /// Create a new deep fractal renderer
    pub fn new(device: &Device, format: TextureFormat, width: u32, height: u32) -> Self {
        // Create compute shader module
        let compute_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("deep-fractal-compute"),
            source: ShaderSource::Wgsl(include_str!("../shaders/deep_fractal.wgsl").into()),
        });

        // Create display shader module
        let display_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("deep-fractal-display"),
            source: ShaderSource::Wgsl(include_str!("../shaders/display.wgsl").into()),
        });

        // Create compute bind group layout
        let compute_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("deep-compute-bind-group-layout"),
            entries: &[
                // Params uniform
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Reference orbit storage
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // SA coefficients storage
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Output texture
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        // Create compute pipeline
        let compute_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("deep-compute-pipeline-layout"),
            bind_group_layouts: &[&compute_bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("deep-compute-pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        // Create display bind group layout
        let display_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("deep-display-bind-group-layout"),
            entries: &[
                // Input texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create display pipeline
        let display_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("deep-display-pipeline-layout"),
            bind_group_layouts: &[&display_bind_group_layout],
            push_constant_ranges: &[],
        });

        let display_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("deep-display-pipeline"),
            layout: Some(&display_pipeline_layout),
            vertex: VertexState {
                module: &display_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &display_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        // Create sampler
        let display_sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("deep-display-sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..Default::default()
        });

        // Create params buffer
        let params_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("deep-params-buffer"),
            size: std::mem::size_of::<DeepShaderParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create reference orbit buffer
        let reference_orbit_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("deep-reference-orbit-buffer"),
            size: (MAX_REF_ORBIT_SIZE as usize * std::mem::size_of::<ReferencePoint>()) as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create SA coefficients buffer
        let sa_coefficients_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("deep-sa-coefficients-buffer"),
            size: (MAX_REF_ORBIT_SIZE as usize * 32) as u64, // 32 bytes per coefficient
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create output texture
        let (output_texture, output_view) = Self::create_output_texture(device, width, height);

        Self {
            compute_pipeline,
            compute_bind_group_layout,
            params_buffer,
            reference_orbit_buffer,
            sa_coefficients_buffer,
            output_texture,
            output_view,
            display_pipeline,
            display_bind_group_layout,
            display_sampler,
            reference_orbit: None,
            series_approx: None,
            size: (width, height),
        }
    }

    /// Create output texture for compute shader
    fn create_output_texture(device: &Device, width: u32, height: u32) -> (Texture, TextureView) {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("deep-output-texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        (texture, view)
    }

    /// Resize the renderer
    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        if self.size != (width, height) {
            self.size = (width, height);
            let (texture, view) = Self::create_output_texture(device, width, height);
            self.output_texture = texture;
            self.output_view = view;
        }
    }

    /// Update reference orbit if needed
    pub fn update_reference(&mut self, params: &DeepZoomParams, queue: &Queue) {
        if params.reference_dirty || self.reference_orbit.is_none() {
            // Calculate new reference orbit
            let orbit = ReferenceOrbit::calculate(
                &params.center_re,
                &params.center_im,
                params.max_iter.min(MAX_REF_ORBIT_SIZE),
                params.escape_radius_sq.sqrt(),
            );

            // Upload to GPU
            queue.write_buffer(&self.reference_orbit_buffer, 0, orbit.as_bytes());

            // Calculate series approximation
            let zoom = params.zoom_factor();
            let max_delta = 2.0 / zoom; // Maximum pixel delta in complex plane

            let sa = SeriesApproximation::calculate(&orbit, max_delta, 1e-6);
            queue.write_buffer(&self.sa_coefficients_buffer, 0, sa.as_bytes());

            self.series_approx = Some(sa);
            self.reference_orbit = Some(orbit);
        }
    }

    /// Render the fractal
    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        params: &mut DeepZoomParams,
    ) {
        // Update reference orbit if needed
        self.update_reference(params, queue);
        params.reference_dirty = false;

        let (width, height) = self.size;

        // Calculate shader parameters
        let zoom = params.zoom_factor();
        let aspect = width as f64 / height as f64;
        let scale = 2.0 / zoom;

        // Calculate corner delta (top-left pixel relative to center)
        let half_width = (width as f64 / 2.0) * (scale * aspect * 2.0 / width as f64);
        let half_height = (height as f64 / 2.0) * (scale * 2.0 / height as f64);

        let corner_re = -half_width;
        let corner_im = -half_height;

        // Pixel step size
        let step_re = scale * aspect * 2.0 / width as f64;
        let step_im = scale * 2.0 / height as f64;

        // Get SA skip count
        let sa_skip = self.series_approx.as_ref().map(|sa| sa.skip_iterations).unwrap_or(0);
        let ref_orbit_len = self.reference_orbit.as_ref().map(|r| r.len() as u32).unwrap_or(0);

        let shader_params = DeepShaderParams {
            resolution: [width, height],
            max_iter: params.max_iter,
            sa_skip,
            escape_radius_sq: params.escape_radius_sq as f32,
            color_scheme: params.color_scheme,
            flags: 1, // Smooth coloring on
            ref_orbit_len,
            corner_delta_re_hi: corner_re as f32,
            corner_delta_re_lo: (corner_re - corner_re as f32 as f64) as f32,
            corner_delta_im_hi: corner_im as f32,
            corner_delta_im_lo: (corner_im - corner_im as f32 as f64) as f32,
            step_re_hi: step_re as f32,
            step_re_lo: (step_re - step_re as f32 as f64) as f32,
            step_im_hi: step_im as f32,
            step_im_lo: (step_im - step_im as f32 as f64) as f32,
        };

        // Upload params
        queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&shader_params));

        // Create compute bind group
        let compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("deep-compute-bind-group"),
            layout: &self.compute_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: self.reference_orbit_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: self.sa_coefficients_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::TextureView(&self.output_view),
                },
            ],
        });

        // Dispatch compute shader
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("deep-compute-pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &compute_bind_group, &[]);

            // Workgroups: 16x16 threads per group
            let workgroups_x = (width + 15) / 16;
            let workgroups_y = (height + 15) / 16;
            compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);
        }

        // Create display bind group
        let display_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("deep-display-bind-group"),
            layout: &self.display_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&self.output_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&self.display_sampler),
                },
            ],
        });

        // Render to screen
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("deep-display-pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.display_pipeline);
            render_pass.set_bind_group(0, &display_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }
    }
}
