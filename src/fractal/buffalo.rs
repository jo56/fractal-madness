use super::{FractalType, LocationPreset};

/// Interesting location presets for Buffalo
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Buffalo,
        },
        LocationPreset {
            name: "Horn",
            center: [-0.5, 0.5],
            zoom: 15.0,
            fractal_type: FractalType::Buffalo,
        },
        LocationPreset {
            name: "Edge Detail",
            center: [0.3, 0.0],
            zoom: 30.0,
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
            zoom: 0.5,
            fractal_type: FractalType::BuffaloJulia,
        },
        LocationPreset {
            name: "Detail",
            center: [0.25, 0.25],
            zoom: 5.0,
            fractal_type: FractalType::BuffaloJulia,
        },
    ]
}
