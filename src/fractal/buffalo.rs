use super::{FractalType, LocationPreset};

/// Interesting location presets for Buffalo
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.5, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Buffalo,
        },
        LocationPreset {
            name: "Horns",
            center: [-1.2, 0.3],
            zoom: 50.0,
            fractal_type: FractalType::Buffalo,
        },
        LocationPreset {
            name: "Spiral",
            center: [-0.2, 0.8],
            zoom: 100.0,
            fractal_type: FractalType::Buffalo,
        },
    ]
}

/// Presets for Buffalo Julia
pub fn julia_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::BuffaloJulia,
        },
    ]
}
