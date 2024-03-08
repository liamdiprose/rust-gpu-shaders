use super::{Bool, Size, Vec2};
use crate::functional::tuple::*;
use bytemuck::{Pod, Zeroable};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

// Only need padding of 2 but Gridchunk is size 4 so this simplifies things
pub const SMOOTH_PADDING: usize = 4;
pub const BASE: usize = 32 - SMOOTH_PADDING;
pub const NUM_Y: usize = BASE + SMOOTH_PADDING;
pub const NUM_X: usize = (BASE + SMOOTH_PADDING) * 3 + SMOOTH_PADDING;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Grid {
    pub grid: [[GridChunk; NUM_Y / 4]; NUM_X],
}

#[cfg(not(target_arch = "spirv"))]
impl Grid {
    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        self.grid[i][j / 4][j % 4] = value;
    }

    pub fn new() -> Self {
        Self {
            grid: [[GridChunk::zeroed(); NUM_Y / 4]; NUM_X],
        }
    }
}

impl Grid {
    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.grid[i][j / 4].index(j % 4)
    }

    fn indices_from_vec2(&self, p: spirv_std::glam::Vec2) -> spirv_std::glam::Vec2 {
        let i = (p.x + 0.5 * NUM_X as f32 / BASE as f32) * BASE as f32;
        let j = (p.y + 0.5) * BASE as f32 + (0.5 * SMOOTH_PADDING as f32);
        spirv_std::glam::vec2(i, j)
    }

    pub fn from_vec2(&self, p: spirv_std::glam::Vec2) -> f32 {
        let ij = self.indices_from_vec2(p);
        self.get(ij.x as usize, ij.y as usize)
    }

    pub fn from_vec2_smooth(&self, p: spirv_std::glam::Vec2) -> f32 {
        let ij = self.indices_from_vec2(p);
        let indices_and_scalars = |x: f32| {
            (
                ((x - 0.5) as usize, (0.5 - x.fract()).max(0.0)),
                (x as usize, 0.5 + ((x - 0.5).fract() - 0.5).abs()),
                ((x + 0.5) as usize, (x.fract() - 0.5).max(0.0)),
            )
        };
        indices_and_scalars(ij.x)
            .map(|a| indices_and_scalars(ij.y).map(|b| (a, b)))
            .map(|a| a.map(|((i, s1), (j, s2))| s1 * s2 * self.get(i, j)).sum())
            .sum()
    }
}

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
            3 => self.x3,
            _ => panic!(),
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl core::ops::Index<usize> for GridChunk {
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
impl core::ops::IndexMut<usize> for GridChunk {
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
    pub smooth: Bool,
}
