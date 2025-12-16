use super::{FractalType, LocationPreset};

/// Interesting location presets for Tricorn (Mandelbar)
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Tricorn,
        },
        LocationPreset {
            name: "Antenna",
            center: [-1.1, 0.0],
            zoom: 10.0,
            fractal_type: FractalType::Tricorn,
        },
        LocationPreset {
            name: "Spiral",
            center: [0.25, 0.5],
            zoom: 20.0,
            fractal_type: FractalType::Tricorn,
        },
    ]
}
