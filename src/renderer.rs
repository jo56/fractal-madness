use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBindingType,
    BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CommandEncoder,
    ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, Extent3d,
    FilterMode, FragmentState, FrontFace, MultisampleState, PipelineLayoutDescriptor, PolygonMode,
    PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, StorageTextureAccess, Texture,
    TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages,
    TextureView, TextureViewDescriptor, TextureViewDimension, VertexState,
};

use crate::fractal::FractalParams;

const FRACTAL_SHADER: &str = include_str!("../shaders/fractal.wgsl");
const FULLSCREEN_SHADER: &str = include_str!("../shaders/fullscreen.wgsl");

pub struct FractalRenderer {
    params_buffer: Buffer,
    storage_texture: Texture,
    storage_view: TextureView,
    compute_pipeline: ComputePipeline,
    compute_bind_group: BindGroup,
    render_pipeline: RenderPipeline,
    render_bind_group: BindGroup,
    size: (u32, u32),
    dirty: bool,
}

impl FractalRenderer {
    pub fn new(device: &Device, surface_format: TextureFormat, width: u32, height: u32) -> Self {
        // Create params uniform buffer
        let params_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("fractal-params-buffer"),
            size: std::mem::size_of::<FractalParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create storage texture for compute output
        let (storage_texture, storage_view) =
            Self::create_storage_texture(device, width, height);

        // Create compute shader and pipeline
        let compute_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("fractal-compute-shader"),
            source: ShaderSource::Wgsl(FRACTAL_SHADER.into()),
        });

        let compute_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("fractal-compute-bind-group-layout"),
                entries: &[
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
                    BindGroupLayoutEntry {
                        binding: 1,
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

        let compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("fractal-compute-bind-group"),
            layout: &compute_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&storage_view),
                },
            ],
        });

        let compute_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("fractal-compute-pipeline-layout"),
            bind_group_layouts: &[&compute_bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
            label: Some("fractal-compute-pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "main",
        });

        // Create fullscreen render shader and pipeline
        let render_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("fullscreen-render-shader"),
            source: ShaderSource::Wgsl(FULLSCREEN_SHADER.into()),
        });

        let render_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("fullscreen-bind-group-layout"),
                entries: &[
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
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("fractal-sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..Default::default()
        });

        // Create a view for sampling (not storage)
        let sample_view = storage_texture.create_view(&TextureViewDescriptor::default());

        let render_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("fullscreen-bind-group"),
            layout: &render_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&sample_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("fullscreen-pipeline-layout"),
            bind_group_layouts: &[&render_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("fullscreen-render-pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        Self {
            params_buffer,
            storage_texture,
            storage_view,
            compute_pipeline,
            compute_bind_group,
            render_pipeline,
            render_bind_group,
            size: (width, height),
            dirty: true,
        }
    }

    fn create_storage_texture(device: &Device, width: u32, height: u32) -> (Texture, TextureView) {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("fractal-storage-texture"),
            size: Extent3d {
                width: width.max(1),
                height: height.max(1),
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

    pub fn resize(&mut self, device: &Device, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if self.size == (width, height) {
            return;
        }

        self.size = (width, height);

        // Recreate storage texture
        let (storage_texture, storage_view) = Self::create_storage_texture(device, width, height);
        self.storage_texture = storage_texture;
        self.storage_view = storage_view;

        // Recreate bind groups with new texture
        self.recreate_bind_groups(device);
        self.dirty = true;
    }

    fn recreate_bind_groups(&mut self, device: &Device) {
        // Recreate compute bind group
        let compute_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("fractal-compute-bind-group-layout"),
                entries: &[
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
                    BindGroupLayoutEntry {
                        binding: 1,
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

        self.compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("fractal-compute-bind-group"),
            layout: &compute_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: self.params_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&self.storage_view),
                },
            ],
        });

        // Recreate render bind group
        let render_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("fullscreen-bind-group-layout"),
                entries: &[
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
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("fractal-sampler"),
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            ..Default::default()
        });

        let sample_view = self.storage_texture.create_view(&TextureViewDescriptor::default());

        self.render_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("fullscreen-bind-group"),
            layout: &render_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&sample_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn render(
        &mut self,
        _device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        params: &FractalParams,
        size: (u32, u32),
    ) {
        // Update params buffer
        queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(params));

        // Only run compute if dirty
        if self.dirty {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("fractal-compute-pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.compute_bind_group, &[]);

            let workgroups_x = (size.0 + 7) / 8;
            let workgroups_y = (size.1 + 7) / 8;
            compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);

            // Dirty flag will be cleared after successful render
        }

        // Render pass to display the result
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("fullscreen-render-pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.render_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.dirty = false;
    }
}
