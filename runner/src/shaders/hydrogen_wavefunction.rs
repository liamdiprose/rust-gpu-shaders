use bytemuck::Zeroable;
use egui::{vec2, Context, Vec2};
use shared::push_constants::hydrogen_wavefunction::ShaderConstants;
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
    shader_constants: ShaderConstants,
    n: i32,
    l: i32,
    m: i32,
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
            shader_constants: ShaderConstants::zeroed(),
            n: 1,
            l: 0,
            m: 0,
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
        // self.camera *= 1.0 / scroll;
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
            n: self.n as u32,
            l: self.l as u32,
            m: self.m,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn ui(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("n:");
            ui.add(
                egui::DragValue::new(&mut self.n)
                    .clamp_range(1..=5)
                    .speed(0.1),
            );
        });
        ui.horizontal(|ui| {
            ui.label("l:");
            ui.add(
                egui::DragValue::new(&mut self.l)
                    .clamp_range(0..=self.n - 1)
                    .speed(0.1),
            );
        });
        ui.horizontal(|ui| {
            ui.label("m:");
            ui.add(
                egui::DragValue::new(&mut self.m)
                    .clamp_range(-self.l..=self.l)
                    .speed(0.1),
            );
        });
    }
}
