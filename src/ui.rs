use egui::{ClippedPrimitive, Context, Slider, TexturesDelta, Ui};
use egui_wgpu::wgpu::{CommandEncoder, Device, Queue, TextureFormat, TextureView};
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::color::ColorScheme;
use crate::constants::ui as ui_const;
use crate::fractal::{
    buffalo, burning_ship, celtic, julia, mandelbrot,
    newton, phoenix, tricorn, FractalParams, FractalType,
};

pub struct UiState {
    ctx: Context,
    state: State,
    renderer: Renderer,
    pending_frame: Option<PreparedFrame>,
    panel_width: f32,
    /// Color scheme per fractal type (indexed by FractalType as u32)
    fractal_colors: [u32; 9],
}

struct PreparedFrame {
    textures_delta: TexturesDelta,
    shapes: Vec<ClippedPrimitive>,
    screen_descriptor: ScreenDescriptor,
}

impl UiState {
    pub fn new(device: &Device, format: TextureFormat, window: &Window) -> Self {
        let ctx = Context::default();

        // Set Windows 95-style sharp corners
        ctx.style_mut(|style| {
            style.visuals.window_rounding = egui::Rounding::ZERO;
            style.visuals.menu_rounding = egui::Rounding::ZERO;
            style.visuals.widgets.noninteractive.rounding = egui::Rounding::ZERO;
            style.visuals.widgets.inactive.rounding = egui::Rounding::ZERO;
            style.visuals.widgets.hovered.rounding = egui::Rounding::ZERO;
            style.visuals.widgets.active.rounding = egui::Rounding::ZERO;
            style.visuals.widgets.open.rounding = egui::Rounding::ZERO;
        });

        let state = State::new(
            ctx.clone(),
            ctx.viewport_id(),
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        let renderer = Renderer::new(device, format, None, 1, false);

        Self {
            ctx,
            state,
            renderer,
            pending_frame: None,
            panel_width: ui_const::PANEL_WIDTH,
            fractal_colors: [
                FractalType::Mandelbrot.default_color_scheme(),
                FractalType::Julia.default_color_scheme(),
                FractalType::BurningShip.default_color_scheme(),
                FractalType::Tricorn.default_color_scheme(),
                FractalType::Celtic.default_color_scheme(),
                FractalType::BuffaloJulia.default_color_scheme(),
                FractalType::CelticJulia.default_color_scheme(),
                FractalType::Newton.default_color_scheme(),
                FractalType::Phoenix.default_color_scheme(),
            ],
        }
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub fn get_panel_width(&self) -> f32 {
        self.panel_width
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
        let mut panel_width = self.panel_width;
        let fractal_colors = &mut self.fractal_colors;

        let full_output = self.ctx.run(raw_input, |ctx| {
            panel_width = Self::build_ui(ctx, params, fractal_colors);
        });
        self.panel_width = panel_width;

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

    fn fractal_type_section(ui: &mut Ui, params: &mut FractalParams, fractal_colors: &mut [u32; 9]) {
        ui.heading("Fractal Type");

        let current = params.get_fractal_type();
        egui::ComboBox::from_id_salt("fractal_type")
            .selected_text(current.name())
            .show_ui(ui, |ui| {
                for ft in FractalType::all() {
                    let selected = *ft == current;
                    if ui.selectable_label(selected, ft.name()).clicked() {
                        // Save current color for current fractal
                        let old_type = params.fractal_type as usize;
                        fractal_colors[old_type] = params.color_scheme;

                        // Switch fractal type
                        params.set_fractal_type(*ft);

                        // Load saved color for new fractal
                        params.color_scheme = fractal_colors[*ft as usize];

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

        // Fractal-specific performance warning thresholds
        let warning_threshold = match params.get_fractal_type() {
            FractalType::Newton => 140,      // 3.5x cost
            FractalType::Phoenix => 330,     // 1.5x cost
            FractalType::BuffaloJulia | FractalType::CelticJulia => 400,  // 1.2x cost
            FractalType::Celtic => 430,      // 1.15x cost
            FractalType::Tricorn | FractalType::BurningShip => 450,  // 1.1x cost
            _ => 500,  // Mandelbrot, Julia (baseline)
        };

        if params.max_iter > warning_threshold {
            ui.colored_label(
                egui::Color32::from_rgb(255, 180, 0),
                "High iterations may reduce performance",
            );
        }

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
        // Show appropriate label based on fractal type
        let label = if params.get_fractal_type() == FractalType::Phoenix {
            "Phoenix Parameters"
        } else {
            "Julia Constant"
        };
        ui.label(label);

        // Parameter sliders
        ui.add(Slider::new(&mut params.julia_c[0], -2.0..=2.0).text("Real"));
        ui.add(Slider::new(&mut params.julia_c[1], -2.0..=2.0).text("Imaginary"));
    }

    fn color_section(ui: &mut Ui, params: &mut FractalParams, fractal_colors: &mut [u32; 9]) {
        ui.label("Color Scheme");

        let current = ColorScheme::from_u32(params.color_scheme);
        egui::ComboBox::from_id_salt("color_scheme")
            .selected_text(current.name())
            .show_ui(ui, |ui| {
                for cs in ColorScheme::all() {
                    let selected = *cs == current;
                    if ui.selectable_label(selected, cs.name()).clicked() {
                        params.color_scheme = *cs as u32;
                        // Save color for current fractal type
                        fractal_colors[params.fractal_type as usize] = params.color_scheme;
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
            FractalType::Celtic => celtic::presets(),
            FractalType::BuffaloJulia => buffalo::julia_presets(),
            FractalType::CelticJulia => celtic::julia_presets(),
            FractalType::Newton => newton::presets(),
            FractalType::Phoenix => phoenix::presets(),
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
            let render_pass = encoder.begin_render_pass(&egui_wgpu::wgpu::RenderPassDescriptor {
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

            // Convert to 'static lifetime as required by egui-wgpu 0.30+
            let mut render_pass = render_pass.forget_lifetime();

            self.renderer
                .render(&mut render_pass, &prepared.shapes, &prepared.screen_descriptor);
        }

        for id in &prepared.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }

    /// Build egui widgets and return the panel width.
    fn build_ui(ctx: &Context, params: &mut FractalParams, fractal_colors: &mut [u32; 9]) -> f32 {
        let response = egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(ui_const::PANEL_WIDTH)
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(8.0))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {

                    Self::fractal_type_section(ui, params, fractal_colors);
                    ui.separator();

                    Self::parameters_section(ui, params);
                    ui.separator();

                    // Show Julia constant for all Julia-type fractals
                    if params.get_fractal_type().needs_julia_c() {
                        Self::julia_section(ui, params);
                        ui.separator();
                    }

                    Self::color_section(ui, params, fractal_colors);
                    ui.separator();

                    Self::navigation_section(ui, params);
                    ui.separator();

                    Self::presets_section(ui, params);
                });
            });
        response.response.rect.width()
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
