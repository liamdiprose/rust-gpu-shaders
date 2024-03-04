pub use super::traits::*;
use crate::{reduce, saturate};
use core::ops::*;
use spirv_std::glam::{Vec2, Vec3};

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
    )+}
}

impl_vec!(Vec2 Vec3);

macro_rules! impl_vec_with_dimensions {
    ($T:tt, $($d:tt)+) => {
        impl Map<f32, f32> for $T {
            type Output = Self;
            fn map<F>(self, f: F) -> Self::Output
            where
                F: Fn(f32) -> f32,
            {
                $T::new($(f(self.$d)),+)
            }
        }
        impl Sum for $T {
            type Output = f32;
            fn sum(self) -> Self::Output {
                reduce!((f32::add), $(self.$d),+)
            }
        }
        impl Product for $T {
            type Output = f32;
            fn product(self) -> Self::Output {
                reduce!((f32::mul), $(self.$d),+)
            }
        }
    }
}

impl_vec_with_dimensions!(Vec2, x y);
impl_vec_with_dimensions!(Vec3, x y z);
