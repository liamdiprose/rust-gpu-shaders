#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::ray_marching::ShaderConstants;
use shared::sdf_3d as sdf;
use shared::*;
use spirv_std::glam::{vec2, vec3, Quat, Vec2Swizzles, Vec3, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

const MAX_STEPS: u32 = 200;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.0001;
const NUM_REFLECTIONS: u32 = 8;

#[repr(u32)]
#[derive(PartialEq, Default)]
pub enum Material {
    #[default]
    Matte,
    Mirror,
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
    pub fn divergent() -> Self {
        Self {
            distance: 1e20,
            material: Default::default(),
        }
    }
}

fn sdf(p: Vec3, _time: f32) -> SdfResult {
    let plane = SdfResult {
        material: Material::Matte,
        distance: sdf::plane(p - vec3(0.0, 0.0, 0.0), Vec3::Y),
    };
    let cubes = SdfResult {
        material: Material::Matte,
        distance: {
            let sdf = |p| sdf::cuboid(p, Vec3::splat(0.5));
            sdf::ops::repeat::repeat_angular_y(p - vec3(0.0, 1.0, 0.0), 12.0, 16, sdf)
        },
    };
    let frame_dim = Vec3::splat(0.05);
    let inner_dim = vec3(0.4, 0.775, 0.3);
    let mirrors = SdfResult {
        material: Material::Mirror,
        distance: {
            let sdf = |p| sdf::cuboid(p, inner_dim + (0.5 * frame_dim) - f32::EPSILON);
            sdf::ops::repeat::repeat_angular_y(p - vec3(0.0, 0.8, 0.0), 4.0, 8, sdf)
        },
    };
    let mirror_frames = SdfResult {
        material: Material::Matte,
        distance: {
            let sdf = |p| sdf::cuboid_frame(p, inner_dim, frame_dim);
            sdf::ops::repeat::repeat_angular_y(p - vec3(0.0, 0.8, 0.0), 4.0, 8, sdf)
        },
    };
    plane.union(mirrors).union(mirror_frames).union(cubes)
}

struct RayMarchResult {
    distance: f32,
    material: Material,
}

fn ray_march(ro: Vec3, rd: Vec3, time: f32) -> RayMarchResult {
    let mut d0 = 0.0;
    let mut material = Material::default();

    for _ in 0..MAX_STEPS {
        let p = ro + rd * d0;
        let result = sdf(p, time);
        let ds = result.distance;
        d0 += ds;
        if ds < SURF_DIST {
            material = result.material;
            break;
        }
        if d0 > MAX_DIST {
            break;
        }
    }

    RayMarchResult {
        distance: d0,
        material,
    }
}

fn soft_shadows(ro: Vec3, rd: Vec3, time: f32) -> f32 {
    let mut t = 0.01;
    let mut ph = 1e20;
    let mut res = 1.0;
    const K: f32 = 32.0;

    for _ in 0..MAX_STEPS {
        let p = ro + rd * t;
        let h = sdf(p, time).distance;
        let y = h * h / (2.0 * ph);
        let d = (h * h - y * y).sqrt();
        res = res.min(K * d / ((t - y).max(0.0)));
        ph = h;
        t += h;
        if h < SURF_DIST {
            res = 0.0;
            break;
        }
        if t > MAX_DIST {
            break;
        }
    }

    res
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
    dif * soft_shadows(p, light_vector, time).max(0.1)
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    output: &mut Vec4,
) {
    let uv = from_pixels(frag_coord.xy(), constants.size);
    let rot = Quat::from_rotation_y(constants.yaw) * Quat::from_rotation_x(constants.pitch);
    let mut rd = rot * vec3(uv.x, uv.y, -1.0).normalize();
    let mut ro: Vec3 = constants.pos.into();
    let mut result = ray_march(ro, rd, constants.time);
    let mut num_mirrored = 0;

    for _ in 0..NUM_REFLECTIONS {
        match result.material {
            Material::Matte => {
                break;
            }
            Material::Mirror => {
                num_mirrored += 1;
                let n = get_normal(ro + rd * result.distance, constants.time);
                ro = ro + rd * result.distance + n * SURF_DIST * 2.0;
                rd = rd - n * n.dot(rd) * 2.0;
                result = ray_march(ro, rd, constants.time);
            }
        }
    }
    let col = if result.distance >= MAX_DIST {
        let p = ro + rd * result.distance;
        vec3(0.1 * result.distance / p.y, 0.2, 0.1)
    } else {
        let dif = get_light(ro + rd * result.distance, constants.time);
        Vec3::splat(dif).lerp(vec3(0.95, 1.0, 0.95), num_mirrored as f32 * 0.0)
    };

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
