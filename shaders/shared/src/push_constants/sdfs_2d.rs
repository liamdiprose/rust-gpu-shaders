use super::{Size, Vec2};
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
    Capsule,
    Torus,
    Line,
    Plane,
    LineSegement,
    PlaneSegment,
    Ray,
    PlaneRay,
}

impl Shape {
    pub fn from_u32(x: u32) -> Self {
        if x >= core::mem::variant_count::<Shape>() as u32 {
            Shape::Circle
        } else {
            unsafe { core::mem::transmute(x) }
        }
    }

    pub fn spec(self) -> ShapeSpec {
        use Shape::*;
        match self {
            Circle => ShapeSpec {
                num_dims: 1,
                num_points: 0,
                is_radial: true,
            },
            Rectangle => ShapeSpec {
                num_dims: 2,
                num_points: 0,
                is_radial: false,
            },
            EquilateralTriangle => ShapeSpec {
                num_dims: 1,
                num_points: 0,
                is_radial: true,
            },
            IsoscelesTriangle => ShapeSpec {
                num_dims: 2,
                num_points: 0,
                is_radial: false,
            },
            Triangle => ShapeSpec {
                num_dims: 0,
                num_points: 3,
                is_radial: false,
            },
            Capsule => ShapeSpec {
                num_dims: 1,
                num_points: 2,
                is_radial: true,
            },
            Torus => ShapeSpec {
                num_dims: 2,
                num_points: 0,
                is_radial: true,
            },
            Plane => ShapeSpec {
                num_dims: 0,
                num_points: 0,
                is_radial: false,
            },
            Line => ShapeSpec {
                num_dims: 0,
                num_points: 0,
                is_radial: false,
            },
            PlaneSegment => ShapeSpec {
                num_dims: 0,
                num_points: 2,
                is_radial: false,
            },
            LineSegement => ShapeSpec {
                num_dims: 0,
                num_points: 2,
                is_radial: false,
            },
            Ray => ShapeSpec {
                num_dims: 0,
                num_points: 1,
                is_radial: false,
            },
            PlaneRay => ShapeSpec {
                num_dims: 0,
                num_points: 1,
                is_radial: false,
            },
        }
    }

    pub fn params(&self) -> Params {
        let is_radial = self.spec().is_radial;
        Params {
            dim: if is_radial {
                Vec2 { x: 0.2, y: 0.05 }
            } else {
                Vec2 { x: 0.5, y: 0.2 }
            },
            ps: [
                Vec2 { x: 0.0, y: 0.0 },
                Vec2 { x: 0.2, y: 0.2 },
                Vec2 { x: -0.4, y: 0.35 },
            ],
            rot: 0.0,
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
    pub dim: Vec2,
    pub ps: [Vec2; 3],
    pub rot: f32,
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
