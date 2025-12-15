//! Series Approximation (SA) for skipping early iterations.
//!
//! Series approximation computes polynomial coefficients that approximate
//! the first N iterations, allowing us to skip them and start the GPU
//! computation from iteration N.
//!
//! For perturbation z_n = Z_n + ε_n where Z_n is the reference orbit:
//! ε_n ≈ A_n * δ + B_n * δ² + C_n * δ³
//!
//! where δ = c - c_ref (the delta from reference center to pixel).

use bytemuck::{Pod, Zeroable};

use super::reference::ReferenceOrbit;

/// Coefficients for series approximation at a single iteration.
/// These approximate ε_n in terms of δ (the pixel delta from center).
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SACoefficients {
    /// A coefficient (real, imag) - linear term
    pub a_re: f32,
    pub a_im: f32,
    /// B coefficient (real, imag) - quadratic term
    pub b_re: f32,
    pub b_im: f32,
    /// C coefficient (real, imag) - cubic term
    pub c_re: f32,
    pub c_im: f32,
    /// Padding for alignment
    pub _pad0: f32,
    pub _pad1: f32,
}

impl Default for SACoefficients {
    fn default() -> Self {
        Self {
            a_re: 1.0,
            a_im: 0.0,
            b_re: 0.0,
            b_im: 0.0,
            c_re: 0.0,
            c_im: 0.0,
            _pad0: 0.0,
            _pad1: 0.0,
        }
    }
}

/// Series approximation data for the compute shader.
pub struct SeriesApproximation {
    /// Coefficients at each iteration
    pub coefficients: Vec<SACoefficients>,

    /// Number of iterations that can be safely skipped using SA
    pub skip_iterations: u32,

    /// Maximum delta magnitude for which SA is valid
    pub max_delta: f64,
}

impl SeriesApproximation {
    /// Calculate series approximation coefficients from reference orbit.
    ///
    /// # Arguments
    /// * `reference` - The reference orbit
    /// * `max_delta` - Maximum pixel delta (in complex plane units)
    /// * `tolerance` - Accuracy tolerance for SA validity
    pub fn calculate(reference: &ReferenceOrbit, max_delta: f64, tolerance: f64) -> Self {
        let max_iter = reference.len();
        let mut coefficients = Vec::with_capacity(max_iter);

        // Initial coefficients: ε_0 = δ, so A_0 = 1, B_0 = 0, C_0 = 0
        let mut a_re: f64 = 1.0;
        let mut a_im: f64 = 0.0;
        let mut b_re: f64 = 0.0;
        let mut b_im: f64 = 0.0;
        let mut c_re: f64 = 0.0;
        let mut c_im: f64 = 0.0;

        let mut skip_iterations = 0u32;
        let max_delta_sq = max_delta * max_delta;

        for i in 0..max_iter {
            // Store current coefficients
            coefficients.push(SACoefficients {
                a_re: a_re as f32,
                a_im: a_im as f32,
                b_re: b_re as f32,
                b_im: b_im as f32,
                c_re: c_re as f32,
                c_im: c_im as f32,
                _pad0: 0.0,
                _pad1: 0.0,
            });

            // Check if SA is still accurate enough
            // Error is approximately |B| * δ² + |C| * δ³
            let b_mag = (b_re * b_re + b_im * b_im).sqrt();
            let c_mag = (c_re * c_re + c_im * c_im).sqrt();
            let error_estimate = b_mag * max_delta_sq + c_mag * max_delta_sq * max_delta;

            if error_estimate < tolerance {
                skip_iterations = i as u32;
            }

            // Get reference orbit point
            if let Some(z_ref) = reference.get(i) {
                let z_re = z_ref.re_hi as f64;
                let z_im = z_ref.im_hi as f64;

                // Update coefficients for next iteration using recurrence relations:
                // ε_{n+1} = 2 * Z_n * ε_n + ε_n² + δ
                //
                // Expanding in terms of δ:
                // A_{n+1} = 2 * Z_n * A_n + 1
                // B_{n+1} = 2 * Z_n * B_n + A_n²
                // C_{n+1} = 2 * Z_n * C_n + 2 * A_n * B_n

                // Complex multiplication: (a + bi)(c + di) = (ac - bd) + (ad + bc)i
                // 2 * Z_n * A_n
                let two_z_a_re = 2.0 * (z_re * a_re - z_im * a_im);
                let two_z_a_im = 2.0 * (z_re * a_im + z_im * a_re);

                // 2 * Z_n * B_n
                let two_z_b_re = 2.0 * (z_re * b_re - z_im * b_im);
                let two_z_b_im = 2.0 * (z_re * b_im + z_im * b_re);

                // 2 * Z_n * C_n
                let two_z_c_re = 2.0 * (z_re * c_re - z_im * c_im);
                let two_z_c_im = 2.0 * (z_re * c_im + z_im * c_re);

                // A_n²
                let a_sq_re = a_re * a_re - a_im * a_im;
                let a_sq_im = 2.0 * a_re * a_im;

                // 2 * A_n * B_n
                let two_a_b_re = 2.0 * (a_re * b_re - a_im * b_im);
                let two_a_b_im = 2.0 * (a_re * b_im + a_im * b_re);

                // Update coefficients
                let new_a_re = two_z_a_re + 1.0;
                let new_a_im = two_z_a_im;

                let new_b_re = two_z_b_re + a_sq_re;
                let new_b_im = two_z_b_im + a_sq_im;

                let new_c_re = two_z_c_re + two_a_b_re;
                let new_c_im = two_z_c_im + two_a_b_im;

                a_re = new_a_re;
                a_im = new_a_im;
                b_re = new_b_re;
                b_im = new_b_im;
                c_re = new_c_re;
                c_im = new_c_im;
            }
        }

        Self {
            coefficients,
            skip_iterations,
            max_delta,
        }
    }

    /// Get number of coefficients
    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.coefficients.is_empty()
    }

    /// Get raw bytes for GPU upload
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.coefficients)
    }

    /// Get coefficients at a specific iteration
    pub fn get(&self, iter: usize) -> Option<&SACoefficients> {
        self.coefficients.get(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigfloat::BigFloat;

    #[test]
    fn test_series_approximation_basic() {
        let center_re = BigFloat::from(-0.5);
        let center_im = BigFloat::from(0.0);
        let orbit = ReferenceOrbit::calculate(&center_re, &center_im, 100, 4.0);

        let sa = SeriesApproximation::calculate(&orbit, 0.001, 1e-6);

        // Should have some coefficients
        assert!(!sa.is_empty());

        // First coefficient should be A=1, B=0, C=0
        let first = sa.get(0).unwrap();
        assert!((first.a_re - 1.0).abs() < 1e-6);
        assert!(first.a_im.abs() < 1e-6);
        assert!(first.b_re.abs() < 1e-6);
        assert!(first.b_im.abs() < 1e-6);
    }
}
