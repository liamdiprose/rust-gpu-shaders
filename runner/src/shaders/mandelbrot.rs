use bytemuck::Zeroable;
use egui::{vec2, Context, Vec2};
use shared::push_constants::mandelbrot::ShaderConstants;
use std::time::Instant;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::MouseButton,
};

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    cursor: Vec2,
    drag_start: Vec2,
    drag_end: Vec2,
    camera: Vec2,
    zoom: f32,
    mouse_button_pressed: bool,
    exponent: f32,
    num_iterations: u32,
    shader_constants: ShaderConstants,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            start: Instant::now(),
            cursor: Vec2::ZERO,
            drag_start: Vec2::ZERO,
            drag_end: Vec2::ZERO,
            camera: Vec2::ZERO,
            zoom: 1.0,
            mouse_button_pressed: false,
            exponent: 2.0,
            num_iterations: 35,
            shader_constants: ShaderConstants::zeroed(),
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_pressed = match state {
                ElementState::Pressed => true,
                ElementState::Released => {
                    self.camera += self.drag_start - self.drag_end;
                    false
                }
            };

            self.drag_start = self.cursor;
            self.drag_end = self.cursor;
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
        if self.mouse_button_pressed {
            self.drag_end = self.cursor;
        }
    }

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        let scroll = match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                let v = 1.0 + 0.1 * y.abs();
                if y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
            MouseScrollDelta::PixelDelta(p) => {
                let v = 1.0 + 0.02 * (1.0 + p.y.abs() as f32).ln();
                if p.y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
        };
        self.zoom *= scroll;
        self.camera *= 1.0 / scroll;
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        self.shader_constants = ShaderConstants {
            width: self.size.width,
            height: self.size.height,
            time: self.start.elapsed().as_secs_f32(),
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            zoom: self.zoom,
            translate_x: self.camera.x + self.drag_start.x - self.drag_end.x,
            translate_y: self.camera.y + self.drag_start.y - self.drag_end.y,
            mouse_button_pressed: !(1 << self.mouse_button_pressed as u32),
            exponent: self.exponent,
            num_iterations: self.num_iterations,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Exponent:");
            ui.add(
                egui::DragValue::new(&mut self.exponent)
                    .clamp_range(1.0..=6.0)
                    .speed(0.1),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Num Iterations:");
            ui.add(
                egui::DragValue::new(&mut self.num_iterations)
                    .clamp_range(2..=200)
                    .speed(1),
            );
        });
    }
}
