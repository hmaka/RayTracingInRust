use crate::{ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
}

impl HitRecord {
    pub fn new(p: &Vec3, normal: &Vec3, t: f32) -> HitRecord {
        HitRecord {
            p: p.clone(),
            normal: normal.clone(),
            t,
        }
    }
}

trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord);
}

pub struct Sphere {
    center: Vec3,
    radius: Vec3,
}

impl Sphere {
    fn new(center: &Vec3, radius: &Vec3) -> Sphere {
        Sphere {
            center: center.clone(),
            radius: radius.clone(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) {}
}
