#![allow(dead_code)]
use rtracer::{Camera, Color, HittableList, Lambertian, Metal, Point3, Sphere};

fn main() {
    let mut world = HittableList::new();
    let mat_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        mat_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let cam = Camera::new(aspect_ratio, image_width, 50, 10);
    cam.render(&world);
}
