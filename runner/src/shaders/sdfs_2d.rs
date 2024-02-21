use bytemuck::Zeroable;
use egui::{Context, CursorIcon};
use glam::{vec2, Vec2};
use shared::push_constants::sdfs_2d::{Params, ShaderConstants, Shape};
use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};
use strum::IntoEnumIterator;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta},
    event_loop::EventLoopProxy,
};

use crate::window::UserEvent;

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    elapsed: Duration,
    cursor: Vec2,
    mouse_button_pressed: bool,
    can_drag: Option<usize>,
    drag_point: Option<usize>,
    shape: Shape,
    params: Vec<Params>,
    shader_constants: ShaderConstants,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            start: Instant::now(),
            elapsed: Duration::ZERO,
            cursor: Vec2::ZERO,
            mouse_button_pressed: false,
            can_drag: None,
            drag_point: None,
            shape: Shape::Circle,
            params: Shape::iter().map(|shape| shape.params()).collect(),
            shader_constants: ShaderConstants::zeroed(),
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            match state {
                ElementState::Pressed => {
                    self.drag_point = self.can_drag;
                    self.mouse_button_pressed = true;
                }
                ElementState::Released => {
                    self.drag_point = None;
                    self.mouse_button_pressed = false;
                }
            }
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
        let num_points = self.shape.spec().num_points;
        if let Some(i) = self.drag_point {
            self.params[self.shape as usize].ps[i] = rotate(
                self.from_pixels(self.cursor),
                self.params[self.shape as usize].rot,
            )
            .into();
        } else if num_points > 0 {
            self.can_drag = self.params[self.shape as usize].ps[0..num_points as usize]
                .iter()
                .position(|p| {
                    (rotate((*p).into(), -self.params[self.shape as usize].rot)
                        - self.from_pixels(self.cursor))
                    .length()
                        < 0.01
                });
        }
    }

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.params[self.shape as usize].rot += PI / 30.0
            * match delta {
                MouseScrollDelta::LineDelta(_, y) => y,
                MouseScrollDelta::PixelDelta(p) => {
                    (1.0 + p.y.abs() as f32).ln() * p.y.signum() as f32
                }
            };
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        self.elapsed = self.start.elapsed();
        self.shader_constants = ShaderConstants {
            size: self.size.into(),
            time: self.elapsed.as_secs_f32(),
            cursor: self.cursor.into(),
            mouse_button_pressed: !(1
                << (self.mouse_button_pressed && self.drag_point.is_none()) as u32),
            shape: self.shape as u32,
            params: self.params[self.shape as usize],
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&mut self, ctx: &Context, ui: &mut egui::Ui, _: &EventLoopProxy<UserEvent>) {
        ctx.set_cursor_icon(if self.drag_point.is_some() {
            CursorIcon::Grabbing
        } else if self.can_drag.is_some() {
            CursorIcon::Grab
        } else {
            CursorIcon::Default
        });
        for shape in Shape::iter() {
            ui.radio_value(&mut self.shape, shape, shape.to_string());
        }
        let spec = self.shape.spec();
        if spec.num_dims > 0 {
            let params = &mut self.params[self.shape as usize];
            let (dim1_max, dim2_max, dim1_label, dim2_label) = {
                if spec.is_radial {
                    (0.5, params.dim.x, "Radius", "Radius2")
                } else {
                    (
                        self.shader_constants.size.aspect_ratio(),
                        1.0,
                        "Width",
                        "Height",
                    )
                }
            };
            ui.horizontal(|ui| {
                ui.label(dim1_label);
                ui.add(
                    egui::DragValue::new(&mut params.dim.x)
                        .clamp_range(0.0..=dim1_max)
                        .speed(0.01),
                );
            });
            if spec.num_dims > 1 {
                ui.horizontal(|ui| {
                    ui.label(dim2_label);
                    ui.add(
                        egui::DragValue::new(&mut params.dim.y)
                            .clamp_range(0.0..=dim2_max)
                            .speed(0.01),
                    );
                });
            }
        }
    }
}

impl Controller {
    fn from_pixels(&self, p: Vec2) -> Vec2 {
        let p = vec2(p.x, -p.y);
        (p - 0.5 * vec2(self.size.width as f32, -(self.size.height as f32)))
            / self.size.height as f32
    }
}

fn rotate(p: Vec2, angle: f32) -> Vec2 {
    p.rotate(Vec2::from_angle(angle))
}
