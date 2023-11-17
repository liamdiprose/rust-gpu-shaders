use spirv_std::glam::{vec2, Vec2};
#[allow(unused_imports)]
use spirv_std::num_traits::Float;

pub fn repeat_x(p: Vec2, factor: f32) -> Vec2 {
    vec2(p.x - factor * (p.x / factor).round(), p.y)
}

pub fn repeat_y(p: Vec2, factor: f32) -> Vec2 {
    vec2(p.x, p.y - factor * (p.y / factor).round())
}

pub fn repeat_xy(mut p: Vec2, factor: Vec2) -> Vec2 {
    p - factor * vec2(p.x / factor.x, p.y / factor.y).round()
}
