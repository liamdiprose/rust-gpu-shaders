use bytemuck::Zeroable;
use egui::{vec2, Context, Vec2};
use shared::push_constants::koch_snowflake::ShaderConstants;
use winit::dpi::{PhysicalPosition, PhysicalSize};

pub struct Controller {
    size: PhysicalSize<u32>,
    cursor: Vec2,
    use_antisnowflake: bool,
    shader_constants: ShaderConstants,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            cursor: Vec2::ZERO,
            use_antisnowflake: false,
            shader_constants: ShaderConstants::zeroed(),
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        self.shader_constants = ShaderConstants {
            width: self.size.width,
            height: self.size.height,
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            use_antisnowflake: self.use_antisnowflake as u32,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn ui(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.radio_value(&mut self.use_antisnowflake, false, "Snowflake");
        ui.radio_value(&mut self.use_antisnowflake, true, "AntiSnowflake");
    }
}
