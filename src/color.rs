use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color, sample_per_pixel: i32) {
    let r = pixel_color.x() / sample_per_pixel as f64;
    let g = pixel_color.y() / sample_per_pixel as f64;
    let b = pixel_color.z() / sample_per_pixel as f64;

    println!(
        "{} {} {}",
        (r * 255.0) as i32,
        (g * 255.0) as i32,
        (b * 255.0) as i32
    );
}
