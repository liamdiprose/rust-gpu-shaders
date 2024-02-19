use core::f32::consts::PI;
use shared::sdf_2d as sdf;
use spirv_std::glam::{vec2, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

fn curve1(t: f32) -> Vec2 {
    vec2((t * 2.0 * PI).sin().tan(), (t * 2.0 * PI).cos().tan()) / (3.0 * PI / 2.0)
}
fn curve2(t: f32) -> Vec2 {
    vec2(t * PI - PI / 2.0, (t * PI).sin()) / PI
}
fn curve(t: f32) -> Vec2 {
    curve1(t)
}

// // using points
pub fn parametric_sdf_points(p: Vec2) -> f32 {
    let count = 100;
    let dt = 1.0 / count as f32;

    let mut d = f32::INFINITY;

    for i in 0..count {
        d = d.min(p.distance(curve(dt * i as f32)));
    }

    d
}

// using line segments
pub fn parametric_sdf_lines(p: Vec2) -> f32 {
    let count = 90;
    let dt = 1.0 / count as f32;

    let mut p0 = curve(0.0);
    let mut d = f32::INFINITY;

    for i in 1..=count {
        let p1 = curve(dt * i as f32);
        d = d.min(sdf::line_segment(p, p0, p1));
        p0 = p1;
    }

    d
}
