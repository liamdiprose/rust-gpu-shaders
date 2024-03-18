use super::{Size, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub translate: Vec2,
    pub zoom: f32,
    pub exponent: f32,
    pub num_iterations: u32,
}
