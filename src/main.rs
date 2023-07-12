pub mod ray;
pub mod vec3;

use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let center = *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let oc = r.origin() - center;
    let b = Vec3::dot(&r.direction(), &oc);
    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discriminant = (b * b) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / a;
    };
}

fn ray_color(ray: &Ray) -> Vec3 {
    let mut t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let mut N: Vec3 = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
        N = N.unit_vector();

        return Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1f32);

    return (Vec3::new(1f32, 1f32, 1f32) * (1f32 - t)) + (Vec3::new(0.5f32, 0.7f32, 1f32) * t);
}

fn main() {
    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio as f32 * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizonal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 1.0);
    let lower_left_corner: Vec3 =
        origin - horizonal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let r: Ray = Ray::new(
                &origin,
                &(lower_left_corner + horizonal * u + vertical * v - origin),
            );

            let pixel_color: Vec3 = ray_color(&r);
            pixel_color.write_color();
        }
    }
}
