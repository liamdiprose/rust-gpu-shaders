use super::{Size, Vec2, Vec3};
use crate::fast_optional::Optional_f32;
use bytemuck::{Pod, Zeroable};

#[cfg_attr(not(target_arch = "spirv"), derive(strum::EnumIter, strum::Display))]
#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Shape {
    Sphere,
    Cuboid,
    CuboidFrame,
    CuboidFrameRadial,
    Capsule,
    Cylinder,
    Torus,
    Disk,
    Plane,
}

impl Shape {
    pub fn from_u32(x: u32) -> Self {
        if x >= core::mem::variant_count::<Shape>() as u32 {
            Shape::Sphere
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
        const L: &'static str = "length";
        match self {
            Sphere | Capsule | Cylinder => &[R],
            Cuboid => &[W, H, L],
            CuboidFrame => &[W, H, L, "Inner Width", "Inner Height", "Inner Length"],
            CuboidFrameRadial => &[W, H, L, R],
            Torus | Disk => &["Major Radius", "Minor Radius"],
            Plane => &[],
        }
    }

    pub fn default_params(&self) -> Params {
        let default_ps = self.default_points();
        let mut ps = [[0.0, 0.0, 0.0]; 3];
        for i in 0..default_ps.len() {
            ps[i] = default_ps[i];
        }

        let default_dims = self.default_dims();
        let mut dims = [0.0; 6];
        for i in 0..default_dims.len() {
            dims[i] = default_dims[i];
        }

        Params {
            dims,
            ps,
            onion: Optional_f32::NONE,
            pad: Optional_f32::NONE,
            repeat: [Optional_f32::NONE; 3],
        }
    }

    pub fn dim_range(&self) -> &[core::ops::RangeInclusive<f32>] {
        use Shape::*;
        match self {
            Sphere => &[0.0..=0.5],
            Cuboid => &[0.0..=0.5, 0.0..=0.5, 0.0..=0.5],
            CuboidFrame => &[
                0.0..=0.5,
                0.0..=0.5,
                0.0..=0.5,
                0.0..=0.1,
                0.0..=0.1,
                0.0..=0.1,
            ],
            CuboidFrameRadial => &[0.0..=0.5, 0.0..=0.5, 0.0..=0.5, 0.0..=0.1],
            Capsule => &[0.0..=0.5],
            Cylinder => &[0.0..=0.5],
            Torus => &[0.0..=0.5, 0.0..=0.5],
            Disk => &[0.0..=0.5, 0.0..=0.5],
            Plane => &[],
        }
    }

    pub fn default_dims(&self) -> &[f32] {
        use Shape::*;
        match self {
            Sphere => &[0.2],
            Cuboid => &[0.2, 0.15, 0.2],
            CuboidFrame => &[0.2, 0.15, 0.2, 0.04, 0.04, 0.04],
            CuboidFrameRadial => &[0.2, 0.15, 0.2, 0.02],
            Capsule | Cylinder => &[0.2],
            Torus => &[0.2, 0.1],
            Disk => &[0.2, 0.02],
            Plane => &[],
        }
    }

    pub fn default_points(&self) -> &[[f32; 3]] {
        use Shape::*;
        match self {
            Capsule => &[[0.0, 0.0, -0.1], [0.1, 0.1, 0.2]],
            Cylinder => &[[0.0, 0.0, -0.3], [0.1, 0.1, 0.3]],
            _ => &[],
        }
    }
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Params {
    pub dims: [f32; 6],
    pub ps: [[f32; 3]; 3],
    pub onion: Optional_f32,
    pub pad: Optional_f32,
    pub repeat: [Optional_f32; 3],
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

pub fn sdf_shape(mut p: spirv_std::glam::Vec3, shape: Shape, params: Params) -> f32 {
    use crate::sdf_3d as sdf;
    use spirv_std::glam::{self, Vec3Swizzles};
    use Shape::*;

    let dim = glam::vec3(params.dims[0], params.dims[1], params.dims[2]);
    let dim2 = glam::vec3(params.dims[3], params.dims[4], params.dims[5]);
    let p0 = params.ps[0].into();
    let p1 = params.ps[1].into();
    let orientation = glam::Vec3::Y;

    if params.repeat[0].has_value() {
        p = sdf::ops::repeat_x(p, params.repeat[0].value)
    }

    if params.repeat[1].has_value() {
        p = sdf::ops::repeat_y(p, params.repeat[1].value)
    }

    if params.repeat[2].has_value() {
        p = sdf::ops::repeat_z(p, params.repeat[2].value)
    }

    let mut d = match shape {
        Sphere => sdf::sphere(p, dim.x),
        Cuboid => sdf::cuboid(p, dim),
        CuboidFrame => sdf::cuboid_frame(p, dim, dim2),
        CuboidFrameRadial => sdf::cuboid_frame_radial(p, dim, dim2.x),
        Capsule => sdf::capsule(p, p0, p1, dim.x),
        Cylinder => sdf::cylinder(p, p0, p1, dim.x),
        Torus => sdf::torus(p, dim.xy(), orientation),
        Disk => sdf::disk(p, dim.xy(), orientation),
        Plane => sdf::plane(p, orientation),
    };

    if params.pad.has_value() {
        d = sdf::ops::pad(d, params.pad.value)
    }

    if params.onion.has_value() {
        d = sdf::ops::onion(d, params.onion.value)
    }

    d
}

pub fn sdf_slice(p: spirv_std::glam::Vec3, slice_z: f32) -> f32 {
    crate::sdf_3d::plane(
        p - slice_z * spirv_std::glam::Vec3::Z,
        spirv_std::glam::Vec3::Z,
    )
}
