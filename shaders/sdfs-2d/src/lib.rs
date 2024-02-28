#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sdfs_2d::ShaderConstants;
use shared::push_constants::sdfs_2d::Shape;
use shared::*;
use shared::{push_constants::sdfs_2d::Params, sdf_2d as sdf};
use spirv_std::glam::{vec3, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn sdf(p: Vec2, shape: u32, params: Params) -> f32 {
    use Shape::*;
    let dim: Vec2 = params.dims.into();
    let radius = dim.x;
    let p0: Vec2 = params.ps[0].into();
    let p1: Vec2 = params.ps[1].into();
    let p2: Vec2 = params.ps[2].into();
    let p = p.rotate(Vec2::from_angle(params.rot));

    match Shape::from_u32(shape) {
        Disk => sdf::disk(p, radius),
        Rectangle => sdf::rectangle(p, dim),
        EquilateralTriangle => sdf::equilateral_triangle(p, radius),
        IsoscelesTriangle => sdf::isosceles_triangle(p, dim),
        Triangle => sdf::triangle(p, p0, p1, p2),
        Capsule => sdf::capsule(p, p0, p1, radius),
        Torus => sdf::torus(p, dim),
        Line => sdf::line(p, Vec2::Y),
        Plane => sdf::plane(p, Vec2::Y),
        LineSegment => sdf::line_segment(p, p0, p1),
        PlaneSegment => sdf::plane_segment(p, p0, p1),
        Ray => sdf::ray(p - p0, Vec2::X),
        PlaneRay => sdf::plane_ray(p - p0, Vec2::X),
    }
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = from_pixels(frag_coord.xy(), constants.size);
    let cursor = from_pixels(constants.cursor.into(), constants.size);

    let col = {
        let d = sdf(uv, constants.shape, constants.params);

        let mut col = if d < 0.0 {
            vec3(0.65, 0.85, 1.0)
        } else {
            vec3(0.9, 0.6, 0.3)
        };
        col *= 1.0 - (-6.0 * d.abs()).exp();
        col *= 0.8 + 0.2 * (150.0 * d).cos();
        col = col.lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.01, d.abs()));

        if constants.mouse_button_pressed & 1 != 0 {
            let d = sdf(cursor, constants.shape, constants.params);
            let thickness = 1.0 / constants.size.height as f32;
            col = col
                .lerp(
                    vec3(1.0, 1.0, 0.0),
                    smoothstep(thickness, 0.0, sdf::disk(uv - cursor, 0.01)),
                )
                .lerp(
                    vec3(1.0, 1.0, 0.0),
                    smoothstep(
                        thickness,
                        0.0,
                        sdf::disk(uv - cursor, d.abs()).abs() - 0.0025,
                    ),
                );
        }

        col
    };

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
