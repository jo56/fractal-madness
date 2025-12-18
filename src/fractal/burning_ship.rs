use super::{FractalType, LocationPreset};

/// Interesting location presets for Burning Ship
pub fn presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [-0.4, -0.6],
            zoom: 0.6,
            fractal_type: FractalType::BurningShip,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "The Ship",
            center: [-1.762, -0.028],
            zoom: 30.0,
            fractal_type: FractalType::BurningShip,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Armada",
            center: [-1.941, -0.015],
            zoom: 100.0,
            fractal_type: FractalType::BurningShip,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Hidden Ship",
            center: [-1.861, -0.001],
            zoom: 500.0,
            fractal_type: FractalType::BurningShip,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Lighthouse",
            center: [-1.755, -0.035],
            zoom: 200.0,
            fractal_type: FractalType::BurningShip,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Antenna",
            center: [-0.15, -1.035],
            zoom: 50.0,
            fractal_type: FractalType::BurningShip,
            power: Some(2.0),
            julia_c: None,
        },
        // Higher power variants
        LocationPreset {
            name: "Cubic (z³)",
            center: [0.0, 0.0],
            zoom: 0.6,
            fractal_type: FractalType::BurningShip,
            power: Some(3.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 0.6,
            fractal_type: FractalType::BurningShip,
            power: Some(4.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 0.6,
            fractal_type: FractalType::BurningShip,
            power: Some(5.0),
            julia_c: None,
        },
    ]
}
