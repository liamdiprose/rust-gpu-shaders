use crate::functional::tuple::*;
use core::f32::consts::TAU;
use spirv_std::glam::{vec2, Mat2, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

// `N` is the number of extra neighboring tiles to check in each dimension
pub struct Repeat<const N: i32 = 0>;

impl<const N: i32> Repeat<N> {
    pub fn repeat_x<F>(p: Vec2, factor: f32, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.x / factor).round();
        let o = (p.x - factor * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = id + i as f32 * o;
            let r = vec2(p.x - factor * rid, p.y);
            d = d.min(sdf(r));
        }
        d
    }

    pub fn repeat_y<F>(p: Vec2, factor: f32, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.y / factor).round();
        let o = (p.y - factor * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = id + i as f32 * o;
            let r = vec2(p.x, p.y - factor * rid);
            d = d.min(sdf(r));
        }
        d
    }

    pub fn repeat_xy<F>(p: Vec2, factor: Vec2, sdf: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p / factor).round();
        let o = (p - factor * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            for j in (0 - N)..(2 + N) {
                let rid = id + vec2(i as f32, j as f32) * o;
                let r = p - factor * rid;
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
