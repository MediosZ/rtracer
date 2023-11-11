#![allow(dead_code)]
use rtracer::color::{write_color, Color};
use rtracer::ray::Ray;
use rtracer::vec3::{Point3, Vec3};

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_dir = ray.dir().unit_vector();
    let a = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - *center;
    let a = ray.dir().dot(&ray.dir());
    let b = 2.0 * oc.dot(&ray.dir());
    let c = oc.dot(&oc) - radius.powi(2);
    (b.powi(2) - 4.0 * a * c) >= 0.0
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_detla_v = viewport_v / image_height as f64;
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_detla_v);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        // eprintln!("Lines remaining: {}", height - i);
        for j in 0..image_width {
            let pixel_center = pixel00 + j as f64 * pixel_delta_u + i as f64 * pixel_detla_v;
            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(pixel_center, ray_dir);
            let color = ray_color(&ray);
            write_color(&color);
        }
    }
    eprintln!("DONE");
}
