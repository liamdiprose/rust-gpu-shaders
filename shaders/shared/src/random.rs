use spirv_std::glam::{vec2, vec3, Vec2, Vec3};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn random3(st: Vec3) -> f32 {
    (st.dot(vec3(12.9898, 78.233, 214.263489)).sin() * 43758.5453123).fract()
}

pub fn random2(st: Vec2) -> f32 {
    (st.dot(vec2(12.9898, 78.233)).sin() * 43758.5453123).fract()
}

pub fn random1(st: f32) -> f32 {
    (st.sin() * 43758.5453123).fract()
}
