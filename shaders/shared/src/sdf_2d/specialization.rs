use spirv_std::glam::Vec2;
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn capsule_x(mut p: Vec2, width: f32, r: f32) -> f32 {
    p.x = p.x.abs();
    p.x -= p.x.min(width);
    p.length() - r
}
