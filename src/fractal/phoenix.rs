use super::{FractalParams, FractalType, LocationPreset};

/// Default Phoenix fractal parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [0.0, 0.0],
        zoom: 0.5,
        max_iter: 256,
        power: 2.0,
        escape_radius: 4.0,
        fractal_type: FractalType::Phoenix as u32,
        ..Default::default()
    }
}

/// Interesting location presets for Phoenix fractal
/// The Phoenix fractal uses z_new = z^2 + c + p*z_prev with c=0.5667, p=-0.5
/// The set is bounded within approximately ±1.35
/// IMPORTANT: Detail is at the SET BOUNDARY, not inside or outside (both are solid)
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Wing Edge",
            center: [0.5, 0.5],
            zoom: 3.0,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Tail",
            center: [0.0, -0.9],
            zoom: 4.0,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Inner Curl",
            center: [-0.15, 0.2],
            zoom: 10.0,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: None,
        },
    ]
}
