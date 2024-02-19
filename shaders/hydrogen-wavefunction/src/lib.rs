#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use core::f32::consts::FRAC_1_SQRT_2;
use push_constants::hydrogen_wavefunction::ShaderConstants;
use shared::*;
use spherical_harmonics::*;
use spirv_std::glam::{vec2, vec3, Mat3, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

// const A: f32 = 1.00054 * 5.29177210903e-11; // Bohr radius
// All units of distance are multiples of the Bohr radius
const A: f32 = 1.0; // Bohr radius

fn laguerre_polynomial(r: u32, s: u32, x: f32) -> f32 {
    let mut sum = 0.0;
    for q in 0..=s {
        sum += (-1.0_f32).powi(q as i32) * factorialu(s + r) * factorialu(s + r) * x.powi(q as i32)
            / (factorialu(s - q) * factorialu(r + q) * factorialu(q));
    }
    sum
}

fn radial_wavefunction(n: u32, l: u32, r: f32) -> f32 {
    let p = (2.0 * r) / (n as f32 * A);
    let normalization_constant = ((2.0 / (n as f32 * A)).powi(3) * factorialu(n - l - 1)
        / (2.0 * n as f32 * factorialu(n + l).powi(3)))
    .sqrt();
    let asymptotic_forms = (-r / (n as f32 * A)).exp() * p.powi(l as i32);
    let lp = laguerre_polynomial(2 * l + 1, n - l - 1, p);
    normalization_constant * asymptotic_forms * lp
}

fn hydrogen_wavefunction(n: u32, l: u32, m: i32, r: f32, theta: f32, phi: f32) -> Complex {
    let radial = radial_wavefunction(n, l, r);
    let angular = spherical_harmonic(m, l, theta, phi, 0.0);
    radial * angular
}

// this function is intentionally overstating the integral for visualization purposes
pub fn integrate_ray(n: u32, l: u32, m: i32, ro: Vec3, rd: Vec3, camera_distance: f32) -> Complex {
    let num_samples = 100;
    let delta_z = 2.0 * camera_distance / num_samples as f32;
    let mut integral = Complex::ZERO;
    let mut pos = ro;
    for _ in 0..num_samples {
        let (r, theta, phi) = to_spherical(pos);
        integral += hydrogen_wavefunction(n, l, m, r, theta, phi);
        pos += rd * delta_z;
    }
    integral * delta_z
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let translate = vec2(constants.translate_x, constants.translate_y) / constants.height as f32;

    let uv = (Complex::from(frag_coord.xy())
        - 0.5 * Complex::new(constants.width as f32, constants.height as f32))
        / constants.height as f32;

    let rm = Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
    let ro = rm * Vec2::ZERO.extend(-constants.camera_distance);
    let rd = rm * uv.extend(1.0).normalize();

    let z = integrate_ray(
        constants.n,
        constants.l,
        constants.m,
        ro,
        rd,
        constants.camera_distance,
    );

    let col = vec3(
        z.dot(Vec2::X),
        z.dot(vec2(-FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
        z.dot(Vec2::splat(-FRAC_1_SQRT_2)),
    )
    .powf(1.0 / constants.root as f32);

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
