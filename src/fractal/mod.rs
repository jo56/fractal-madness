use bytemuck::{Pod, Zeroable};

pub mod burning_ship;
pub mod buffalo;
pub mod celtic;
pub mod julia;
pub mod mandelbrot;
pub mod newton;
pub mod phoenix;
pub mod tricorn;

/// Fractal type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FractalType {
    // Classic escape-time fractals
    Mandelbrot = 0,
    Julia = 1,
    BurningShip = 2,
    Tricorn = 3,
    Celtic = 4,
    // Julia variants
    BuffaloJulia = 5,
    CelticJulia = 6,
    // Advanced fractals
    Newton = 7,
    Phoenix = 8,
}

impl FractalType {
    pub fn all() -> &'static [FractalType] {
        &[
            // Classic fractals
            FractalType::Mandelbrot,
            FractalType::Tricorn,
            FractalType::Celtic,
            FractalType::BurningShip,
            // Julia variants
            FractalType::Julia,
            FractalType::BuffaloJulia,
            FractalType::CelticJulia,
            // Advanced fractals
            FractalType::Newton,
            FractalType::Phoenix,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot",
            FractalType::Julia => "Julia",
            FractalType::BurningShip => "Burning Ship",
            FractalType::Tricorn => "Tricorn",
            FractalType::Celtic => "Celtic",
            FractalType::BuffaloJulia => "Buffalo Julia",
            FractalType::CelticJulia => "Celtic Julia",
            FractalType::Newton => "Newton",
            FractalType::Phoenix => "Phoenix",
        }
    }

    /// Returns true if this fractal type uses the julia_c parameter
    pub fn needs_julia_c(&self) -> bool {
        matches!(
            self,
            FractalType::Julia
                | FractalType::BuffaloJulia
                | FractalType::CelticJulia
        )
    }

    /// Returns the recommended default color scheme for this fractal type
    pub fn default_color_scheme(&self) -> u32 {
        match self {
            FractalType::Mandelbrot => 0,                  // Classic
            FractalType::Julia => 0,                       // Classic
            FractalType::BurningShip => 1,                 // Fire
            FractalType::Tricorn => 5,                     // Electric
            FractalType::Celtic => 8,                      // Forest
            FractalType::BuffaloJulia => 11,               // Plasma
            FractalType::CelticJulia => 19,                // Aurora
            FractalType::Newton => 3,                      // Rainbow (shows root basins)
            FractalType::Phoenix => 12,                    // Cosmic
        }
    }
}

/// Fractal rendering parameters
/// Must match the WGSL struct layout exactly and stay 16-byte aligned for uniforms
/// Total size: 64 bytes
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct FractalParams {
    pub center: [f32; 2],       // offset 0  (8 bytes)
    pub zoom: f32,               // offset 8  (4 bytes)
    pub max_iter: u32,           // offset 12 (4 bytes)
    pub power: f32,              // offset 16 (4 bytes)
    pub escape_radius: f32,      // offset 20 (4 bytes)
    pub fractal_type: u32,       // offset 24 (4 bytes)
    pub color_scheme: u32,       // offset 28 (4 bytes)
    pub julia_c: [f32; 2],       // offset 32 (8 bytes)
    pub flags: u32,              // offset 40 (4 bytes)
    pub _pad: u32,               // offset 44 (4 bytes)
    pub resolution: [f32; 2],    // offset 48 (8 bytes)
    pub ui_offset: f32,          // offset 56 (4 bytes) - horizontal offset for UI panel
    pub ui_offset_y: f32,        // offset 60 (4 bytes) - vertical offset for centering
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            center: [-0.5, 0.0],
            zoom: 1.0,
            max_iter: 256,
            power: 2.0,
            escape_radius: 4.0,
            fractal_type: FractalType::Mandelbrot as u32,
            color_scheme: 0,
            julia_c: [-0.7, 0.27015],
            flags: 1, // smooth coloring on by default
            _pad: 0,
            resolution: [1280.0, 1400.0], // default, will be updated each frame
            ui_offset: 0.0,              // will be updated each frame based on UI panel width
            ui_offset_y: 0.0,            // will be updated each frame for vertical centering
        }
    }
}

impl FractalParams {
    // Flag bit constants
    pub const FLAG_SMOOTH: u32 = 1;
    pub const FLAG_INVERT: u32 = 2;
    pub const FLAG_OFFSET: u32 = 4;

    pub fn get_fractal_type(&self) -> FractalType {
        match self.fractal_type {
            0 => FractalType::Mandelbrot,
            1 => FractalType::Julia,
            2 => FractalType::BurningShip,
            3 => FractalType::Tricorn,
            4 => FractalType::Celtic,
            5 => FractalType::BuffaloJulia,
            6 => FractalType::CelticJulia,
            7 => FractalType::Newton,
            8 => FractalType::Phoenix,
            _ => FractalType::Mandelbrot,
        }
    }

    pub fn set_fractal_type(&mut self, t: FractalType) {
        self.fractal_type = t as u32;
    }

    pub fn smooth(&self) -> bool {
        (self.flags & Self::FLAG_SMOOTH) != 0
    }

    pub fn set_smooth(&mut self, v: bool) {
        if v {
            self.flags |= Self::FLAG_SMOOTH;
        } else {
            self.flags &= !Self::FLAG_SMOOTH;
        }
    }

    pub fn invert(&self) -> bool {
        (self.flags & Self::FLAG_INVERT) != 0
    }

    pub fn set_invert(&mut self, v: bool) {
        if v {
            self.flags |= Self::FLAG_INVERT;
        } else {
            self.flags &= !Self::FLAG_INVERT;
        }
    }

    pub fn offset(&self) -> bool {
        (self.flags & Self::FLAG_OFFSET) != 0
    }

    pub fn set_offset(&mut self, v: bool) {
        if v {
            self.flags |= Self::FLAG_OFFSET;
        } else {
            self.flags &= !Self::FLAG_OFFSET;
        }
    }

    pub fn pan(&mut self, dx: f32, dy: f32, width: f32, height: f32) {
        // Convert pixel delta to normalized coordinates, then to complex plane
        let scale = 2.0 / self.zoom;
        let aspect = width / height;
        self.center[0] -= (dx / width) * scale * aspect * 2.0;
        self.center[1] -= (dy / height) * scale * 2.0;
    }

    pub fn zoom_by(&mut self, delta: f32) {
        self.zoom *= (1.0 + delta).max(0.1);
        self.zoom = self.zoom.clamp(1e-10, 1e10);
    }

    pub fn reset(&mut self) {
        let fractal_type = self.fractal_type;
        let color_scheme = self.color_scheme;
        let flags = self.flags;

        *self = Self::default();
        self.fractal_type = fractal_type;
        self.color_scheme = color_scheme;
        self.flags = flags;

        // Apply fractal-specific defaults
        match self.get_fractal_type() {
            FractalType::Mandelbrot => {
                self.center = [-0.5, 0.0];
            }
            FractalType::Julia => {
                self.center = [0.0, 0.0];
            }
            FractalType::BurningShip => {
                self.center = [-0.4, -0.6];
            }
            FractalType::Tricorn => {
                self.center = [-0.3, 0.0];
            }
            FractalType::Celtic => {
                self.center = [-0.5, 0.0];
            }
            FractalType::BuffaloJulia
            | FractalType::CelticJulia => {
                self.center = [0.0, 0.0];
            }
            FractalType::Newton => {
                self.center = [0.0, 0.0];
                self.zoom = 0.8;
                self.max_iter = 15;  // Newton is expensive, start low
            }
            FractalType::Phoenix => {
                self.center = [0.0, 0.0];
                self.zoom = 0.5;
            }
        }
    }
}

/// Location preset
#[derive(Debug, Clone)]
pub struct LocationPreset {
    pub name: &'static str,
    pub center: [f32; 2],
    pub zoom: f32,
    pub fractal_type: FractalType,
    pub power: Option<f32>,
}

impl LocationPreset {
    pub fn apply(&self, params: &mut FractalParams) {
        params.center = self.center;
        params.zoom = self.zoom;
        params.set_fractal_type(self.fractal_type);
        if let Some(p) = self.power {
            params.power = p;
        }
    }
}
