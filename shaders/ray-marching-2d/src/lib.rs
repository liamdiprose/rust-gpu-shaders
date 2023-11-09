#![cfg_attr(target_arch = "spirv", no_std)]

pub mod operator;
pub mod sdf;

use shared::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3, Vec4};
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
    min!(
        sdf::circle(p - vec2(-0.2, 0.2), 0.15),
        sdf::rectangle(p - vec2(0.0, -0.2), vec2(0.4, 0.15)),
        sdf::capsule(p, vec2(-0.2, 0.0), vec2(0.5, 0.2), 0.05),
        sdf::torus(p - vec2(-0.2, -0.5), vec2(0.15, 0.05)),
    )
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = constants.zoom
        * (vec2(frag_coord.x, -frag_coord.y)
            - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32;

    let cursor = constants.zoom
        * (vec2(constants.cursor_x, -constants.cursor_y)
            - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32;

    let ro = constants.zoom
        * (vec2(constants.drag_end_x, -constants.drag_end_y)
            - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32;

    let rd = (0.99999 * cursor - ro).normalize();

    let de = sdf(uv, constants.time);

    let mut col = vec3(0.0, 0.0, smoothstep(1.0 / constants.height as f32, 0.0, de));
    col = mix_vec3(
        col,
        vec3(0.0, 1.0, 1.0),
        smoothstep(1.0 / constants.height as f32, 0.0, Float::abs(de)),
    );

    let mut d0 = 0.0;
    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = Float::abs(sdf(p, constants.time));
        col = mix_vec3(
            col,
            Vec3::X,
            smoothstep(
                4.0 / constants.height as f32,
                0.0,
                sdf::line(uv, p, p + rd * ds),
            ),
        );
        col = mix_vec3(
            col,
            vec3(1.0, 0.3, 0.2),
            smoothstep(
                1.0 / constants.height as f32,
                0.0,
                sdf::circle(uv - p, 0.006),
            ),
        );
        col = mix_vec3(
            col,
            vec3(1.0, 0.0, 0.1),
            smoothstep(
                1.0 / constants.height as f32,
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
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}
