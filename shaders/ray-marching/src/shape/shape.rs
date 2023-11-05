use spirv_std::glam::Vec3;

pub trait Shape {
    fn distance_estimate(self, p: Vec3) -> f32;
}
