use crate::shape::shape::Shape;
use spirv_std::glam::{vec4, Vec3, Vec4, Vec4Swizzles};

pub struct Sphere(Vec4);

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Sphere(vec4(x,y,z,r))
    }

    fn pos(&self) -> Vec3 {
        self.0.xyz()
    }

    fn radius(&self) -> f32 {
        self.0.w
    }
}

impl Shape for Sphere {
    fn distance_estimate(self, p: Vec3) -> f32 {
        self.pos().distance(p) - self.radius()
    }
}
