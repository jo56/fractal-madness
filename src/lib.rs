use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
#[cfg(target_arch = "wasm32")]
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event_loop::ActiveEventLoop,
    window::WindowId,
};
#[cfg(not(target_arch = "wasm32"))]
use winit::event::Event;

mod color;
mod constants;
mod fractal;
mod input;
mod renderer;
mod ui;
mod webgpu;

use crate::fractal::FractalParams;
use crate::input::InputState;
use crate::renderer::FractalRenderer;
use crate::ui::UiState;
use crate::webgpu::WebGpuState;

/// Handle input events that affect fractal parameters
/// Returns true if the renderer should be marked dirty
fn handle_input_event(
    event: &WindowEvent,
    input: &mut InputState,
    params: &mut FractalParams,
    gpu_size: (u32, u32),
) -> bool {
    match event {
        WindowEvent::MouseInput { state: btn_state, button, .. } => {
            input.handle_mouse_button(*button, *btn_state == ElementState::Pressed);
            false
        }
        WindowEvent::CursorMoved { position, .. } => {
            if let Some(delta) = input.handle_cursor_move(position.x as f32, position.y as f32) {
                let (width, height) = gpu_size;
                params.pan(delta.0, delta.1, width as f32, height as f32);
                true
            } else {
                false
            }
        }
        WindowEvent::MouseWheel { delta, .. } => {
            let scroll = match delta {
                MouseScrollDelta::LineDelta(_, y) => *y,
                MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
            };
            params.zoom_by(scroll * 0.1);
            true
        }
        _ => false,
    }
}

/// Render a single frame to the screen
fn render_frame(
    gpu: &WebGpuState,
    renderer: &mut FractalRenderer,
    ui: &mut UiState,
    params: &FractalParams,
    window: &Window,
) {
    let Ok(output) = gpu.surface.get_current_texture() else { return };
    let view = output.texture.create_view(&Default::default());
    let mut encoder = gpu.device.create_command_encoder(&Default::default());

    renderer.render(
        &gpu.device,
        &gpu.queue,
        &mut encoder,
        &view,
        params,
        gpu.size,
    );

    ui.render(
        &gpu.device,
        &gpu.queue,
        &mut encoder,
        &view,
        window,
        gpu.format,
    );

    gpu.queue.submit(std::iter::once(encoder.finish()));
    output.present();
}

#[cfg(target_arch = "wasm32")]
struct App {
    window: Arc<Window>,
    gpu: WebGpuState,
    renderer: FractalRenderer,
    ui: UiState,
    input: InputState,
    params: FractalParams,
}

#[cfg(target_arch = "wasm32")]
impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        // Window already created, nothing to do
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let consumed = self.ui.handle_window_event(&self.window, &event);

        if !consumed && handle_input_event(&event, &mut self.input, &mut self.params, self.gpu.size) {
            self.renderer.mark_dirty();
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                self.gpu.resize(new_size.width, new_size.height);
                self.renderer.resize(&self.gpu.device, new_size.width, new_size.height);
            }
            WindowEvent::RedrawRequested => {
                sync_canvas_size(&self.window, &mut self.gpu);

                // Update resolution for shader aspect ratio
                self.params.resolution = [self.gpu.size.0 as f32, self.gpu.size.1 as f32];

                let ui_changed = self.ui.prepare(&self.window, &mut self.params);
                if ui_changed {
                    self.renderer.mark_dirty();
                }

                // Calculate ui_offset to center fractal in visible area (excluding panel)
                let scale_factor = self.window.scale_factor() as f32;
                let panel_width_physical = self.ui.get_panel_width() * scale_factor;
                let canvas_width = self.gpu.size.0 as f32;
                let canvas_height = self.gpu.size.1 as f32;
                let panel_proportion = panel_width_physical / canvas_width;
                let aspect = canvas_width / canvas_height;
                self.params.ui_offset = -panel_proportion * aspect;
                self.params.ui_offset_y = 0.0;

                render_frame(&self.gpu, &mut self.renderer, &mut self.ui, &self.params, &self.window);
                self.window.request_redraw();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        sync_canvas_size(&self.window, &mut self.gpu);
        self.window.request_redraw();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Failed to initialize logger");
    log::info!("Fractal Madness starting...");
}

#[wasm_bindgen]
pub async fn run() {
    if let Err(e) = run_inner().await {
        log::error!("Application error: {}", e);
        #[cfg(target_arch = "wasm32")]
        show_error(&e);
    }
}

#[cfg(target_arch = "wasm32")]
fn show_error(msg: &str) {
    use wasm_bindgen::JsCast;
    use crate::constants::html;
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(banner) = document.get_element_by_id(html::ERROR_BANNER_ID) {
                if let Some(title) = banner.query_selector("h2").ok().flatten() {
                    title.set_text_content(Some("Error"));
                }
                if let Some(text) = banner.query_selector("p").ok().flatten() {
                    text.set_text_content(Some(msg));
                }
                if let Some(el) = banner.dyn_ref::<web_sys::HtmlElement>() {
                    el.set_class_name("");
                }
            }
        }
    }
}

async fn run_inner() -> Result<(), String> {
    let event_loop = EventLoop::new().map_err(|e| format!("Failed to create event loop: {e}"))?;

    #[cfg(target_arch = "wasm32")]
    let window = {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;
        use crate::constants::{canvas, html};

        let canvas_el = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id(html::CANVAS_ID))
            .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .ok_or("Failed to find canvas element")?;

        let window_attrs = Window::default_attributes()
            .with_title("Fractal Madness")
            .with_inner_size(PhysicalSize::new(canvas::DEFAULT_WIDTH, canvas::WASM_WINDOW_HEIGHT))
            .with_canvas(Some(canvas_el.clone()));

        #[allow(deprecated)]
        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .map_err(|e| format!("Failed to create window: {e}"))?,
        );

        resize_canvas_to_window(&canvas_el, &window);

        window
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window = {
        use crate::constants::canvas;
        let window_attrs = Window::default_attributes()
            .with_title("Fractal Madness")
            .with_inner_size(PhysicalSize::new(canvas::DEFAULT_WIDTH, canvas::WASM_WINDOW_HEIGHT));

        #[allow(deprecated)]
        Arc::new(
            event_loop
                .create_window(window_attrs)
                .map_err(|e| format!("Failed to create window: {e}"))?,
        )
    };

    #[allow(unused_mut)] // mut needed for non-WASM targets
    let mut gpu = WebGpuState::new(window.clone()).await?;
    #[allow(unused_mut)]
    let mut renderer = FractalRenderer::new(&gpu.device, gpu.format, gpu.size.0, gpu.size.1);
    #[allow(unused_mut)]
    let mut ui = UiState::new(&gpu.device, gpu.format, &window);
    #[allow(unused_mut)]
    let mut input = InputState::new();
    #[allow(unused_mut)]
    let mut params = FractalParams::default();

    log::info!("Initialization complete, starting render loop");

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;

        let app = App {
            window,
            gpu,
            renderer,
            ui,
            input,
            params,
        };
        event_loop.spawn_app(app);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        event_loop
            .run(move |event, target| {
                match event {
                    Event::WindowEvent { event, .. } => {
                        let consumed = ui.handle_window_event(&window, &event);

                        if !consumed && handle_input_event(&event, &mut input, &mut params, gpu.size) {
                            renderer.mark_dirty();
                        }

                        match event {
                            WindowEvent::CloseRequested => target.exit(),
                            WindowEvent::Resized(new_size) => {
                                gpu.resize(new_size.width, new_size.height);
                                renderer.resize(&gpu.device, new_size.width, new_size.height);
                            }
                            WindowEvent::RedrawRequested => {
                                // Update resolution for shader aspect ratio
                                params.resolution = [gpu.size.0 as f32, gpu.size.1 as f32];

                                let ui_changed = ui.prepare(&window, &mut params);
                                if ui_changed {
                                    renderer.mark_dirty();
                                }

                                render_frame(&gpu, &mut renderer, &mut ui, &params, &window);
                                window.request_redraw();
                            }
                            _ => {}
                        }
                    }
                    Event::AboutToWait => {
                        window.request_redraw();
                    }
                    _ => {}
                }
            })
            .map_err(|e| format!("Event loop error: {e}"))?;
    }

    Ok(())
}

/// Calculate canvas dimensions based on parent element size
#[cfg(target_arch = "wasm32")]
fn calculate_canvas_dimensions(canvas: &web_sys::HtmlCanvasElement) -> (f64, f64, u32, u32) {
    use crate::constants::{canvas as canvas_const, FRAME_BORDER_ADJUSTMENT};

    let dpr = web_sys::window()
        .map(|w| w.device_pixel_ratio())
        .unwrap_or(1.0);

    let default_size = (
        canvas_const::DEFAULT_WIDTH as f64,
        canvas_const::DEFAULT_HEIGHT as f64,
    );

    // Subtract frame size for gray border
    let (logical_width, logical_height) = if let Some(parent) = canvas.parent_element() {
        let w = (parent.client_width() as f64) - FRAME_BORDER_ADJUSTMENT;
        let h = (parent.client_height() as f64) - FRAME_BORDER_ADJUSTMENT;
        if w > 0.0 && h > 0.0 {
            (w, h)
        } else {
            default_size
        }
    } else {
        default_size
    };

    let width = (logical_width * dpr).round().max(1.0) as u32;
    let height = (logical_height * dpr).round().max(1.0) as u32;

    (logical_width, logical_height, width, height)
}

/// Apply canvas size to both the canvas element and window
#[cfg(target_arch = "wasm32")]
fn apply_canvas_size(
    canvas: &web_sys::HtmlCanvasElement,
    window: &winit::window::Window,
    logical_width: f64,
    logical_height: f64,
    width: u32,
    height: u32,
) {
    canvas.set_width(width);
    canvas.set_height(height);
    let logical_size = LogicalSize::new(logical_width, logical_height);
    let _ = window.request_inner_size(logical_size);
}

#[cfg(target_arch = "wasm32")]
fn resize_canvas_to_window(canvas: &web_sys::HtmlCanvasElement, window: &winit::window::Window) {
    let (logical_width, logical_height, width, height) = calculate_canvas_dimensions(canvas);
    apply_canvas_size(canvas, window, logical_width, logical_height, width, height);
}

#[cfg(target_arch = "wasm32")]
fn sync_canvas_size(window: &winit::window::Window, gpu: &mut WebGpuState) {
    use wasm_bindgen::JsCast;
    use crate::constants::html;

    let Some(canvas) = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id(html::CANVAS_ID))
        .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
    else {
        return;
    };

    let (logical_width, logical_height, width, height) = calculate_canvas_dimensions(&canvas);

    if (width, height) != gpu.size {
        apply_canvas_size(&canvas, window, logical_width, logical_height, width, height);
        gpu.resize(width, height);
    }
}
