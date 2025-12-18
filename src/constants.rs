//! Application-wide constants

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
