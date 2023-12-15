use crate::tuple::{Map, MinElement, Zip};
use crate::{saturate, PI};
use spirv_std::glam::{vec2, BVec3, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub mod ops;

pub fn circle(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

pub fn rectangle(p: Vec2, dim: Vec2) -> f32 {
    let v = p.abs() - dim / 2.0;
    let e = v.max(Vec2::ZERO).length();
    let i = v.max_element().min(0.0);
    e + i
}

/// d must be normalized or else it will scale space
pub fn plane(p: Vec2, d: Vec2) -> f32 {
    d.perp_dot(p)
}

/// d must be normalized or else it will scale space
pub fn line(p: Vec2, d: Vec2) -> f32 {
    plane(p, d).abs()
}

/// d must be normalized or else it will scale space
pub fn ray(p: Vec2, d: Vec2) -> f32 {
    let t = p.dot(d).max(0.0);
    p.distance(t * d)
}

pub fn plane_ray(p: Vec2, d: Vec2) -> f32 {
    ray(p, d) * plane(p, d).signum()
}

pub fn plane_segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    line_segment(p, a, b) * plane(p - a, b - a).signum()
}

pub fn line_segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let t = saturate(ap.dot(ab) / ab.length_squared());
    let c = a + t * ab;
    p.distance(c)
}

pub fn capsule(p: Vec2, a: Vec2, b: Vec2, r: f32) -> f32 {
    line_segment(p, a, b) - r
}

pub fn torus(p: Vec2, r: Vec2) -> f32 {
    (p.length() - r.x).abs() - r.y
}

pub fn equilateral_triangle(p: Vec2, r: f32) -> f32 {
    let (s, c) = (PI / 6.0).sin_cos();
    isosceles_triangle(p - vec2(0.0, 0.5 * r * s), vec2(2.0 * r * c, r + r * s))
}

pub fn isosceles_triangle(p: Vec2, dim: Vec2) -> f32 {
    if dim == Vec2::ZERO {
        return p.length();
    }
    let p = vec2(p.x.abs(), p.y + dim.y / 2.0);
    ops::intersection(
        plane_ray(p - vec2(dim.x / 2.0, 0.0), Vec2::NEG_X),
        plane_segment(p, vec2(0.0, dim.y), vec2(dim.x / 2.0, 0.0)),
    )
}

/// https://iquilezles.org/articles/distfunctions2d/
pub fn triangle(p: Vec2, p0: Vec2, p1: Vec2, p2: Vec2) -> f32 {
    let e = (p1 - p0, p2 - p1, p0 - p2);
    let w = (p - p0, p - p1, p - p2);
    let ew = e.zip(w);
    let sgn = {
        let s = (e.0.x * e.2.y - e.0.y * e.2.x).signum();
        -ew.map(|(e, w)| s * (w.x * e.y - w.y * e.x))
            .min_element()
            .signum()
    };
    let d = ew.map(|(e, w)| (w - e * saturate(w.dot(e) / e.length_squared())).length_squared());
    sgn * (p - p0).length_squared().min(d.min_element()).sqrt()
}

/// https://iquilezles.org/articles/distfunctions2d/
pub fn polygon<const N: usize>(p: Vec2, ps: &[Vec2; N]) -> f32 {
    let mut d = (p - ps[0]).length_squared();
    let mut s = 1.0;
    let mut j = N - 1;
    for i in 0..N {
        let e = ps[j] - ps[i];
        let w = p - ps[i];
        let b = w - e * saturate(w.dot(e) / e.length_squared());
        d = d.min(b.length_squared());
        let c = BVec3::new(p.y >= ps[i].y, p.y < ps[j].y, e.x * w.y > e.y * w.x);
        if c.all() || (!c).all() {
            s = -s;
        }
        j = i;
    }
    s * d.sqrt()
}
