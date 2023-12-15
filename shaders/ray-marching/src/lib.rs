#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::ray_marching::ShaderConstants;
use shared::sdf_3d as sdf;
use shared::*;
use spirv_std::glam::{vec2, vec3, Mat3, Vec2Swizzles, Vec3, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.01;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), *$(,)?) => (min!($($y),*).min($x))
}

fn sdf(p: Vec3, time: f32) -> f32 {
    min!(
        sdf::plane(p - vec3(0.0, -1.8, 0.0), Vec3::Y),
        sdf::sphere(
            sdf::ops::repeat_xz(p - vec3(0.0, -2.0, 0.0), vec2(1.0, 1.0)),
            0.5 + 0.2 * time.sin()
        ),
        sdf::torus(p - vec3(2.0, 1.0, 0.0), vec2(0.6, 0.2)),
        sdf::cuboid(p - vec3(-2.0, 1.0, 0.0), vec3(0.5, 0.3, 0.4)),
        sdf::tetrahedron(p - vec3(4.0, 1.0, 0.0), 0.5),
        sdf::capsule(p, vec3(-5.0, 1.0, 0.0), vec3(-4.0, 1.0, 0.0), 0.5),
        sdf::line_segment(p, vec3(-0.5, 1.0, 2.0), vec3(0.5, 1.0, 2.0)),
        sdf::cylinder(
            Mat3::from_rotation_y(time).mul_vec3(p - vec3(6.0, 1.0, 0.0)),
            vec3(-0.5, 0.0, 0.0),
            vec3(0.5, 0.0, 0.0),
            0.5
        ),
    )
}

fn ray_march(ro: Vec3, rd: Vec3, time: f32) -> (f32, f32) {
    let mut d0 = 0.0;
    let mut cd = f32::INFINITY;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf(p, time);
        cd = cd.min(ds);
        d0 += ds;
        if ds < SURF_DIST {
            cd = 0.0;
            break;
        }
        if d0 > MAX_DIST {
            break;
        }
    }

    (d0, cd)
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
    let light_pos = vec3(8.0 * time.sin(), 5.0, 8.0 * time.cos());
    let light_vector = (light_pos - p).normalize();
    let normal_vector = get_normal(p, time);
    let mut dif = light_vector.dot(normal_vector).clamp(0.0, 1.0);
    let (d, _) = ray_march(p + normal_vector * SURF_DIST * 2.0, light_vector, time);
    if d < light_pos.distance(p) {
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

    let (d, cd) = ray_march(ro, rd, constants.time);
    let dif = get_light(ro + rd * d, constants.time);
    let c = cd.abs().atan() / (PI / 2.0);
    let col = vec3(dif, dif, dif).lerp(vec3(1.0 - c, 0.5 - c, 0.2 - c), 0.2);

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
