use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f32,
}

trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord);
}
