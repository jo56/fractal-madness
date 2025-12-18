use super::{FractalType, LocationPreset};

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
            fractal_type: FractalType::Newton,
            power: Some(3.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Triple Junction",
            center: [0.0, 0.0],
            zoom: 4.0,
            fractal_type: FractalType::Newton,
            power: Some(3.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Basin Boundary",
            center: [0.25, 0.43],
            zoom: 8.0,
            fractal_type: FractalType::Newton,
            power: Some(3.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Spiral Detail",
            center: [0.15, 0.26],
            zoom: 25.0,
            fractal_type: FractalType::Newton,
            power: Some(3.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Fractal Vein",
            center: [0.1, 0.17],
            zoom: 60.0,
            fractal_type: FractalType::Newton,
            power: Some(3.0),
            julia_c: None,
        },
        // Different power variants (changes number of root basins)
        LocationPreset {
            name: "Quadratic (z²)",
            center: [0.0, 0.0],
            zoom: 1.5,
            fractal_type: FractalType::Newton,
            power: Some(2.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Quartic (z⁴)",
            center: [0.0, 0.0],
            zoom: 1.5,
            fractal_type: FractalType::Newton,
            power: Some(4.0),
            julia_c: None,
        },
        LocationPreset {
            name: "Quintic (z⁵)",
            center: [0.0, 0.0],
            zoom: 1.5,
            fractal_type: FractalType::Newton,
            power: Some(5.0),
            julia_c: None,
        },
    ]
}
