use super::{Size, Vec3};
use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub time: f32,
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}
