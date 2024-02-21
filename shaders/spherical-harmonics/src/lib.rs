#![cfg_attr(target_arch = "spirv", no_std)]

use complex::Complex;
use core::f32::consts::FRAC_1_SQRT_2;
use push_constants::spherical_harmonics::{ShaderConstants, Variant};
use shared::{spherical_harmonics::*, *};
use spirv_std::glam::{
    vec2, vec3, Quat, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles, Vec4, Vec4Swizzles,
};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn ray_intersect_box_frame(ro: Vec3, rd: Vec3, dim: Vec2) -> bool {
    let e = vec2(dim.x, -dim.x);
    let o = vec3(dim.y, -dim.y, 0.0);
    ray_intersect_aabb(ro, rd, e.yyy(), e.yxy() + o.xzx())
        || ray_intersect_aabb(ro, rd, e.yyy(), e.xyy() + o.zxx())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.yxy() + o.zyx())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.xyy() + o.yzx())
        || ray_intersect_aabb(ro, rd, e.yyx(), e.yxx() + o.xzy())
        || ray_intersect_aabb(ro, rd, e.yyx(), e.xyx() + o.zxy())
        || ray_intersect_aabb(ro, rd, e.xxx(), e.yxx() + o.zyy())
        || ray_intersect_aabb(ro, rd, e.xxx(), e.xyx() + o.yzy())
        || ray_intersect_aabb(ro, rd, e.yyy(), e.yyx() + o.xxz())
        || ray_intersect_aabb(ro, rd, e.yxy(), e.yxx() + o.xyz())
        || ray_intersect_aabb(ro, rd, e.xyy(), e.xyx() + o.yxz())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.xxx() + o.yyz())
}

fn ray_intersect_aabb(ro: Vec3, rd: Vec3, a: Vec3, b: Vec3) -> bool {
    let t1 = (a - ro) / rd;
    let t2 = (b - ro) / rd;
    t1.max(t2).min_element() > t1.min(t2).max_element()
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = (Complex::from(frag_coord.xy())
        - 0.5 * Complex::new(constants.size.width as f32, constants.size.height as f32))
        / constants.size.height as f32;
    let rot: Quat = constants.quat.into();
    let r = 0.3 / constants.zoom;

    let col = if uv.length_squared() <= r * r {
        let pos = {
            let x = uv.x;
            let y = uv.y;
            let z = -(r * r - x * x - y * y).sqrt();
            rot * uv.extend(z)
        };
        let (_, theta, phi) = to_spherical(pos);
        let m = constants.m;
        let l = constants.l;
        match Variant::from_u32(constants.variant) {
            Variant::Real => {
                let x = real_spherical_harmonic(m, l, theta, phi, constants.time);
                vec3(x, 0.0, -x)
            }
            Variant::Complex => {
                let z = spherical_harmonic(m, l, theta, phi, constants.time);
                vec3(
                    z.dot(Vec2::X),
                    z.dot(vec2(-FRAC_1_SQRT_2, FRAC_1_SQRT_2)),
                    z.dot(Vec2::splat(-FRAC_1_SQRT_2)),
                )
            }
        }
    } else {
        let ro = rot * uv.extend(-constants.zoom);
        let rd = rot * Vec2::ZERO.extend(1.0);
        if ray_intersect_box_frame(ro, rd, vec2(r, 0.002 / constants.zoom)) {
            vec3(0.1, 0.1, 0.08)
        } else {
            Vec3::ZERO
        }
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
