pub use crate::functional::traits::*;
use core::ops::*;
pub use tuple::Map;

macro_rules! replace_expr {
    ($_:tt $sub:tt) => {
        $sub
    };
}

macro_rules! reduce {
    ($name:tt, $x:expr) => ( $x );
    ($name:tt, $x:expr, $($xs:expr),+) => {
        {
            $name($x, reduce!($name, $($xs),+))
        }
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
    }
}

tuple_impls! { 0 1 }
tuple_impls! { 0 1 2 }
tuple_impls! { 0 1 2 3 }
tuple_impls! { 0 1 2 3 4 }
tuple_impls! { 0 1 2 3 4 5 }
