use crate::model::Vertex;
use crate::window::UserEvent;
use bytemuck::Zeroable;
use egui::{Color32, Context, Rect, RichText, Sense, Stroke, Ui};
use glam::{vec2, vec3, Quat, Vec2, Vec3, Vec3Swizzles};
use shared::push_constants::spherical_harmonics_shape::{ShaderConstants, Variant};
use std::{
    f32::consts::{PI, TAU},
    time::Instant,
};
use strum::IntoEnumIterator;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta},
    event_loop::EventLoopProxy,
};

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    cursor: Vec2,
    last_cursor: Vec2,
    rot: Quat,
    mouse_button_pressed: bool,
    shader_constants: ShaderConstants,
    vertices: Vec<Vertex>,
    camera: crate::camera::Camera,
    l: u32,
    m: i32,
    variant: Variant,
    negative_m: bool,
    include_time_factor: bool,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        let l = 2;
        let m = 1;

        Self {
            size,
            start: Instant::now(),
            cursor: Vec2::ZERO,
            last_cursor: Vec2::ZERO,
            rot: Quat::IDENTITY,
            mouse_button_pressed: false,
            shader_constants: ShaderConstants::zeroed(),
            vertices: create_vertices(m, l),
            camera: crate::camera::Camera {
                eye: Vec3::Z * 2.0,
                target: Vec3::ZERO,
                up: Vec3::Y,
                aspect: size.width as f32 / size.height as f32,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            l,
            m,
            variant: Variant::Real,
            negative_m: false,
            include_time_factor: false,
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
        if self.mouse_button_pressed {
            let angles = PI * (self.cursor - self.last_cursor) / self.size.height as f32;
            let e = self.camera.eye.abs();
            let p = vec3(
                angles.y * (e.y + e.z),
                angles.x * (e.x + e.z),
                angles.dot(e.yx()),
            );
            self.rot = Quat::from_scaled_axis(p).mul_quat(self.rot).normalize();
        }
        self.last_cursor = self.cursor;
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
        self.camera.eye *= scroll;
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.camera.aspect = size.width as f32 / size.height as f32;
    }

    fn update(&mut self) {
        self.shader_constants = ShaderConstants {
            rot: self.rot.into(),
            view_proj: self.camera.build_view_projection_matrix().into(),
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&mut self, ctx: &Context, ui: &mut Ui, event_proxy: &EventLoopProxy<UserEvent>) {
        for variant in Variant::iter() {
            if ui
                .radio(self.variant == variant, variant.to_string())
                .clicked()
                && self.variant != variant
            {
                self.variant = variant;
                signal_new_vertices(event_proxy);
            }
        }
        if ui
            .checkbox(&mut self.include_time_factor, "Include time factor")
            .clicked()
            && self.include_time_factor
        {
            self.start = Instant::now();
        }

        let (rect, response) = ui.allocate_at_least([220.0; 2].into(), Sense::drag());
        let l_max = 9;

        if let Some(mouse_pos) = response.interact_pointer_pos() {
            let v = ((mouse_pos - rect.left_top()) * (l_max + 1) as f32 / rect.width())
                .clamp(egui::Vec2::ZERO, egui::Vec2::splat(l_max as f32));
            let prev_l = self.l;
            let prev_m = self.m;
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
            if prev_l != self.l || prev_m != self.m {
                self.vertices = create_vertices(self.m, self.l);
                signal_new_vertices(event_proxy)
            }
        }

        let circle_radius = rect.width() / (l_max + 1) as f32 / 2.0;
        for l in 0..=l_max {
            for m in 0..=l as i32 {
                let circle_pos = rect.left_top()
                    + egui::vec2(m as f32, l as f32)
                        * ((rect.width() - circle_radius * 2.0) / l_max as f32)
                    + egui::Vec2::splat(circle_radius);
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

        ui.put(
            Rect::from_min_max(rect.min + egui::vec2(rect.width() - 150.0, 4.0), rect.max),
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

    fn vertices(&self) -> Option<&[Vertex]> {
        Some(self.vertices.as_slice())
    }
}

fn signal_new_vertices(event_proxy: &EventLoopProxy<UserEvent>) {
    if event_proxy.send_event(UserEvent::NewVerticesReady).is_err() {
        panic!("Event loop dead");
    }
}

fn create_vertices(m: i32, l: u32) -> Vec<Vertex> {
    const I_MAX: u32 = 220;
    const J_MAX: u32 = 220;
    let mut vertices = Vec::with_capacity((I_MAX * J_MAX * 6) as usize);
    for i in 0..I_MAX {
        let theta1 = PI * i as f32 / I_MAX as f32;
        let theta2 = PI * (i + 1) as f32 / I_MAX as f32;
        for j in 0..J_MAX {
            let phi1 = TAU * j as f32 / J_MAX as f32;
            let phi2 = TAU * (j + 1) as f32 / J_MAX as f32;

            let r11 = real_spherical_harmonic(m, l, theta1, phi1, 0.0);
            let p11 = from_spherical(r11.abs(), theta1, phi1);
            let r12 = real_spherical_harmonic(m, l, theta1, phi2, 0.0);
            let p12 = from_spherical(r12.abs(), theta1, phi2);
            let r21 = real_spherical_harmonic(m, l, theta2, phi1, 0.0);
            let p21 = from_spherical(r21.abs(), theta2, phi1);
            let r22 = real_spherical_harmonic(m, l, theta2, phi2, 0.0);
            let p22 = from_spherical(r22.abs(), theta2, phi2);

            vertices.push(Vertex {
                position: p11.into(),
                color: vec3(1.0, r11, -r11).into(),
            });
            vertices.push(Vertex {
                position: p12.into(),
                color: vec3(1.0, r12, -r12).into(),
            });
            vertices.push(Vertex {
                position: p22.into(),
                color: vec3(1.0, r22, -r22).into(),
            });
            vertices.push(Vertex {
                position: p11.into(),
                color: vec3(1.0, r11, -r11).into(),
            });
            vertices.push(Vertex {
                position: p21.into(),
                color: vec3(1.0, r21, -r21).into(),
            });
            vertices.push(Vertex {
                position: p22.into(),
                color: vec3(1.0, r22, -r22).into(),
            });
        }
    }
    vertices
}

use shared::complex::Complex;
fn from_spherical(r: f32, theta: f32, phi: f32) -> Vec3 {
    let (st, ct) = theta.sin_cos();
    let (sp, cp) = phi.sin_cos();
    r * vec3(sp * ct, sp * st, cp)
}
fn factorialu(n: u32) -> f32 {
    let mut x = 1.0;
    for i in 2..=n {
        x *= i as f32;
    }
    x
}

fn binomial(n: u32, k: u32) -> f32 {
    let mut x = 1.0;
    for i in 1..=k {
        x *= (n + 1 - i) as f32 / i as f32;
    }
    x
}

fn general_binomial(n: f32, k: u32) -> f32 {
    let mut x = 1.0;
    for i in 0..k {
        x *= n - i as f32;
    }
    x / factorialu(k)
}

fn legendre_polynomial(m: i32, l: u32, x: f32) -> Complex {
    fn legendre_polynomial_positive(m: u32, l: u32, x: f32) -> Complex {
        let mut sm = 0.0;
        for k in m..=l {
            sm += factorialu(k) / factorialu(k - m)
                * x.powi((k - m) as i32)
                * binomial(l, k)
                * general_binomial(((l + k) as f32 - 1.0) / 2.0, l);
        }
        let bb = Complex::new(1.0 - x * x, 0.0).powf(m as f32 / 2.0);
        (-1.0_f32).powi(m as i32) * 2.0_f32.powi(l as i32) * bb * sm
    }
    if m < 0 {
        (-1.0_f32).powi(-m) * factorialu(l + m as u32) / factorialu(l - m as u32)
            * legendre_polynomial_positive((-m) as u32, l, x)
    } else {
        legendre_polynomial_positive(m as u32, l, x)
    }
}
fn spherical_harmonic(m: i32, l: u32, theta: f32, phi: f32, time: f32) -> Complex {
    let normalization_constant = (((2 * l + 1) as f32 * factorialu(l - m as u32))
        / (4.0 * PI * factorialu(l + m as u32)))
    .sqrt();
    let angular = (Complex::I * phi * m as f32).exp();
    let lp = legendre_polynomial(m, l, theta.cos());
    normalization_constant * lp * angular * Complex::from_angle(time)
}
fn real_spherical_harmonic(m: i32, l: u32, theta: f32, phi: f32, time: f32) -> f32 {
    if m == 0 {
        spherical_harmonic(m, l, theta, phi, time).x
    } else if m > 0 {
        2.0_f32.sqrt() * spherical_harmonic(m, l, theta, phi, time).x
    } else {
        2.0_f32.sqrt() * spherical_harmonic(-m, l, theta, phi, time).y
    }
}
