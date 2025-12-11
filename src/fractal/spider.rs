use super::{FractalType, LocationPreset};

/// Interesting location presets for Spider fractal
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Spider,
        },
        LocationPreset {
            name: "Web",
            center: [-0.5, 0.5],
            zoom: 10.0,
            fractal_type: FractalType::Spider,
        },
        LocationPreset {
            name: "Legs",
            center: [0.3, -0.3],
            zoom: 50.0,
            fractal_type: FractalType::Spider,
        },
        LocationPreset {
            name: "Center",
            center: [0.0, 0.0],
            zoom: 5.0,
            fractal_type: FractalType::Spider,
        },
    ]
}
