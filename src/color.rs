use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear: f64) -> f64 {
    linear.sqrt()
}

pub fn write_color(pixel_color: &Color, sample_per_pixel: usize) {
    let r = pixel_color.x() / sample_per_pixel as f64;
    let g = pixel_color.y() / sample_per_pixel as f64;
    let b = pixel_color.z() / sample_per_pixel as f64;
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    println!(
        "{} {} {}",
        (r * 255.0) as usize,
        (g * 255.0) as usize,
        (b * 255.0) as usize
    );
}
