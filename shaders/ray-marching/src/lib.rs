#![cfg_attr(target_arch = "spirv", no_std)]

pub mod distance_estimate;
pub mod operator;

use shared::*;
use spirv_std::glam::{vec2, vec3, vec4, Mat3, Vec2, Vec2Swizzles, Vec3, Vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.01;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), *$(,)?) => (min!($($y),*).min($x))
}

fn distance_estimate(p: Vec3, time: f32) -> f32 {
    min!(
        distance_estimate::plane(p),
        distance_estimate::sphere(p - vec3(0.0, 1.0, 6.0), 0.5),
        distance_estimate::torus(p - vec3(1.0, 1.0, 6.0), vec2(0.5, 0.1)),
        distance_estimate::cuboid(p - vec3(-1.0, 1.0, 6.0), vec3(0.5, 0.3, 0.4)),
        distance_estimate::tetrahedron(p - vec3(2.0, 1.0, 6.0), 0.5),
        distance_estimate::capsule(p - vec3(-2.5, 1.0, 6.0), vec3(1.0, 0.0, 0.0), 0.5),
        distance_estimate::cylinder(
            Mat3::from_rotation_y(time).mul_vec3(p - vec3(3.5, 1.0, 6.0)),
            vec3(1.0, 0.0, 0.0),
            0.5
        ),
    )
}

fn ray_march(ro: Vec3, rd: Vec3, time: f32) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = distance_estimate(p, time);
        d0 += ds;
        if d0 > MAX_DIST || ds < SURF_DIST {
            break;
        }
    }

    d0
}

fn get_normal(p: Vec3, time: f32) -> Vec3 {
    let d = distance_estimate(p, time);
    let e = vec2(0.01, 0.0);
    let n = d - vec3(
        distance_estimate(p - e.xyy(), time),
        distance_estimate(p - e.yxy(), time),
        distance_estimate(p - e.yyx(), time),
    );
    n.normalize()
}

fn get_light(p: Vec3, time: f32) -> f32 {
    let light_pos = vec3(2.0 * Float::sin(time), 5.0, 6.0 + 2.0 * Float::cos(time));
    let light_vector = (light_pos - p).normalize();
    let normal_vector = get_normal(p, time);
    let mut dif = light_vector.dot(normal_vector).clamp(0.0, 1.0);
    let d = ray_march(p + normal_vector * SURF_DIST * 2.0, light_vector, time);
    if d < (light_pos - p).length() {
        dif *= 0.1;
    }
    dif
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

    let mut uv = constants.zoom
        * (coord - 0.5 * vec2(constants.width as f32, constants.height as f32))
        / constants.height as f32;
    uv.y *= -1.0;

    let ro = vec3(0.0, 1.0, 0.0);
    let rd = vec3(uv.x, uv.y, 1.0).normalize();

    let d = ray_march(ro, rd, constants.time);
    let dif = get_light(ro + rd * d, constants.time);
    let col = vec3(dif, dif, dif);

    *output = vec4(col.x, col.y, col.z, 1.0);
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
