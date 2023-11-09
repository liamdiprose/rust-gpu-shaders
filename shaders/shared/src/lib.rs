#![cfg_attr(target_arch = "spirv", no_std, feature(lang_items))]

pub mod complex;

use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,

    pub cursor_x: f32,
    pub cursor_y: f32,
    pub drag_start_x: f32,
    pub drag_start_y: f32,
    pub drag_end_x: f32,
    pub drag_end_y: f32,
    pub zoom: f32,
    pub translate_x: f32,
    pub translate_y: f32,

    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,

    /// The last time each mouse button (Left, Middle or Right) was pressed,
    /// or `f32::NEG_INFINITY` for buttons which haven't been pressed yet.
    ///
    /// If this is the first frame after the press of some button, that button's
    /// entry in `mouse_button_press_time` will exactly equal `time`.
    pub mouse_button_press_time: [f32; 3],
}

pub fn saturate(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = saturate((x - edge0) / (edge1 - edge0));
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}
