use egui::{ClippedPrimitive, Context, Slider, TexturesDelta, Ui};
use egui_wgpu::wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::color::ColorScheme;
use crate::fractal::julia::julia_presets;
use crate::fractal::{
    buffalo, burning_ship, celtic, heart, julia, mandelbrot,
    perpendicular, tricorn, FractalParams, FractalType,
};

pub struct UiState {
    ctx: Context,
    state: State,
    renderer: Renderer,
    pending_frame: Option<PreparedFrame>,
}

struct PreparedFrame {
    textures_delta: TexturesDelta,
    shapes: Vec<ClippedPrimitive>,
    screen_descriptor: ScreenDescriptor,
}

impl UiState {
    pub fn new(device: &Device, format: TextureFormat, window: &Window) -> Self {
        let ctx = Context::default();
        let state = State::new(
            ctx.clone(),
            ctx.viewport_id(),
            window,
            Some(window.scale_factor() as f32),
            None,
        );
        let renderer = Renderer::new(device, format, None, 1);

        Self {
            ctx,
            state,
            renderer,
            pending_frame: None,
        }
    }

    pub fn handle_window_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.state.on_window_event(window, event);
        response.consumed
    }

    /// Run egui for this frame and stage paint jobs. Returns true if params changed.
    pub fn prepare(&mut self, window: &Window, params: &mut FractalParams) -> bool {
        let size = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width.max(1), size.height.max(1)],
            pixels_per_point: window.scale_factor() as f32,
        };

        let raw_input = self.state.take_egui_input(window);
        let params_before = *params;
        let full_output = self.ctx.run(raw_input, |ctx| {
            Self::build_ui(ctx, params);
        });

        self.state
            .handle_platform_output(window, full_output.platform_output);

        let shapes = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        self.pending_frame = Some(PreparedFrame {
            textures_delta: full_output.textures_delta,
            shapes,
            screen_descriptor,
        });

        !params_equal(&params_before, params)
    }

    fn fractal_type_section(ui: &mut Ui, params: &mut FractalParams) {
        ui.label("Fractal Type");

        let current = params.get_fractal_type();
        egui::ComboBox::from_id_source("fractal_type")
            .selected_text(current.name())
            .show_ui(ui, |ui| {
                for ft in FractalType::all() {
                    let selected = *ft == current;
                    if ui.selectable_label(selected, ft.name()).clicked() {
                        params.set_fractal_type(*ft);
                        params.reset();
                    }
                }
            });
    }

    fn parameters_section(ui: &mut Ui, params: &mut FractalParams) {
        ui.label("Parameters");

        // Iterations (increased to 10000)
        let mut max_iter = params.max_iter as f32;
        ui.add(
            Slider::new(&mut max_iter, 10.0..=10000.0)
                .logarithmic(true)
                .text("Iterations"),
        );
        params.max_iter = max_iter as u32;

        // Power
        ui.add(Slider::new(&mut params.power, 2.0..=8.0).text("Power"));

        // Escape radius
        ui.add(
            Slider::new(&mut params.escape_radius, 2.0..=100.0)
                .logarithmic(true)
                .text("Escape Radius"),
        );
    }

    fn julia_section(ui: &mut Ui, params: &mut FractalParams) {
        ui.label("Julia Constant");

        // C real part
        ui.add(Slider::new(&mut params.julia_c[0], -2.0..=2.0).text("Real"));

        // C imaginary part
        ui.add(Slider::new(&mut params.julia_c[1], -2.0..=2.0).text("Imaginary"));

        // Julia presets
        ui.label("Julia Presets:");
        egui::Grid::new("julia_presets_grid")
            .num_columns(2)
            .show(ui, |ui| {
                let presets = julia_presets();
                for (i, preset) in presets.iter().enumerate() {
                    if ui.button(preset.name).clicked() {
                        params.julia_c = preset.c;
                    }
                    if (i + 1) % 2 == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn color_section(ui: &mut Ui, params: &mut FractalParams) {
        ui.label("Color Scheme");

        let current = ColorScheme::from_u32(params.color_scheme);
        egui::ComboBox::from_id_source("color_scheme")
            .selected_text(current.name())
            .show_ui(ui, |ui| {
                for cs in ColorScheme::all() {
                    let selected = *cs == current;
                    if ui.selectable_label(selected, cs.name()).clicked() {
                        params.color_scheme = *cs as u32;
                    }
                }
            });

        ui.horizontal(|ui| {
            let mut smooth = params.smooth();
            if ui.checkbox(&mut smooth, "Smooth").changed() {
                params.set_smooth(smooth);
            }

            let mut invert = params.invert();
            if ui.checkbox(&mut invert, "Invert").changed() {
                params.set_invert(invert);
            }

            let mut offset = params.offset();
            if ui.checkbox(&mut offset, "Offset").changed() {
                params.set_offset(offset);
            }
        });
    }

    fn navigation_section(ui: &mut Ui, params: &mut FractalParams) {
        ui.label("Navigation");

        // Zoom slider (logarithmic)
        let mut log_zoom = params.zoom.log10();
        ui.add(Slider::new(&mut log_zoom, -10.0..=10.0).text("Zoom (log)"));
        params.zoom = 10.0_f32.powf(log_zoom);

        // Center coordinates (display only)
        ui.horizontal(|ui| {
            ui.label(format!("Center: ({:.6}, {:.6})", params.center[0], params.center[1]));
        });

        // Reset button
        if ui.button("Reset View").clicked() {
            params.reset();
        }
    }

    fn presets_section(ui: &mut Ui, params: &mut FractalParams) {
        ui.label("Location Presets");

        let presets = match params.get_fractal_type() {
            FractalType::Mandelbrot => mandelbrot::presets(),
            FractalType::Julia => julia::presets(),
            FractalType::BurningShip => burning_ship::presets(),
            FractalType::Tricorn => tricorn::presets(),
            FractalType::Buffalo => buffalo::presets(),
            FractalType::Celtic => celtic::presets(),
            FractalType::PerpendicularMandelbrot => perpendicular::mandelbrot_presets(),
            FractalType::PerpendicularBurningShip => perpendicular::burning_ship_presets(),
            FractalType::Heart => heart::presets(),
            FractalType::TricornJulia => tricorn::julia_presets(),
            FractalType::BuffaloJulia => buffalo::julia_presets(),
            FractalType::CelticJulia => celtic::julia_presets(),
            FractalType::BurningShipJulia => burning_ship::presets(),
        };

        egui::Grid::new("location_presets_grid")
            .num_columns(2)
            .show(ui, |ui| {
                for (i, preset) in presets.iter().enumerate() {
                    if ui.button(preset.name).clicked() {
                        preset.apply(params);
                    }
                    if (i + 1) % 2 == 0 {
                        ui.end_row();
                    }
                }
        });
    }

    /// Paint the previously prepared egui frame over the target view.
    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        _window: &Window,
        _format: TextureFormat,
    ) {
        let Some(prepared) = self.pending_frame.take() else {
            return;
        };

        for (id, image_delta) in &prepared.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        self.renderer
            .update_buffers(device, queue, encoder, &prepared.shapes, &prepared.screen_descriptor);

        {
            let mut render_pass = encoder.begin_render_pass(&egui_wgpu::wgpu::RenderPassDescriptor {
                label: Some("egui-render-pass"),
                color_attachments: &[Some(egui_wgpu::wgpu::RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    ops: egui_wgpu::wgpu::Operations {
                        load: egui_wgpu::wgpu::LoadOp::Load,
                        store: egui_wgpu::wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer
                .render(&mut render_pass, &prepared.shapes, &prepared.screen_descriptor);
        }

        for id in &prepared.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }

    /// Build egui widgets and return true if params changed.
    fn build_ui(ctx: &Context, params: &mut FractalParams) {
        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Fractal Madness");
                ui.separator();

                Self::fractal_type_section(ui, params);
                ui.separator();

                Self::parameters_section(ui, params);
                ui.separator();

                // Show Julia constant for all Julia-type fractals
                if params.get_fractal_type().needs_julia_c() {
                    Self::julia_section(ui, params);
                    ui.separator();
                }

                Self::color_section(ui, params);
                ui.separator();

                Self::navigation_section(ui, params);
                ui.separator();

                Self::presets_section(ui, params);
            });
    }
}

fn params_equal(a: &FractalParams, b: &FractalParams) -> bool {
    a.center == b.center
        && a.zoom == b.zoom
        && a.max_iter == b.max_iter
        && a.power == b.power
        && a.escape_radius == b.escape_radius
        && a.fractal_type == b.fractal_type
        && a.color_scheme == b.color_scheme
        && a.julia_c == b.julia_c
        && a.flags == b.flags
}
