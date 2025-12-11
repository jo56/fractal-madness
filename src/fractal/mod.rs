use bytemuck::{Pod, Zeroable};

pub mod burning_ship;
pub mod julia;
pub mod mandelbrot;

/// Fractal type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FractalType {
    Mandelbrot = 0,
    Julia = 1,
    BurningShip = 2,
}

impl FractalType {
    pub fn all() -> &'static [FractalType] {
        &[
            FractalType::Mandelbrot,
            FractalType::Julia,
            FractalType::BurningShip,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            FractalType::Mandelbrot => "Mandelbrot",
            FractalType::Julia => "Julia",
            FractalType::BurningShip => "Burning Ship",
        }
    }
}

/// Fractal rendering parameters
/// Must match the WGSL struct layout exactly and stay 16-byte aligned for uniforms
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct FractalParams {
    pub center: [f32; 2],
    pub zoom: f32,
    pub max_iter: u32,
    pub power: f32,
    pub escape_radius: f32,
    pub fractal_type: u32,
    pub color_scheme: u32,
    pub julia_c: [f32; 2],
    pub flags: u32,
    pub _pad: u32,  // Explicit padding to reach 48 bytes (multiple of 16)
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            center: [-0.5, 0.0],
            zoom: 0.8,
            max_iter: 256,
            power: 2.0,
            escape_radius: 4.0,
            fractal_type: FractalType::Mandelbrot as u32,
            color_scheme: 0,
            julia_c: [-0.7, 0.27015],
            flags: 1, // smooth coloring on by default
            _pad: 0,
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
}

impl LocationPreset {
    pub fn apply(&self, params: &mut FractalParams) {
        params.center = self.center;
        params.zoom = self.zoom;
        params.set_fractal_type(self.fractal_type);
    }
}
