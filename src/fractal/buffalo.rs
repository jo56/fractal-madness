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
        // Higher power variants
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::BuffaloJulia, power: Some(3.0),
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::BuffaloJulia, power: Some(4.0),
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::BuffaloJulia, power: Some(5.0),
        },
    ]
}
