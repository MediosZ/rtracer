#![allow(dead_code)]
use rtracer::{rand, rand_range, texture::ImageTexture, BVHNode, Camera, CheckerTexture, Color, Dielectric, HittableList, Lambertian, Metal, NoiseTexture, Point3, Quad, Sphere, Vec3
};
use std::rc::Rc;


fn setup_camera(aspect_ratio:f64, image_width:usize, lookfrom:Point3, lookat:Point3, vup:Vec3, sample_per_pixel:usize, max_depth:usize, fov:f64, defocus_angle:f64, focus_dist:f64) -> Camera {
    Camera::new(
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
    )
}


fn quads() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let left_red = Rc::new(Lambertian::new_from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::new_from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Rc::new(Lambertian::new_from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Rc::new(Lambertian::new_from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new_from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let cam = setup_camera(1.0, 400, Point3::new(0.0, 0.0, 9.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 100, 50, 80.0, 0.0, 10.0);
    (world, cam)

}

fn two_perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let perlin_surface = Rc::new(Lambertian::new(Box::new(NoiseTexture::new(4.0))));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, perlin_surface.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, perlin_surface)));
    let cam = setup_camera(16.0 / 9.0, 400, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 100, 50, 20.0, 0.6, 10.0);
    (world, cam)}

fn two_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let checker = CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
        0.32,
    );
    let mat_ground = Rc::new(Lambertian::new(Box::new(checker)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, mat_ground.clone())));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, mat_ground)));
    let cam = setup_camera(16.0 / 9.0, 400, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 100, 50, 20.0, 0.6, 10.0);
    (world, cam)
}

fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let earth_texture = ImageTexture::new("earthmap.jpg");
    let earth_surface = Rc::new(Lambertian::new(Box::new(earth_texture)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface)));
    let cam = setup_camera(16.0 / 9.0, 400, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 100, 50, 20.0, 0.6, 10.0);
    (world, cam)
}

fn random_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let mat_ground = Rc::new(Lambertian::new(Box::new(CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
        0.32
    ))));

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
                    let mat = Rc::new(Lambertian::new_from_color(Color::random() * Color::random()));
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

    let mat_2: Rc<Lambertian> = Rc::new(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1)));
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
    let cam = setup_camera(16.0 / 9.0, 400, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 100, 50, 20.0, 0.6, 10.0);
    (HittableList::new_from_node(Rc::new(BVHNode::new(world.into()))), cam)
}

fn main() {
    // let cam = setup_camera_quad();
    // let world = random_spheres();
    // let world = two_spheres();
    // let world = earth();
    // let world = two_perlin_spheres();
    let (world, cam) = quads();
    cam.render(&world);
}
