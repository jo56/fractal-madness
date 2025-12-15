use super::{FractalType, LocationPreset};

/// Interesting location presets for Heart fractal
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.4,
            fractal_type: FractalType::Heart,
        },
        LocationPreset {
            name: "Chamber",
            center: [0.0, 0.5],
            zoom: 10.0,
            fractal_type: FractalType::Heart,
        },
        LocationPreset {
            name: "Edge",
            center: [-0.5, 0.3],
            zoom: 20.0,
            fractal_type: FractalType::Heart,
        },
    ]
}
