use spirv_std::glam::{vec2, Vec2};

pub fn repeat_x(p: Vec2, factor: f32) -> Vec2 {
    p - factor * vec2(p.x / factor, p.y).round()
}

pub fn repeat_y(p: Vec2, factor: f32) -> Vec2 {
    p - factor * vec2(p.x, p.y / factor).round()
}

pub fn repeat_xy(p: Vec2, factor: Vec2) -> Vec2 {
    p - factor * vec2(p.x / factor.x, p.y / factor.y).round()
}
