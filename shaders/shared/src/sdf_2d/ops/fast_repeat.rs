use spirv_std::glam::{vec2, IVec2, Vec2, Vec2Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_x(p: Vec2, s: f32) -> Vec2 {
    vec2(p.x - s * (p.x / s).round(), p.y)
}

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_y(p: Vec2, s: f32) -> Vec2 {
    vec2(p.x, p.y - s * (p.y / s).round())
}

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_xy(p: Vec2, s: Vec2) -> Vec2 {
    p - s * (p / s).round()
}

// fast space repetition by mirroring every other instance
pub fn repeat_mirrored(p: Vec2, s: f32) -> Vec2 {
    let id = (p / s).round();
    let r = p - s * id;
    vec2(
        if (id.x as i32 & 1) == 0 { r.x } else { -r.x },
        if (id.y as i32 & 1) == 0 { r.y } else { -r.y },
    )
}

// fast space repetition by mirroring every other instance
pub fn repeat_mirrored_x(p: Vec2, s: f32) -> Vec2 {
    let id = (p.x / s).round();
    let r = p.x - s * id;
    vec2(if (id as i32 & 1) == 0 { r } else { -r }, p.y)
}

// fast space repetition by mirroring every other instance
pub fn repeat_mirrored_y(p: Vec2, s: f32) -> Vec2 {
    let id = (p.y / s).round();
    let r = p.y - s * id;
    vec2(p.x, if (id as i32 & 1) == 0 { r } else { -r })
}

// the sdf must be symmetric with respect to the tile boundaries
pub fn repeat_rectangular(mut p: Vec2, size: IVec2, s: f32) -> Vec2 {
    p = (p / s).abs() - (size.as_vec2() * 0.5 - 0.5);
    p = if p.x < p.y { p.yx() } else { p };
    p.y -= p.y.round().min(0.0);
    p * s
}
