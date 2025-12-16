use super::{FractalParams, FractalType, LocationPreset};

/// Default Mandelbrot parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [-0.5, 0.0],
        zoom: 1.0,
        max_iter: 256,
        power: 2.0,
        escape_radius: 4.0,
        fractal_type: FractalType::Mandelbrot as u32,
        ..Default::default()
    }
}

/// Interesting location presets for Mandelbrot
/// Note: Coordinates are limited to 4-5 decimal places to match f32 precision.
/// Zoom levels are kept moderate (max ~2000) to avoid precision loss artifacts.
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.5, 0.0],
            zoom: 1.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Seahorse Valley",
            center: [-0.7436, 0.1318],
            zoom: 300.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Elephant Valley",
            center: [0.2817, 0.5771],
            zoom: 500.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Triple Spiral",
            center: [-0.088, 0.654],
            zoom: 50.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Mini Mandelbrot",
            center: [-1.7498, 0.0],
            zoom: 2000.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Lightning",
            center: [-0.1703, -1.0651],
            zoom: 200.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Starfish",
            center: [-0.374, 0.6598],
            zoom: 1500.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        LocationPreset {
            name: "Sun",
            center: [-0.7766, -0.1366],
            zoom: 2000.0,
            fractal_type: FractalType::Mandelbrot, power: None,
        },
        // Multibrot presets (higher powers with n-fold symmetry)
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Mandelbrot, power: Some(3.0),
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Mandelbrot, power: Some(4.0),
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Mandelbrot, power: Some(5.0),
        },
    ]
}
