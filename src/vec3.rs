use std::{
    assert_eq, fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { x, y, z };
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
#[test]
fn make_new_vec() {
    use std::assert_eq;

    let x_cord = 16f32;
    let y_cord = 25.6f32;
    let z_cord = 100.66f32;

    let test_vec3: Vec3 = Vec3::new(x_cord, y_cord, z_cord);

    assert_eq!(test_vec3.x, x_cord);
    assert_eq!(test_vec3.y, y_cord);
    assert_eq!(test_vec3.z, z_cord)
}

#[test]
fn add_vectors() {
    let v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let v2: Vec3 = Vec3::new(4.1f32, 5.6f32, 9f32);

    let v3: Vec3 = v1 + v2;

    let expected: Vec3 = Vec3::new(5.1f32, 6.6f32, 10f32);

    assert_eq!(v3, expected);
}

#[test]
fn add_assign() {
    let mut v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let v2: Vec3 = Vec3::new(4.1f32, 5.6f32, 9f32);

    v1 += v2;

    let expected: Vec3 = Vec3::new(5.1f32, 6.6f32, 10f32);

    assert_eq!(v1, expected);
}
#[test]
fn subtract_vectors() {
    let v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let v2: Vec3 = Vec3::new(4.1f32, 5.6f32, 9f32);

    let v3: Vec3 = v2 - v1;

    let expected: Vec3 = Vec3::new(3.1f32, 4.6f32, 8f32);

    assert_eq!(v3, expected);
}

#[test]
fn subtract_assign() {
    let v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let mut v2: Vec3 = Vec3::new(4.1f32, 5.6f32, 9f32);

    v2 -= v1;

    let expected: Vec3 = Vec3::new(3.1f32, 4.6f32, 8f32);

    assert_eq!(v2, expected);
}

#[test]
fn constant_multiplication_vector() {
    let v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let c: f32 = 16f32;

    let v1 = v1 * c;
    let expected: Vec3 = Vec3::new(16f32, 16f32, 16f32);
    assert_eq!(v1, expected);
}

#[test]
fn constant_multiplication_assign() {
    let mut v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let c: f32 = 16f32;

    v1 *= c;
    let expected: Vec3 = Vec3::new(16f32, 16f32, 16f32);
    assert_eq!(v1, expected);
}

#[test]
fn const_division_vector() {
    let v1: Vec3 = Vec3::new(16f32, 16f32, 16f32);
    let c: f32 = 16f32;

    let v1 = v1 / c;
    let expected: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    assert_eq!(v1, expected);
}

#[test]
fn const_division_assign() {
    let mut v1: Vec3 = Vec3::new(16f32, 16f32, 16f32);
    let c: f32 = 16f32;

    v1 /= c;
    let expected: Vec3 = Vec3::new(1f32, 1f32, 1f32);
}

//#[test]
//fn length_squared() {
//    let v1: vec3 = vec3::new(4f32, 3f32, 8f32);
//
//    let lsqr: f32 = v1.length_squared();
//
//    assert_eq!(lsqr, 89f32);
//}
//
//#[test]
//fn length() {
//    let v1: vec3 = vec3::new(4f32, 3f32, 8f32);
//
//    let len: f32 = v1.length();
//
//    assert_eq!(len, 9.4339811f32);
//}
