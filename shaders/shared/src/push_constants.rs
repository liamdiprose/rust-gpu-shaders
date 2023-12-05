pub mod koch_snowflake;
pub mod mandelbrot;
pub mod ray_marching;
pub mod ray_marching_2d;
pub mod sierpinski_triangle;

pub fn largest_size() -> usize {
    use core::mem::size_of;
    size_of::<koch_snowflake::ShaderConstants>()
        .max(size_of::<mandelbrot::ShaderConstants>())
        .max(size_of::<ray_marching::ShaderConstants>())
        .max(size_of::<ray_marching_2d::ShaderConstants>())
        .max(size_of::<sierpinski_triangle::ShaderConstants>())
}
