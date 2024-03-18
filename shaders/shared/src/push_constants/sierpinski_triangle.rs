use super::{Size, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub zoom: f32,
    pub dim: Vec2,
}
