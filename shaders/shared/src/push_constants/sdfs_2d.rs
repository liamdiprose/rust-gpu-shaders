use bytemuck::{Pod, Zeroable};

#[cfg_attr(not(target_arch = "spirv"), derive(strum::EnumIter, strum::Display))]
#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Shape {
    Circle,
    Rectangle,
    EquilateralTriangle,
    IsoscelesTriangle,
    Triangle,
}

impl Shape {
    pub fn from_u32(x: u32) -> Self {
        if x >= core::mem::variant_count::<Shape>() as u32 {
            Shape::Circle
        } else {
            unsafe { core::mem::transmute(x) }
        }
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Params {
    pub radius: f32,
    pub width: f32,
    pub height: f32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,

    pub cursor_x: f32,
    pub cursor_y: f32,

    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,

    pub rotation: f32,
    pub shape: u32,
    pub params: Params,
}
