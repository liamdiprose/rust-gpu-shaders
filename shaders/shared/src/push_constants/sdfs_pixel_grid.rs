use super::{Size, Vec2};
use bytemuck::{Pod, Zeroable};
use core::ops::{Index, IndexMut};

pub const NUM_Y: usize = 8;
pub const NUM_X: usize = 16;
pub type Grid = [[GridChunk; NUM_Y / 4]; NUM_X];

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct GridChunk {
    pub x0: f32,
    pub x1: f32,
    pub x2: f32,
    pub x3: f32,
}

impl GridChunk {
    pub fn index(&self, index: usize) -> f32 {
        match index {
            0 => self.x0,
            1 => self.x1,
            2 => self.x2,
            _ => self.x3,
        }
    }
}

impl Index<usize> for GridChunk {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x0,
            1 => &self.x1,
            2 => &self.x2,
            3 => &self.x3,
            _ => panic!(),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl IndexMut<usize> for GridChunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x0,
            1 => &mut self.x1,
            2 => &mut self.x2,
            3 => &mut self.x3,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub time: f32,
    pub cursor: Vec2,
    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,
    pub zoom: f32,
}
