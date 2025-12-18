use super::{FractalType, LocationPreset};

/// Location presets for Julia (includes Julia constant variations)
pub fn presets() -> Vec<LocationPreset> {
    vec![
        // Julia constant presets
        LocationPreset {
            name: "Classic",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.7, 0.27015]),
        },
        LocationPreset {
            name: "Dragon",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.8, 0.156]),
        },
        LocationPreset {
            name: "San Marco",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.75, 0.0]),
        },
        LocationPreset {
            name: "Siegel Disk",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.391, -0.587]),
        },
        LocationPreset {
            name: "Dendrite",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([0.0, 1.0]),
        },
        LocationPreset {
            name: "Spiral",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.4, 0.6]),
        },
        LocationPreset {
            name: "Douady Rabbit",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.123, 0.745]),
        },
        LocationPreset {
            name: "Snowflake",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([0.285, 0.01]),
        },
        LocationPreset {
            name: "Galaxies",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.7269, 0.1889]),
        },
        LocationPreset {
            name: "Lightning",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(2.0),
            julia_c: Some([-0.162, 1.04]),
        },
        // Higher power variants
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(3.0),
            julia_c: Some([-0.7, 0.27015]),
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(4.0),
            julia_c: Some([-0.7, 0.27015]),
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia,
            power: Some(5.0),
            julia_c: Some([-0.7, 0.27015]),
        },
    ]
}
