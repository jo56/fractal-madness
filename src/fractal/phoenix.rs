use super::{FractalType, LocationPreset};

/// Phoenix fractal presets with different parameter variations
/// The Phoenix fractal uses z_new = z^2 + c + p*z_prev
/// Different p values (stored in julia_c) produce entirely different shapes
/// When p=0, it reduces to standard Mandelbrot
pub fn presets() -> Vec<LocationPreset> {
    vec![
        // Classic Ushiki Phoenix
        LocationPreset {
            name: "Classic",
            center: [0.0, 0.0],
            zoom: 1.2,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: Some([0.5667, -0.5]),
        },
        // Negative real p - creates inverted/mirrored structures
        LocationPreset {
            name: "Inverted",
            center: [0.0, 0.0],
            zoom: 0.8,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: Some([-0.5, -0.5]),
        },
        // Pure imaginary p - creates rotational effects
        LocationPreset {
            name: "Vortex",
            center: [0.0, 0.0],
            zoom: 1.0,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: Some([0.0, 0.8]),
        },
        // Large positive p - dramatic expansion
        LocationPreset {
            name: "Explosion",
            center: [0.0, 0.0],
            zoom: 0.6,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: Some([0.8, 0.0]),
        },
        // Small p near zero - closer to Mandelbrot
        LocationPreset {
            name: "Subtle",
            center: [-0.5, 0.0],
            zoom: 1.0,
            fractal_type: FractalType::Phoenix,
            power: Some(2.0),
            julia_c: Some([0.1, -0.1]),
        },
    ]
}
