// Deep Mandelbrot zoom compute shader using perturbation theory
// Uses double-double arithmetic (two f32s) to emulate f64 precision

// ============================================================================
// UNIFORMS AND BUFFERS
// ============================================================================

struct DeepParams {
    // Resolution (width, height)
    resolution: vec2<u32>,
    // Maximum iterations
    max_iter: u32,
    // Iterations to skip via series approximation
    sa_skip: u32,
    // Escape radius squared
    escape_radius_sq: f32,
    // Color scheme index
    color_scheme: u32,
    // Flags (smooth coloring, etc.)
    flags: u32,
    // Reference orbit length
    ref_orbit_len: u32,
    // Top-left corner delta from center (double-double: hi, lo)
    corner_delta_re_hi: f32,
    corner_delta_re_lo: f32,
    corner_delta_im_hi: f32,
    corner_delta_im_lo: f32,
    // Pixel step size in complex plane (double-double)
    step_re_hi: f32,
    step_re_lo: f32,
    step_im_hi: f32,
    step_im_lo: f32,
}

// Reference orbit: each point is (re_hi, re_lo, im_hi, im_lo)
struct RefPoint {
    re_hi: f32,
    re_lo: f32,
    im_hi: f32,
    im_lo: f32,
}

// SA coefficients: A, B, C for polynomial approximation
struct SACoeffs {
    a_re: f32,
    a_im: f32,
    b_re: f32,
    b_im: f32,
    c_re: f32,
    c_im: f32,
    _pad0: f32,
    _pad1: f32,
}

@group(0) @binding(0) var<uniform> params: DeepParams;
@group(0) @binding(1) var<storage, read> ref_orbit: array<RefPoint>;
@group(0) @binding(2) var<storage, read> sa_coeffs: array<SACoeffs>;
@group(0) @binding(3) var output: texture_storage_2d<rgba8unorm, write>;

// ============================================================================
// DOUBLE-DOUBLE ARITHMETIC
// Emulates ~48 bits of mantissa using two f32 values (hi + lo)
// ============================================================================

// Two-sum algorithm: compute s = a + b with error e such that a + b = s + e exactly
fn two_sum(a: f32, b: f32) -> vec2<f32> {
    let s = a + b;
    let v = s - a;
    let e = (a - (s - v)) + (b - v);
    return vec2<f32>(s, e);
}

// Double-double addition: (a_hi + a_lo) + (b_hi + b_lo)
fn dd_add(a_hi: f32, a_lo: f32, b_hi: f32, b_lo: f32) -> vec2<f32> {
    let s = two_sum(a_hi, b_hi);
    let e = a_lo + b_lo + s.y;
    let r = two_sum(s.x, e);
    return r;
}

// Double-double subtraction
fn dd_sub(a_hi: f32, a_lo: f32, b_hi: f32, b_lo: f32) -> vec2<f32> {
    return dd_add(a_hi, a_lo, -b_hi, -b_lo);
}

// Split a f32 into high and low parts for multiplication
fn split(a: f32) -> vec2<f32> {
    let c = 4097.0 * a;  // 2^12 + 1
    let a_hi = c - (c - a);
    let a_lo = a - a_hi;
    return vec2<f32>(a_hi, a_lo);
}

// Two-product algorithm: compute p = a * b with error e
fn two_prod(a: f32, b: f32) -> vec2<f32> {
    let p = a * b;
    let a_s = split(a);
    let b_s = split(b);
    let e = ((a_s.x * b_s.x - p) + a_s.x * b_s.y + a_s.y * b_s.x) + a_s.y * b_s.y;
    return vec2<f32>(p, e);
}

// Double-double multiplication: (a_hi + a_lo) * (b_hi + b_lo)
fn dd_mul(a_hi: f32, a_lo: f32, b_hi: f32, b_lo: f32) -> vec2<f32> {
    let p = two_prod(a_hi, b_hi);
    let e = a_hi * b_lo + a_lo * b_hi + p.y;
    let r = two_sum(p.x, e);
    return r;
}

// Double-double from single f32
fn dd_from_f32(a: f32) -> vec2<f32> {
    return vec2<f32>(a, 0.0);
}

// ============================================================================
// COMPLEX DOUBLE-DOUBLE OPERATIONS
// Complex numbers stored as (re_hi, re_lo, im_hi, im_lo)
// ============================================================================

// Complex addition
fn cdd_add(a_re: vec2<f32>, a_im: vec2<f32>, b_re: vec2<f32>, b_im: vec2<f32>) -> vec4<f32> {
    let re = dd_add(a_re.x, a_re.y, b_re.x, b_re.y);
    let im = dd_add(a_im.x, a_im.y, b_im.x, b_im.y);
    return vec4<f32>(re.x, re.y, im.x, im.y);
}

// Complex subtraction
fn cdd_sub(a_re: vec2<f32>, a_im: vec2<f32>, b_re: vec2<f32>, b_im: vec2<f32>) -> vec4<f32> {
    let re = dd_sub(a_re.x, a_re.y, b_re.x, b_re.y);
    let im = dd_sub(a_im.x, a_im.y, b_im.x, b_im.y);
    return vec4<f32>(re.x, re.y, im.x, im.y);
}

// Complex multiplication: (a + bi)(c + di) = (ac - bd) + (ad + bc)i
fn cdd_mul(a_re: vec2<f32>, a_im: vec2<f32>, b_re: vec2<f32>, b_im: vec2<f32>) -> vec4<f32> {
    // Real part: ac - bd
    let ac = dd_mul(a_re.x, a_re.y, b_re.x, b_re.y);
    let bd = dd_mul(a_im.x, a_im.y, b_im.x, b_im.y);
    let re = dd_sub(ac.x, ac.y, bd.x, bd.y);

    // Imag part: ad + bc
    let ad = dd_mul(a_re.x, a_re.y, b_im.x, b_im.y);
    let bc = dd_mul(a_im.x, a_im.y, b_re.x, b_re.y);
    let im = dd_add(ad.x, ad.y, bc.x, bc.y);

    return vec4<f32>(re.x, re.y, im.x, im.y);
}

// Complex squaring (optimized): (a + bi)² = (a² - b²) + 2ab*i
fn cdd_sq(a_re: vec2<f32>, a_im: vec2<f32>) -> vec4<f32> {
    let a_sq = dd_mul(a_re.x, a_re.y, a_re.x, a_re.y);
    let b_sq = dd_mul(a_im.x, a_im.y, a_im.x, a_im.y);
    let re = dd_sub(a_sq.x, a_sq.y, b_sq.x, b_sq.y);

    let ab = dd_mul(a_re.x, a_re.y, a_im.x, a_im.y);
    let im = dd_add(ab.x, ab.y, ab.x, ab.y);  // 2 * ab

    return vec4<f32>(re.x, re.y, im.x, im.y);
}

// Magnitude squared (returns f32, sufficient for escape check)
fn cdd_mag_sq(re: vec2<f32>, im: vec2<f32>) -> f32 {
    return re.x * re.x + im.x * im.x;
}

// ============================================================================
// COLOR PALETTES (copied from main shader)
// ============================================================================

fn palette_classic(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * cos(6.28318 * (t + 0.0)),
        0.5 + 0.5 * cos(6.28318 * (t + 0.33)),
        0.5 + 0.5 * cos(6.28318 * (t + 0.67))
    );
}

fn palette_fire(t: f32) -> vec3<f32> {
    return vec3<f32>(
        min(1.0, t * 3.0),
        max(0.0, min(1.0, t * 3.0 - 1.0)),
        max(0.0, t * 3.0 - 2.0)
    );
}

fn palette_ocean(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.0,
        0.3 + 0.7 * t,
        0.5 + 0.5 * t
    );
}

fn palette_rainbow(t: f32) -> vec3<f32> {
    let h = t * 6.0;
    let i = floor(h);
    let f = h - i;
    let q = 1.0 - f;

    let idx = i32(i) % 6;
    if (idx == 0) { return vec3<f32>(1.0, f, 0.0); }
    if (idx == 1) { return vec3<f32>(q, 1.0, 0.0); }
    if (idx == 2) { return vec3<f32>(0.0, 1.0, f); }
    if (idx == 3) { return vec3<f32>(0.0, q, 1.0); }
    if (idx == 4) { return vec3<f32>(f, 0.0, 1.0); }
    return vec3<f32>(1.0, 0.0, q);
}

fn get_color(t: f32, scheme: u32) -> vec3<f32> {
    switch (scheme) {
        case 0u: { return palette_classic(t); }
        case 1u: { return palette_fire(t); }
        case 2u: { return palette_ocean(t); }
        case 3u: { return palette_rainbow(t); }
        default: { return palette_classic(t); }
    }
}

// ============================================================================
// MAIN COMPUTE KERNEL
// ============================================================================

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;

    // Bounds check
    if (x >= params.resolution.x || y >= params.resolution.y) {
        return;
    }

    // Calculate pixel delta from center
    // delta = corner_delta + pixel * step
    let px = f32(x);
    let py = f32(y);

    // Pixel step contribution
    let step_x_re = dd_mul(params.step_re_hi, params.step_re_lo, px, 0.0);
    let step_x_im = dd_mul(params.step_im_hi, params.step_im_lo, px, 0.0);
    let step_y_re = dd_mul(params.step_re_hi, params.step_re_lo, 0.0, 0.0);  // No real contribution from y
    let step_y_im = dd_mul(params.step_im_hi, params.step_im_lo, py, 0.0);

    // Delta = corner + pixel_offset
    var delta_re = dd_add(params.corner_delta_re_hi, params.corner_delta_re_lo, step_x_re.x, step_x_re.y);
    var delta_im = dd_add(params.corner_delta_im_hi, params.corner_delta_im_lo, step_y_im.x, step_y_im.y);

    // Initialize epsilon from series approximation
    var eps_re = delta_re;
    var eps_im = delta_im;

    // Apply SA if we have coefficients and skip iterations > 0
    if (params.sa_skip > 0u && params.sa_skip < arrayLength(&sa_coeffs)) {
        let sa = sa_coeffs[params.sa_skip];

        // ε ≈ A*δ + B*δ² + C*δ³
        // For now, just use linear term: ε ≈ A*δ
        let a_re = vec2<f32>(sa.a_re, 0.0);
        let a_im = vec2<f32>(sa.a_im, 0.0);

        let a_delta = cdd_mul(a_re, a_im, delta_re, delta_im);
        eps_re = vec2<f32>(a_delta.x, a_delta.y);
        eps_im = vec2<f32>(a_delta.z, a_delta.w);

        // Add quadratic term: + B*δ²
        let b_re = vec2<f32>(sa.b_re, 0.0);
        let b_im = vec2<f32>(sa.b_im, 0.0);
        let delta_sq = cdd_sq(delta_re, delta_im);
        let b_delta_sq = cdd_mul(b_re, b_im, vec2<f32>(delta_sq.x, delta_sq.y), vec2<f32>(delta_sq.z, delta_sq.w));

        let eps_with_b = cdd_add(eps_re, eps_im, vec2<f32>(b_delta_sq.x, b_delta_sq.y), vec2<f32>(b_delta_sq.z, b_delta_sq.w));
        eps_re = vec2<f32>(eps_with_b.x, eps_with_b.y);
        eps_im = vec2<f32>(eps_with_b.z, eps_with_b.w);
    }

    // Start iteration from sa_skip
    var iter = params.sa_skip;
    var escaped = false;
    var final_mag_sq: f32 = 0.0;

    // Main perturbation iteration loop
    while (iter < params.max_iter && iter < params.ref_orbit_len) {
        // Get reference orbit point
        let z_ref = ref_orbit[iter];
        let z_ref_re = vec2<f32>(z_ref.re_hi, z_ref.re_lo);
        let z_ref_im = vec2<f32>(z_ref.im_hi, z_ref.im_lo);

        // Perturbation iteration: ε' = 2*Z_ref*ε + ε² + δ

        // 2 * Z_ref * ε
        let two_z_eps = cdd_mul(
            dd_add(z_ref_re.x, z_ref_re.y, z_ref_re.x, z_ref_re.y),  // 2 * z_ref_re
            dd_add(z_ref_im.x, z_ref_im.y, z_ref_im.x, z_ref_im.y),  // 2 * z_ref_im
            eps_re,
            eps_im
        );

        // ε²
        let eps_sq = cdd_sq(eps_re, eps_im);

        // 2*Z_ref*ε + ε²
        let sum1 = cdd_add(
            vec2<f32>(two_z_eps.x, two_z_eps.y),
            vec2<f32>(two_z_eps.z, two_z_eps.w),
            vec2<f32>(eps_sq.x, eps_sq.y),
            vec2<f32>(eps_sq.z, eps_sq.w)
        );

        // + δ
        let new_eps = cdd_add(
            vec2<f32>(sum1.x, sum1.y),
            vec2<f32>(sum1.z, sum1.w),
            delta_re,
            delta_im
        );

        eps_re = vec2<f32>(new_eps.x, new_eps.y);
        eps_im = vec2<f32>(new_eps.z, new_eps.w);

        // Calculate Z = Z_ref + ε for escape check
        let z = cdd_add(z_ref_re, z_ref_im, eps_re, eps_im);
        let z_re_final = vec2<f32>(z.x, z.y);
        let z_im_final = vec2<f32>(z.z, z.w);
        let mag_sq = cdd_mag_sq(z_re_final, z_im_final);

        // Glitch detection: if |ε| > |Z_ref| * 0.001, we've lost accuracy
        let eps_mag_sq = cdd_mag_sq(eps_re, eps_im);
        let ref_mag_sq = cdd_mag_sq(z_ref_re, z_ref_im);
        if (eps_mag_sq > ref_mag_sq * 0.000001) {
            // Glitch detected - for now, just mark as escaped to avoid artifacts
            // A more sophisticated approach would use a secondary reference
            escaped = true;
            final_mag_sq = params.escape_radius_sq + 1.0;
            break;
        }

        // Check escape
        if (mag_sq > params.escape_radius_sq) {
            escaped = true;
            final_mag_sq = mag_sq;
            break;
        }

        iter = iter + 1u;
    }

    // Color the pixel
    var color: vec4<f32>;
    if (!escaped) {
        // Inside the set - black
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else {
        // Smooth coloring
        var t: f32;
        if ((params.flags & 1u) != 0u) {
            // Smooth iteration count
            let log_zn = log(max(final_mag_sq, 1.0)) / 2.0;
            let nu = log(max(log_zn / log(2.0), 1e-10)) / log(2.0);
            let smooth_iter = f32(iter) + 1.0 - nu;
            t = smooth_iter / f32(params.max_iter);
        } else {
            t = f32(iter) / f32(params.max_iter);
        }

        let rgb = get_color(fract(t * 5.0), params.color_scheme);
        color = vec4<f32>(rgb, 1.0);
    }

    textureStore(output, vec2<i32>(i32(x), i32(y)), color);
}
