#![cfg_attr(target_arch = "spirv", no_std)]

pub mod operator;
pub mod sdf;

use shared::*;
use spirv_std::glam::{vec2, vec3, vec4, Mat3, Vec2, Vec2Swizzles, Vec3, Vec4};
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const PI: f32 = 3.14159265358979323846264338327950288f32;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.01;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), *$(,)?) => (min!($($y),*).min($x))
}

fn sdf(p: Vec3, time: f32) -> f32 {
    min!(
        sdf::plane(p),
        sdf::sphere(p - vec3(0.0, 1.0, 0.0), 0.5),
        sdf::torus(p - vec3(2.0, 1.0, 0.0), vec2(0.6, 0.2)),
        sdf::cuboid(p - vec3(-2.0, 1.0, 0.0), vec3(0.5, 0.3, 0.4)),
        sdf::tetrahedron(p - vec3(4.0, 1.0, 0.0), 0.5),
        sdf::capsule(p - vec3(-4.5, 1.0, 0.0), vec3(1.0, 0.0, 0.0), 0.5),
        sdf::cylinder(
            Mat3::from_rotation_y(time).mul_vec3(p - vec3(6.0, 1.0, 0.0)),
            vec3(1.0, 0.0, 0.0),
            0.5
        ),
    )
}

fn ray_march(ro: Vec3, rd: Vec3, time: f32) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf(p, time);
        d0 += ds;
        if d0 > MAX_DIST || ds < SURF_DIST {
            break;
        }
    }

    d0
}

fn get_normal(p: Vec3, time: f32) -> Vec3 {
    let d = sdf(p, time);
    let e = vec2(0.01, 0.0);
    let n = d - vec3(
        sdf(p - e.xyy(), time),
        sdf(p - e.yxy(), time),
        sdf(p - e.yyx(), time),
    );
    n.normalize()
}

fn get_light(p: Vec3, time: f32) -> f32 {
    let light_pos = vec3(2.0 * Float::sin(time), 5.0, 2.0 * Float::cos(time));
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
    let translate = -vec2(
        constants.translate_x + constants.drag_start_x - constants.drag_end_x,
        constants.translate_y + constants.drag_start_y - constants.drag_end_y,
    ) / constants.height as f32
        * PI;

    let uv = (vec2(frag_coord.x, -frag_coord.y)
        - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32;

    let rm = Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
    let ro = rm.mul_vec3(vec3(0.0, 1.0, -constants.zoom));
    let rd = rm.mul_vec3(vec3(uv.x, uv.y, 1.0)).normalize();

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
