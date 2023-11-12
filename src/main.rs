#![allow(dead_code)]
use rtracer::{Camera, Color, Dielectric, HittableList, Lambertian, Metal, Point3, Sphere, Vec3};

fn main() {
    let mut world = HittableList::new();
    let mat_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let mat_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    // let mat_center = Box::new(Dielectric::new(1.5));
    let mat_left: Box<Dielectric> = Box::new(Dielectric::new(1.5));
    let mat_left2: Box<Dielectric> = Box::new(Dielectric::new(1.5));
    let mat_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

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
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        mat_left2,
    )));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let fov = 20.0;
    let max_depth = 10;
    let sample_per_pixel = 50;
    let lookfrom = Point3::new(-2.0, 2.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let cam = Camera::new(
        aspect_ratio,
        image_width,
        lookfrom,
        lookat,
        vup,
        sample_per_pixel,
        max_depth,
        fov,
    );
    cam.render(&world);
}
