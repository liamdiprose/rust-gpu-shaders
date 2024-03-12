pub use super::traits::*;
use crate::reduce;
use core::ops::*;
use spirv_std::glam::{Vec2, Vec3};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

macro_rules! replace_expr {
    ($_:tt $sub:tt) => {
        $sub
    };
}

macro_rules! tuple_impls {
    ( $( $idx:tt )+ ) => {
        impl MinElement for ($(replace_expr!($idx f32),)+)
        {
            type Output = f32;
            fn min_element(self) -> Self::Output {
                reduce!((f32::min),$(self.$idx),+)
            }
        }
        impl MaxElement for ($(replace_expr!($idx f32),)+)
        {
            type Output = f32;
            fn max_element(self) -> Self::Output {
                reduce!((f32::max),$(self.$idx),+)
            }
        }
        impl Sum for ($(replace_expr!($idx f32),)+)
        {
            type Output = f32;
            fn sum(self) -> Self::Output {
                reduce!((f32::add), $(self.$idx),+)
            }
        }
        impl Product for ($(replace_expr!($idx f32),)+)
        {
            type Output = f32;
            fn product(self) -> Self::Output {
                reduce!((f32::mul), $(self.$idx),+)
            }
        }
        impl<T> Zip for ($(replace_expr!($idx T),)+) {
            type Output = ($(replace_expr!($idx (T, T)),)+);
            fn zip(self, other: Self) -> Self::Output {
                 ($((self.$idx, other.$idx)),+,)
            }
        }
        impl<T, U> Map<T, U> for ($(replace_expr!($idx T),)+) {
            type Output = ($(replace_expr!($idx U),)+);
            fn map<F>(self, f: F) -> Self::Output
            where
                F: Fn(T) -> U,
            {
                ($(f(self.$idx)),+,)
            }
        }
        impl<T> Prepend<T> for ($(replace_expr!($idx T),)+)
        {
            type Output = (T, $(replace_expr!($idx T),)+);
            fn prepend(self, value: T) -> Self::Output {
                (value, $(self.$idx),+)
            }
        }
        impl MinLength for ($(replace_expr!($idx Vec2),)+)
        {
            fn min_length(self) -> f32 {
                self.map(Vec2::length_squared).min_element().sqrt()
            }
        }
        impl MinLength for ($(replace_expr!($idx Vec3),)+)
        {
            fn min_length(self) -> f32 {
                self.map(Vec3::length_squared).min_element().sqrt()
            }
        }
    }
}

tuple_impls! { 0 1 }
tuple_impls! { 0 1 2 }
tuple_impls! { 0 1 2 3 }
tuple_impls! { 0 1 2 3 4 }
tuple_impls! { 0 1 2 3 4 5 }
