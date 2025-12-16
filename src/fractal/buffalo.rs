use super::{FractalType, LocationPreset};

/// Presets for Buffalo Julia
pub fn julia_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::BuffaloJulia, power: Some(2.0),
        },
        LocationPreset {
            name: "Detail",
            center: [0.25, 0.25],
            zoom: 5.0,
            fractal_type: FractalType::BuffaloJulia, power: Some(2.0),
        },
    ]
}
