use spirv_std::glam::{vec2, Vec2};
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

pub fn ray(p: Vec2, d: Vec2) -> f32 {
    let t = (p.dot(d) / d.length_squared()).max(0.0);
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
    let t = (ap.dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    let c = a + t * ab;
    p.distance(c)
}

pub fn capsule(p: Vec2, a: Vec2, b: Vec2, r: f32) -> f32 {
    line_segment(p, a, b) - r
}

pub fn torus(p: Vec2, r: Vec2) -> f32 {
    (p.length() - r.x).abs() - r.y
}

pub fn equilateral_triangle(mut p: Vec2, r: f32) -> f32 {
    p.x = p.x.abs();
    let k = 3.0.sqrt();

    ops::intersection(
        plane_ray(p - vec2(r, -r / k), Vec2::NEG_X),
        plane_segment(p, vec2(0.0, 2.0 * r / k), vec2(r, -r / k)),
    )
}
