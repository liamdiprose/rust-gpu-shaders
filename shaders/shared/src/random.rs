use crate::functional::tuple::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn random31(p: Vec3) -> f32 {
    (p.dot(vec3(12.9898, 78.233, 214.2634)).sin() * 43758.5453).fract()
}

pub fn random21(p: Vec2) -> f32 {
    (p.dot(vec2(12.9898, 78.233)).sin() * 43758.5453).fract()
}

pub fn random11(p: f32) -> f32 {
    (p.sin() * 43758.5453).fract()
}

pub fn random33(p: Vec3) -> Vec3 {
    let v: Vec3 = (
        vec3(127.1, 311.7, 564.536),
        vec3(269.5, 183.3, 93.481),
        vec3(12.9898, 78.233, 214.2634),
    )
        .map(|p2| p.dot(p2).sin())
        .into();
    (v * 43758.5453).fract()
}

pub fn random22(p: Vec2) -> Vec2 {
    let v: Vec2 = (vec2(127.1, 311.7), vec2(269.5, 183.3))
        .map(|p2| p.dot(p2).sin())
        .into();
    (v * 43758.5453).fract()
}
