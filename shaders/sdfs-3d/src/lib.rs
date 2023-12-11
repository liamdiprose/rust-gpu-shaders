#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sdfs_3d::ShaderConstants;
use shared::push_constants::sdfs_3d::{sdf_shape, Shape};
use shared::*;
use shared::{push_constants::sdfs_3d::Params, sdf_3d as sdf, sdf_3d::ops};
use spirv_std::glam::{vec2, vec3, Mat3, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.0001;

fn sdf(p: Vec3, shape: Shape, slice_z: f32, params: Params) -> f32 {
    ops::difference(
        sdf_shape(p, shape, params),
        sdf::plane(p - slice_z * Vec3::Z, Vec3::Z),
    )
}

fn distance_texture_sdf(p: Vec3, d: f32) -> f32 {
    ops::union(vec2(p.xy().length() - d, p.z).length(), p.length())
}

fn ray_march(ro: Vec3, rd: Vec3, shape: Shape, slice_z: f32, params: Params) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf(p, shape, slice_z, params);
        d0 += ds;
        if ds < SURF_DIST || d0 > MAX_DIST {
            break;
        }
    }

    d0
}

fn ray_march_distance_texture(ro: Vec3, rd: Vec3, cursor_3d_pos: Vec3, distance: f32) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = distance_texture_sdf(p - cursor_3d_pos, distance);
        d0 += ds;
        if ds < 0.005 || d0 > MAX_DIST {
            break;
        }
    }

    d0
}

fn get_d_at_slice(ro: Vec3, rd: Vec3, shape: Shape, slice_z: f32, params: Params) -> f32 {
    let x = (slice_z - ro.z) / rd.z;
    let p = ro + rd * x;
    sdf_shape(p, shape, params)
}

fn get_normal(p: Vec3, shape: Shape, slice_z: f32, params: Params) -> Vec3 {
    let d = sdf(p, shape, slice_z, params);
    let e = 0.01 * Vec2::X;
    let n = d - vec3(
        sdf(p - e.xyy(), shape, slice_z, params),
        sdf(p - e.yxy(), shape, slice_z, params),
        sdf(p - e.yyx(), shape, slice_z, params),
    );
    n.normalize()
}

fn get_light(p: Vec3, shape: Shape, slice_z: f32, params: Params) -> f32 {
    let light_pos = vec3(1.0, 5.0, -1.0);
    let light_vector = (light_pos - p).normalize();
    let normal_vector = get_normal(p, shape, slice_z, params);
    let mut dif = saturate(light_vector.dot(normal_vector));
    let d = ray_march(
        p + normal_vector * SURF_DIST * 2.0,
        light_vector,
        shape,
        slice_z,
        params,
    );
    if d < light_pos.distance(p) {
        dif *= 0.1;
    }
    dif
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
    let translate =
        -vec2(constants.translate_x, constants.translate_y) / constants.height as f32 * PI;

    let uv = from_pixels(frag_coord.x, frag_coord.y, constants);
    let cursor_3d_pos = vec3(constants.cursor_x, constants.cursor_y, constants.cursor_z);

    let rm = Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
    let ro = rm.mul_vec3(-Vec3::Z);
    let rd = rm.mul_vec3(uv.extend(1.0)).normalize();

    let slice_z = constants.slice_z;
    let distance = constants.distance;

    let shape = Shape::from_u32(constants.shape);
    let slice_d = get_d_at_slice(ro, rd, shape, slice_z, constants.params);
    let mut col = if slice_d < 0.0 {
        vec3(0.65, 0.85, 1.0)
    } else {
        let d = ray_march(ro, rd, shape, slice_z, constants.params);
        if d > MAX_DIST {
            vec3(0.9, 0.6, 0.3)
        } else {
            let dif = get_light(ro + rd * d, shape, slice_z, constants.params);
            Vec3::splat(dif).lerp(vec3(0.9, 0.6, 0.3), 0.5)
        }
    };
    col *= 1.0 - (-6.0 * slice_d.abs()).exp();
    col *= 0.8 + 0.2 * (150.0 * slice_d).cos();
    col = col.lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.01, slice_d.abs()));
    if constants.mouse_button_pressed & 1 != 0 {
        // TODO: probably can make this more efficient
        let d = ray_march_distance_texture(ro, rd, cursor_3d_pos, distance);
        if d < MAX_DIST {
            col = vec3(1.0, 1.0, 0.0);
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
