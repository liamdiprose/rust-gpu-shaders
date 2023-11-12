use spirv_std::glam::{vec2, Vec2, Vec3, Vec3Swizzles};
use spirv_std::num_traits::Float;

pub fn plane(p: Vec3) -> f32 {
    p.y
}

pub fn sphere(p: Vec3, r: f32) -> f32 {
    p.length() - r
}

pub fn torus(p: Vec3, r: Vec2) -> f32 {
    vec2(p.xz().length() - r.x, p.y).length() - r.y
}

pub fn tetrahedron(p: Vec3, r: f32) -> f32 {
    let md = (-p.x - p.y - p.z)
        .max(-p.x + p.y + p.z)
        .max(p.x - p.y + p.z)
        .max(p.x + p.y - p.z);
    (md - r) / Float::sqrt(3.0)
}

pub fn line(p: Vec3, ab: Vec3) -> f32 {
    let a = -ab / 2.0;
    let ap = p - a;
    let t = (ap.dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    let c = a + t * ab;
    p.distance(c)
}

pub fn capsule(p: Vec3, ab: Vec3, r: f32) -> f32 {
    line(p, ab) - r
}

pub fn cylinder(p: Vec3, ab: Vec3, r: f32) -> f32 {
    let a = -ab / 2.0;
    let ap = p - a;
    let t = ap.dot(ab) / ab.length_squared();
    let c = a + t * ab;
    let x = p.distance(c) - r;
    let y = ((t - 0.5).abs() - 0.5) * ab.length();
    let e = vec2(x, y).max(Vec2::ZERO).length();
    let i = x.max(y).min(0.0);
    e + i
}

pub fn cuboid(p: Vec3, dim: Vec3) -> f32 {
    (p.abs() - dim).max(Vec3::ZERO).length()
}
