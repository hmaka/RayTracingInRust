use crate::vec3::Vec3;

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
}
