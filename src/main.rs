pub mod vec3;
use vec3::Vec3;

fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel_color: Vec3 = Vec3::new(
                (i as f32) / (image_width - 1) as f32,
                (j as f32) / (image_height - 1) as f32,
                0.25f32,
            );
            pixel_color.write_color();
        }
    }
}
