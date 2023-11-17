use spirv_std::glam::{vec3, Vec2, Vec3, Vec3Swizzles};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

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
    (p.xz() - factor * (p.xz() / factor).round()).extend(p.y).xzy()
}

pub fn repeat_yz(p: Vec3, factor: Vec2) -> Vec3 {
    (p.yz() - factor * (p.yz() / factor).round()).extend(p.x).zxy()
}

pub fn repeat_xyz(p: Vec3, factor: Vec3) -> Vec3 {
    p - factor * (p / factor).round()
}
