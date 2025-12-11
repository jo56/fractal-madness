use super::{FractalType, LocationPreset};

/// Interesting location presets for Heart fractal
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Heart,
        },
        LocationPreset {
            name: "Chambers",
            center: [-0.2, 0.8],
            zoom: 50.0,
            fractal_type: FractalType::Heart,
        },
        LocationPreset {
            name: "Arteries",
            center: [-1.5, 0.0],
            zoom: 100.0,
            fractal_type: FractalType::Heart,
        },
    ]
}
