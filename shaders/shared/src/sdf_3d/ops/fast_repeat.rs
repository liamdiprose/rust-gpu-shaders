use spirv_std::glam::{vec3, UVec3, Vec2, Vec3, Vec3Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
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

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_rectangular(mut p: Vec3, s: f32, size: UVec3) -> Vec3 {
    p = (p / s).abs() - (size.as_vec3() * 0.5 - 0.5);
    p = if p.x < p.y { p.yxz() } else { p };
    p = if p.z < p.y { p.yzx() } else { p };
    p.y -= p.y.round().min(0.0);
    p * s
}
