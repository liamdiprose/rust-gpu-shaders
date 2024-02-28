use spirv_std::glam::{vec2, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn repeat_x(p: Vec2, factor: f32) -> Vec2 {
    vec2(p.x - factor * (p.x / factor).round(), p.y)
}

pub fn repeat_y(p: Vec2, factor: f32) -> Vec2 {
    vec2(p.x, p.y - factor * (p.y / factor).round())
}

pub fn repeat_xy(p: Vec2, factor: Vec2) -> Vec2 {
    p - factor * (p / factor).round()
}

pub fn union(a: f32, b: f32) -> f32 {
    a.min(b)
}

pub fn intersection(a: f32, b: f32) -> f32 {
    a.max(b)
}

pub fn difference(a: f32, b: f32) -> f32 {
    intersection(a, -b)
}

pub fn symmetric_difference(a: f32, b: f32) -> f32 {
    difference(union(a, b), intersection(a, b))
}

pub fn pad(d: f32, r: f32) -> f32 {
    d - r
}

pub fn onion(d: f32, r: f32) -> f32 {
    d.abs() - r
}

pub fn smooth_union(a: f32, b: f32, k: f32) -> f32 {
    let h = (k - (a - b).abs()).max(0.0) / k;
    a.min(b) - h * h * k * (1.0 / 4.0)
}
