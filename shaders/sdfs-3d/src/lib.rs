#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sdfs_3d::ShaderConstants;
use shared::push_constants::sdfs_3d::{sdf_shape, Shape};
use shared::*;
use shared::{push_constants::sdfs_3d::Params, sdf_3d as sdf, sdf_3d::ops};
use spirv_std::glam::{vec2, vec3, Mat3, Vec2, Vec2Swizzles, Vec3, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.0001;

fn slicer_sdf(p: Vec3, slice_z: f32) -> f32 {
    sdf::plane(p - vec3(0.0, 0.0, slice_z), vec3(0.0, 0.0, 1.0))
}

fn sdf(
    p: Vec3,
    shape: Shape,
    slice_z: f32,
    mouse_button_pressed: bool,
    cursor_3d_pos: Vec3,
    distance: f32,
    params: Params,
) -> f32 {
    let mut d = sdf_shape(p, shape, params);
    d = ops::union(d, sdf::plane(p - vec3(0.0, -0.5, 0.0), vec3(0.0, 1.0, 0.0)));
    d = ops::difference(d, slicer_sdf(p, slice_z));
    d = ops::union(d, sdf::plane(p - vec3(0.0, -0.7, 0.0), vec3(0.0, 1.0, 0.0)));
    if mouse_button_pressed {
        let rm = Mat3::from_rotation_x(PI / 2.0);
        d = ops::union(
            d,
            sdf::torus(rm.mul_vec3(p - cursor_3d_pos), vec2(distance, 0.01)),
        );
        d = ops::union(d, sdf::sphere(p - cursor_3d_pos, 0.01));
    }
    d
}

fn ray_march(
    ro: Vec3,
    rd: Vec3,
    shape: Shape,
    slice_z: f32,
    mouse_button_pressed: bool,
    cursor_3d_pos: Vec3,
    distance: f32,
    params: Params,
) -> (f32, f32) {
    let mut d0 = 0.0;
    let mut cd = f32::INFINITY;

    // p = vec3(_, _, slice_z)
    // p.z * x = slice_z
    // let p = ro + rd * slice_z / p.z;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = sdf(
            p,
            shape,
            slice_z,
            mouse_button_pressed,
            cursor_3d_pos,
            distance,
            params,
        );
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

fn get_normal(
    p: Vec3,
    shape: Shape,
    slice_z: f32,
    mouse_button_pressed: bool,
    cursor_3d_pos: Vec3,
    distance: f32,
    params: Params,
) -> Vec3 {
    let d = sdf(
        p,
        shape,
        slice_z,
        mouse_button_pressed,
        cursor_3d_pos,
        distance,
        params,
    );
    let e = vec2(0.01, 0.0);
    let n = d - vec3(
        sdf(
            p - e.xyy(),
            shape,
            slice_z,
            mouse_button_pressed,
            cursor_3d_pos,
            distance,
            params,
        ),
        sdf(
            p - e.yxy(),
            shape,
            slice_z,
            mouse_button_pressed,
            cursor_3d_pos,
            distance,
            params,
        ),
        sdf(
            p - e.yyx(),
            shape,
            slice_z,
            mouse_button_pressed,
            cursor_3d_pos,
            distance,
            params,
        ),
    );
    n.normalize()
}

fn get_light(
    p: Vec3,
    shape: Shape,
    slice_z: f32,
    mouse_button_pressed: bool,
    cursor_3d_pos: Vec3,
    distance: f32,
    params: Params,
) -> f32 {
    let light_pos = vec3(1.0, 5.0, -1.0);
    let light_vector = (light_pos - p).normalize();
    let normal_vector = get_normal(
        p,
        shape,
        slice_z,
        mouse_button_pressed,
        cursor_3d_pos,
        distance,
        params,
    );
    let mut dif = light_vector.dot(normal_vector).clamp(0.0, 1.0);
    let (d, _) = ray_march(
        p + normal_vector * SURF_DIST * 2.0,
        light_vector,
        shape,
        slice_z,
        mouse_button_pressed,
        cursor_3d_pos,
        distance,
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
    let ro = rm.mul_vec3(vec3(0.0, 0.0, -1.0));
    let rd = rm.mul_vec3(vec3(uv.x, uv.y, 1.0)).normalize();

    let slice_z = constants.slice_z;
    let distance = constants.distance;

    let shape = Shape::from_u32(constants.shape);
    let mouse_button_pressed = constants.mouse_button_pressed & 1 != 0;
    let (d, cd) = ray_march(
        ro,
        rd,
        shape,
        slice_z,
        mouse_button_pressed,
        cursor_3d_pos,
        distance,
        constants.params,
    );
    let dif = get_light(
        ro + rd * d,
        shape,
        slice_z,
        mouse_button_pressed,
        cursor_3d_pos,
        distance,
        constants.params,
    );
    let c = cd.abs().atan() / (PI / 2.0);
    let mut col = vec3(dif, dif, dif)
        // vec3(0.0,0.0,0.0)
        .lerp(vec3(1.0 - c, 0.5 - c, 0.2 - c), 0.2);
    // let col = col.lerp(vec3((cd*10.0).sin(), (cd*10.0).sin(), 0.0),0.1);
    // let mut col = if d < 100.0 {
    //     vec3(0.65, 0.85, 1.0)
    // } else {
    //     vec3(0.9, 0.6, 0.3)
    // };
    // col *= 1.0 - (-6.0 * d.abs()).exp();
    // col *= 0.8 + 0.2 * (150.0 * d).cos();
    // col = col.lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.01, d.abs()));
    if d > MAX_DIST {
        col = vec3(0.2, 0.3, 0.7);
    }
    // if d < 100.0 {
    //     vec3(0.65, 0.85, 1.0)
    // } else {
    //     vec3(0.9, 0.6, 0.3)
    // }

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
