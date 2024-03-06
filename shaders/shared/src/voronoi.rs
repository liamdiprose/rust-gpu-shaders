use crate::random::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn voronoi(p: Vec2) -> Vec3 {
    let ip = p.floor();
    let fp = p.fract();

    // first pass - regular voronoi
    let mut mg = Vec2::ZERO;
    let mut mr = Vec2::ZERO;

    let mut md = 8.0;

    for j in -1..=1 {
        for i in -1..=1 {
            let g = vec2(i as f32, j as f32);
            let o = random22(ip + g);
            let r = g + o - fp;
            let d = r.dot(r);

            if d < md {
                md = d;
                mr = r;
                mg = g;
            }
        }
    }

    // second pass - distance to borders
    md = 8.0;
    for j in -2..=2 {
        for i in -2..=2 {
            let g = mg + vec2(i as f32, j as f32);
            let o = random22(ip + g);
            let r = g + o - fp;

            if (mr - r).dot(mr - r) > 0.00001 {
                md = md.min((0.5 * (mr + r)).dot((r - mr).normalize()));
            }
        }
    }

    vec3(md, mr.x, mr.y)
}
