use super::{FractalType, LocationPreset};

/// Interesting location presets for Newton z³-1
pub fn z3_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::NewtonZ3,
        },
        LocationPreset {
            name: "Junction",
            center: [0.0, 0.0],
            zoom: 5.0,
            fractal_type: FractalType::NewtonZ3,
        },
        LocationPreset {
            name: "Detail",
            center: [0.5, 0.866],
            zoom: 20.0,
            fractal_type: FractalType::NewtonZ3,
        },
    ]
}

/// Interesting location presets for Newton z⁴-1
pub fn z4_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::NewtonZ4,
        },
        LocationPreset {
            name: "Cross",
            center: [0.0, 0.0],
            zoom: 5.0,
            fractal_type: FractalType::NewtonZ4,
        },
        LocationPreset {
            name: "Boundary",
            center: [0.707, 0.707],
            zoom: 20.0,
            fractal_type: FractalType::NewtonZ4,
        },
    ]
}

/// Interesting location presets for Nova
pub fn nova_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Nova,
        },
        LocationPreset {
            name: "Spiral",
            center: [0.5, 0.5],
            zoom: 5.0,
            fractal_type: FractalType::Nova,
        },
        LocationPreset {
            name: "Edge",
            center: [-0.5, 0.0],
            zoom: 10.0,
            fractal_type: FractalType::Nova,
        },
    ]
}
