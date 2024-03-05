use crate::functional::tuple::*;
use core::f32::consts::TAU;
use spirv_std::glam::{vec2, IVec2, Mat2, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

// `N` is the number of extra neighboring tiles to check in each dimension
pub struct Repeat<const N: i32 = 0>;

impl<const N: i32> Repeat<N> {
    pub fn repeat_x<F>(p: Vec2, s: f32, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.x / s).round();
        let o = (p.x - s * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = id + i as f32 * o;
            let r = vec2(p.x - s * rid, p.y);
            d = d.min(sdf(r));
        }
        d
    }

    pub fn repeat_y<F>(p: Vec2, s: f32, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.y / s).round();
        let o = (p.y - s * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = id + i as f32 * o;
            let r = vec2(p.x, p.y - s * rid);
            d = d.min(sdf(r));
        }
        d
    }

    pub fn repeat_xy<F>(p: Vec2, s: Vec2, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p / s).round();
        let o = (p - s * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            for j in (0 - N)..(2 + N) {
                let rid = id + vec2(i as f32, j as f32) * o;
                let r = p - s * rid;
                d = d.min(sdf(r));
            }
        }
        d
    }
}

// `N` is the number of extra neighboring tiles to check in each dimension
pub struct RepeatLimited<const N: i32 = 0>;

impl<const N: i32> RepeatLimited<N> {
    pub fn repeat_x<F>(p: Vec2, s: f32, lima: i32, limb: i32, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.x / s).round();
        let o = (p.x - s * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = (id + i as f32 * o).clamp(-lima as f32, limb as f32);
            let r = vec2(p.x - s * rid, p.y);
            d = d.min(sdf(r));
        }
        d
    }

    pub fn repeat_y<F>(p: Vec2, s: f32, lima: i32, limb: i32, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.y / s).round();
        let o = (p.y - s * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = (id + i as f32 * o).clamp(-lima as f32, limb as f32);
            let r = vec2(p.x, p.y - s * rid);
            d = d.min(sdf(r));
        }
        d
    }

    pub fn repeat_xy<F>(p: Vec2, s: Vec2, lima: IVec2, limb: IVec2, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p / s).round();
        let o = (p - s * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            for j in (0 - N)..(2 + N) {
                let rid =
                    (id + vec2(i as f32, j as f32) * o).clamp(-lima.as_vec2(), limb.as_vec2());
                let r = p - s * rid;
                d = d.min(sdf(r));
            }
        }
        d
    }
}

/// Repeats `n` times around a circle of radius `r`
pub fn repeat_r<F>(p: Vec2, n: i32, r: f32, sdf: F) -> f32
where
    F: Fn(Vec2) -> f32,
{
    let sp = TAU / n as f32;
    let an = p.y.atan2(p.x);
    let id = (an / sp).floor();
    (-sp * id, -sp * (id + 1.0))
        .map(Mat2::from_angle)
        .map(|x| x * p - vec2(r, 0.0))
        .map(sdf)
        .min_element()
}
