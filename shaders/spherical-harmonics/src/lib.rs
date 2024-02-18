#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use core::f32::consts::{FRAC_1_SQRT_2, PI};
use push_constants::spherical_harmonics::{ShaderConstants, Variant};
use shared::*;
use spirv_std::glam::{
    vec2, vec3, Quat, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles,
};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

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
        (-1.0).powi(m as i32) * 2.0.powi(l as i32) * bb * sm
    }
    if m < 0 {
        (-1.0).powi(-m) * factorialu(l + m as u32) / factorialu(l - m as u32)
            * legendre_polynomial_positive((-m) as u32, l, x)
    } else {
        legendre_polynomial_positive(m as u32, l, x)
    }
}

fn spherical_harmonic(m: i32, l: u32, theta: f32, phi: f32, time: f32) -> Complex {
    let normalization_constant = (((2 * l + 1) as f32 * factorialu(l - m as u32))
        / (4.0 * PI * factorialu(l + m as u32)))
    .sqrt();
    let angular = Complex::from_angle(phi * m as f32);
    let lp = legendre_polynomial(m, l, theta.cos());
    normalization_constant * lp * angular * Complex::from_angle(time)
}

fn real_spherical_harmonic(m: i32, l: u32, theta: f32, phi: f32, time: f32) -> f32 {
    let sh = spherical_harmonic(m.abs(), l, theta, phi, time);
    if m == 0 {
        sh.x
    } else if m > 0 {
        2.0.sqrt() * sh.x
    } else {
        2.0.sqrt() * sh.y
    }
}

fn ray_intersect_box_frame(ro: Vec3, rd: Vec3, dim: Vec2) -> bool {
    let e = vec2(dim.x, -dim.x);
    let o = vec3(dim.y, -dim.y, 0.0);
    ray_intersect_aabb(ro, rd, e.yyy(), e.yxy() + o.xzx())
        || ray_intersect_aabb(ro, rd, e.yyy(), e.xyy() + o.zxx())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.yxy() + o.zyx())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.xyy() + o.yzx())
        || ray_intersect_aabb(ro, rd, e.yyx(), e.yxx() + o.xzy())
        || ray_intersect_aabb(ro, rd, e.yyx(), e.xyx() + o.zxy())
        || ray_intersect_aabb(ro, rd, e.xxx(), e.yxx() + o.zyy())
        || ray_intersect_aabb(ro, rd, e.xxx(), e.xyx() + o.yzy())
        || ray_intersect_aabb(ro, rd, e.yyy(), e.yyx() + o.xxz())
        || ray_intersect_aabb(ro, rd, e.yxy(), e.yxx() + o.xyz())
        || ray_intersect_aabb(ro, rd, e.xyy(), e.xyx() + o.yxz())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.xxx() + o.yyz())
}

fn ray_intersect_aabb(ro: Vec3, rd: Vec3, a: Vec3, b: Vec3) -> bool {
    let t1 = (a - ro) / rd;
    let t2 = (b - ro) / rd;
    t1.max(t2).min_element() > t1.min(t2).max_element()
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = (Complex::from(frag_coord.xy())
        - 0.5 * Complex::new(constants.size.width as f32, constants.size.height as f32))
        / constants.size.height as f32;
    let rot: Quat = constants.quat.into();
    let r = 0.3 / constants.zoom;

    let col = if uv.length_squared() <= r * r {
        let pos = {
            let x = uv.x;
            let y = uv.y;
            let z = -(r * r - x * x - y * y).sqrt();
            rot * uv.extend(z)
        };
        let (_, theta, phi) = to_spherical(pos);
        let m = constants.m;
        let l = constants.l;
        match Variant::from_u32(constants.variant) {
            Variant::Real => {
                let x = real_spherical_harmonic(m, l, theta, phi, constants.time);
                vec3(x, 0.0, -x)
            }
            Variant::Complex => {
                let z = spherical_harmonic(m, l, theta, phi, constants.time);
                vec3(
                    z.dot(Vec2::X),
                    z.dot(vec2(-FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
                    z.dot(Vec2::splat(-FRAC_1_SQRT_2)),
                )
            }
        }
    } else {
        let ro = rot * uv.extend(-constants.zoom);
        let rd = rot * Vec2::ZERO.extend(1.0);
        if ray_intersect_box_frame(ro, rd, vec2(r, 0.002 / constants.zoom)) {
            vec3(0.1, 0.1, 0.08)
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
