use bytemuck::Zeroable;
use shared::push_constants::sierpinski_triangle::ShaderConstants;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

#[derive(Clone, Copy)]
pub struct Options {}

impl Options {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Controller {
    scroll: f64,
    shader_constants: ShaderConstants,
}

impl crate::controller::Controller for Controller {
    fn new() -> Self {
        Self {
            scroll: 0.0,
            shader_constants: ShaderConstants::zeroed(),
        }
    }

    fn mouse_input(&mut self, _state: ElementState, _button: MouseButton) {}

    fn mouse_move(&mut self, _position: PhysicalPosition<f64>) {}

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.scroll += match delta {
            MouseScrollDelta::LineDelta(_, y) => y as f64,
            MouseScrollDelta::PixelDelta(p) => 1.0 + 0.1 * (1.0 + p.y).ln(),
        };
    }

    fn update(&mut self, width: u32, height: u32, _options: crate::shaders::Options) {
        let c = 59.87868500430847;
        let v = 34.102688577484;
        let scroll = if self.scroll > c {
            self.scroll - v * (1.0 + ((self.scroll - c) / v).floor())
        } else if self.scroll < -1.0 {
            -1.0 - (-self.scroll).log10()
        } else {
            self.scroll
        };
        let zoom = 0.85_f64.powf(scroll);

        self.shader_constants = ShaderConstants {
            width: width,
            height: height,
            zoom: zoom as f32,
            x: -0.08443636,
            y: -0.087451585,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }
}
