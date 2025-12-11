use super::{FractalParams, FractalType, LocationPreset};

/// Default Burning Ship parameters
#[allow(dead_code)]
pub fn default_params() -> FractalParams {
    FractalParams {
        center: [-0.4, -0.6],
        zoom: 0.6,
        max_iter: 256,
        power: 2.0,
        escape_radius: 4.0,
        fractal_type: FractalType::BurningShip as u32,
        ..Default::default()
    }
}

/// Interesting location presets for Burning Ship
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.4, -0.6],
            zoom: 0.6,
            fractal_type: FractalType::BurningShip,
        },
        LocationPreset {
            name: "The Ship",
            center: [-1.762, -0.028],
            zoom: 30.0,
            fractal_type: FractalType::BurningShip,
        },
        LocationPreset {
            name: "Armada",
            center: [-1.941, -0.015],
            zoom: 100.0,
            fractal_type: FractalType::BurningShip,
        },
        LocationPreset {
            name: "Hidden Ship",
            center: [-1.861, -0.001],
            zoom: 500.0,
            fractal_type: FractalType::BurningShip,
        },
        LocationPreset {
            name: "Lighthouse",
            center: [-1.755, -0.035],
            zoom: 200.0,
            fractal_type: FractalType::BurningShip,
        },
        LocationPreset {
            name: "Antenna",
            center: [-0.15, -1.035],
            zoom: 50.0,
            fractal_type: FractalType::BurningShip,
        },
    ]
}
