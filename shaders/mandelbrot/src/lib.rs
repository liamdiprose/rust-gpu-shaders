#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use push_constants::mandelbrot::ShaderConstants;
use shared::*;
use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let coord = Complex::new(
        frag_coord.x + constants.translate_x,
        frag_coord.y + constants.translate_y,
    );

    let uv = constants.zoom
        * (coord - 0.5 * Complex::new(constants.width as f32, constants.height as f32))
        / constants.height as f32;

    let mut z = Complex::ZERO;
    let mut n = constants.num_iterations;
    while z.norm_squared() < 4.0 && n > 0 {
        z = z.powf(constants.exponent as f32) + uv;
        n -= 1;
    }

    let c = n as f32 / 35.0;
    *output = vec4(c, c, c, 1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
