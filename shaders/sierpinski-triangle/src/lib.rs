#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sierpinski_triangle::ShaderConstants;
use shared::sdf_2d as sdf;
use shared::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec2Swizzles, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn sierpinski_triangle(mut p: Vec2, mut r: f32, m: u32) -> f32 {
    let n = vec2(0.5, -0.5 * SQRT_3);

    r /= SQRT_3;
    let mut d = sdf::equilateral_triangle(p, 2.0 * r);

    for _ in 0..m {
        p.x = p.x.abs();
        d = sdf::equilateral_triangle(p, r);
        p += n.yx() * r;
        p -= n * n.dot(p - Vec2::Y * r).min(0.0) * 2.0;
        r *= 0.5;
    }

    d
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = constants.zoom * from_pixels(frag_coord.xy(), constants.size);
    let dim: Vec2 = constants.dim.into();

    let d = sierpinski_triangle(uv - dim, 0.25, 22);
    let thickness = constants.zoom / constants.size.height as f32;
    let col = vec3(0.9, 0.6, 0.4) * smoothstep(thickness, 0.0, d);

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
