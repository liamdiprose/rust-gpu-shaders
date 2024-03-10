pub trait MinElement {
    type Output;
    fn min_element(self) -> Self::Output;
}

pub trait MaxElement {
    type Output;
    fn max_element(self) -> Self::Output;
}

pub trait Sum {
    type Output;
    fn sum(self) -> Self::Output;
}

pub trait Product {
    type Output;
    fn product(self) -> Self::Output;
}

pub trait Zip {
    type Output;
    fn zip(self, other: Self) -> Self::Output;
}

pub trait Map<T, U> {
    type Output;
    fn map<F>(self, f: F) -> Self::Output
    where
        F: Fn(T) -> U,
        T: Copy;
}

pub trait Prepend<T> {
    type Output;
    fn prepend(self, value: T) -> Self::Output;
}

pub trait MinLength {
    fn min_length(self) -> f32;
}
