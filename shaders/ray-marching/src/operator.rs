use spirv_std::glam::{vec3, Vec2, Vec3, Vec3Swizzles};
use spirv_std::num_traits::Euclid;

pub fn repeat_x(p: Vec3, factor: f32) -> Vec3 {
    let x = Euclid::rem_euclid(&p.x, &factor) - 0.5 * factor;
    vec3(x, p.y, p.z)
}

pub fn repeat_y(p: Vec3, factor: f32) -> Vec3 {
    let y = Euclid::rem_euclid(&p.y, &factor) - 0.5 * factor;
    vec3(p.x, y, p.z)
}

pub fn repeat_z(p: Vec3, factor: f32) -> Vec3 {
    let z = Euclid::rem_euclid(&p.z, &factor) - 0.5 * factor;
    vec3(p.x, p.y, z)
}

pub fn repeat_xy(p: Vec3, factor: Vec2) -> Vec3 {
    let tmp = p.xy().rem_euclid(factor) - 0.5 * factor;
    vec3(tmp.x, tmp.y, p.z)
}

pub fn repeat_xz(p: Vec3, factor: Vec2) -> Vec3 {
    let tmp = p.xz().rem_euclid(factor) - 0.5 * factor;
    vec3(tmp.x, p.y, tmp.y)
}

pub fn repeat_yz(p: Vec3, factor: Vec2) -> Vec3 {
    let tmp = p.yz().rem_euclid(factor) - 0.5 * factor;
    vec3(p.x, tmp.y, tmp.y)
}

pub fn repeat_xyz(p: Vec3, factor: Vec3) -> Vec3 {
    p.rem_euclid(factor) - 0.5 * factor
}
