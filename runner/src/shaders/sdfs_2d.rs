use bytemuck::Zeroable;
use glam::{vec2, Vec2};
use shared::push_constants::sdfs_2d::{Params, ShaderConstants, Shape};
use shared::PI;
use std::time::{Duration, Instant};
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

#[derive(Clone, Copy)]
pub struct Options {
    pub shape: Shape,
    pub can_drag: bool,
    pub is_dragging: bool,
    pub params: Params,
}

impl Options {
    pub fn new() -> Self {
        Self {
            shape: Shape::Circle,
            can_drag: false,
            is_dragging: false,
            params: Params {
                radius: 0.25,
                width: 0.5,
                height: 0.3,
                x0: 0.0,
                y0: 0.0,
                x1: 0.1,
                y1: 0.2,
                x2: -0.4,
                y2: 0.3,
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
    can_drag: Option<usize>,
    drag_point: Option<usize>,
    points: [Vec2; 3],
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
        if let Some(i) = self.drag_point {
            self.points[i] = rotate(&self.from_pixels(self.cursor), self.rotation);
        } else {
            self.can_drag = self.points.iter().position(|p| {
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

    fn update(&mut self, width: u32, height: u32, options: &mut crate::shaders::Options) {
        let options = &mut options.sdfs_2d;
        options.can_drag = self.can_drag.is_some();
        options.is_dragging = self.drag_point.is_some();
        self.options = *options;
        self.elapsed = self.start.elapsed();

        self.shader_constants = ShaderConstants {
            width,
            height,
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
                ..self.options.params
            },
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }
}

impl Controller {
    fn from_pixels(&self, p: Vec2) -> Vec2 {
        let p = vec2(p.x, -p.y);
        (p - 0.5
            * vec2(
                self.shader_constants.width as f32,
                -(self.shader_constants.height as f32),
            ))
            / self.shader_constants.height as f32
    }
}

fn rotate(p: &Vec2, angle: f32) -> Vec2 {
    p.rotate(Vec2::from_angle(angle))
}
