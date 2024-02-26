use crate::push_constants::{Quat, Size};
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
    pub zoom: f32,
    pub l: u32,
    pub m: i32,
    pub rot: Quat,
    pub variant: u32,
}
