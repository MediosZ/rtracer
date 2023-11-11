#![allow(dead_code)]
use rtracer::color::{write_color, Color};
use rtracer::vec3::Vec3;
fn gen_ppm() {
    let width = 256;
    let height = 256;
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
    for i in 0..height {
        eprintln!("Lines remaining: {}", height - i);
        for j in 0..width {
            write_color(&Color::new(
                j as f64 / (width - 1) as f64,
                i as f64 / (height - 1) as f64,
                0.0,
            ));
        }
    }
    eprintln!("DONE");
}

fn main() {
    gen_ppm();
}
