use spirv_std::glam::Vec2;

pub fn circle(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

pub fn rectangle(p: Vec2, dim: Vec2) -> f32 {
    (p.abs() - dim).max_element()
}

pub fn line(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ap = p - a;
    let ab = b - a;
    let t = (ap.dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    let c = a + t * ab;
    p.distance(c)
}
