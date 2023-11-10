use spirv_std::glam::Vec2;
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

pub fn line(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let t = (ap.dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    let c = a + t * ab;
    p.distance(c)
}

pub fn capsule(p: Vec2, a: Vec2, b: Vec2, r: f32) -> f32 {
    line(p, a, b) - r
}

pub fn torus(p: Vec2, r: Vec2) -> f32 {
    Float::abs(p.length() - r.x) - r.y
}
