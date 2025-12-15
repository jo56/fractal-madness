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
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.5, 0.0],
            zoom: 1.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Seahorse Valley",
            center: [-0.743643887037158, 0.131825904205330],
            zoom: 500.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Elephant Valley",
            center: [0.281717921930775, 0.5771052841488505],
            zoom: 1000.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Triple Spiral",
            center: [-0.088, 0.654],
            zoom: 50.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Mini Mandelbrot",
            center: [-1.7497591451303, 0.0],
            zoom: 10000.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Lightning",
            center: [-0.170337, -1.06506],
            zoom: 200.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Starfish",
            center: [-0.374004139, 0.659792175],
            zoom: 5000.0,
            fractal_type: FractalType::Mandelbrot,
        },
        LocationPreset {
            name: "Sun",
            center: [-0.776592847, -0.136640848],
            zoom: 10000.0,
            fractal_type: FractalType::Mandelbrot,
        },
    ]
}
