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

    let expected: Vec3 = Vec3::new(4.2f32, 6.6f32, 10f32);

    assert_eq!(v3, expected);
}

#[test]
fn subtract_vectors() {
    let v1: Vec3 = Vec3::new(1f32, 1f32, 1f32);
    let v2: Vec3 = Vec3::new(4.1f32, 5.6f32, 9f32);

    let v3: Vec3 = v1 - v2;

    let expected: Vec3 = Vec3::new(2.2f32, 3.6f32, 8f32);

    assert_eq!(v3, expected);
}
