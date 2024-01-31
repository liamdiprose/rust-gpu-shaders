use bytemuck::Zeroable;
use egui::{vec2, Context, Vec2};
use glam::Quat;
use shared::push_constants::spherical_harmonics::ShaderConstants;
use std::time::Instant;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::MouseButton,
};

use core::f32::consts::PI;

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    cursor: Vec2,
    drag_start: Vec2,
    drag_end: Vec2,
    q: Quat,
    zoom: f32,
    mouse_button_pressed: bool,
    shader_constants: ShaderConstants,
    l: u32,
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
            q: Quat::IDENTITY,
            zoom: 1.0,
            mouse_button_pressed: false,
            shader_constants: ShaderConstants::zeroed(),
            l: 2,
            m: 1,
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.mouse_button_pressed = match state {
                ElementState::Pressed => true,
                ElementState::Released => {
                    let angles = PI * (self.drag_start - self.drag_end) / self.size.height as f32;
                    self.q = self
                        .q
                        .mul_quat(Quat::from_rotation_y(-angles.x))
                        .mul_quat(Quat::from_rotation_x(angles.y))
                        .normalize();
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
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        let angles = PI * (self.drag_start - self.drag_end) / self.size.height as f32;
        let q = self
            .q
            .mul_quat(Quat::from_rotation_y(-angles.x))
            .mul_quat(Quat::from_rotation_x(angles.y));
        self.shader_constants = ShaderConstants {
            width: self.size.width,
            height: self.size.height,
            time: self.start.elapsed().as_secs_f32(),
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            zoom: self.zoom,
            mouse_button_pressed: !(1 << self.mouse_button_pressed as u32),
            l: self.l,
            m: self.m,
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn ui(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("l:");
            ui.add(
                egui::DragValue::new(&mut self.l)
                    .clamp_range(0..=10)
                    .speed(0.1),
            );
        });
        ui.horizontal(|ui| {
            ui.label("m:");
            ui.add(
                egui::DragValue::new(&mut self.m)
                    .clamp_range(-(self.l as i32)..=self.l as i32)
                    .speed(0.1),
            );
        });
    }
}
