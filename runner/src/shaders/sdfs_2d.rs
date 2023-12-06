use bytemuck::Zeroable;
use egui::{vec2, Vec2};
use shared::push_constants::sdfs_2d::{Params, ShaderConstants, Shape};
use shared::PI;
use std::time::{Duration, Instant};
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

#[derive(Clone, Copy)]
pub struct Options {
    pub shape: Shape,
    pub params: Params,
}

impl Options {
    pub fn new() -> Self {
        Self {
            shape: Shape::Circle,
            params: Params {
                radius: 0.25,
                width: 0.5,
                height: 0.3,
                x: 0.0,
                y: 0.0,
            },
        }
    }
}

pub struct Controller {
    start: Instant,
    elapsed: Duration,
    cursor: Vec2,
    rotation: f32,
    mouse_button_pressed: bool,
    options: Options,
    shader_constants: ShaderConstants,
}

impl crate::controller::Controller for Controller {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            elapsed: Duration::ZERO,
            cursor: Vec2::ZERO,
            rotation: 0.0,
            mouse_button_pressed: false,
            options: Options::new(),
            shader_constants: ShaderConstants::zeroed(),
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_pressed = match state {
                ElementState::Pressed => true,
                ElementState::Released => false,
            };
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
    }

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.rotation += PI / 30.0
            * match delta {
                MouseScrollDelta::LineDelta(_, y) => y,
                MouseScrollDelta::PixelDelta(p) => {
                    (1.0 + p.y.abs() as f32).ln() * p.y.signum() as f32
                }
            };
    }

    fn update(&mut self, width: u32, height: u32, options: crate::shaders::Options) {
        self.options = options.sdfs_2d;
        self.elapsed = self.start.elapsed();

        self.shader_constants = ShaderConstants {
            width,
            height,
            time: self.elapsed.as_secs_f32(),

            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            mouse_button_pressed: !(1 << self.mouse_button_pressed as u32),

            rotation: self.rotation,
            shape: self.options.shape as u32,
            params: self.options.params,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }
}
