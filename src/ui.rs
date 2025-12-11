use egui::{Context, Slider, Ui};
use egui_wgpu::wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::color::ColorScheme;
use crate::fractal::julia::julia_presets;
use crate::fractal::{burning_ship, julia, mandelbrot, FractalParams, FractalType};

pub struct UiState {
    ctx: Context,
    state: State,
    renderer: Renderer,
    last_params: FractalParams,
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
            last_params: FractalParams::default(),
        }
    }

    pub fn handle_window_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.state.on_window_event(window, event);
        response.consumed
    }

    /// Build the UI and return true if parameters changed
    pub fn build(&mut self, params: &mut FractalParams) -> bool {
        self.last_params = *params;

        let ctx = self.ctx.clone();
        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(280.0)
            .show(&ctx, |ui| {
                ui.heading("Fractal Madness");
                ui.separator();

                Self::fractal_type_section(ui, params);
                ui.separator();

                Self::parameters_section(ui, params);
                ui.separator();

                if params.get_fractal_type() == FractalType::Julia {
                    Self::julia_section(ui, params);
                    ui.separator();
                }

                Self::color_section(ui, params);
                ui.separator();

                Self::navigation_section(ui, params);
                ui.separator();

                Self::presets_section(ui, params);
            });

        // Check if params changed
        !params_equal(&self.last_params, params)
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

        // Iterations
        let mut max_iter = params.max_iter as f32;
        ui.add(
            Slider::new(&mut max_iter, 10.0..=5000.0)
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

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        output_view: &TextureView,
        window: &Window,
        _format: TextureFormat,
    ) {
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [
                window.inner_size().width,
                window.inner_size().height,
            ],
            pixels_per_point: window.scale_factor() as f32,
        };

        let raw_input = self.state.take_egui_input(window);
        let full_output = self.ctx.run(raw_input, |_| {});

        self.state.handle_platform_output(window, full_output.platform_output);

        let tris = self
            .ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        self.renderer
            .update_buffers(device, queue, encoder, &tris, &screen_descriptor);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui-render-pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer
                .render(&mut render_pass, &tris, &screen_descriptor);
        }

        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }
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
