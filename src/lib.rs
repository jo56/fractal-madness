use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
#[cfg(not(target_arch = "wasm32"))]
use winit::event::Event;

mod color;
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

        if !consumed {
            match &event {
                WindowEvent::MouseInput { state: btn_state, button, .. } => {
                    self.input.handle_mouse_button(*button, *btn_state == ElementState::Pressed);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if let Some(delta) = self.input.handle_cursor_move(position.x as f32, position.y as f32) {
                        let (width, height) = self.gpu.size;
                        self.params.pan(delta.0, delta.1, width as f32, height as f32);
                        self.renderer.mark_dirty();
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let scroll = match delta {
                        MouseScrollDelta::LineDelta(_, y) => *y,
                        MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                    };
                    self.params.zoom_by(scroll * 0.1);
                    self.renderer.mark_dirty();
                }
                _ => {}
            }
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

                if let Ok(output) = self.gpu.surface.get_current_texture() {
                    let view = output.texture.create_view(&Default::default());
                    let mut encoder = self.gpu.device.create_command_encoder(&Default::default());

                    self.renderer.render(
                        &self.gpu.device,
                        &self.gpu.queue,
                        &mut encoder,
                        &view,
                        &self.params,
                        self.gpu.size,
                    );

                    self.ui.render(
                        &self.gpu.device,
                        &self.gpu.queue,
                        &mut encoder,
                        &view,
                        &self.window,
                        self.gpu.format,
                    );

                    self.gpu.queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                }

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
        show_error(&e);
    }
}

fn show_error(msg: &str) {
    use wasm_bindgen::JsCast;
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(banner) = document.get_element_by_id("error-banner") {
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

        let canvas = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("screen"))
            .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .ok_or("Failed to find canvas element")?;

        let window_attrs = Window::default_attributes()
            .with_title("Fractal Madness")
            .with_inner_size(PhysicalSize::new(1280, 1400))
            .with_canvas(Some(canvas.clone()));

        #[allow(deprecated)]
        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .map_err(|e| format!("Failed to create window: {e}"))?,
        );

        resize_canvas_to_window(&canvas, &window);

        window
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window = {
        let window_attrs = Window::default_attributes()
            .with_title("Fractal Madness")
            .with_inner_size(PhysicalSize::new(1280, 1400));

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

                        if !consumed {
                            match &event {
                                WindowEvent::MouseInput { state: btn_state, button, .. } => {
                                    input.handle_mouse_button(*button, *btn_state == ElementState::Pressed);
                                }
                                WindowEvent::CursorMoved { position, .. } => {
                                    if let Some(delta) = input.handle_cursor_move(position.x as f32, position.y as f32) {
                                        let (width, height) = gpu.size;
                                        params.pan(delta.0, delta.1, width as f32, height as f32);
                                        renderer.mark_dirty();
                                    }
                                }
                                WindowEvent::MouseWheel { delta, .. } => {
                                    let scroll = match delta {
                                        MouseScrollDelta::LineDelta(_, y) => *y,
                                        MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
                                    };
                                    params.zoom_by(scroll * 0.1);
                                    renderer.mark_dirty();
                                }
                                _ => {}
                            }
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

                                if let Ok(output) = gpu.surface.get_current_texture() {
                                    let view = output.texture.create_view(&Default::default());
                                    let mut encoder = gpu.device.create_command_encoder(&Default::default());

                                    renderer.render(
                                        &gpu.device,
                                        &gpu.queue,
                                        &mut encoder,
                                        &view,
                                        &params,
                                        gpu.size,
                                    );

                                    ui.render(
                                        &gpu.device,
                                        &gpu.queue,
                                        &mut encoder,
                                        &view,
                                        &window,
                                        gpu.format,
                                    );

                                    gpu.queue.submit(std::iter::once(encoder.finish()));
                                    output.present();
                                }

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

#[cfg(target_arch = "wasm32")]
fn resize_canvas_to_window(canvas: &web_sys::HtmlCanvasElement, window: &winit::window::Window) {
    let dpr = web_sys::window()
        .map(|w| w.device_pixel_ratio())
        .unwrap_or(1.0);

    let (logical_width, logical_height) = if let Some(parent) = canvas.parent_element() {
        let w = parent.client_width() as f64;
        let h = parent.client_height() as f64;
        if w > 0.0 && h > 0.0 {
            (w, h)
        } else {
            (1280.0, 800.0)
        }
    } else {
        (1280.0, 800.0)
    };

    let width = (logical_width * dpr).round().max(1.0) as u32;
    let height = (logical_height * dpr).round().max(1.0) as u32;

    canvas.set_width(width);
    canvas.set_height(height);

    let logical_size = LogicalSize::new(logical_width, logical_height);
    let _ = window.request_inner_size(logical_size);
}

#[cfg(target_arch = "wasm32")]
fn sync_canvas_size(window: &winit::window::Window, gpu: &mut WebGpuState) {
    use wasm_bindgen::JsCast;

    let Some(canvas) = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("screen"))
        .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
    else {
        return;
    };

    let dpr = web_sys::window()
        .map(|w| w.device_pixel_ratio())
        .unwrap_or(1.0);

    let (logical_width, logical_height) = if let Some(parent) = canvas.parent_element() {
        let w = parent.client_width() as f64;
        let h = parent.client_height() as f64;
        if w > 0.0 && h > 0.0 {
            (w, h)
        } else {
            (1280.0, 800.0)
        }
    } else {
        (1280.0, 800.0)
    };

    let width = (logical_width * dpr).round().max(1.0) as u32;
    let height = (logical_height * dpr).round().max(1.0) as u32;

    if (width, height) != gpu.size {
        canvas.set_width(width);
        canvas.set_height(height);

        let logical_size = LogicalSize::new(logical_width, logical_height);
        let _ = window.request_inner_size(logical_size);
        gpu.resize(width, height);
    }
}
