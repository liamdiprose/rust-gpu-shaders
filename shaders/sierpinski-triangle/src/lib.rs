#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sierpinski_triangle::ShaderConstants;
use shared::sdf_2d as sdf;
use shared::sdf_2d::ops::difference;
use shared::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn sierpinski_triangle(mut p: Vec2, mut r: f32, m: u32) -> f32 {
    let n = Vec2::from_angle(-PI / 3.0);
    let c = (PI / 6.0).cos();

    let mut d = sdf::equilateral_triangle(p, r / c);
    r /= SQRT_3;

    for _ in 0..m {
        p.x = p.x.abs();
        d = difference(
            d,
            difference(
                sdf::plane_segment(p, vec2(c * r, 0.5 * r), vec2(0.0, -r)),
                sdf::plane_ray(p - vec2(c * r, 0.5 * r), Vec2::NEG_X),
            ),
        );
        p = vec2(p.x - c * r, p.y + r * 0.5);
        p -= n * n.dot(p - vec2(0.0, r)).min(0.0) * 2.0;
        r *= 0.5;
    }

    d
}

fn from_pixels(x: f32, y: f32, constants: &ShaderConstants) -> Vec2 {
    constants.zoom * (vec2(x, -y) - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let coord = vec2(frag_coord.x, frag_coord.y);
    let uv = from_pixels(coord.x, coord.y, constants);

    let mut col = Vec3::ZERO;

    let d = sierpinski_triangle(uv - vec2(constants.x, constants.y), 0.25, 22);
    col += vec3(0.9, 0.6, 0.4) * smoothstep(constants.zoom / constants.height as f32, 0.0, d);

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
