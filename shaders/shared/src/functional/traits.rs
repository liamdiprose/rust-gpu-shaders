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

pub trait Map {
    fn map<F>(self, f: F) -> Self
    where
        F: Fn(f32) -> f32;
}
