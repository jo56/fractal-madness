use super::{FractalParams, FractalType, LocationPreset};

/// Default Magnet Type I fractal parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [1.5, 0.0],
        zoom: 0.4,
        max_iter: 256,
        power: 2.0,
        escape_radius: 100.0,
        fractal_type: FractalType::Magnet as u32,
        ..Default::default()
    }
}

/// Interesting location presets for Magnet Type I fractal
/// Based on the Ising model in physics: z = ((z^2 + c - 1) / (2z + c - 2))^2
/// Main set structure is near x=1. Detail is at boundaries between converged and escaped regions.
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::Magnet,
        },
        LocationPreset {
            name: "Main Body",
            center: [1.0, 0.0],
            zoom: 1.5,
            fractal_type: FractalType::Magnet,
        },
        LocationPreset {
            name: "Cusp",
            center: [1.2, 0.1],
            zoom: 12.0,
            fractal_type: FractalType::Magnet,
        },
    ]
}
