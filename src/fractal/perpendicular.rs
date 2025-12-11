use super::{FractalType, LocationPreset};

/// Interesting location presets for Perpendicular Mandelbrot
pub fn mandelbrot_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.5, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::PerpendicularMandelbrot,
        },
        LocationPreset {
            name: "Branch",
            center: [-0.2, 0.8],
            zoom: 50.0,
            fractal_type: FractalType::PerpendicularMandelbrot,
        },
        LocationPreset {
            name: "Spike",
            center: [-1.75, 0.0],
            zoom: 100.0,
            fractal_type: FractalType::PerpendicularMandelbrot,
        },
    ]
}

/// Interesting location presets for Perpendicular Burning Ship
pub fn burning_ship_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.5, -0.5],
            zoom: 0.8,
            fractal_type: FractalType::PerpendicularBurningShip,
        },
        LocationPreset {
            name: "Mast",
            center: [-1.8, -0.02],
            zoom: 50.0,
            fractal_type: FractalType::PerpendicularBurningShip,
        },
        LocationPreset {
            name: "Detail",
            center: [-0.5, -0.9],
            zoom: 100.0,
            fractal_type: FractalType::PerpendicularBurningShip,
        },
    ]
}
