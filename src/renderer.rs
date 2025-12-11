use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferUsages,
    ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState, FrontFace,
    MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureFormat,
    TextureView, VertexState,
};

use crate::fractal::FractalParams;

const FRACTAL_SHADER: &str = include_str!("../shaders/fractal.wgsl");

pub struct FractalRenderer {
    params_buffer: Buffer,
    render_pipeline: RenderPipeline,
    render_bind_group: BindGroup,
}

impl FractalRenderer {
    pub fn new(device: &Device, surface_format: TextureFormat, _width: u32, _height: u32) -> Self {
        // Create params uniform buffer
        let params_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("fractal-params-buffer"),
            size: std::mem::size_of::<FractalParams>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create shader module
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("fractal-shader"),
            source: ShaderSource::Wgsl(FRACTAL_SHADER.into()),
        });

        // Create bind group layout (just uniform buffer for params)
        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("fractal-bind-group-layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let render_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("fractal-bind-group"),
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: params_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("fractal-pipeline-layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("fractal-render-pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
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
            render_pipeline,
            render_bind_group,
        }
    }

    pub fn resize(&mut self, _device: &Device, _width: u32, _height: u32) {
        // No storage texture to resize - fragment shader handles all sizes
    }

    pub fn mark_dirty(&mut self) {
        // No longer needed - fragment shader recalculates every frame
    }

    pub fn render(
        &mut self,
        _device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        params: &FractalParams,
        _size: (u32, u32),
    ) {
        // Update params buffer
        queue.write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(params));

        // Render pass - fractal is calculated directly in fragment shader
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("fractal-render-pass"),
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
    }
}
