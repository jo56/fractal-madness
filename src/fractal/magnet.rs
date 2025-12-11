use super::{FractalType, LocationPreset};

/// Interesting location presets for Magnet Type I
pub fn type_i_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::MagnetI,
        },
        LocationPreset {
            name: "Pole",
            center: [1.0, 0.0],
            zoom: 5.0,
            fractal_type: FractalType::MagnetI,
        },
        LocationPreset {
            name: "Field Lines",
            center: [0.5, 0.5],
            zoom: 10.0,
            fractal_type: FractalType::MagnetI,
        },
        LocationPreset {
            name: "Detail",
            center: [0.2, 0.0],
            zoom: 20.0,
            fractal_type: FractalType::MagnetI,
        },
    ]
}

/// Interesting location presets for Magnet Type II
pub fn type_ii_presets() -> Vec<LocationPreset> {
    vec![
        LocationPreset {
            name: "Overview",
            center: [0.0, 0.0],
            zoom: 0.5,
            fractal_type: FractalType::MagnetII,
        },
        LocationPreset {
            name: "Core",
            center: [1.0, 0.0],
            zoom: 5.0,
            fractal_type: FractalType::MagnetII,
        },
        LocationPreset {
            name: "Attraction",
            center: [0.3, 0.3],
            zoom: 10.0,
            fractal_type: FractalType::MagnetII,
        },
    ]
}
