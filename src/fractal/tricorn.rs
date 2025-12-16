use super::{FractalType, LocationPreset};

/// Interesting location presets for Tricorn (Mandelbar)
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Tricorn, power: None,
        },
        LocationPreset {
            name: "Antenna",
            center: [-1.1, 0.0],
            zoom: 10.0,
            fractal_type: FractalType::Tricorn, power: None,
        },
        LocationPreset {
            name: "Spiral",
            center: [0.25, 0.5],
            zoom: 20.0,
            fractal_type: FractalType::Tricorn, power: None,
        },
        // Higher power variants
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Tricorn, power: Some(3.0),
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Tricorn, power: Some(4.0),
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Tricorn, power: Some(5.0),
        },
    ]
}
