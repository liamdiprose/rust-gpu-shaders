use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub use_antisnowflake: u32,
}
