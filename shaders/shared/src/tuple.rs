pub use tuple::Map;

pub trait MinElement {
    type Output;
    fn min_element(self) -> Self::Output;
}

pub trait Zip {
    type Output;
    fn zip(self, other: Self) -> Self::Output;
}

macro_rules! replace_expr {
    ($_:tt $sub:tt) => {
        $sub
    };
}

macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            ($x).min(min!($($xs),+))
        }
    };
}

macro_rules! tuple_impls {
    ( $( $idx:tt )+ ) => {
        impl MinElement for ($(replace_expr!($idx f32),)+)
        {
            type Output = f32;
            fn min_element(self) -> Self::Output {
                min!($(self.$idx),+)
            }
        }
        impl<T> Zip for ($(replace_expr!($idx T),)+) {
            type Output = ($(replace_expr!($idx (T, T)),)+);
            fn zip(self, other: Self) -> Self::Output {
                 ($((self.$idx, other.$idx)),+,)
            }
        }
    };
}

tuple_impls! { 0 }
tuple_impls! { 0 1 }
tuple_impls! { 0 1 2 }
tuple_impls! { 0 1 2 3 }
tuple_impls! { 0 1 2 3 4 }
tuple_impls! { 0 1 2 3 4 5 }
tuple_impls! { 0 1 2 3 4 5 6 }
tuple_impls! { 0 1 2 3 4 5 6 7 }
tuple_impls! { 0 1 2 3 4 5 6 7 8 }
tuple_impls! { 0 1 2 3 4 5 6 7 8 9 }
tuple_impls! { 0 1 2 3 4 5 6 7 8 9 10 }
tuple_impls! { 0 1 2 3 4 5 6 7 8 9 10 11 }
