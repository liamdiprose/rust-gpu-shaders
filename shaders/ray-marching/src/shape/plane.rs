use crate::shape::shape::Shape;
use spirv_std::glam::Vec3;

pub struct Plane();

impl Plane {
    pub fn new() -> Self {
        Plane()
    }
}

impl Shape for Plane {
    fn distance_estimate(self, p: Vec3) -> f32 {
        p.y
    }
}
