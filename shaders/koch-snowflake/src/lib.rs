#![cfg_attr(target_arch = "spirv", no_std)]

use core::f32::consts::PI;
use shared::*;
use spirv_std::glam::{vec2, Vec2, Vec2Swizzles, Vec3, Vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn from_pixels(x: f32, y: f32, constants: &ShaderConstants) -> Vec2 {
    (vec2(x, -y) - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32
}

fn koch_curve(mut p: Vec2, r: f32, m: u32) -> f32 {
    let angle = (11.0 / 6.0) * PI;
    let n = Vec2::from_angle(angle);

    p.x += r * 0.5;
    let mut scale = 1.0;

    for _ in 0..m {
        scale *= 3.0;
        p *= 3.0;
        p.x -= r * 1.5;

        p.x = p.x.abs();
        p.x -= r * 0.5;
        p -= n * n.dot(p).min(0.0) * 2.0;
    }

    p.y / scale
}

fn koch_snowflake(mut p: Vec2, r: f32, m: u32) -> f32 {
    let angle = (5.0 / 6.0) * PI;
    let n = Vec2::from_angle(angle).yx();
    p.x = p.x.abs();
    p.y += r * angle.tan() * 0.5;
    p -= n * n.dot(p - vec2(r / 2.0, 0.0)).max(0.0) * 2.0;
    koch_curve(p, r, m)
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = from_pixels(frag_coord.x, frag_coord.y, constants);
    let cursor = from_pixels(constants.cursor_x, constants.cursor_y, constants);

    let mut col = Vec3::ZERO;

    let n = 9.0 * Float::log2(1.0 + cursor.length());
    let d = koch_snowflake(uv, 0.8, n as u32);

    col += smoothstep(1.0 / (constants.height as f32), 0.0, d.abs());

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
