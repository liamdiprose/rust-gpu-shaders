pub use crate::tuple::{Product, Sum};
use core::ops::*;
use spirv_std::glam::{vec3, Vec3};

pub trait Map {
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32;
}

impl Map for Vec3 {
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        vec3(f(self.x), f(self.y), f(self.z))
    }
}

impl Sum for Vec3 {
    type Output = f32;
    fn sum(self) -> Self::Output {
        self.x + self.y + self.z
    }
}

impl Product for Vec3 {
    type Output = f32;
    fn product(self) -> Self::Output {
        self.x * self.y * self.z
    }
}
