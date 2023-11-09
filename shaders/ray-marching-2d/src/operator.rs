use spirv_std::glam::{vec2, Vec2};
use spirv_std::num_traits::Euclid;

pub fn repeat_x(p: Vec2, factor: f32) -> Vec2 {
    let x = Euclid::rem_euclid(&p.x, &factor) - 0.5 * factor;
    vec2(x, p.y)
}

pub fn repeat_y(p: Vec2, factor: f32) -> Vec2 {
    let y = Euclid::rem_euclid(&p.y, &factor) - 0.5 * factor;
    vec2(p.x, y)
}

pub fn repeat_xy(p: Vec2, factor: Vec2) -> Vec2 {
    p.rem_euclid(factor) - 0.5 * factor
}
