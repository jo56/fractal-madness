use std::sync::Arc;
use wasm_bindgen::prelude::*;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

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
        use winit::platform::web::WindowBuilderExtWebSys;

        let canvas = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("screen"))
            .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .ok_or("Failed to find canvas element")?;

        let window = Arc::new(
            WindowBuilder::new()
                .with_title("Fractal Madness")
                .with_inner_size(PhysicalSize::new(1280, 1400))
                .with_canvas(Some(canvas.clone()))
                .build(&event_loop)
                .map_err(|e| format!("Failed to create window: {e}"))?,
        );

        resize_canvas_to_window(&canvas, &window);

        window
    };

    #[cfg(not(target_arch = "wasm32"))]
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Fractal Madness")
            .with_inner_size(PhysicalSize::new(1280, 1400))
            .build(&event_loop)
            .map_err(|e| format!("Failed to create window: {e}"))?,
    );

    let mut gpu = WebGpuState::new(window.clone()).await?;
    let mut renderer = FractalRenderer::new(&gpu.device, gpu.format, gpu.size.0, gpu.size.1);
    let mut ui = UiState::new(&gpu.device, gpu.format, &window);
    let mut input = InputState::new();
    let mut params = FractalParams::default();

    log::info!("Initialization complete, starting render loop");

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;

        event_loop.spawn(move |event, target| {
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
                        WindowEvent::CloseRequested => {
                            target.exit();
                        }
                        WindowEvent::Resized(new_size) => {
                            gpu.resize(new_size.width, new_size.height);
                            renderer.resize(&gpu.device, new_size.width, new_size.height);
                        }
                        WindowEvent::RedrawRequested => {
                            #[cfg(target_arch = "wasm32")]
                            sync_canvas_size(&window, &mut gpu);

                            // Update resolution for shader aspect ratio
                            params.resolution = [gpu.size.0 as f32, gpu.size.1 as f32];

                            let ui_changed = ui.prepare(&window, &mut params);
                            if ui_changed {
                                renderer.mark_dirty();
                            }

                            // Calculate ui_offset to center fractal in visible area (excluding panel)
                            // Panel width is in logical points, convert to physical pixels
                            let scale_factor = window.scale_factor() as f32;
                            let panel_width_physical = ui.get_panel_width() * scale_factor;
                            let canvas_width = gpu.size.0 as f32;
                            let canvas_height = gpu.size.1 as f32;
                            let panel_proportion = panel_width_physical / canvas_width;
                            let aspect = canvas_width / canvas_height;
                            // Shift the visible center to the right to account for the left panel
                            params.ui_offset = -panel_proportion * aspect;

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
                    #[cfg(target_arch = "wasm32")]
                    sync_canvas_size(&window, &mut gpu);
                    window.request_redraw();
                }
                _ => {}
            }
        });
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

    // Use fixed dimensions for the canvas
    let logical_width = 1280.0;
    let logical_height = 1400.0;

    // Set inline styles to override CSS constraints
    let _ = canvas.style().set_property("width", "1280px");
    let _ = canvas.style().set_property("height", "1400px");

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

    // Use fixed dimensions for the canvas
    let logical_width = 1280.0;
    let logical_height = 1400.0;

    // Set inline styles to override CSS constraints
    let _ = canvas.style().set_property("width", "1280px");
    let _ = canvas.style().set_property("height", "1400px");

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
