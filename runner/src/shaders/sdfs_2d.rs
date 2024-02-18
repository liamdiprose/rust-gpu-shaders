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

#[derive(Clone)]
pub struct Options {
    pub shape: Shape,
    pub can_drag: bool,
    pub is_dragging: bool,
    pub params: Vec<Params>,
}

impl Options {
    pub fn new() -> Self {
        Self {
            shape: Shape::Circle,
            can_drag: false,
            is_dragging: false,
            params: Shape::iter().map(|shape| shape.params()).collect(),
        }
    }
}

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    elapsed: Duration,
    cursor: Vec2,
    rotation: f32,
    mouse_button_pressed: bool,
    can_drag: Option<usize>,
    drag_point: Option<usize>,
    points: [Vec2; 3],
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
            rotation: 0.0,
            mouse_button_pressed: false,
            can_drag: None,
            drag_point: None,
            points: [vec2(0.0, 0.0), vec2(0.1, 0.2), vec2(-0.4, 0.3)],
            options: Options::new(),
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
        let num_points = self.options.shape.spec().num_points;
        if let Some(i) = self.drag_point {
            self.points[i] = rotate(&self.from_pixels(self.cursor), self.rotation);
        } else if num_points > 0 {
            self.can_drag = self.points[0..num_points as usize].iter().position(|p| {
                (rotate(p, -self.rotation) - self.from_pixels(self.cursor)).length() < 0.01
            });
        }
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

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        self.options.can_drag = self.can_drag.is_some();
        self.options.is_dragging = self.drag_point.is_some();
        self.elapsed = self.start.elapsed();
        self.shader_constants = ShaderConstants {
            width: self.size.width,
            height: self.size.height,
            time: self.elapsed.as_secs_f32(),
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            mouse_button_pressed: !(1
                << (self.mouse_button_pressed && !self.options.is_dragging) as u32),
            rotation: self.rotation,

            shape: self.options.shape as u32,
            params: Params {
                x0: self.points[0].x,
                y0: self.points[0].y,
                x1: self.points[1].x,
                y1: self.points[1].y,
                x2: self.points[2].x,
                y2: self.points[2].y,
                ..self.options.params[self.options.shape as usize]
            },
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&mut self, ctx: &Context, ui: &mut egui::Ui, _: &EventLoopProxy<UserEvent>) {
        ctx.set_cursor_icon(if self.options.is_dragging {
            CursorIcon::Grabbing
        } else if self.options.can_drag {
            CursorIcon::Grab
        } else {
            CursorIcon::Default
        });
        for shape in Shape::iter() {
            ui.radio_value(&mut self.options.shape, shape, shape.to_string());
        }
        let spec = self.options.shape.spec();
        if spec.num_dims > 0 {
            let params = &mut self.options.params[self.options.shape as usize];
            let (dim1_max, dim2_max, dim1_label, dim2_label) = {
                if spec.is_radial {
                    (0.5, params.dim1, "Radius", "Radius2")
                } else {
                    (
                        (self.shader_constants.width as f32)
                            / (self.shader_constants.height as f32),
                        1.0,
                        "Width",
                        "Height",
                    )
                }
            };
            ui.horizontal(|ui| {
                ui.label(dim1_label);
                ui.add(
                    egui::DragValue::new(&mut params.dim1)
                        .clamp_range(0.0..=dim1_max)
                        .speed(0.01),
                );
            });
            if spec.num_dims > 1 {
                ui.horizontal(|ui| {
                    ui.label(dim2_label);
                    ui.add(
                        egui::DragValue::new(&mut params.dim2)
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

fn rotate(p: &Vec2, angle: f32) -> Vec2 {
    p.rotate(Vec2::from_angle(angle))
}
