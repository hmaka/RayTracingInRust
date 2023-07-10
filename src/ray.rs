use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        return Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        };
    }

    pub fn origin(&self) -> Vec3 {
        return self.origin.clone();
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction.clone();
    }

    pub fn at(&self, t: f32) -> Vec3 {
        return self.origin + (self.direction * t);
    }
}

#[cfg(test)]
#[test]
fn new() {
    let v1: Vec3 = Vec3::new(0f32, 0f32, 0f32);
    let v2: Vec3 = Vec3::new(1f32, 1f32, 1f32);

    let r1: Ray = Ray::new(&v1, &v2);

    assert_eq!(r1.origin(), v1);
    assert_eq!(r1.direction(), v2)
}
