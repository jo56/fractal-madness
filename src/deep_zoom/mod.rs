//! Deep zoom module for Mandelbrot visualization using perturbation theory.
//!
//! This module enables zooming to depths of 10^100+ by using:
//! - Arbitrary precision arithmetic for reference orbit calculation
//! - Perturbation theory to compute deltas on GPU with f64 precision
//! - Series approximation to skip early iterations
//! - Glitch detection and correction

pub mod params;
pub mod reference;
pub mod series_approx;

pub use params::{DeepZoomParams, RenderMode};
pub use reference::{ReferenceOrbit, ReferencePoint};
pub use series_approx::SeriesApproximation;
