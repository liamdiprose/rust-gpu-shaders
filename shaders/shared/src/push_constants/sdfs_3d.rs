use crate::sdf_3d as sdf;
use bytemuck::{Pod, Zeroable};
use spirv_std::glam::{vec2, vec3, Vec3};

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
            dim1: if is_radial { 0.2 } else { 0.5 },
            dim2: if is_radial { 0.1 } else { 0.2 },
            dim3: 0.4,
            dim4: 0.01,
            dim5: 0.01,
            dim6: 0.01,
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            x1: 0.2,
            y1: 0.2,
            z1: 0.1,
            x2: -0.4,
            y2: 0.35,
            z2: 0.4,
        }
    }
}

pub fn sdf_shape(p: Vec3, shape: Shape, params: Params) -> f32 {
    use Shape::*;
    let Params {
        dim1,
        dim2,
        dim3,
        dim4,
        dim5,
        dim6,
        x0,
        y0,
        z0,
        x1,
        y1,
        z1,
        ..
    } = params;
    let radius = dim1;
    let radius2 = dim2;
    let width = dim1;
    let height = dim2;
    let length = dim3;
    let p0 = vec3(x0, y0, z0);
    let p1 = vec3(x1, y1, z1);
    match shape {
        Sphere => sdf::sphere(p, radius),
        Cuboid => sdf::cuboid(p, vec3(width, height, length)),
        CuboidFrame => sdf::cuboid_frame(p, vec3(width, height, length), vec3(dim4, dim5, dim6)),
        Capsule => sdf::capsule(p, p0, p1, radius),
        Torus => sdf::torus(p, vec2(radius, radius2)),
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
    pub dim1: f32,
    pub dim2: f32,
    pub dim3: f32,
    pub dim4: f32,
    pub dim5: f32,
    pub dim6: f32,
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
    pub x2: f32,
    pub y2: f32,
    pub z2: f32,
}

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct ShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,

    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub rotation: f32,
    pub slice_z: f32,
    pub translate_x: f32,
    pub translate_y: f32,
    pub distance: f32,

    /// Bit mask of the pressed buttons (0 = Left, 1 = Middle, 2 = Right).
    pub mouse_button_pressed: u32,

    // pub rotation: f32,
    pub shape: u32,
    pub params: Params,
}
