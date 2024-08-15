use spirv_std::glam::*;

fn project_onto_segment(lhs: Vec2, rhs: Vec2) -> Vec2 {
    rhs * (lhs.dot(rhs) / rhs.length_squared()).clamp(0.0, 1.0)
}

pub fn disk(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

pub fn capsule_x(p: Vec2, l: f32, r: f32) -> f32 {
    p.distance(Vec2::X * p.x.clamp(-l, l)) - r
}

pub fn capsule(p: Vec2, a: Vec2, b: Vec2, r: f32) -> f32 {
    p.distance(a + project_onto_segment(p - a, b - a)) - r
}
