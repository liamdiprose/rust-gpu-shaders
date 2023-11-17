use spirv_std::glam::{vec3, Vec2, Vec3, Vec3Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

pub use crate::sdf_2d::ops::{difference, intersection, symmetric_difference, union};

pub fn repeat_x(p: Vec3, factor: f32) -> Vec3 {
    vec3(p.x - factor * (p.x / factor).round(), p.y, p.z)
}

pub fn repeat_y(p: Vec3, factor: f32) -> Vec3 {
    vec3(p.x, p.y - factor * (p.y / factor).round(), p.z)
}

pub fn repeat_z(p: Vec3, factor: f32) -> Vec3 {
    vec3(p.x, p.y, p.z - factor * (p.z / factor).round())
}

pub fn repeat_xy(p: Vec3, factor: Vec2) -> Vec3 {
    (p.xy() - factor * (p.xy() / factor).round()).extend(p.z)
}

pub fn repeat_xz(p: Vec3, factor: Vec2) -> Vec3 {
    let p0 = p.xz() - factor * (p.xz() / factor).round();
    vec3(p0.x, p.y, p0.y)
}

pub fn repeat_yz(p: Vec3, factor: Vec2) -> Vec3 {
    let p0 = p.yz() - factor * (p.yz() / factor).round();
    vec3(p.x, p0.y, p0.y)
}

pub fn repeat_xyz(p: Vec3, factor: Vec3) -> Vec3 {
    p - factor * (p / factor).round()
}
