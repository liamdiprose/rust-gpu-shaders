use bytemuck::Zeroable;
use egui::{vec2, Vec2, Context};
use shared::push_constants::mandelbrot::ShaderConstants;
use winit::dpi::PhysicalSize;
use std::time::{Duration, Instant};
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

#[derive(Clone, Copy)]
pub struct Options {
    pub exponent: f32,
    pub num_iterations: u32,
}

impl Options {
    pub fn new() -> Self {
        Self {
            exponent: 2.0,
            num_iterations: 35,
        }
    }
}

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    elapsed: Duration,
    cursor: Vec2,
    drag_start: Vec2,
    drag_end: Vec2,
    camera: Vec2,
    zoom: f32,
    mouse_button_pressed: bool,

    scroll: f32,
    drag: Vec2,
    prev_cursor: Vec2,
    options: Options,
    shader_constants: ShaderConstants,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            start: Instant::now(),
            elapsed: Duration::ZERO,
            cursor: Vec2::ZERO,
            drag_start: Vec2::ZERO,
            drag_end: Vec2::ZERO,
            camera: Vec2::ZERO,
            zoom: 1.0,
            mouse_button_pressed: false,

            scroll: 1.0,
            drag: Vec2::ZERO,
            prev_cursor: Vec2::ZERO,
            options: Options::new(),
            shader_constants: ShaderConstants::zeroed(),
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_pressed = match state {
                ElementState::Pressed => true,
                ElementState::Released => {
                    self.drag = self.drag_start - self.drag_end;
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
        self.scroll *= match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                let v = 1.0 + 0.1 * y.abs();
                if y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
            MouseScrollDelta::PixelDelta(p) => {
                let v = 1.0 + 0.1 * (1.0 + p.y.abs() as f32).ln();
                if p.y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
        };
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        self.elapsed = self.start.elapsed();
        self.zoom *= self.scroll;
        self.camera *= 1.0 / self.scroll;
        self.camera += self.drag;

        self.shader_constants = ShaderConstants {
            width: self.size.width,
            height: self.size.height,
            time: self.elapsed.as_secs_f32(),
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            drag_start_x: self.drag_start.x,
            drag_start_y: self.drag_start.y,
            drag_end_x: self.drag_end.x,
            drag_end_y: self.drag_end.y,
            zoom: self.zoom,
            translate_x: self.camera.x,
            translate_y: self.camera.y,
            mouse_button_pressed: !(1 << self.mouse_button_pressed as u32),

            exponent: self.options.exponent,
            num_iterations: self.options.num_iterations,
        };
        self.finish_update();
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn ui(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Exponent:");
            ui.add(
                egui::DragValue::new(&mut self.options.exponent)
                    .clamp_range(1.0..=6.0)
                    .speed(0.1),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Num Iterations:");
            ui.add(
                egui::DragValue::new(&mut self.options.num_iterations)
                    .clamp_range(2..=200)
                    .speed(1),
            );
        });
    }
}

impl Controller {
    pub fn finish_update(&mut self) {
        self.scroll = 1.0;
        self.drag = Vec2::ZERO;
        self.prev_cursor = self.cursor;
    }
}
