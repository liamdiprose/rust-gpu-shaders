use spirv_std::glam::{vec2, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_x(p: Vec2, factor: f32) -> Vec2 {
    vec2(p.x - factor * (p.x / factor).round(), p.y)
}

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_y(p: Vec2, factor: f32) -> Vec2 {
    vec2(p.x, p.y - factor * (p.y / factor).round())
}

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_xy(p: Vec2, factor: Vec2) -> Vec2 {
    p - factor * (p / factor).round()
}
