#![cfg_attr(target_arch = "spirv", no_std)]

use shared::sdf_2d as sdf;
use shared::*;
use spirv_std::glam::{vec2, vec3, Mat2, Vec2, Vec4};
use spirv_std::num_traits::Euclid;
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.001;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), *$(,)?) => (min!($($y),*).min($x))
}

fn sdf(p: Vec2, time: f32) -> f32 {
    let c = 0.6;
    let r = 0.15;
    let x = Euclid::rem_euclid(&(time / 2.0), &4.0);
    let angle = c / (2.0 * r)
        * if x > 3.0 {
            x - 4.0
        } else if x > 1.0 {
            2.0 - x
        } else {
            x
        };
    min!(
        sdf::torus(p - vec2(0.0, -0.2), vec2(r - 0.03 - 0.005, 0.03)),
        sdf::capsule(
            Mat2::from_angle(angle).mul_vec2(p - vec2(0.0, -0.2)) - vec2(-r * angle, r),
            vec2(-c / 2.0, 0.0),
            vec2(c / 2.0, 0.0),
            0.005
        ),
        sdf::rectangle(p - vec2(0.0, -0.745), vec2(0.2, 0.4)),
        sdf::plane(p - vec2(0.0, -0.4)),
    )
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
    let uv = from_pixels(frag_coord.x, frag_coord.y, constants);
    let cursor = from_pixels(constants.cursor_x, constants.cursor_y, constants);
    let ro = from_pixels(constants.drag_end_x, constants.drag_end_y, constants);

    let rd = (0.99999 * cursor - ro).normalize();

    let mut col = {
        let d = sdf(uv, constants.time);

        if d < 0.0 {
            vec3(10.0 * -d, -d, 0.0)
        } else {
            vec3(10.0 * d, 0.0, d / 6.0)
        }
    };

    let mut d0 = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = Float::abs(sdf(p, constants.time));
        col = col
            .lerp(
                vec3(0.0, 0.6, 0.0),
                smoothstep(
                    6.0 / constants.height as f32,
                    0.0,
                    sdf::line(uv, p, p + rd * Float::max(ds, Float::epsilon())),
                ),
            )
            .lerp(
                vec3(0.5, 0.6, 0.4),
                smoothstep(
                    1.0 / constants.height as f32,
                    0.0,
                    sdf::circle(uv - p, 0.006),
                ),
            )
            .lerp(
                vec3(0.2, 0.4, 0.1),
                smoothstep(
                    2.0 / constants.height as f32,
                    0.0,
                    Float::abs(sdf::circle(uv - p, ds)),
                ),
            );
        d0 += ds;
        if d0 > MAX_DIST || ds < SURF_DIST {
            break;
        }
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
