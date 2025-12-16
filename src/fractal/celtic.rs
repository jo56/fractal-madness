use super::{FractalType, LocationPreset};

/// Interesting location presets for Celtic
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Celtic,
        },
        LocationPreset {
            name: "Knot",
            center: [-0.75, 0.0],
            zoom: 10.0,
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
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
        },
        LocationPreset {
            name: "Detail",
            center: [0.3, 0.3],
            zoom: 5.0,
            fractal_type: FractalType::CelticJulia,
        },
    ]
}
