use super::{FractalParams, FractalType, LocationPreset};

/// Default Nova fractal parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [0.0, 0.0],
        zoom: 0.8,
        max_iter: 256,
        power: 3.0,
        escape_radius: 4.0,
        fractal_type: FractalType::Nova as u32,
        julia_c: [0.5, 0.0],
        ..Default::default()
    }
}

/// Nova constant presets (like Julia presets)
#[derive(Debug, Clone)]
pub struct NovaPreset {
    pub name: &'static str,
    pub c: [f32; 2],
}

pub fn nova_c_presets() -> Vec<NovaPreset> {
    vec![
        NovaPreset {
            name: "Classic",
            c: [0.5, 0.0],
        },
        NovaPreset {
            name: "Lace",
            c: [1.0, 0.0],
        },
        NovaPreset {
            name: "Spiral",
            c: [0.0, 0.5],
        },
        NovaPreset {
            name: "Starburst",
            c: [-0.5, 0.5],
        },
        NovaPreset {
            name: "Web",
            c: [0.25, 0.25],
        },
        NovaPreset {
            name: "Crystal",
            c: [0.3, -0.3],
        },
    ]
}

/// Interesting location presets for Nova fractal
/// Nova combines Newton's method with Julia-style constant: z = z - (z^n - 1)/(nz^(n-1)) + c
/// With default c=(0.5, 0), the Newton structure is displaced. Target boundary regions.
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 1.0,
            fractal_type: FractalType::Nova,
        },
        LocationPreset {
            name: "Displaced Junction",
            center: [0.3, 0.0],
            zoom: 3.0,
            fractal_type: FractalType::Nova,
        },
    ]
}
