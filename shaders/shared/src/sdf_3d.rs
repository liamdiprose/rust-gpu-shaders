use crate::saturate;
use crate::functional::tuple::*;
use spirv_std::glam::{vec2, vec3, Vec2, Vec3};
use spirv_std::num_traits::Float;

pub mod ops;

// `n` must be normalized
pub fn plane(p: Vec3, n: Vec3) -> f32 {
    p.dot(n)
}

// `n` must be normalized
pub fn torus(p: Vec3, r: Vec2, n: Vec3) -> f32 {
    vec2(p.cross(n).length() - r.x, p.dot(n)).length() - r.y
}

// `n` must be normalized
pub fn disk(p: Vec3, r: Vec2, n: Vec3) -> f32 {
    let v = vec2(p.cross(n).length() - r.x, p.dot(n).abs());
    v.max(Vec2::ZERO).length() + v.max_element().min(0.0) - r.y
}

pub fn sphere(p: Vec3, r: f32) -> f32 {
    p.length() - r
}

pub fn tetrahedron(p: Vec3, r: f32) -> f32 {
    let md = (-p.x - p.y - p.z)
        .max(-p.x + p.y + p.z)
        .max(p.x - p.y + p.z)
        .max(p.x + p.y - p.z);
    (md - r) / 3.0.sqrt()
}

pub fn line_segment(p: Vec3, a: Vec3, b: Vec3) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let t = saturate(ap.dot(ab) / ab.length_squared());
    p.distance(a + t * ab)
}

pub fn capsule(p: Vec3, a: Vec3, b: Vec3, r: f32) -> f32 {
    line_segment(p, a, b) - r
}

pub fn cylinder(p: Vec3, a: Vec3, b: Vec3, r: f32) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let t = ap.dot(ab) / ab.length_squared();
    let x = p.distance(a + t * ab) - r;
    let y = ((t - 0.5).abs() - 0.5) * ab.length();
    let e = vec2(x, y).max(Vec2::ZERO).length();
    let i = x.max(y).min(0.0);
    e + i
}

pub fn cuboid(p: Vec3, dim: Vec3) -> f32 {
    let v = p.abs() - dim / 2.0;
    let e = v.max(Vec3::ZERO).length();
    let i = v.max_element().min(0.0);
    e + i
}

pub fn cuboid_frame_radial(p: Vec3, dim: Vec3, r: f32) -> f32 {
    let v = p.abs() - dim / 2.0;
    (
        vec3(v.x, v.y, v.z.max(0.0)),
        vec3(v.x, v.y.max(0.0), v.z),
        vec3(v.x.max(0.0), v.y, v.z),
    )
        .map(|v| v.length_squared())
        .min_element()
        .sqrt()
        - r
}

pub fn cuboid_frame(p: Vec3, dim: Vec3, dim2: Vec3) -> f32 {
    let p = p.abs() - dim / 2.0;
    let q = (p + dim2 / 2.0).abs() - dim2 / 2.0;
    (
        vec3(p.x, q.y, q.z),
        vec3(q.x, p.y, q.z),
        vec3(q.x, q.y, p.z),
    )
        .map(|p| p.max(Vec3::ZERO).length() + p.max_element().min(0.0))
        .min_element()
}
