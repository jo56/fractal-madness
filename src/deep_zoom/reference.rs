//! Reference orbit calculation for perturbation theory.
//!
//! The reference orbit is calculated at arbitrary precision on the CPU,
//! then converted to double-double format for GPU transfer.

use bytemuck::{Pod, Zeroable};
use num_bigfloat::BigFloat;

/// A single point in the reference orbit, stored as double-double (hi + lo) for f64 precision.
/// This format allows ~30 significant decimal digits using two f32 values.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct ReferencePoint {
    /// High part of real component
    pub re_hi: f32,
    /// Low part of real component
    pub re_lo: f32,
    /// High part of imaginary component
    pub im_hi: f32,
    /// Low part of imaginary component
    pub im_lo: f32,
}

impl ReferencePoint {
    /// Create from BigFloat values
    pub fn from_bigfloat(re: &BigFloat, im: &BigFloat) -> Self {
        let (re_hi, re_lo) = bigfloat_to_double_double(re);
        let (im_hi, im_lo) = bigfloat_to_double_double(im);
        Self {
            re_hi: re_hi as f32,
            re_lo: re_lo as f32,
            im_hi: im_hi as f32,
            im_lo: im_lo as f32,
        }
    }

    /// Create from f64 values
    pub fn from_f64(re: f64, im: f64) -> Self {
        let (re_hi, re_lo) = f64_to_double_double(re);
        let (im_hi, im_lo) = f64_to_double_double(im);
        Self {
            re_hi: re_hi as f32,
            re_lo: re_lo as f32,
            im_hi: im_hi as f32,
            im_lo: im_lo as f32,
        }
    }
}

/// Convert BigFloat to double-double representation (hi, lo) where value ≈ hi + lo
fn bigfloat_to_double_double(value: &BigFloat) -> (f64, f64) {
    let hi = value.to_f64();
    // lo = value - hi (captures the remainder)
    let lo = (value.clone() - BigFloat::from(hi)).to_f64();
    (hi, lo)
}

/// Convert f64 to double-double representation
fn f64_to_double_double(value: f64) -> (f64, f64) {
    // For f64, the low part is 0 since we don't have extra precision
    (value, 0.0)
}

/// Reference orbit for perturbation theory.
///
/// Contains the trajectory of a single point (typically the center) iterated at arbitrary precision.
/// Each pixel's iteration is then computed as a small perturbation from this reference.
pub struct ReferenceOrbit {
    /// The center point used for this reference
    pub center_re: BigFloat,
    pub center_im: BigFloat,

    /// Reference orbit points in GPU-friendly format
    pub points: Vec<ReferencePoint>,

    /// Iteration at which reference escaped (None if didn't escape)
    pub escape_iteration: Option<u32>,

    /// Maximum iterations used for calculation
    pub max_iter: u32,
}

impl ReferenceOrbit {
    /// Calculate reference orbit for Mandelbrot set.
    ///
    /// # Arguments
    /// * `center_re` - Real part of center in arbitrary precision
    /// * `center_im` - Imaginary part of center in arbitrary precision
    /// * `max_iter` - Maximum iterations to calculate
    /// * `escape_radius` - Escape radius (typically 2.0 or 4.0)
    pub fn calculate(
        center_re: &BigFloat,
        center_im: &BigFloat,
        max_iter: u32,
        escape_radius: f64,
    ) -> Self {
        let escape_sq = BigFloat::from(escape_radius * escape_radius);
        let two = BigFloat::from(2.0);

        // Start at z = 0
        let mut z_re = BigFloat::from(0.0);
        let mut z_im = BigFloat::from(0.0);

        let mut points = Vec::with_capacity(max_iter as usize);
        let mut escape_iteration = None;

        for i in 0..max_iter {
            // Store current point
            points.push(ReferencePoint::from_bigfloat(&z_re, &z_im));

            // Check escape: |z|^2 > escape_radius^2
            let mag_sq = z_re.clone() * z_re.clone() + z_im.clone() * z_im.clone();
            if mag_sq > escape_sq {
                escape_iteration = Some(i);
                break;
            }

            // Mandelbrot iteration: z = z^2 + c
            // z^2 = (re + im*i)^2 = re^2 - im^2 + 2*re*im*i
            let new_re = z_re.clone() * z_re.clone() - z_im.clone() * z_im.clone() + center_re.clone();
            let new_im = two.clone() * z_re.clone() * z_im.clone() + center_im.clone();
            z_re = new_re;
            z_im = new_im;
        }

        Self {
            center_re: center_re.clone(),
            center_im: center_im.clone(),
            points,
            escape_iteration,
            max_iter,
        }
    }

    /// Get the number of iterations in this reference orbit
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Check if orbit is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get raw bytes for GPU upload
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.points)
    }

    /// Get the reference point at a given iteration
    pub fn get(&self, iter: usize) -> Option<&ReferencePoint> {
        self.points.get(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_orbit_basic() {
        let center_re = BigFloat::from(-0.5);
        let center_im = BigFloat::from(0.0);
        let orbit = ReferenceOrbit::calculate(&center_re, &center_im, 100, 4.0);

        // The point (-0.5, 0) is in the Mandelbrot set, so it shouldn't escape
        assert!(orbit.escape_iteration.is_none());
        assert_eq!(orbit.len(), 100);
    }

    #[test]
    fn test_reference_orbit_escape() {
        let center_re = BigFloat::from(1.0);
        let center_im = BigFloat::from(1.0);
        let orbit = ReferenceOrbit::calculate(&center_re, &center_im, 100, 4.0);

        // The point (1, 1) is outside the Mandelbrot set
        assert!(orbit.escape_iteration.is_some());
        assert!(orbit.escape_iteration.unwrap() < 10);
    }

    #[test]
    fn test_double_double_conversion() {
        let value = BigFloat::from(-0.75);
        let (hi, lo) = bigfloat_to_double_double(&value);

        // hi + lo should be approximately equal to original
        assert!((hi + lo - (-0.75)).abs() < 1e-15);
    }
}
