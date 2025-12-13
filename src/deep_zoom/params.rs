//! Deep zoom parameters and state management.

use num_bigfloat::BigFloat;

/// Rendering mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    /// Standard f32 fragment shader (fast, limited zoom depth ~10^13)
    Standard,
    /// Deep zoom with perturbation theory (slower, unlimited zoom depth)
    Deep,
}

impl RenderMode {
    /// Automatically select render mode based on zoom level
    pub fn auto_select(log10_zoom: f64) -> Self {
        if log10_zoom > 10.0 {
            RenderMode::Deep
        } else {
            RenderMode::Standard
        }
    }
}

/// Parameters for deep zoom rendering using perturbation theory.
#[derive(Clone)]
pub struct DeepZoomParams {
    /// Real component of center in arbitrary precision
    pub center_re: BigFloat,
    /// Imaginary component of center in arbitrary precision
    pub center_im: BigFloat,

    /// Zoom level as log10 (e.g., 100.0 means 10^100 zoom)
    pub log10_zoom: f64,

    /// Maximum iterations for escape-time algorithm
    pub max_iter: u32,

    /// Escape radius squared
    pub escape_radius_sq: f64,

    /// Precision in bits (scales with zoom depth)
    pub precision: u32,

    /// Whether deep zoom mode is enabled
    pub enabled: bool,

    /// Whether reference orbit needs recalculation
    pub reference_dirty: bool,

    /// Current render mode
    pub render_mode: RenderMode,

    /// Fractal type (0 = Mandelbrot, matches FractalType enum)
    pub fractal_type: u32,

    /// Color scheme index
    pub color_scheme: u32,
}

impl Default for DeepZoomParams {
    fn default() -> Self {
        Self {
            center_re: BigFloat::from(-0.5),
            center_im: BigFloat::from(0.0),
            log10_zoom: 0.0,
            max_iter: 1000,
            escape_radius_sq: 16.0,
            precision: 128,
            enabled: false,
            reference_dirty: true,
            render_mode: RenderMode::Standard,
            fractal_type: 0,
            color_scheme: 0,
        }
    }
}

impl DeepZoomParams {
    /// Create new deep zoom params centered at given coordinates
    pub fn new(center_re: f64, center_im: f64) -> Self {
        Self {
            center_re: BigFloat::from(center_re),
            center_im: BigFloat::from(center_im),
            ..Default::default()
        }
    }

    /// Get the actual zoom factor (10^log10_zoom)
    pub fn zoom_factor(&self) -> f64 {
        10.0_f64.powf(self.log10_zoom)
    }

    /// Zoom in/out by a delta factor
    pub fn zoom_by(&mut self, delta: f64) {
        // Delta is typically 0.1 for scroll wheel
        self.log10_zoom += delta * 0.5;
        self.log10_zoom = self.log10_zoom.max(0.0);

        // Update precision based on zoom depth
        self.update_precision();

        // Mark reference as dirty when zooming
        self.reference_dirty = true;

        // Auto-select render mode
        self.render_mode = RenderMode::auto_select(self.log10_zoom);
    }

    /// Pan the view by pixel delta
    pub fn pan(&mut self, dx: f64, dy: f64, width: f64, height: f64) {
        // Calculate complex plane delta
        let zoom = self.zoom_factor();
        let aspect = width / height;
        let scale = 2.0 / zoom;

        // Calculate delta in complex plane coordinates
        let delta_re = (dx / width) * scale * aspect * 2.0;
        let delta_im = (dy / height) * scale * 2.0;

        // Subtract from center (pan in opposite direction of mouse movement)
        self.center_re = self.center_re.clone() - BigFloat::from(delta_re);
        self.center_im = self.center_im.clone() - BigFloat::from(delta_im);

        // Mark reference as dirty
        self.reference_dirty = true;
    }

    /// Update precision based on zoom depth
    fn update_precision(&mut self) {
        // Need approximately log2(zoom) + 64 bits of precision
        // log2(10^x) = x * log2(10) ≈ x * 3.322
        let needed_bits = (self.log10_zoom * 3.322) as u32 + 64;
        self.precision = needed_bits.max(128).min(4096);
    }

    /// Reset to default view
    pub fn reset(&mut self) {
        self.center_re = BigFloat::from(-0.5);
        self.center_im = BigFloat::from(0.0);
        self.log10_zoom = 0.0;
        self.precision = 128;
        self.reference_dirty = true;
        self.render_mode = RenderMode::Standard;
    }

    /// Set center from string coordinates (for precise input)
    pub fn set_center_from_strings(&mut self, re: &str, im: &str) -> Result<(), String> {
        self.center_re = BigFloat::parse(re)
            .ok_or_else(|| format!("Invalid real coordinate: {}", re))?;
        self.center_im = BigFloat::parse(im)
            .ok_or_else(|| format!("Invalid imaginary coordinate: {}", im))?;
        self.reference_dirty = true;
        Ok(())
    }

    /// Get center as f64 (for display/UI, may lose precision)
    pub fn center_f64(&self) -> (f64, f64) {
        (self.center_re.to_f64(), self.center_im.to_f64())
    }

    /// Sync from standard FractalParams when switching modes
    pub fn sync_from_standard(&mut self, center: [f32; 2], zoom: f32) {
        self.center_re = BigFloat::from(center[0] as f64);
        self.center_im = BigFloat::from(center[1] as f64);
        self.log10_zoom = zoom.log10() as f64;
        self.reference_dirty = true;
        self.update_precision();
    }

    /// Export to standard f32 values (for standard renderer, may lose precision)
    pub fn to_standard(&self) -> ([f32; 2], f32) {
        let center = [self.center_re.to_f64() as f32, self.center_im.to_f64() as f32];
        let zoom = self.zoom_factor() as f32;
        (center, zoom)
    }
}
