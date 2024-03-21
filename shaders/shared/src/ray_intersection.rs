use spirv_std::glam::{vec2, vec3, Vec2, Vec2Swizzles, Vec3, Vec3Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;

pub fn ray_intersect_sphere(ro: Vec3, rd: Vec3, p: Vec3, r: f32) -> bool {
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

pub fn ray_intersect_hemisphere(ro: Vec3, rd: Vec3, p: Vec3, n: Vec3, ra: f32) -> f32 {
    let oc = ro - p;
    let b = oc.dot(rd);
    let qc = oc - b * rd;
    let h = ra * ra - qc.dot(qc);
    if h < 0.0 {
        0.0
    } else {
        let h = h.sqrt();
        let v = vec2(-b - h, -b + h);
        if (ro + rd * v.x - p).dot(n) > 0.0 {
            v.x
        } else if (ro + rd * v.y - p).dot(n) > 0.0 {
            ((p - ro) / rd).dot(n)
        } else {
            0.0
        }
    }
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

pub fn ray_intersect_capsule(ro: Vec3, rd: Vec3, pa: Vec3, pb: Vec3, r: f32) -> f32 {
    let ba = pb - pa;
    let oa = ro - pa;
    let baba = ba.dot(ba);
    let bard = ba.dot(rd);
    let baoa = ba.dot(oa);
    let rdoa = rd.dot(oa);
    let oaoa = oa.dot(oa);
    let a = baba - bard * bard;
    let b = baba * rdoa - baoa * bard;
    let c = baba * oaoa - baoa * baoa - r * r * baba;
    let h = b * b - a * c;
    if h >= 0.0 {
        let t = (-b - h.sqrt()) / a;
        let y = baoa + t * bard;
        // body
        if y > 0.0 && y < baba {
            return t;
        }
        // caps
        let oc = if y <= 0.0 { oa } else { ro - pb };
        let b = rd.dot(oc);
        let c = oc.dot(oc) - r * r;
        let h = b * b - c;
        if h > 0.0 {
            return -b - h.sqrt();
        }
    }
    -1.0
}
