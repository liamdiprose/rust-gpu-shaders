use spirv_std::glam::Vec3;

pub fn plane(p: Vec3) -> f32 {
    p.y
}

pub fn sphere(p: Vec3, pos: Vec3, r: f32) -> f32 {
    pos.distance(p) - r
}
