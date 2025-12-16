use super::{FractalType, LocationPreset};

/// Interesting location presets for Perpendicular Mandelbrot
pub fn mandelbrot_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::PerpendicularMandelbrot,
        },
        LocationPreset {
            name: "Spike",
            center: [-1.0, 0.0],
            zoom: 15.0,
            fractal_type: FractalType::PerpendicularMandelbrot,
        },
        LocationPreset {
            name: "Branch",
            center: [0.25, 0.45],
            zoom: 25.0,
            fractal_type: FractalType::PerpendicularMandelbrot,
        },
    ]
}

/// Interesting location presets for Perpendicular Burning Ship
pub fn burning_ship_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::PerpendicularBurningShip,
        },
        LocationPreset {
            name: "Ship",
            center: [-0.5, -0.5],
            zoom: 10.0,
            fractal_type: FractalType::PerpendicularBurningShip,
        },
        LocationPreset {
            name: "Detail",
            center: [-1.0, 0.0],
            zoom: 20.0,
            fractal_type: FractalType::PerpendicularBurningShip,
        },
    ]
}
