use spirv_std::glam::{vec2, Vec2};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

// `N` is the number of extra neighboring tiles to check in each dimension
pub struct Repeat<const N: i32 = 0>;

impl<const N: i32> Repeat<N> {
    pub fn repeat_x<F>(p: Vec2, factor: f32, f: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.x / factor).round();
        let o = (p.x - factor * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = id + i as f32 * o;
            let r = vec2(p.x - factor * rid, p.y);
            d = d.min(f(r));
        }
        d
    }

    pub fn repeat_y<F>(p: Vec2, factor: f32, f: F) -> f32
    where
        F: Fn(Vec2) -> f32,
    {
        let id = (p.y / factor).round();
        let o = (p.y - factor * id).signum();

        let mut d = f32::MAX;
        for i in (0 - N)..(2 + N) {
            let rid = id + i as f32 * o;
            let r = vec2(p.x, p.y - factor * rid);
            d = d.min(f(r));
        }
        d
    }

    pub fn repeat_xy<F>(p: Vec2, factor: Vec2, f: F) -> f32
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
                d = d.min(f(r));
            }
        }
        d
    }
}

// impl<const N: u32> Repeat<N> {
//     pub fn repeat_x(p: Vec2, factor: f32) -> [Vec2; 2 + 2 * N as usize] {
//         let id = (p.x / factor).round();
//         let o = (p.x - factor * id).signum();
//         let mut result = [Vec2::ZERO; 2 + 2 * N as usize];
//
//         for i in (0 - N as i32)..(2 + N as i32) {
//             let rid = id + i as f32 * o;
//             let r = vec2(p.x - factor * rid, p.y);
//             result[i as usize] = r;
//         }
//
//         result
//     }
//
//     pub fn repeat_y(p: Vec2, factor: f32) -> [Vec2; 2 + 2 * N as usize] {
//         let id = (p.y / factor).round();
//         let o = (p.y - factor * id).signum();
//         let mut result = [Vec2::ZERO; 2 + 2 * N as usize];
//
//         for i in (0 - N as i32)..(2 + N as i32) {
//             let rid = id + i as f32 * o;
//             let r = vec2(p.x, p.y - factor * rid);
//             result[i as usize] = r;
//         }
//
//         result
//     }
//
//     pub fn repeat_xy<F>(p: Vec2, factor: Vec2, f: F) -> f32
//     where
//         F: Fn(Vec2) -> f32,
//     {
//         let id = (p / factor).round();
//         let o = (p - factor * id).signum();
//
//         let mut d = 1e20;
//         for i in (0 - N as i32)..(2 + N as i32) {
//             for j in (0 - N as i32)..(2 + N as i32) {
//                 let rid = id + vec2(i as f32, j as f32) * o;
//                 let r = p - factor * rid;
//                 d = d.min(f(r));
//             }
//         }
//         d
//     }
// }
