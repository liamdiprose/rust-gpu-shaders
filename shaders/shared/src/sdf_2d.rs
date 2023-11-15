use spirv_std::glam::{vec2, Vec2};
use spirv_std::num_traits::Float;

pub fn circle(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

pub fn rectangle(p: Vec2, dim: Vec2) -> f32 {
    let v = p.abs() - dim;
    let e = v.max(Vec2::ZERO).length();
    let i = Float::min(0.0, v.max_element());
    e + i
}

/// d must be normalized or else it will scale space
pub fn infinite_plane(p: Vec2, d: Vec2) -> f32 {
    d.perp_dot(p)
}

/// d must be normalized or else it will scale space
pub fn infinite_line(p: Vec2, d: Vec2) -> f32 {
    infinite_plane(p, d).abs()
}

pub fn ray(p: Vec2, d: Vec2) -> f32 {
    let t = (p.dot(d) / d.length_squared()).max(0.0);
    p.distance(t * d)
}

pub fn plane_ray(p: Vec2, d: Vec2) -> f32 {
    ray(p, d) * infinite_plane(p, d).signum()
}

pub fn plane_segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    line_segment(p, a, b) * infinite_plane(p - a, b - a).signum()
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
    Float::abs(p.length() - r.x) - r.y
}

pub fn equilateral_triangle(mut p: Vec2, r: f32) -> f32 {
    p.x = p.x.abs();
    let k = Float::sqrt(3.0);

    Float::max(
        plane_ray(p - vec2(r, -r / k), Vec2::NEG_X),
        plane_segment(p, vec2(0.0, 2.0 * r / k), vec2(r, -r / k)),
    )
}
