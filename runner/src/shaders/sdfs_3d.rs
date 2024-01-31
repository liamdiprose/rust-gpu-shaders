use bytemuck::Zeroable;
use egui::{Context, CursorIcon};
use glam::{vec2, vec3, Mat3, Vec2, Vec3, Vec3Swizzles};
use shared::push_constants::sdfs_3d::{sdf_shape, Params, ShaderConstants, Shape};
use shared::sdf_3d as sdf;
use shared::PI;
use std::time::{Duration, Instant};
use strum::IntoEnumIterator;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

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
            shape: Shape::Sphere,
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
    mouse_button_pressed: u32,
    can_drag: Option<usize>,
    drag_point: Option<usize>,
    points: [Vec2; 3],
    options: Options,
    shader_constants: ShaderConstants,
    drag_start: Vec2,
    drag_end: Vec2,
    drag: Vec2,
    camera: Vec2,
    scroll: f32,
    slice_z: f32,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            size,
            start: Instant::now(),
            elapsed: Duration::ZERO,
            cursor: Vec2::ZERO,
            rotation: 1.0,
            mouse_button_pressed: 0,
            can_drag: None,
            drag_point: None,
            points: [vec2(0.0, 0.0), vec2(0.1, 0.2), vec2(-0.4, 0.3)],
            options: Options::new(),
            shader_constants: ShaderConstants::zeroed(),
            drag_start: Vec2::ZERO,
            drag_end: Vec2::ZERO,
            drag: Vec2::ZERO,
            camera: vec2(0.2, -0.1) * size.height as f32,
            scroll: 0.0,
            slice_z: 0.0,
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => match state {
                ElementState::Pressed => {
                    self.mouse_button_pressed |= 1;
                }
                ElementState::Released => {
                    self.mouse_button_pressed &= !1;
                }
            },
            MouseButton::Right => {
                match state {
                    ElementState::Pressed => {
                        self.mouse_button_pressed |= 1 << 2;
                    }
                    ElementState::Released => {
                        self.drag = self.drag_start - self.drag_end;
                        self.mouse_button_pressed &= !(1 << 2);
                    }
                }

                self.drag_start = self.cursor;
                self.drag_end = self.cursor;
            }
            _ => {}
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
        // if self.mouse_button_pressed & 1 != 0 {
        //     self.slice_z = self.from_pixels(self.cursor).x;
        // }
        if self.mouse_button_pressed & (1 << 2) != 0 {
            self.drag_end = self.cursor;
        }
    }

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.scroll += match delta {
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
        self.slice_z += self.scroll;
        self.rotation *= self.scroll;
        self.camera += self.drag;
        self.elapsed = self.start.elapsed();

        const MAX_STEPS: u32 = 100;
        const MAX_DIST: f32 = 100.0;
        const SURF_DIST: f32 = 0.0001;
        // TODO: probably an analytical solution for this
        let cursor_3d_pos = {
            let cursor = self.from_pixels(self.cursor);
            let translate = -(self.camera + self.drag_start - self.drag_end)
                / self.shader_constants.height as f32
                * PI;
            let rm =
                Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
            let ro = rm.mul_vec3(vec3(0.0, 0.0, -1.0));
            let rd = rm.mul_vec3((cursor).extend(1.0)).normalize();
            let mut d0 = 0.0;

            for _ in 0..MAX_STEPS {
                let p = ro + rd * d0;
                let ds = slicer_sdf(p, self.slice_z).abs();
                d0 += ds;
                if d0 > MAX_DIST {
                    break;
                }
                if ds < SURF_DIST {
                    break;
                }
            }

            (ro + rd * d0).xy().extend(self.slice_z)
        };

        let params = Params {
            x0: self.points[0].x,
            y0: self.points[0].y,
            x1: self.points[1].x,
            y1: self.points[1].y,
            x2: self.points[2].x,
            y2: self.points[2].y,
            ..self.options.params[self.options.shape as usize]
        };

        let distance = if self.mouse_button_pressed & 1 != 0 {
            let sqrt_half_num_rays = 9;
            let mut dd = f32::INFINITY;

            for i in -sqrt_half_num_rays..sqrt_half_num_rays - 1 {
                for j in -sqrt_half_num_rays..sqrt_half_num_rays - 1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let ro = cursor_3d_pos;
                    let rd = vec3(i as f32, j as f32, 0.0).normalize();
                    let mut d0 = 0.0;

                    for _ in 0..MAX_STEPS {
                        let p = ro + rd * d0;
                        let ds = sdf_shape(p, self.options.shape, params).abs();
                        d0 += ds;
                        if d0 > MAX_DIST {
                            break;
                        }
                        if ds < SURF_DIST {
                            break;
                        }
                    }

                    dd = dd.min(d0);
                }
            }
            dd
        } else {
            0.0
        };

        self.shader_constants = ShaderConstants {
            width: self.size.width,
            height: self.size.height,
            time: self.elapsed.as_secs_f32(),
            cursor_x: cursor_3d_pos.x,
            cursor_y: cursor_3d_pos.y,
            cursor_z: cursor_3d_pos.z,
            mouse_button_pressed: if self.options.is_dragging {
                0
            } else {
                self.mouse_button_pressed
            },
            rotation: self.rotation,
            slice_z: self.slice_z,
            translate_x: self.camera.x + self.drag_start.x - self.drag_end.x,
            translate_y: self.camera.y + self.drag_start.y - self.drag_end.y,
            distance,

            shape: self.options.shape as u32,
            params,
        };
        self.finish_update();
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn ui(&mut self, ctx: &Context, ui: &mut egui::Ui) {
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
            if spec.num_dims > 2 {
                ui.horizontal(|ui| {
                    ui.label("Length");
                    ui.add(
                        egui::DragValue::new(&mut params.dim3)
                            .clamp_range(0.0..=1.0)
                            .speed(0.01),
                    );
                });
            }
            if self.options.shape == Shape::CuboidFrame {
                ui.horizontal(|ui| {
                    ui.label("Inner Width");
                    ui.add(
                        egui::DragValue::new(&mut params.dim4)
                            .clamp_range(0.0..=params.dim1 / 2.0)
                            .speed(0.001),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Inner Height");
                    ui.add(
                        egui::DragValue::new(&mut params.dim5)
                            .clamp_range(0.0..=params.dim2 / 2.0)
                            .speed(0.001),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Inner Length");
                    ui.add(
                        egui::DragValue::new(&mut params.dim6)
                            .clamp_range(0.0..=params.dim3 / 2.0)
                            .speed(0.001),
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
    pub fn finish_update(&mut self) {
        self.scroll = 0.0;
        self.drag = Vec2::ZERO;
    }
}

fn slicer_sdf(p: Vec3, slice_z: f32) -> f32 {
    sdf::plane(p - vec3(0.0, 0.0, slice_z), vec3(0.0, 0.0, 1.0))
}
