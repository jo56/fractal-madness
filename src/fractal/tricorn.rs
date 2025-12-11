use super::{FractalType, LocationPreset};

/// Interesting location presets for Tricorn (Mandelbar)
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.3, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Tricorn,
        },
        LocationPreset {
            name: "Spiral Arms",
            center: [-0.1, 0.87],
            zoom: 50.0,
            fractal_type: FractalType::Tricorn,
        },
        LocationPreset {
            name: "Detail",
            center: [-1.0, 0.3],
            zoom: 100.0,
            fractal_type: FractalType::Tricorn,
        },
        LocationPreset {
            name: "Edge",
            center: [0.3, 0.5],
            zoom: 200.0,
            fractal_type: FractalType::Tricorn,
        },
    ]
}

/// Presets for Tricorn Julia
pub fn julia_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::TricornJulia,
        },
        LocationPreset {
            name: "Detail",
            center: [0.0, 0.5],
            zoom: 10.0,
            fractal_type: FractalType::TricornJulia,
        },
    ]
}
