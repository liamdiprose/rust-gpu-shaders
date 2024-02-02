use bytemuck::{Pod, Zeroable};
use spirv_std::glam;
#[cfg(not(target_arch = "spirv"))]
use winit::dpi::PhysicalSize;

pub mod hydrogen_wavefunction;
pub mod koch_snowflake;
pub mod mandelbrot;
pub mod ray_marching;
pub mod ray_marching_2d;
pub mod sdfs_2d;
pub mod sdfs_3d;
pub mod sierpinski_triangle;
pub mod spherical_harmonics;

pub fn largest_size() -> usize {
    use core::mem::size_of;
    size_of::<koch_snowflake::ShaderConstants>()
        .max(size_of::<mandelbrot::ShaderConstants>())
        .max(size_of::<ray_marching::ShaderConstants>())
        .max(size_of::<ray_marching_2d::ShaderConstants>())
        .max(size_of::<sierpinski_triangle::ShaderConstants>())
        .max(size_of::<sdfs_2d::ShaderConstants>())
        .max(size_of::<sdfs_3d::ShaderConstants>())
        .max(size_of::<hydrogen_wavefunction::ShaderConstants>())
        .max(size_of::<spherical_harmonics::ShaderConstants>())
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[cfg(not(target_arch = "spirv"))]
impl From<PhysicalSize<u32>> for Size {
    fn from(PhysicalSize { width, height }: PhysicalSize<u32>) -> Self {
        Size { width, height }
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<glam::Vec2> for Vec2 {
    fn from(glam::Vec2 { x, y }: glam::Vec2) -> Self {
        Vec2 { x, y }
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<glam::Quat> for Quat {
    fn from(q: glam::Quat) -> Self {
        Quat {
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        }
    }
}

impl Into<glam::Quat> for Quat {
    fn into(self) -> glam::Quat {
        glam::Quat::from_xyzw(self.x, self.y, self.z, self.w)
    }
}
