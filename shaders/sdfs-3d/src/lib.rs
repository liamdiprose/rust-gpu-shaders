#![cfg_attr(target_arch = "spirv", no_std)]

use crate::functional::vec::*;
use ray_intersection::ray_intersects_sphere;
use shared::{
    push_constants::sdfs_3d::{Params, ShaderConstants, Shape},
    ray_intersection::ray_intersect_capsule,
    sdf_3d::{self as sdf, ops},
    *,
};
use spirv_std::glam::{vec3, Mat3, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

#[derive(PartialEq)]
#[repr(C)]
enum RayMarchResult {
    Divergent,
    Shape,
    DistanceTexture,
}

const COL_INSIDE: Vec3 = vec3(0.65, 0.85, 1.0);
const COL_OUTSIDE: Vec3 = vec3(0.9, 0.6, 0.3);
const YELLOW: Vec3 = vec3(1.0, 1.0, 0.0);

const MAX_STEPS: u32 = 512;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.0002;

pub fn sdf_shape(mut p: Vec3, shape: Shape, params: Params) -> f32 {
    use Shape::*;

    let dim = vec3(params.dims[0], params.dims[1], params.dims[2]);
    let dim2 = vec3(params.dims[3], params.dims[4], params.dims[5]);
    let p0 = params.ps[0].into();
    let p1 = params.ps[1].into();
    let orientation = Vec3::Y;

    if params.repeat[0].has_value() {
        p = sdf::ops::fast_repeat::repeat_x(p, params.repeat[0].value)
    }

    if params.repeat[1].has_value() {
        p = sdf::ops::fast_repeat::repeat_y(p, params.repeat[1].value)
    }

    if params.repeat[2].has_value() {
        p = sdf::ops::fast_repeat::repeat_z(p, params.repeat[2].value)
    }

    let mut d = match shape {
        Sphere => sdf::sphere(p, dim.x),
        Cuboid => sdf::cuboid(p, dim),
        CuboidFrame => sdf::cuboid_frame(p, dim, dim2),
        CuboidFrameRadial => sdf::cuboid_frame_radial(p, dim, dim2.x),
        Capsule => sdf::capsule(p, p0, p1, dim.x),
        Cylinder => sdf::cylinder(p, p0, p1, dim.x),
        Torus => sdf::torus(p, dim.xy(), orientation),
        Disk => sdf::disk(p, dim.xy(), orientation),
        Plane => sdf::plane(p, orientation),
    };

    if params.pad.has_value() {
        d = sdf::ops::pad(d, params.pad.value)
    }

    if params.onion.has_value() {
        d = sdf::ops::onion(d, params.onion.value)
    }

    d
}

pub fn sdf_slice(p: spirv_std::glam::Vec3, slice_z: f32) -> f32 {
    crate::sdf_3d::plane(
        p - slice_z * spirv_std::glam::Vec3::Z,
        spirv_std::glam::Vec3::Z,
    )
}

fn sdf_ball(p: Vec3, cursor: Vec3, cursor_d: f32) -> f32 {
    sdf::sphere(p - cursor, cursor_d)
}

fn ray_march(
    ro: Vec3,
    rd: Vec3,
    shape: Shape,
    slice_z: f32,
    params: Params,
    cursor: Vec3,
    abs_cursor_d: f32,
    mouse_pressed: bool,
) -> (f32, RayMarchResult) {
    use RayMarchResult::*;
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let slice_d = sdf_slice(p, slice_z);
        let sliced_shape_d = ops::difference(sdf_shape(p, shape, params), slice_d);
        let ds = if mouse_pressed {
            let sliced_ball_d = ops::difference(sdf_ball(p, cursor, abs_cursor_d), slice_d);
            sliced_shape_d.min(sliced_ball_d)
        } else {
            sliced_shape_d
        };
        d0 += ds;
        if ds < SURF_DIST {
            let sliced_ball_d = ops::difference(sdf_ball(p, cursor, abs_cursor_d), slice_d);
            return (
                d0,
                if mouse_pressed && ds == sliced_ball_d {
                    DistanceTexture
                } else {
                    Shape
                },
            );
        }
        if d0 > MAX_DIST {
            return (d0, Divergent);
        }
    }

    let p = ro + rd * 1e15;
    let sliced_shape_d = ops::difference(sdf_shape(p, shape, params), sdf_slice(p, slice_z));
    (
        d0,
        if sliced_shape_d < 0.0 {
            Shape
        } else {
            Divergent
        },
    )
}

fn ray_march_distance_texture(
    ro: Vec3,
    rd: Vec3,
    slice_z: f32,
    cursor: Vec3,
    abs_cursor_d: f32,
) -> f32 {
    let mut d0 = 0.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let ds = ops::difference(sdf::sphere(p - cursor, abs_cursor_d), sdf_slice(p, slice_z));
        d0 += ds;
        if ds < SURF_DIST || d0 > MAX_DIST {
            break;
        }
    }

    d0
}

fn get_d_to_shape_at_slice(ro: Vec3, rd: Vec3, shape: Shape, slice_z: f32, params: Params) -> f32 {
    let x = (slice_z - ro.z) / rd.z;
    if x < 0.0 {
        MAX_DIST * 8.0 // Makes a nice color
    } else {
        sdf_shape(ro + rd * x, shape, params)
    }
}

fn get_d_to_cursor_at_slice(ro: Vec3, rd: Vec3, slice_z: f32, cursor: Vec3) -> f32 {
    let x = (slice_z - ro.z) / rd.z;
    if x < 0.0 {
        MAX_DIST
    } else {
        (ro + rd * x).distance(cursor)
    }
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let translate: Vec2 = constants.translate.into();
    let cursor: Vec3 = constants.cursor.into();

    let uv = from_pixels(frag_coord.xy(), constants.size);

    let rm = Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
    let ro = rm.mul_vec3(-Vec3::Z);
    let rd = rm.mul_vec3(uv.extend(1.0)).normalize();

    let slice_z = constants.slice_z;
    let mouse_pressed = constants.mouse_button_pressed & 1 != 0;
    let shape = Shape::from_u32(constants.shape);
    let slice_d = get_d_to_shape_at_slice(ro, rd, shape, slice_z, constants.params);
    let cursor_d = sdf_shape(cursor, shape, constants.params);
    let (d0, ray_march_result) = ray_march(
        ro,
        rd,
        shape,
        slice_z,
        constants.params,
        cursor,
        cursor_d.abs(),
        mouse_pressed,
    );
    let d1 = if mouse_pressed {
        ray_march_distance_texture(ro, rd, slice_z, cursor, cursor_d.abs())
    } else {
        MAX_DIST
    };
    let shape_col =
        Vec3::splat((ro + rd * d0).map(|x| (x * 50.0).sin().abs()).sum() * 0.3) / d0.max(1.0);

    let col = if ray_march_result == RayMarchResult::Divergent {
        COL_OUTSIDE
    } else if d1 >= MAX_DIST {
        shape_col
    } else {
        let d_to_cursor = get_d_to_cursor_at_slice(ro, rd, slice_z, cursor);
        let sphere_surface_col = YELLOW
            * ((ro.z + rd.z * d1 - cursor.z) * 30.0 / cursor_d.abs().sqrt())
                .sin()
                .abs();
        let sphere_intersection_col = YELLOW * (d_to_cursor * PI * 4.0 / cursor_d).sin().abs();
        let sphere_surface_only = !(mouse_pressed && d_to_cursor < cursor_d.abs());
        sphere_surface_col
            .lerp(
                sphere_intersection_col,
                if sphere_surface_only {
                    0.0
                } else if ro.z < 0.0 {
                    1.0
                } else {
                    0.5
                },
            )
            .lerp(
                shape_col,
                if ray_march_result == RayMarchResult::DistanceTexture
                    || (slice_d < 0.0 && ro.z < 0.0)
                {
                    0.0
                } else {
                    0.5
                },
            )
    };

    let mut col = if (ray_march_result == RayMarchResult::DistanceTexture)
        || (ray_march_result == RayMarchResult::Shape && ro.z > slice_z && slice_d > 0.0)
    {
        col
    } else {
        let base = if slice_d < 0.0 && d1 >= MAX_DIST {
            COL_INSIDE
        } else {
            col
        };
        let s = if slice_d < 0.0 && ro.z > slice_z {
            0.8
        } else {
            1.0
        };
        let phase = if slice_d.abs() < 1.0 {
            0.0
        } else if slice_d > 0.0 {
            PI / 2.0
        } else {
            PI
        };
        let angle = if slice_d.abs() < 1.0 {
            slice_d
        } else {
            1.0 / slice_d
        };
        col.lerp(
            (base * (1.0 - (-6.0 * slice_d.abs()).exp()))
                * (0.8 + 0.2 * (150.0 * angle + phase).cos()),
            s,
        )
        .lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.005, slice_d.abs()))
    };

    let d2 = if mouse_pressed {
        let der = {
            let e = 0.01;
            vec3(
                sdf_shape(cursor - e * Vec3::X, shape, constants.params)
                    - sdf_shape(cursor + e * Vec3::X, shape, constants.params),
                sdf_shape(cursor - e * Vec3::Y, shape, constants.params)
                    - sdf_shape(cursor + e * Vec3::Y, shape, constants.params),
                sdf_shape(cursor - e * Vec3::Z, shape, constants.params)
                    - sdf_shape(cursor + e * Vec3::Z, shape, constants.params),
            ) / (2.0 * e)
        };
        let r = (cursor_d.abs() / 8.0).min(0.01);
        let pa = cursor;
        let pb = cursor + der.normalize_or_zero() * cursor_d;
        ray_intersect_capsule(ro, rd, pa, pb, r)
    } else {
        0.0
    };
    if d2 > 0.0 {
        let p2 = ro + rd * d2;
        let x = (p2.distance(cursor) * PI * 8.0 / cursor_d).sin().abs();
        let base = if ray_march_result == RayMarchResult::Shape {
            col + YELLOW * 0.1
        } else {
            YELLOW
        };
        col = base * x;
    }

    for i in 0..2 {
        let p2: Vec3 = constants.params.ps[i].into();
        if ray_intersects_sphere(ro, rd, p2, 0.01) {
            col = col.lerp(Vec3::ONE, 0.2);
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
