use super::{FractalType, LocationPreset};

/// Interesting location presets for Celtic
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.5, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Celtic,
        },
        LocationPreset {
            name: "Knot",
            center: [-0.1, 0.65],
            zoom: 50.0,
            fractal_type: FractalType::Celtic,
        },
        LocationPreset {
            name: "Weave",
            center: [-1.25, 0.0],
            zoom: 100.0,
            fractal_type: FractalType::Celtic,
        },
    ]
}

/// Presets for Celtic Julia
pub fn julia_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::CelticJulia,
        },
    ]
}
