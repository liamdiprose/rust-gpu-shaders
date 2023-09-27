#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use shared::*;
use spirv_std::glam::{vec2, vec4, Vec2, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let coord = vec2(frag_coord.x, frag_coord.y);

    let uv = 2.0
        * (coord - vec2(constants.width as f32 * 0.75, constants.height as f32 * 0.5))
        / constants.height as f32;

    let mut z = Vec2::ZERO;
    let mut n = 35;
    while z.length() < 2.0 && n > 0 {
        z = z.square() + uv;
        n -= 1;
    }

    let c = n as f32 / 35.0;
    *output = vec4(c, c, c, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}
