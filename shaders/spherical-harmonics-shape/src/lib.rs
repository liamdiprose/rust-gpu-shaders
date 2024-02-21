#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::spherical_harmonics_shape::ShaderConstants;
use shared::*;
use spirv_std::glam::{Mat4, Quat, Vec3, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(
    col: Vec3,
    #[spirv(frag_coord)] _frag_coord: Vec4,
    #[spirv(push_constant)] _constants: &ShaderConstants,
    output: &mut Vec4,
) {
    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    pos: Vec3,
    col: Vec3,
    #[spirv(push_constant)] constants: &ShaderConstants,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
    out_col: &mut Vec3,
) {
    let rot: Quat = constants.rot.into();
    let view_proj: Mat4 = constants.view_proj.into();

    *out_pos = view_proj * (rot * pos).extend(1.0);
    *out_col = col;
}
