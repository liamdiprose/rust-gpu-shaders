use super::{Bool, Size, Vec2};
use bytemuck::{Pod, Zeroable};

pub const MAX_NUM_POINTS: usize = 5;

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub time: f32,
    pub cursor: Vec2,
    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,
    pub points: [[f32; 2]; MAX_NUM_POINTS],
    pub smooth: Bool,
    pub derivative_at_cursor: Vec2,
}
