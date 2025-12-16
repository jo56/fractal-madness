// Fractal fragment shader
// Renders Mandelbrot, Julia, and Burning Ship fractals
// Uses fragment shader for WebGL2 compatibility

struct FractalParams {
    center: vec2<f32>,          // offset 0  (8 bytes)
    zoom: f32,                   // offset 8  (4 bytes)
    max_iter: u32,               // offset 12 (4 bytes)
    power: f32,                  // offset 16 (4 bytes)
    escape_radius: f32,          // offset 20 (4 bytes)
    fractal_type: u32,           // offset 24 (4 bytes)
    color_scheme: u32,           // offset 28 (4 bytes)
    julia_c: vec2<f32>,          // offset 32 (8 bytes)
    flags: u32,                  // offset 40 (4 bytes) - bit 0: smooth, bit 1: invert, bit 2: offset
    _pad: u32,                   // offset 44 (4 bytes)
    resolution: vec2<f32>,       // offset 48 (8 bytes) - canvas width, height
    ui_offset: f32,              // offset 56 (4 bytes) - horizontal offset for UI panel
    ui_offset_y: f32,            // offset 60 (4 bytes) - vertical offset for centering
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0) var<uniform> params: FractalParams;

const PI: f32 = 3.14159265359;

// Classic escape-time fractals
const FRACTAL_MANDELBROT: u32 = 0u;
const FRACTAL_JULIA: u32 = 1u;
const FRACTAL_BURNING_SHIP: u32 = 2u;
const FRACTAL_TRICORN: u32 = 3u;
const FRACTAL_BUFFALO: u32 = 4u;
const FRACTAL_CELTIC: u32 = 5u;
const FRACTAL_PERPENDICULAR_MANDELBROT: u32 = 6u;
const FRACTAL_PERPENDICULAR_BURNING_SHIP: u32 = 7u;
const FRACTAL_HEART: u32 = 8u;
// Julia variants
const FRACTAL_TRICORN_JULIA: u32 = 9u;
const FRACTAL_BUFFALO_JULIA: u32 = 10u;
const FRACTAL_CELTIC_JULIA: u32 = 11u;
const FRACTAL_BURNING_SHIP_JULIA: u32 = 12u;

// Flag bit masks
const FLAG_SMOOTH: u32 = 1u;
const FLAG_INVERT: u32 = 2u;
const FLAG_OFFSET: u32 = 4u;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Generate fullscreen triangle (3 vertices, no vertex buffer)
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0)
    );

    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0)
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = uvs[vertex_index];
    return output;
}

// Complex number operations
fn cmul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(
        a.x * b.x - a.y * b.y,
        a.x * b.y + a.y * b.x
    );
}

fn cpow(z: vec2<f32>, n: f32) -> vec2<f32> {
    let r = length(z);
    let theta = atan2(z.y, z.x);
    let rn = pow(r, n);
    return vec2<f32>(
        rn * cos(n * theta),
        rn * sin(n * theta)
    );
}

// Fractal iteration functions
fn iterate_mandelbrot(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

fn iterate_julia(z_init: vec2<f32>, c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = z_init;
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

fn iterate_burning_ship(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        z = vec2<f32>(abs(z.x), abs(z.y));
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Tricorn (Mandelbar): z = conj(z)^2 + c
fn iterate_tricorn(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        z = vec2<f32>(z.x, -z.y); // conjugate
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Tricorn Julia: z = conj(z)^2 + c (starting from z_init)
fn iterate_tricorn_julia(z_init: vec2<f32>, c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = z_init;
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        z = vec2<f32>(z.x, -z.y); // conjugate
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Buffalo: z = abs(z)^2 - z + c
fn iterate_buffalo(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        let az = vec2<f32>(abs(z.x), abs(z.y));
        if (power == 2.0) {
            z = cmul(az, az) - z + c;
        } else {
            z = cpow(az, power) - z + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Buffalo Julia
fn iterate_buffalo_julia(z_init: vec2<f32>, c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = z_init;
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        let az = vec2<f32>(abs(z.x), abs(z.y));
        if (power == 2.0) {
            z = cmul(az, az) - z + c;
        } else {
            z = cpow(az, power) - z + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Celtic: z = (abs(real(z^2)), imag(z^2)) + c
fn iterate_celtic(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        if (power == 2.0) {
            let z2 = cmul(z, z);
            z = vec2<f32>(abs(z2.x), z2.y) + c;
        } else {
            let zp = cpow(z, power);
            z = vec2<f32>(abs(zp.x), zp.y) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Celtic Julia
fn iterate_celtic_julia(z_init: vec2<f32>, c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = z_init;
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        if (power == 2.0) {
            let z2 = cmul(z, z);
            z = vec2<f32>(abs(z2.x), z2.y) + c;
        } else {
            let zp = cpow(z, power);
            z = vec2<f32>(abs(zp.x), zp.y) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Perpendicular Mandelbrot: z = (abs(z.x), z.y)^2 + c
fn iterate_perpendicular_mandelbrot(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        z = vec2<f32>(abs(z.x), z.y);
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Perpendicular Burning Ship: z = (z.x, abs(z.y))^2 + c
fn iterate_perpendicular_burning_ship(c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        z = vec2<f32>(z.x, abs(z.y));
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Heart: z = (z.x * abs(z.x), z.y^2) + c - creates heart-like shapes
fn iterate_heart(c: vec2<f32>, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = vec2<f32>(0.0, 0.0);
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        let new_x = z.x * z.x - z.y * z.y + c.x;
        let new_y = 2.0 * abs(z.x) * z.y + c.y;
        z = vec2<f32>(new_x, new_y);
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Burning Ship Julia
fn iterate_burning_ship_julia(z_init: vec2<f32>, c: vec2<f32>, power: f32, max_iter: u32, escape_radius: f32) -> vec2<f32> {
    var z = z_init;
    var i: u32 = 0u;
    let escape2 = escape_radius * escape_radius;

    while (i < max_iter && dot(z, z) < escape2) {
        z = vec2<f32>(abs(z.x), abs(z.y));
        if (power == 2.0) {
            z = cmul(z, z) + c;
        } else {
            z = cpow(z, power) + c;
        }
        i = i + 1u;
    }

    return vec2<f32>(f32(i), dot(z, z));
}

// Color palette functions
fn palette_classic(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * cos(2.0 * PI * (t + 0.0)),
        0.5 + 0.5 * cos(2.0 * PI * (t + 0.33)),
        0.5 + 0.5 * cos(2.0 * PI * (t + 0.67))
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
        0.3 + 0.4 * t,
        0.5 + 0.5 * t
    );
}

fn palette_rainbow(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * sin(2.0 * PI * t),
        0.5 + 0.5 * sin(2.0 * PI * (t + 0.33)),
        0.5 + 0.5 * sin(2.0 * PI * (t + 0.67))
    );
}

fn palette_grayscale(t: f32) -> vec3<f32> {
    return vec3<f32>(t, t, t);
}

fn palette_electric(t: f32) -> vec3<f32> {
    return vec3<f32>(
        t,
        t * t,
        1.0
    );
}

fn palette_neon(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * sin(2.0 * PI * t * 2.0),
        0.5 + 0.5 * sin(2.0 * PI * t * 3.0),
        0.5 + 0.5 * sin(2.0 * PI * t * 5.0)
    );
}

fn palette_sunset(t: f32) -> vec3<f32> {
    let a = vec3<f32>(0.5, 0.5, 0.5);
    let b = vec3<f32>(0.5, 0.5, 0.5);
    let c = vec3<f32>(1.0, 0.7, 0.4);
    let d = vec3<f32>(0.0, 0.15, 0.2);
    return a + b * cos(2.0 * PI * (c * t + d));
}

fn palette_forest(t: f32) -> vec3<f32> {
    // Multi-zone forest: floor browns -> moss -> vibrant leaves -> sunlit canopy
    if (t < 0.25) {
        let s = t / 0.25;
        return vec3<f32>(0.15 + 0.1 * s, 0.1 + 0.2 * s, 0.05 + 0.05 * s);  // Dark browns
    } else if (t < 0.5) {
        let s = (t - 0.25) / 0.25;
        return vec3<f32>(0.25 - 0.1 * s, 0.3 + 0.35 * s, 0.1 + 0.1 * s);   // Moss greens
    } else if (t < 0.75) {
        let s = (t - 0.5) / 0.25;
        return vec3<f32>(0.15 + 0.25 * s, 0.65 + 0.2 * s, 0.2 - 0.05 * s); // Vibrant leaves
    } else {
        let s = (t - 0.75) / 0.25;
        return vec3<f32>(0.4 + 0.4 * s, 0.85 + 0.15 * s, 0.15 + 0.35 * s); // Sunlit canopy
    }
}

fn palette_lava(t: f32) -> vec3<f32> {
    return vec3<f32>(
        min(1.0, 0.5 + t),
        max(0.0, t - 0.3) * 1.5,
        0.0
    );
}

fn palette_ice(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.7 + 0.3 * t,
        0.85 + 0.15 * t,
        1.0
    );
}

fn palette_plasma(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * sin(3.0 * PI * t),
        0.5 + 0.5 * sin(3.0 * PI * t + 2.094),
        0.5 + 0.5 * sin(3.0 * PI * t + 4.188)
    );
}

fn palette_cosmic(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.1 + 0.4 * pow(t, 0.5),
        0.0 + 0.2 * t,
        0.3 + 0.7 * pow(t, 0.7)
    );
}

fn palette_autumn(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.8 + 0.2 * sin(PI * t),
        0.3 + 0.4 * t,
        0.1 + 0.1 * t
    );
}

fn palette_matrix(t: f32) -> vec3<f32> {
    // Enhanced Matrix with digital rain effect - varied greens with subtle highlights
    let base = 0.1 + 0.9 * t;
    let pulse = 0.5 + 0.5 * sin(t * 20.0 * PI);
    return vec3<f32>(
        0.02 * pulse * t,              // Subtle warmth
        base * (0.7 + 0.3 * pulse),    // Pulsing green
        0.05 + 0.15 * pow(t, 2.0)      // Cyan highlights
    );
}

fn palette_vintage(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.6 + 0.3 * t,
        0.5 + 0.3 * t,
        0.4 + 0.2 * t
    );
}

fn palette_candy(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * sin(4.0 * PI * t),
        0.5 + 0.5 * sin(4.0 * PI * t + 1.0),
        0.5 + 0.5 * sin(4.0 * PI * t + 2.0)
    );
}

fn palette_metal(t: f32) -> vec3<f32> {
    let base = 0.3 + 0.7 * t;
    return vec3<f32>(
        base,
        base * 0.9,
        base * 0.8
    );
}

fn palette_toxic(t: f32) -> vec3<f32> {
    // Radioactive glow with multiple color zones
    let pulse = 0.5 + 0.5 * sin(t * 8.0 * PI);
    if (t < 0.3) {
        let s = t / 0.3;
        return vec3<f32>(0.1 * s, 0.2 + 0.3 * s, 0.05 + 0.1 * s);           // Dark sludge
    } else if (t < 0.6) {
        let s = (t - 0.3) / 0.3;
        return vec3<f32>(0.1 + 0.3 * s * pulse, 0.5 + 0.4 * s, 0.15 + 0.15 * s); // Glowing
    } else if (t < 0.85) {
        let s = (t - 0.6) / 0.25;
        return vec3<f32>(0.4 + 0.5 * s, 0.9 + 0.1 * s, 0.3 - 0.1 * s);      // Bright toxic
    } else {
        let s = (t - 0.85) / 0.15;
        return vec3<f32>(0.9 + 0.1 * s, 1.0, 0.2 + 0.6 * s);                // Hot spots
    }
}

fn palette_aurora(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.2 + 0.3 * sin(2.0 * PI * t),
        0.5 + 0.5 * sin(2.0 * PI * t + 1.5),
        0.3 + 0.5 * sin(2.0 * PI * t + 3.0)
    );
}

fn palette_desert(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.8 + 0.2 * t,
        0.6 + 0.2 * t,
        0.3 + 0.2 * t
    );
}

fn palette_deep_sea(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.0 + 0.1 * t,
        0.1 + 0.3 * t,
        0.3 + 0.5 * t
    );
}

fn palette_magma(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.1 + 0.9 * pow(t, 0.5),
        0.0 + 0.5 * pow(t, 1.5),
        0.2 + 0.3 * pow(t, 3.0)
    );
}

fn palette_bw_bands(t: f32) -> vec3<f32> {
    let v = step(0.5, fract(t * 10.0));
    return vec3<f32>(v, v, v);
}

fn palette_psychedelic(t: f32) -> vec3<f32> {
    return vec3<f32>(
        0.5 + 0.5 * sin(10.0 * PI * t),
        0.5 + 0.5 * sin(10.0 * PI * t + 2.094),
        0.5 + 0.5 * sin(10.0 * PI * t + 4.188)
    );
}

fn palette_thermal(t: f32) -> vec3<f32> {
    // 7-zone smooth thermal imaging: black -> blue -> cyan -> green -> yellow -> orange -> red -> white
    if (t < 0.15) {
        let s = t / 0.15;
        return vec3<f32>(0.0, 0.0, s * 0.5);                    // Black to dark blue
    } else if (t < 0.3) {
        let s = (t - 0.15) / 0.15;
        return vec3<f32>(0.0, s * 0.8, 0.5 + s * 0.5);          // Dark blue to cyan
    } else if (t < 0.45) {
        let s = (t - 0.3) / 0.15;
        return vec3<f32>(s * 0.5, 0.8 + s * 0.2, 1.0 - s * 0.5); // Cyan to greenish
    } else if (t < 0.6) {
        let s = (t - 0.45) / 0.15;
        return vec3<f32>(0.5 + s * 0.5, 1.0, 0.5 - s * 0.5);    // Green to yellow
    } else if (t < 0.75) {
        let s = (t - 0.6) / 0.15;
        return vec3<f32>(1.0, 1.0 - s * 0.4, 0.0);              // Yellow to orange
    } else if (t < 0.9) {
        let s = (t - 0.75) / 0.15;
        return vec3<f32>(1.0, 0.6 - s * 0.6, 0.0);              // Orange to red
    } else {
        let s = (t - 0.9) / 0.1;
        return vec3<f32>(1.0, s, s);                            // Red to white
    }
}

fn get_color(t: f32, scheme: u32) -> vec3<f32> {
    switch(scheme) {
        case 0u: { return palette_classic(t); }
        case 1u: { return palette_fire(t); }
        case 2u: { return palette_ocean(t); }
        case 3u: { return palette_rainbow(t); }
        case 4u: { return palette_grayscale(t); }
        case 5u: { return palette_electric(t); }
        case 6u: { return palette_neon(t); }
        case 7u: { return palette_sunset(t); }
        case 8u: { return palette_forest(t); }
        case 9u: { return palette_lava(t); }
        case 10u: { return palette_ice(t); }
        case 11u: { return palette_plasma(t); }
        case 12u: { return palette_cosmic(t); }
        case 13u: { return palette_autumn(t); }
        case 14u: { return palette_matrix(t); }
        case 15u: { return palette_vintage(t); }
        case 16u: { return palette_candy(t); }
        case 17u: { return palette_metal(t); }
        case 18u: { return palette_toxic(t); }
        case 19u: { return palette_aurora(t); }
        case 20u: { return palette_desert(t); }
        case 21u: { return palette_deep_sea(t); }
        case 22u: { return palette_magma(t); }
        case 23u: { return palette_bw_bands(t); }
        case 24u: { return palette_psychedelic(t); }
        case 25u: { return palette_thermal(t); }
        default: { return palette_classic(t); }
    }
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Convert UV to normalized device coordinates
    // UV: (0,0) bottom-left to (1,1) top-right after clipping
    // We need to handle the oversized triangle
    let uv = clamp(input.uv, vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 1.0));

    // Map to complex plane centered at params.center
    // uv goes 0->1, we want -1 to 1 range, then scale by aspect and zoom
    // ui_offset shifts the visible center to account for the left UI panel
    // ui_offset_y shifts vertically for proper centering
    let aspect = params.resolution.x / params.resolution.y;
    let ndc = vec2<f32>(
        (uv.x - 0.5) * 2.0 * aspect + params.ui_offset,
        (uv.y - 0.5) * 2.0 + params.ui_offset_y
    );

    let c = params.center + ndc / params.zoom;

    // Iterate based on fractal type
    var result: vec2<f32>;

    switch(params.fractal_type) {
        case FRACTAL_MANDELBROT: {
            result = iterate_mandelbrot(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_JULIA: {
            result = iterate_julia(c, params.julia_c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_BURNING_SHIP: {
            result = iterate_burning_ship(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_TRICORN: {
            result = iterate_tricorn(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_BUFFALO: {
            result = iterate_buffalo(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_CELTIC: {
            result = iterate_celtic(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_PERPENDICULAR_MANDELBROT: {
            result = iterate_perpendicular_mandelbrot(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_PERPENDICULAR_BURNING_SHIP: {
            result = iterate_perpendicular_burning_ship(c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_HEART: {
            result = iterate_heart(c, params.max_iter, params.escape_radius);
        }
        case FRACTAL_TRICORN_JULIA: {
            result = iterate_tricorn_julia(c, params.julia_c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_BUFFALO_JULIA: {
            result = iterate_buffalo_julia(c, params.julia_c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_CELTIC_JULIA: {
            result = iterate_celtic_julia(c, params.julia_c, params.power, params.max_iter, params.escape_radius);
        }
        case FRACTAL_BURNING_SHIP_JULIA: {
            result = iterate_burning_ship_julia(c, params.julia_c, params.power, params.max_iter, params.escape_radius);
        }
        default: {
            result = iterate_mandelbrot(c, params.power, params.max_iter, params.escape_radius);
        }
    }

    let iter = result.x;
    let z_mag2 = result.y;

    // Calculate color
    var color: vec3<f32>;

    if (iter >= f32(params.max_iter)) {
        // Point is in the set
        color = vec3<f32>(0.0, 0.0, 0.0);
    } else {
        var t: f32;

        if ((params.flags & FLAG_SMOOTH) != 0u) {
            // Smooth coloring
            let log_zn = log(max(z_mag2, 1.0)) / 2.0;
            let nu = log(max(log_zn / log(2.0), 1e-10)) / log(params.power);
            t = (iter + 1.0 - nu) / f32(params.max_iter);
        } else {
            t = iter / f32(params.max_iter);
        }

        // Apply offset
        if ((params.flags & FLAG_OFFSET) != 0u) {
            t = fract(t * 5.0);
        }

        // Apply invert
        if ((params.flags & FLAG_INVERT) != 0u) {
            t = 1.0 - t;
        }

        color = get_color(t, params.color_scheme);
    }

    return vec4<f32>(color, 1.0);
}
