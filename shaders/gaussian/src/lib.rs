#![cfg_attr(target_arch = "spirv", no_std)]

use core::f32::consts::PI;
use push_constants::spherical_harmonics::ShaderConstants;
use spirv_std::spirv;
use spirv_std::glam::{vec2, vec3, Quat, Vec2, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles, Mat2};
use shared::{sdf_3d as sdf, *};
use spirv_std::num_traits::Float;
use geometric_algebra::{epga2d, GeometricProduct};

fn gaussian(x: Vec2, covariance: Mat2) -> f32 {
    Float::exp(-0.5 * x.dot(covariance.inverse() * x))
}

fn gaussian_ga(x: epga2d::IdealPoint, covariance: epga2d::Plane) -> f32 {
    // Float::exp(-0.5 * x.group0().dot(covariance.geometric_product(x)))
    1.0
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = from_pixels(frag_coord.x, frag_coord.y, constants);
    // let covariance = Mat2::IDENTITY * 0.02;
    let s = Mat2::from_diagonal(vec2((constants.time).cos() * 0.40 * PI - 1.6, 1.0) * 0.1);
    let r = Mat2::from_angle(constants.time.sin() * PI);

    let covariance = r * s * s.transpose() * r.transpose();
    let v = gaussian(uv, covariance);

    let point = epga2d::IdealPoint::new(uv.x, uv.y);
    let covariance_ga = epga2d::Plane::new(0.0, 1.0, 1.0);

    // let v = gaussian_ga(point, covariance_ga);

    *output = (vec3(0.1, 0.1, 0.1) * v).extend(1.0)
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}

fn from_pixels(x: f32, y: f32, constants: &ShaderConstants) -> Vec2 {
    (vec2(x, -y) - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32
}
