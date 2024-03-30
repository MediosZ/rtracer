#![allow(dead_code)]
use rtracer::{
    rand, rand_range, Camera, Color, Dielectric, HittableList, Lambertian, Metal, Point3, Sphere, BVHNode,
    Vec3,
};
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = rand();
            let center = Point3::new(i as f64 + 0.9 * rand(), 0.2, j as f64 + 0.9 * rand());
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mat = Rc::new(Lambertian::new(Color::random() * Color::random()));
                    let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_moving(center, center2, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    let mat = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                } else {
                    let mat = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, mat)));
                }
            }
        }
    }
    // let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // // let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    // // let mat_center = Rc::new(Dielectric::new(1.5));

    let mat_1: Rc<Dielectric> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        mat_1,
    )));

    let mat_2: Rc<Lambertian> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat_2,
    )));

    let mat_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        mat_3,
    )));

    let world = HittableList::new_from_node(Rc::new(BVHNode::new(world.into())));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let fov = 20.0;
    let max_depth = 50;
    let sample_per_pixel = 100;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let cam = Camera::new(
        aspect_ratio,
        image_width,
        lookfrom,
        lookat,
        vup,
        sample_per_pixel,
        max_depth,
        fov,
        defocus_angle,
        focus_dist,
    );
    cam.render(&world);
}
