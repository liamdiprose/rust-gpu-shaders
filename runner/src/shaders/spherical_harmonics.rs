use bytemuck::Zeroable;
use egui::{vec2, Color32, Context, Event, Rect, RichText, Sense, Stroke, Ui, Vec2};
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
    negative_m: bool,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            start: Instant::now(),
            cursor: Vec2::ZERO,
            drag_start: Vec2::ZERO,
            drag_end: Vec2::ZERO,
            q: Quat::from_xyzw(-0.004286735, -0.18652226, -0.000813862, 0.98244107),
            zoom: 1.0,
            mouse_button_pressed: false,
            shader_constants: ShaderConstants::zeroed(),
            l: 2,
            m: 1,
            negative_m: false,
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

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&mut self, ctx: &Context, ui: &mut Ui) {
        let (rect, response) = ui.allocate_at_least([220.0; 2].into(), Sense::drag());
        let l_max = 9;
        let circle_radius = rect.width() / (l_max + 1) as f32 / 2.0;
        for l in 0..=l_max {
            for m in 0..=l as i32 {
                let circle_pos = rect.left_top()
                    + vec2(m as f32, l as f32)
                        * ((rect.width() - circle_radius * 2.0) / l_max as f32)
                    + Vec2::splat(circle_radius);
                ui.painter().circle(
                    circle_pos,
                    circle_radius,
                    if l == self.l && m == self.m {
                        Color32::DARK_GREEN
                    } else if l == self.l && m == -self.m {
                        Color32::from_rgb(0, 0x64, 0x64)
                    } else {
                        Color32::DARK_GRAY
                    },
                    Stroke::NONE,
                );
            }
        }

        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let v = ((mouse_pos - rect.left_top()) * (l_max + 1) as f32 / rect.width())
                .clamp(Vec2::ZERO, Vec2::splat(l_max as f32));
            if v.x > v.y {
                let dif = v.x - v.y;
                self.l = (v.y + (dif / 2.0)) as u32;
                self.m = (v.x - (dif / 2.0)) as i32;
            } else {
                self.l = v.y as u32;
                self.m = v.x as i32;
            }
            ctx.input(|input| {
                if input.pointer.any_pressed() {
                    self.negative_m = input.pointer.secondary_pressed();
                }
            });
            if self.negative_m {
                self.m = -self.m;
            }
        }

        ui.put(
            Rect::from_min_max(rect.min + vec2(rect.width() - 150.0, 4.0), rect.max),
            |ui: &mut Ui| {
                ui.horizontal_wrapped(|ui| {
                    let text_size = 36.0;
                    ui.spacing_mut().item_spacing *= 0.0;
                    ui.heading(RichText::new("Y").size(text_size));
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!(" {}", self.m)).size(text_size / 2.0));
                        ui.label(RichText::new(format!("{}", self.l)).size(text_size / 2.0));
                    });
                    ui.heading(RichText::new("(θ, φ)").size(text_size))
                })
                .inner
            },
        );
        ui.advance_cursor_after_rect(rect);
    }
}
