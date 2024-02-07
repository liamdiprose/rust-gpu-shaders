#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use core::f32::consts::FRAC_1_SQRT_2;
use push_constants::spherical_harmonics::ShaderConstants;
use shared::{sdf_3d as sdf, *};
use spirv_std::glam::{vec2, vec3, Quat, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 10.0;
const SURF_DIST: f32 = 0.001;

fn to_spherical(pos: Vec3) -> (f32, f32, f32) {
    let r = pos.length();
    let theta = (pos.z / r).acos();
    let phi = pos.y.signum() * (pos.x / pos.xy().length()).acos();
    (r, theta, phi)
}

fn factorialu(n: u32) -> f32 {
    let mut x = 1.0;
    for i in 2..=n {
        x *= i as f32;
    }
    x
}

fn binomial(n: u32, k: u32) -> f32 {
    let mut x = 1.0;
    for i in 1..=k {
        x *= (n + 1 - i) as f32 / i as f32;
    }
    x
}

fn general_binomial(n: f32, k: u32) -> f32 {
    let mut x = 1.0;
    for i in 0..k {
        x *= n - i as f32;
    }
    x / factorialu(k)
}

fn legendre_polynomial(m: i32, l: u32, x: f32) -> Complex {
    fn legendre_polynomial_positive(m: u32, l: u32, x: f32) -> Complex {
        let mut sm = 0.0;
        for k in m..=l {
            sm += factorialu(k) / factorialu(k - m)
                * x.powi((k - m) as i32)
                * binomial(l, k)
                * general_binomial(((l + k) as f32 - 1.0) / 2.0, l);
        }
        let bb = Complex::new(1.0 - x * x, 0.0).powf(m as f32 / 2.0);
        (-1.0_f32).powi(m as i32) * 2.0_f32.powi(l as i32) * bb * sm
    }
    if m < 0 {
        (-1.0_f32).powi(-m) * factorialu(l + m as u32) / factorialu(l - m as u32)
            * legendre_polynomial_positive((-m) as u32, l, x)
    } else {
        legendre_polynomial_positive(m as u32, l, x)
    }
}

fn spherical_harmonic(m: i32, l: u32, theta: f32, phi: f32) -> Complex {
    let normalization_constant = (((2 * l + 1) as f32 * factorialu(l - m as u32))
        / (4.0 * PI * factorialu(l + m as u32)))
    .sqrt();
    let angular = (Complex::I * phi * m as f32).exp();
    let lp = legendre_polynomial(m, l, theta.cos());
    normalization_constant * lp * angular
}

fn ray_march_sphere(ro: Vec3, rd: Vec3) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf::sphere(p, 0.3);
        d0 += ds;
        if ds < SURF_DIST || d0 > MAX_DIST {
            break;
        }
    }

    d0
}

fn ray_march_cage(ro: Vec3, rd: Vec3) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf::cuboid_frame(p, Vec3::splat(0.6), Vec3::splat(0.001));
        d0 += ds;
        if ds < SURF_DIST || d0 > MAX_DIST {
            break;
        }
    }

    d0
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = (Complex::from(frag_coord.xy())
        - 0.5 * Complex::new(constants.width as f32, constants.height as f32))
        / constants.height as f32;

    let rq = Quat::from_xyzw(constants.x, constants.y, constants.z, constants.w);
    let ro = rq.mul_vec3(Vec2::ZERO.extend(-constants.zoom));
    let rd = rq.mul_vec3(uv.extend(1.0).normalize());

    let d = ray_march_sphere(ro, rd);
    let col = if d < MAX_DIST {
        let (_, theta, phi) = to_spherical(ro + rd * d);
        let z = spherical_harmonic(constants.m, constants.l, theta, phi);
        vec3(
            z.dot(Vec2::X),
            z.dot(vec2(-FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
            z.dot(Vec2::splat(-FRAC_1_SQRT_2)),
        )
    } else {
        if ray_march_cage(ro, rd) < MAX_DIST {
            vec3(0.1, 0.1, 0.05)
        } else {
            Vec3::ZERO
        }
    };

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
