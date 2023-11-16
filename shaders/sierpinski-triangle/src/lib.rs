#![cfg_attr(target_arch = "spirv", no_std)]

use shared::sdf_2d as sdf;
use shared::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3, Vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn sdf(p: Vec2, _time: f32) -> f32 {
    let r = 1.0;
    sierpinski_triangle(p + vec2(0.65 * r, 0.0), r, 20)
}

fn sierpinski_triangle(mut p: Vec2, mut r: f32, m: u32) -> f32 {
    let n = Vec2::from_angle(-PI / 3.0);
    let c = (PI / 6.0).cos();

    let mut d = sdf::equilateral_triangle(p, r);
    r = r / SQRT_3;

    for _ in 0..m {
        p.x = p.x.abs();
        d = subtract(
            d,
            subtract(
                sdf::plane_segment(p, vec2(c * r, 0.5 * r), vec2(0.0, -r)),
                sdf::plane_ray(p - vec2(c * r, 0.5 * r), Vec2::NEG_X),
            ),
        );
        r = 0.5 * r;
        p = vec2(p.x - 2.0 * c * r, p.y + r);
        p -= n * n.dot(p - vec2(0.0, 2.0 * r)).min(0.0) * 2.0;
    }

    d
}

pub fn subtract(a: f32, b: f32) -> f32 {
    Float::max(a, -b)
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
    let coord = vec2(
        frag_coord.x + constants.translate_x + (constants.drag_start_x - constants.drag_end_x),
        frag_coord.y + constants.translate_y + (constants.drag_start_y - constants.drag_end_y),
    );
    let uv = from_pixels(coord.x, coord.y, constants);

    let mut col = Vec3::ZERO;

    let d = sdf(uv, constants.time);
    col += vec3(0.9, 0.6, 0.4) * smoothstep(constants.zoom / constants.height as f32, 0.0, d);

    if constants.mouse_button_pressed & 1 != 0 {
        let cursor = from_pixels(constants.drag_end_x, constants.drag_end_y, constants);

        let ds = Float::abs(sdf(cursor, constants.time));
        col = col.lerp(
            vec3(0.1, 0.6, 0.8),
            smoothstep(
                2.0 * constants.zoom / constants.height as f32,
                0.0,
                Float::abs(sdf::circle(uv - cursor, ds))
                    .min(sdf::circle(uv - cursor, constants.zoom * 0.005)),
            ),
        );
    }

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
