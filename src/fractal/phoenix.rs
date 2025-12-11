use super::{FractalType, LocationPreset};

/// Interesting location presets for Phoenix fractal
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Phoenix,
        },
        LocationPreset {
            name: "Wing",
            center: [0.3, 0.5],
            zoom: 10.0,
            fractal_type: FractalType::Phoenix,
        },
        LocationPreset {
            name: "Feathers",
            center: [-0.5, 0.2],
            zoom: 50.0,
            fractal_type: FractalType::Phoenix,
        },
        LocationPreset {
            name: "Tail",
            center: [0.0, -0.8],
            zoom: 20.0,
            fractal_type: FractalType::Phoenix,
        },
    ]
}
