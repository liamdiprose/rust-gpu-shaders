#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use push_constants::hydrogen_wavefunction::ShaderConstants;
use shared::*;
use spirv_std::glam::{vec3, vec4, Vec2, Vec3, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const A: f32 = 1.00054 * 5.29177210903e-11; // Bohr radius

fn factorialu(n: u32) -> f32 {
    [
        1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
    ][n as usize] as f32
}

fn laguerre_polynomial(n: u32, alpha: f32, x: f32) -> f32 {
    let l0 = 1.0;
    let l1 = 1.0 + alpha - x;
    match n {
        0 => l0,
        1 => l1,
        _ => {
            let mut v0 = l0;
            let mut v = l1;
            for i in 1..n {
                let k = i as f32;
                (v0, v) = (
                    v,
                    ((2.0 * k + 1.0 + alpha - x) * v - (k + alpha) * v0) / (k + 1.0),
                );
                // println!("{}", v);
            }

            v
        }
    }
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

fn legendre_polynomial(m: i32, mut l: i32, x: f32) -> Complex {
    fn legendre_polynomial_positive(m: u32, l: u32, x: f32) -> Complex {
        // 1.0
        let mut sm = 0.0;
        for k in m..=l {
            sm += factorialu(k) / factorialu(k - m)
                * x.powi((k - m) as i32)
                * binomial(l, k)
                * general_binomial(((l + k) as f32 - 1.0) / 2.0, l);
        }
        // let sm = 1.0;
        // let bb = if x == 1.0 {
        //     // Complex::ONE
        //     Complex::new(1.0 - x * x, 0.0).powf(m as f32 / 2.0)
        // } else {
        //     Complex::new(1.0 - x * x, 0.0).powf(m as f32 / 2.0)
        // };
        let bb = Complex::new(1.0 - x * x, 0.0).powf(m as f32 / 2.0);
        // println!("sm: {}", sm);
        // println!("x: {}", x);
        // println!("bb: {:?}", bb);
        // println!("m: {}", m);
        // println!("{}", m as f32 / 2.0);
        (-1.0_f32).powi(m as i32) * 2.0_f32.powi(l as i32) * bb * sm
        // (-1.0_f32).powi(m as i32) * 2.0_f32.powi(l as i32) * (1.0 - x * x).powf(m as f32 / 2.0) * sm
    }
    if l < 0 {
        l = -l - 1;
    }
    if m < 0 {
        (-1.0_f32).powi(-m) * factorialu((l + m) as u32) / factorialu((l - m) as u32)
            * legendre_polynomial_positive((-m) as u32, l as u32, x)
    } else {
        legendre_polynomial_positive(m as u32, l as u32, x)
    }
}

fn spherical_harmonic(m: i32, l: i32, theta: f32, phi: f32) -> Complex {
    let normalization_constant = (((2 * l + 1) as f32 * factorialu((l - m) as u32))
        / (4.0 * PI * factorialu((l + m) as u32)))
    .sqrt();
    let angular = (Complex::I * phi * m as f32).exp();
    let lp = legendre_polynomial(m, l, theta.cos());
    // println!("(m,l,theta,phi): ({}, {}, {}, {})", m, l, theta, phi);
    // println!("nc:  {}", normalization_constant);
    // println!("ang: {:?}", angular);
    // println!("lp : {:?}", lp);
    normalization_constant * lp * angular
}

fn hydrogen_wavefunction(n: u32, l: u32, m: i32, theta: f32, phi: f32, r: f32) -> Complex {
    let p = (2.0 * r) / (n as f32 * A);
    let normalization_constant = ((2.0 / (n as f32 * A)) * factorialu(n - l - 1)
        / (2.0 * n as f32 * factorialu(n + l)).powi(3))
    .sqrt();
    let asymptotic_forms = (-r / (n as f32 * A)).exp() * p.powi(l as i32);
    let lp = laguerre_polynomial((n - l) as u32 - 1, 2.0 * l as f32 + 1.0, p);
    let radial = asymptotic_forms * lp;
    let angular = spherical_harmonic(m, l as i32, theta, phi);
    // println!("(n,l)     ({},{})", n,l);
    // println!("p     {:?}", p);
    // println!("r     {:?}", r);
    // println!("p     {:?}", p);
    // println!("nc     {:?}", normalization_constant);
    // println!("1       {:?}", (-r * p / 2.0).exp());
    // println!("2       {:?}", (r * p).powi(l as i32));
    // println!("asmfrms {:?}", asymptotic_forms);
    // println!("lp      {:?}", lp);
    // println!("radial  {:?}", radial);
    // println!("angular {:?}", angular);
    normalization_constant * radial * angular
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let coord = Complex::new(
        frag_coord.x,
        frag_coord.y,
    );

    let uv = constants.zoom
        * A
        * 64.0
        * (coord - 0.5 * Complex::new(constants.width as f32, constants.height as f32))
        / constants.height as f32;

    let mut phi = uv.arg();
    if phi < 0.0 {
        phi += 2.0 * PI;
    }
    // let theta = constants.translate_x / constants.height as f32;
    let theta = constants.time * PI;
    let z = hydrogen_wavefunction(
        constants.n,
        constants.l,
        constants.m,
        theta,
        phi,
        uv.distance(Vec2::ZERO),
    );

    let red = z.dot(Vec2::X);
    let green = z.dot(Vec2::from_angle(3.0*PI/4.0));
    let blue = z.dot(Vec2::from_angle(-3.0*PI/4.0));

    let col = vec3(
        red,
        green,
        blue
    );
    let c = z.norm_squared();
    let col = vec3(c,c,c);
    // let c = z.norm_squared().powf(1.0/6.0);
    // let mut col = vec3(z.x.signum() * c, c, z.y.signum() * c);
    // if z.norm_squared() > 1.0 {
    //     col = vec3(1.0,0.0,0.0);
    // }
    // if z.norm_squared() > 2.0 {
    //     col = vec3(0.0,0.0,1.0);
    // }
    // if c > 0.01 {
        // col = vec3(z.x, c, z.y);
    // } else {
    //     col = vec3(0.0,0.0,0.0);
    // }

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
