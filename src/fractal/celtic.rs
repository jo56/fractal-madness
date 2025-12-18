use super::{FractalType, LocationPreset};

/// Interesting location presets for Celtic
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Celtic,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Knot",
            center: [-0.75, 0.0],
            zoom: 10.0,
            fractal_type: FractalType::Celtic,
            power: Some(2.0),
            julia_c: None,
        },
        // Higher power variants
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Celtic,
            power: Some(3.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Celtic,
            power: Some(4.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Celtic,
            power: Some(5.0),
            julia_c: None,
        },
    ]
}

/// Presets for Celtic Julia (includes Julia constant variations)
pub fn julia_presets() -> Vec<LocationPreset> {
    vec![
        // Julia constant presets
        LocationPreset {
            name: "Classic",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
            power: Some(2.0),
            julia_c: Some([-0.7, 0.27015]),
        },
        LocationPreset {
            name: "Dragon",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
            power: Some(2.0),
            julia_c: Some([-0.8, 0.156]),
        },
        LocationPreset {
            name: "San Marco",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
            power: Some(2.0),
            julia_c: Some([-0.75, 0.0]),
        },
        LocationPreset {
            name: "Siegel Disk",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
            power: Some(2.0),
            julia_c: Some([-0.391, -0.587]),
        },
        LocationPreset {
            name: "Spiral",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
            power: Some(2.0),
            julia_c: Some([-0.4, 0.6]),
        },
        LocationPreset {
            name: "Snowflake",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::CelticJulia,
            power: Some(2.0),
            julia_c: Some([0.285, 0.01]),
        },
    ]
}
