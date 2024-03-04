#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub mod repeat;
pub mod fast_repeat;

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
