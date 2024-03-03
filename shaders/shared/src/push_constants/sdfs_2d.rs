use super::{Size, Vec2};
use crate::fast_optional::Optional_f32;
use bytemuck::{Pod, Zeroable};

#[cfg_attr(not(target_arch = "spirv"), derive(strum::EnumIter, strum::Display))]
#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Shape {
    Disk,
    Rectangle,
    EquilateralTriangle,
    IsoscelesTriangle,
    Triangle,
    Capsule,
    Torus,
    Line,
    Plane,
    LineSegment,
    PlaneSegment,
    Ray,
    PlaneRay,
    Hexagon,
    Pentagon,
}

impl Shape {
    pub fn from_u32(x: u32) -> Self {
        if x >= core::mem::variant_count::<Shape>() as u32 {
            Shape::Disk
        } else {
            unsafe { core::mem::transmute(x) }
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Shape {
    pub fn labels(self) -> &'static [&'static str] {
        use Shape::*;
        const R: &'static str = "Radius";
        const W: &'static str = "Width";
        const H: &'static str = "Height";
        match self {
            Disk | Capsule | Hexagon | Pentagon | EquilateralTriangle => &[R],
            Rectangle | IsoscelesTriangle => &[W, H],
            Torus => &["Major Radius", "Minor Radius"],
            Triangle | Plane | Line | Ray | PlaneRay | LineSegment | PlaneSegment => &[],
        }
    }

    pub fn default_params(&self) -> Params {
        let default_ps = self.default_points();
        let mut ps = [[0.0, 0.0]; 3];
        for i in 0..default_ps.len() {
            ps[i] = default_ps[i];
        }

        let default_dims = self.default_dims();
        let mut dims = [0.0; 2];
        for i in 0..default_dims.len() {
            dims[i] = default_dims[i];
        }

        Params {
            dims,
            ps,
            rot: 0.0,
            pad: Optional_f32::NONE,
            onion: Optional_f32::NONE,
            repeat: [Optional_f32::NONE; 2],
        }
    }

    pub fn dim_range(&self) -> &[core::ops::RangeInclusive<f32>] {
        use Shape::*;
        match self {
            Disk | Capsule | EquilateralTriangle | Hexagon | Pentagon => &[0.0..=0.5],
            Rectangle => &[0.0..=1.0, 0.0..=1.0],
            IsoscelesTriangle => &[0.0..=1.0, -0.5..=0.5],
            Torus => &[0.0..=0.5, 0.0..=0.2],
            Triangle | Plane | Line | Ray | PlaneRay | LineSegment | PlaneSegment => &[],
        }
    }

    pub fn default_dims(&self) -> &[f32] {
        use Shape::*;
        match self {
            Disk | Capsule | EquilateralTriangle | Hexagon | Pentagon => &[0.2],
            Rectangle | IsoscelesTriangle => &[0.4, 0.3],
            Torus => &[0.2, 0.1],
            Triangle | Plane | Line | Ray | PlaneRay | LineSegment | PlaneSegment => &[],
        }
    }

    pub fn default_points(&self) -> &[[f32; 2]] {
        use Shape::*;
        match self {
            Triangle => &[[-0.1, -0.2], [0.3, 0.2], [0.2, -0.3]],
            Capsule | LineSegment | PlaneSegment => &[[-0.1, -0.1], [0.2, 0.1]],
            Ray | PlaneRay => &[[0.0, 0.0]],
            _ => &[],
        }
    }
}

pub struct ShapeSpec {
    pub num_dims: u32,
    pub num_points: u32,
    pub is_radial: bool,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Params {
    pub dims: [f32; 2],
    pub ps: [[f32; 2]; 3],
    pub rot: f32,
    pub onion: Optional_f32,
    pub pad: Optional_f32,
    pub repeat: [Optional_f32; 2],
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub time: f32,
    pub cursor: Vec2,
    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,
    pub shape: u32,
    pub params: Params,
}
