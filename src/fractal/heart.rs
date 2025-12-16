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
    ]
}
