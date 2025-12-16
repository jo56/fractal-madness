use super::{FractalParams, FractalType, LocationPreset};

/// Default Newton fractal parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [0.0, 0.0],
        zoom: 0.8,
        max_iter: 256,
        power: 3.0,
        escape_radius: 4.0,
        fractal_type: FractalType::Newton as u32,
        ..Default::default()
    }
}

/// Interesting location presets for Newton fractal
/// The Newton fractal shows basin boundaries for root-finding of z^n = 1
/// For z³-1, roots are at (1,0), (-0.5, 0.866), (-0.5, -0.866)
/// IMPORTANT: Detail is at BOUNDARIES between basins, not near roots (which are solid)
/// Boundaries radiate from the origin where all three basins meet
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 1.5,
            fractal_type: FractalType::Newton, power: None,
        },
        LocationPreset {
            name: "Triple Junction",
            center: [0.0, 0.0],
            zoom: 4.0,
            fractal_type: FractalType::Newton, power: None,
        },
        LocationPreset {
            name: "Basin Boundary",
            center: [0.25, 0.43],
            zoom: 8.0,
            fractal_type: FractalType::Newton, power: None,
        },
        LocationPreset {
            name: "Spiral Detail",
            center: [0.15, 0.26],
            zoom: 25.0,
            fractal_type: FractalType::Newton, power: None,
        },
        LocationPreset {
            name: "Fractal Vein",
            center: [0.1, 0.17],
            zoom: 60.0,
            fractal_type: FractalType::Newton, power: None,
        },
    ]
}
