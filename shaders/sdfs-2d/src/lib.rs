#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sdfs_2d::ShaderConstants;
use shared::push_constants::sdfs_2d::Shape;
use shared::*;
use shared::{push_constants::sdfs_2d::Params, sdf_2d as sdf};
use spirv_std::glam::{vec2, vec3, Vec2, Vec3, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn sdf(
    p: Vec2,
    shape: u32,
    rotation: f32,
    Params {
        radius,
        width,
        height,
        ..
    }: Params,
) -> f32 {
    use Shape::*;
    let p = p.rotate(Vec2::from_angle(rotation));
    match Shape::from_u32(shape) {
        Circle => sdf::circle(p, radius),
        Rectangle => sdf::rectangle(p, vec2(width, height)),
        EquilateralTriangle => sdf::equilateral_triangle(p, radius),
    }
}

fn from_pixels(x: f32, y: f32, constants: &ShaderConstants) -> Vec2 {
    (vec2(x, -y) - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = from_pixels(frag_coord.x, frag_coord.y, constants);
    let cursor = from_pixels(constants.cursor_x, constants.cursor_y, constants);

    let col = {
        let d = sdf(uv, constants.shape, constants.rotation, constants.params);

        let mut col = if d < 0.0 {
            vec3(0.65, 0.85, 1.0)
        } else {
            vec3(0.9, 0.6, 0.3)
        };
        col *= 1.0 - (-6.0 * d.abs()).exp();
        col *= 0.8 + 0.2 * (150.0 * d).cos();
        col = col.lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.01, d.abs()));

        if constants.mouse_button_pressed & 1 != 0 {
            let d = sdf(
                cursor,
                constants.shape,
                constants.rotation,
                constants.params,
            );
            col = col
                .lerp(
                    vec3(1.0, 1.0, 0.0),
                    smoothstep(
                        1.0 / constants.height as f32,
                        0.0,
                        sdf::circle(uv - cursor, 0.01),
                    ),
                )
                .lerp(
                    vec3(1.0, 1.0, 0.0),
                    smoothstep(
                        1.0 / constants.height as f32,
                        0.0,
                        sdf::circle(uv - cursor, d.abs()).abs() - 0.0025,
                    ),
                );
        }

        col
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
