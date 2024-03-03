pub use super::traits::*;
use crate::saturate;
use core::ops::*;
use spirv_std::glam::{vec3, Vec2, Vec3};

pub trait Projection {
    fn project_onto_segment(self, rhs: Self) -> Self;
    fn reject_from_segment(self, rhs: Self) -> Self;
}

macro_rules! impl_vec {
    ($($T:ty)+) => {$(
        impl Projection for $T {
            fn project_onto_segment(self, rhs: $T) -> $T {
                rhs * saturate(self.dot(rhs) / rhs.length_squared())
            }

            fn reject_from_segment(self, rhs: $T) -> $T {
                self - self.project_onto_segment(rhs)
            }
        }
    )*}
}

impl_vec!(Vec2 Vec3);

impl Map<f32, f32> for Vec3 {
    type Output = Self;
    fn map<F>(self, f: F) -> Self::Output
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
