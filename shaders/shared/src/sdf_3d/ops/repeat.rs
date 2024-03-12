use crate::functional::tuple::*;
use core::f32::consts::TAU;
use spirv_std::glam::{Mat3, Vec3};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

/// Repeats `n` times around a circle of radius `r` facing the X axis
pub fn repeat_angular_x<F>(p: Vec3, r: f32, n: u32, sdf: F) -> f32
where
    F: Fn(Vec3) -> f32,
{
    let sp = TAU / n as f32;
    let an = p.z.atan2(p.y);
    let id = (an / sp).floor();
    (-sp * id, -sp * (id + 1.0))
        .map(Mat3::from_rotation_x)
        .map(|x| x * p - Vec3::Y * r)
        .map(sdf)
        .min_element()
}

/// Repeats `n` times around a circle of radius `r` facing the Y axis
pub fn repeat_angular_y<F>(p: Vec3, r: f32, n: u32, sdf: F) -> f32
where
    F: Fn(Vec3) -> f32,
{
    let sp = TAU / n as f32;
    let an = p.x.atan2(p.z);
    let id = (an / sp).floor();
    (-sp * id, -sp * (id + 1.0))
        .map(Mat3::from_rotation_y)
        .map(|x| x * p - Vec3::Z * r)
        .map(sdf)
        .min_element()
}

/// Repeats `n` times around a circle of radius `r` facing the Z axis
pub fn repeat_angular_z<F>(p: Vec3, r: f32, n: u32, sdf: F) -> f32
where
    F: Fn(Vec3) -> f32,
{
    let sp = TAU / n as f32;
    let an = p.y.atan2(p.x);
    let id = (an / sp).floor();
    (-sp * id, -sp * (id + 1.0))
        .map(Mat3::from_rotation_z)
        .map(|x| x * p - Vec3::X * r)
        .map(sdf)
        .min_element()
}
