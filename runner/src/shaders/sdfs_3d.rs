use crate::window::UserEvent;
use bytemuck::Zeroable;
use egui::{Context, CursorIcon};
use glam::{vec2, Mat3, Vec2, Vec3, Vec3Swizzles};
use shared::{
    from_pixels,
    push_constants::sdfs_3d::{sdf_shape, sdf_slice, Params, ShaderConstants, Shape},
};
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

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    elapsed: Duration,
    mouse_button_pressed: u32,
    cursor: Vec2,
    prev_cursor: Vec2,
    can_drag: Option<usize>,
    drag_point: Option<usize>,
    shape: Shape,
    params: Vec<Params>,
    shader_constants: ShaderConstants,
    camera: Vec2,
    slice_z: f32,
    cursor_3d_pos: Vec3,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            start: Instant::now(),
            elapsed: Duration::ZERO,
            cursor: Vec2::ZERO,
            prev_cursor: Vec2::ZERO,
            mouse_button_pressed: 0,
            can_drag: None,
            drag_point: None,
            shape: Shape::Sphere,
            params: Shape::iter().map(|shape| shape.params()).collect(),
            shader_constants: ShaderConstants::zeroed(),
            camera: vec2(0.2, -0.1),
            slice_z: 0.0,
            cursor_3d_pos: Vec3::ZERO,
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => match state {
                ElementState::Pressed => {
                    self.drag_point = self.can_drag;
                    self.mouse_button_pressed |= 1;
                }
                ElementState::Released => {
                    self.drag_point = None;
                    self.mouse_button_pressed &= !1;
                }
            },
            MouseButton::Right => match state {
                ElementState::Pressed => {
                    self.mouse_button_pressed |= 1 << 2;
                }
                ElementState::Released => {
                    self.mouse_button_pressed &= !(1 << 2);
                }
            },
            _ => {}
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
        let num_points = self.shape.spec().num_points;
        let translate = self.camera;
        let rm = Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
        if let Some(i) = self.drag_point {
            let pc = from_pixels(self.prev_cursor, self.size.into());
            let cc = from_pixels(self.cursor, self.size.into());
            let t = rm.mul_vec3((cc - pc).extend(0.0));
            let p: Vec3 = self.params[self.shape as usize].ps[i].into();
            self.params[self.shape as usize].ps[i] = (p + t).into();
        } else if num_points > 0 {
            let ro = rm.mul_vec3(-Vec3::Z);
            let rd = rm
                .mul_vec3(from_pixels(self.cursor, self.size.into()).extend(1.0))
                .normalize();
            self.can_drag = self.params[self.shape as usize].ps[0..num_points as usize]
                .iter()
                .position(|p| ray_intersects_point(ro, rd, (*p).into(), 0.05));
        }
        if self.mouse_button_pressed & (1 << 2) != 0 {
            self.camera += PI * (self.cursor - self.prev_cursor) / self.size.height as f32;
        }
        self.prev_cursor = self.cursor;
    }

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.slice_z += match delta {
            MouseScrollDelta::LineDelta(_, y) => 0.01 * y,
            MouseScrollDelta::PixelDelta(p) => {
                0.01 * (1.0 + p.y.abs() as f32).ln() * p.y.signum() as f32
            }
        };
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size.width = size.width;
        self.size.height = size.height;
    }

    fn update(&mut self) {
        self.elapsed = self.start.elapsed();

        const MAX_STEPS: u32 = 100;
        const MAX_DIST: f32 = 100.0;
        const SURF_DIST: f32 = 0.0001;
        // TODO: probably an analytical solution for this
        self.cursor_3d_pos = {
            let cursor = from_pixels(self.cursor, self.size.into());
            let translate = self.camera;
            let rm =
                Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
            let ro = rm.mul_vec3(-Vec3::Z);
            let rd = rm.mul_vec3(cursor.extend(1.0)).normalize();
            let mut d0 = 0.0;

            for _ in 0..MAX_STEPS {
                let p = ro + rd * d0;
                let ds = sdf_slice(p, self.slice_z).abs();
                d0 += ds;
                if d0 > MAX_DIST {
                    break;
                }
                if ds < SURF_DIST {
                    break;
                }
            }
            let mut p = (ro + rd * d0).xy().extend(self.slice_z);

            let mut d = sdf_shape(p, self.shape, self.params[self.shape as usize]);
            while d > 1.0 {
                p = (p.xy() + (-p.xy()).normalize() * (d - 1.0)).extend(self.slice_z);
                d = sdf_shape(p, self.shape, self.params[self.shape as usize]);
            }

            p
        };

        self.shader_constants = ShaderConstants {
            size: self.size.into(),
            time: self.elapsed.as_secs_f32(),
            cursor: self.cursor_3d_pos.into(),
            mouse_button_pressed: if self.drag_point.is_some() {
                self.mouse_button_pressed & !1
            } else {
                self.mouse_button_pressed
            },
            slice_z: self.slice_z,
            translate: self.camera.into(),
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
            if spec.num_dims > 2 {
                ui.horizontal(|ui| {
                    ui.label("Length");
                    ui.add(
                        egui::DragValue::new(&mut params.dim.z)
                            .clamp_range(0.0..=1.0)
                            .speed(0.01),
                    );
                });
            }
            if self.shape == Shape::CuboidFrame {
                ui.horizontal(|ui| {
                    ui.label("Inner Width");
                    ui.add(
                        egui::DragValue::new(&mut params.inner_dim.x)
                            .clamp_range(0.0..=params.dim.x / 2.0)
                            .speed(0.001),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Inner Height");
                    ui.add(
                        egui::DragValue::new(&mut params.inner_dim.y)
                            .clamp_range(0.0..=params.dim.y / 2.0)
                            .speed(0.001),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Inner Length");
                    ui.add(
                        egui::DragValue::new(&mut params.inner_dim.z)
                            .clamp_range(0.0..=params.dim.z / 2.0)
                            .speed(0.001),
                    );
                });
            }
        }
    }
}

fn ray_intersects_point(ro: Vec3, rd: Vec3, p: Vec3, r: f32) -> bool {
    let v = ro - p;
    let b = 2.0 * rd.dot(v);
    let c = v.dot(v) - r * r;
    let d = b * b - 4.0 * c;
    if d > 0.0 {
        let x1 = (-b - d.sqrt()) / 2.0;
        let x2 = (-b + d.sqrt()) / 2.0;
        if x1 >= 0.0 && x2 >= 0.0 {
            return true;
        }
        if x1 < 0.0 && x2 >= 0.0 {
            return true;
        }
    }
    false
}
