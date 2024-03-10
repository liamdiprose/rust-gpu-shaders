#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::ray_marching::ShaderConstants;
use shared::sdf_3d as sdf;
use shared::*;
use spirv_std::glam::{vec2, vec3, Mat3, Vec2Swizzles, Vec3, Vec4};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.01;
const NUM_REFLECTIONS: u32 = 8;

#[repr(u32)]
#[derive(PartialEq)]
pub enum Material {
    Matte,
    Mirror,
}

impl Default for Material {
    fn default() -> Self {
        Self::Matte
    }
}

pub struct SdfResult {
    pub distance: f32,
    pub material: Material,
}

impl SdfResult {
    pub fn union(self, other: Self) -> Self {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }
}

fn sdf(p: Vec3, _time: f32) -> SdfResult {
    let plane = SdfResult {
        material: Material::Matte,
        distance: sdf::plane(p - vec3(0.0, 0.0, 0.0), Vec3::Y),
    };
    let torus = SdfResult {
        material: Material::Matte,
        distance: sdf::torus(p - vec3(0.0, 1.0, 0.0), vec2(0.5, 0.2), Vec3::X),
    };
    let sphere = SdfResult {
        material: Material::Matte,
        distance: sdf::sphere(p - vec3(1.2, 1.6, -1.0), 0.3),
    };
    let mirror = SdfResult {
        material: Material::Mirror,
        distance: sdf::cuboid(p - vec3(2.0, 1.0, 0.0), vec3(0.3, 2.0, 2.4)),
    };
    let mirror2 = SdfResult {
        material: Material::Mirror,
        distance: sdf::cuboid(p - vec3(-5.0, 1.0, 0.0), vec3(0.3, 1.0, 2.4)),
    };
    plane
        .union(torus)
        .union(mirror)
        .union(mirror2)
        .union(sphere)
}

struct RayMarchResult {
    distance: f32,
    closest_distance: f32,
    material: Material,
}

fn ray_march(ro: Vec3, rd: Vec3, time: f32) -> RayMarchResult {
    let mut d0 = 0.0;
    let mut cd = MAX_DIST;
    let mut material = Material::default();

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let result = sdf(p, time);
        let ds = result.distance;
        cd = cd.min(ds);
        d0 += ds;
        if ds < SURF_DIST {
            material = result.material;
            cd = 0.0;
            break;
        }
        if d0 > MAX_DIST {
            break;
        }
    }

    RayMarchResult {
        distance: d0,
        closest_distance: cd,
        material,
    }
}

fn get_normal(p: Vec3, time: f32) -> Vec3 {
    let d = sdf(p, time).distance;
    let e = vec2(0.01, 0.0);
    let n = d - vec3(
        sdf(p - e.xyy(), time).distance,
        sdf(p - e.yxy(), time).distance,
        sdf(p - e.yyx(), time).distance,
    );
    n.normalize()
}

fn get_light(p: Vec3, time: f32) -> f32 {
    let time = time * 0.1;
    let light_pos = vec3(8.0 * time.sin(), 5.0, 8.0 * time.cos());
    let light_vector = (light_pos - p).normalize();
    let normal_vector = get_normal(p, time);
    let dif = saturate(light_vector.dot(normal_vector));
    let result = ray_march(p + normal_vector * SURF_DIST * 2.0, light_vector, time);
    if result.distance < light_pos.distance(p) {
        dif * 0.1
    } else {
        dif
    }
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let translate = -vec2(
        constants.translate_x + constants.drag_start_x - constants.drag_end_x,
        constants.translate_y + constants.drag_start_y - constants.drag_end_y,
    ) / constants.height as f32
        * PI;

    let uv = (vec2(frag_coord.x, -frag_coord.y)
        - 0.5 * vec2(constants.width as f32, -(constants.height as f32)))
        / constants.height as f32;

    let rm = Mat3::from_rotation_y(translate.x).mul_mat3(&Mat3::from_rotation_x(translate.y));
    let mut ro = rm.mul_vec3(vec3(0.0, 1.0, -constants.zoom));
    let mut rd = rm.mul_vec3(vec3(uv.x, uv.y, 1.0)).normalize();
    let mut result = ray_march(ro, rd, constants.time);

    for _ in 0..NUM_REFLECTIONS {
        match result.material {
            Material::Matte => {
                break;
            }
            Material::Mirror => {
                let n = get_normal(ro + rd * result.distance, constants.time);
                ro = ro + rd * result.distance + n * SURF_DIST * 2.0;
                rd = rd - n * n.dot(rd) * 2.0;
                result = ray_march(ro, rd, constants.time);
            }
        }
    }
    let dif = get_light(ro + rd * result.distance, constants.time);
    let c = result.closest_distance.abs().atan() / (PI / 2.0);
    let col = vec3(dif, dif, dif).lerp(vec3(1.0 - c, 0.5 - c, 0.2 - c), 0.2);

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
