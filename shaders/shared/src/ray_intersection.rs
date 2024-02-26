use spirv_std::glam::{vec2, vec3, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn ray_intersects_sphere(ro: Vec3, rd: Vec3, p: Vec3, r: f32) -> bool {
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

pub fn ray_intersect_box_frame(ro: Vec3, rd: Vec3, dim: Vec2) -> bool {
    let e = vec2(-dim.x, dim.x);
    let o = vec3(-dim.y, dim.y, 0.0);
    ray_intersect_aabb(ro, rd, e.yyy(), e.yxy() + o.xzx())
        || ray_intersect_aabb(ro, rd, e.yyy(), e.xyy() + o.zxx())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.yxy() + o.zyx())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.xyy() + o.yzx())
        || ray_intersect_aabb(ro, rd, e.yyx(), e.yxx() + o.xzy())
        || ray_intersect_aabb(ro, rd, e.yyx(), e.xyx() + o.zxy())
        || ray_intersect_aabb(ro, rd, e.xxx(), e.yxx() + o.zyy())
        || ray_intersect_aabb(ro, rd, e.xxx(), e.xyx() + o.yzy())
        || ray_intersect_aabb(ro, rd, e.yyy(), e.yyx() + o.xxz())
        || ray_intersect_aabb(ro, rd, e.yxy(), e.yxx() + o.xyz())
        || ray_intersect_aabb(ro, rd, e.xyy(), e.xyx() + o.yxz())
        || ray_intersect_aabb(ro, rd, e.xxy(), e.xxx() + o.yyz())
}

pub fn ray_intersect_aabb(ro: Vec3, rd: Vec3, a: Vec3, b: Vec3) -> bool {
    let t1 = (a - ro) / rd;
    let t2 = (b - ro) / rd;
    t1.max(t2).min_element() > t1.min(t2).max_element()
}
