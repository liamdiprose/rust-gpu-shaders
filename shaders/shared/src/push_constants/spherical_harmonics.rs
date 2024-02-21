use crate::push_constants::{Quat, Size, Vec2};
use bytemuck::{Pod, Zeroable};

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Variant {
    Real,
    Complex,
}

impl Variant {
    pub fn from_u32(x: u32) -> Self {
        if x >= core::mem::variant_count::<Variant>() as u32 {
            Variant::Real
        } else {
            unsafe { core::mem::transmute(x) }
        }
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub time: f32,
    pub cursor: Vec2,
    pub zoom: f32,
    pub l: u32,
    pub m: i32,
    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,
    pub quat: Quat,
    pub variant: u32,
}
