use structopt::StructOpt;
use strum::{Display, EnumIter, EnumString};

mod app;
mod context;
mod controller;
mod fps_counter;
mod render_pass;
mod shader;
mod shaders;
mod state;
mod ui;
mod window;

#[derive(EnumString, EnumIter, Display, PartialEq, Eq, Copy, Clone)]
pub enum RustGPUShader {
    Mandelbrot,
    RayMarching,
    RayMarching2D,
    SierpinskiTriangle,
    KochSnowflake,
    SDFs2D,
    SDFs3D,
    HydrogenWavefunction,
    SphericalHarmonics,
}

#[derive(StructOpt, Clone)]
#[structopt(name = "example-runner-wgpu")]
pub struct Options {
    #[structopt(short, long, default_value = "Mandelbrot")]
    shader: RustGPUShader,

    #[structopt(long)]
    force_spirv_passthru: bool,
}

pub fn main() {
    let options: Options = Options::from_args();

    app::start(options);
}
