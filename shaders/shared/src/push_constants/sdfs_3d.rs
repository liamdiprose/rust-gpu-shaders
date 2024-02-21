use super::{vec3, Size, Vec2, Vec3};
use bytemuck::{Pod, Zeroable};

#[cfg_attr(not(target_arch = "spirv"), derive(strum::EnumIter, strum::Display))]
#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Shape {
    Sphere,
    Cuboid,
    CuboidFrame,
    Capsule,
    Torus,
}

impl Shape {
    pub fn from_u32(x: u32) -> Self {
        if x >= core::mem::variant_count::<Shape>() as u32 {
            Shape::Sphere
        } else {
            unsafe { core::mem::transmute(x) }
        }
    }

    pub fn spec(self) -> ShapeSpec {
        use Shape::*;
        match self {
            Sphere => ShapeSpec {
                num_dims: 1,
                num_points: 0,
                is_radial: true,
            },
            Cuboid => ShapeSpec {
                num_dims: 3,
                num_points: 0,
                is_radial: false,
            },
            CuboidFrame => ShapeSpec {
                num_dims: 3,
                num_points: 0,
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
        }
    }

    pub fn params(&self) -> Params {
        let is_radial = self.spec().is_radial;
        Params {
            dim: if is_radial {
                vec3(0.2, 0.1, 0.4)
            } else {
                vec3(0.5, 0.2, 0.4)
            },
            inner_dim: vec3(0.01, 0.01, 0.01),
            ps: [
                vec3(0.0, 0.0, 0.0),
                vec3(0.2, 0.2, 0.1),
                vec3(0.4, 0.35, 0.4),
            ],
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
    pub dim: Vec3,
    pub inner_dim: Vec3,
    pub ps: [Vec3; 3],
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub size: Size,
    pub time: f32,

    pub cursor: Vec3,
    pub slice_z: f32,
    pub translate: Vec2,

    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,

    // pub rotation: f32,
    pub shape: u32,
    pub params: Params,
}

pub fn sdf_shape(p: spirv_std::glam::Vec3, shape: Shape, params: Params) -> f32 {
    use crate::sdf_3d as sdf;
    use spirv_std::glam::{self, Vec3Swizzles};
    use Shape::*;
    let dim: glam::Vec3 = params.dim.into();
    let inner_dim: glam::Vec3 = params.inner_dim.into();
    let radius = dim.x;
    let p0 = params.ps[0].into();
    let p1 = params.ps[1].into();
    match shape {
        Sphere => sdf::sphere(p, radius),
        Cuboid => sdf::cuboid(p, dim),
        CuboidFrame => sdf::cuboid_frame(p, dim, inner_dim),
        Capsule => sdf::capsule(p, p0, p1, radius),
        Torus => sdf::torus(p, dim.xy()),
    }
}

pub fn sdf_slice(p: spirv_std::glam::Vec3, slice_z: f32) -> f32 {
    crate::sdf_3d::plane(
        p - slice_z * spirv_std::glam::Vec3::Z,
        spirv_std::glam::Vec3::Z,
    )
}
