//! Application-wide constants

/// Input handling constants
pub mod input {
    /// Divisor for converting pixel scroll delta to normalized scroll value
    pub const SCROLL_PIXEL_DIVISOR: f32 = 100.0;
    /// Multiplier for converting scroll value to zoom factor
    pub const ZOOM_SCROLL_MULTIPLIER: f32 = 0.1;
}

/// Performance warning thresholds per fractal type
pub mod performance {
    /// Newton fractal iteration warning threshold (3.5x cost)
    pub const NEWTON_WARNING_THRESHOLD: u32 = 140;
    /// Phoenix fractal iteration warning threshold (1.5x cost)
    pub const PHOENIX_WARNING_THRESHOLD: u32 = 330;
    /// Julia variant (Buffalo/Celtic) iteration warning threshold (1.2x cost)
    pub const JULIA_VARIANT_WARNING_THRESHOLD: u32 = 400;
    /// Celtic fractal iteration warning threshold (1.15x cost)
    pub const CELTIC_WARNING_THRESHOLD: u32 = 430;
    /// Escape variant (Tricorn/BurningShip) iteration warning threshold (1.1x cost)
    pub const ESCAPE_VARIANT_WARNING_THRESHOLD: u32 = 450;
    /// Baseline (Mandelbrot/Julia) iteration warning threshold
    pub const BASELINE_WARNING_THRESHOLD: u32 = 500;
}

/// Default canvas dimensions
pub mod canvas {
    /// Default canvas width
    pub const DEFAULT_WIDTH: u32 = 1280;
    /// Default canvas height (used in WASM canvas sizing)
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub const DEFAULT_HEIGHT: u32 = 800;
    /// Extended height for WASM window
    pub const WASM_WINDOW_HEIGHT: u32 = 1400;
}

/// UI panel dimensions
pub mod ui {
    /// Default side panel width in logical pixels
    pub const PANEL_WIDTH: f32 = 280.0;
    /// Minimum iteration count for the slider
    pub const MIN_ITERATIONS: f32 = 10.0;
    /// Maximum iteration count for the slider
    pub const MAX_ITERATIONS: f32 = 10000.0;
    /// Warning text color (orange) as RGB values
    pub const WARNING_COLOR: (u8, u8, u8) = (255, 180, 0);
}

/// HTML element IDs for WASM integration
#[cfg(target_arch = "wasm32")]
pub mod html {
    /// Canvas element ID
    pub const CANVAS_ID: &str = "screen";
    /// Error banner element ID
    pub const ERROR_BANNER_ID: &str = "error-banner";
}

/// Frame border adjustment (8px each side = 16px total)
#[cfg(target_arch = "wasm32")]
pub const FRAME_BORDER_ADJUSTMENT: f64 = 16.0;
