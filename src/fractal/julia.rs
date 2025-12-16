use super::{FractalParams, FractalType, LocationPreset};

/// Default Julia parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [0.0, 0.0],
        zoom: 0.8,
        max_iter: 256,
        power: 2.0,
        escape_radius: 4.0,
        fractal_type: FractalType::Julia as u32,
        julia_c: [-0.7, 0.27015],
        ..Default::default()
    }
}

/// Interesting Julia set constant presets
#[derive(Debug, Clone)]
pub struct JuliaPreset {
    pub name: &'static str,
    pub c: [f32; 2],
}

pub fn julia_presets() -> Vec<JuliaPreset> {
    vec![
        JuliaPreset {
            name: "Classic",
            c: [-0.7, 0.27015],
        },
        JuliaPreset {
            name: "Dragon",
            c: [-0.8, 0.156],
        },
        JuliaPreset {
            name: "San Marco",
            c: [-0.75, 0.0],
        },
        JuliaPreset {
            name: "Siegel Disk",
            c: [-0.391, -0.587],
        },
        JuliaPreset {
            name: "Dendrite",
            c: [0.0, 1.0],
        },
        JuliaPreset {
            name: "Spiral",
            c: [-0.4, 0.6],
        },
        JuliaPreset {
            name: "Douady Rabbit",
            c: [-0.123, 0.745],
        },
        JuliaPreset {
            name: "Snowflake",
            c: [0.285, 0.01],
        },
        JuliaPreset {
            name: "Galaxies",
            c: [-0.7269, 0.1889],
        },
        JuliaPreset {
            name: "Lightning",
            c: [-0.162, 1.04],
        },
    ]
}

/// Location presets for Julia (different views of same Julia set)
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia, power: None,
        },
        LocationPreset {
            name: "Spiral Detail",
            center: [0.3, 0.3],
            zoom: 5.0,
            fractal_type: FractalType::Julia, power: None,
        },
        LocationPreset {
            name: "Edge",
            center: [1.0, 0.0],
            zoom: 3.0,
            fractal_type: FractalType::Julia, power: None,
        },
        // Higher power variants
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia, power: Some(3.0),
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia, power: Some(4.0),
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Julia, power: Some(5.0),
        },
    ]
}
