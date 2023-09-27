use spirv_std::glam::Vec2;

pub trait Complex {
    fn square(self) -> Self;
}

impl Complex for Vec2 {
    fn square(self) -> Self {
        Vec2::new(self.x * self.x - self.y * self.y, 2.0 * self.x * self.y)
    }
}
