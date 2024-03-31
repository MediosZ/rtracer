#![allow(dead_code)]
use rtracer::{
    create_box, rand, rand_range, texture::ImageTexture, BVHNode, Camera, CheckerTexture, Color,
    ConstantMedium, Dielectric, DiffuseLight, HittableList, Lambertian, Metal, NoiseTexture,
    Point3, Quad, RotateY, Sphere, Translate, Vec3,
};
use std::rc::Rc;

fn setup_camera(
    aspect_ratio: f64,
    image_width: usize,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    sample_per_pixel: usize,
    max_depth: usize,
    fov: f64,
    defocus_angle: f64,
    focus_dist: f64,
) -> Camera {
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
        Color::new(0.0, 0.0, 0.0),
    )
}

fn final_scene() -> (HittableList, Camera) {
    let mut boxes1 = HittableList::new();
    let ground = Rc::new(Lambertian::new_from_color(Color::new(0.48, 0.83, 0.53)));
    const BOXES_PER_SIDE: usize = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Rc::new(create_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = HittableList::new();
    world.add(Rc::new(BVHNode::new(boxes1.into())));

    let light = Rc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::new_from_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        moving_sphere_material,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));
    let boundary = Rc::new(Sphere::new(
        Point3::new(265.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    ));

    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::new_from_color(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::new_from_color(
        boundary.clone(),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));
    let emat = Rc::new(Lambertian::new(Box::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = NoiseTexture::new(0.1);
    world.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::new(Box::new(pertext))),
    )));
    let mut boxes2 = HittableList::new();
    let white = Rc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Rc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(BVHNode::new(boxes2.into())), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let cam = setup_camera(
        1.0,
        800,
        Point3::new(478.0, 278.0, -600.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        1000,
        40,
        40.0,
        0.0,
        10.0,
    );
    (world, cam)
}

fn cornell_box() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let red = Rc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343.0, 554.0, 443.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Rc::new(box1), 15.0);
    let box1 = Translate::new(Rc::new(box1), Vec3::new(265.0, 0.0, 295.0));
    let box1 = ConstantMedium::new_from_color(Rc::new(box1), 0.01, Color::new(0.0, 0.0, 0.0));
    world.add(Rc::new(box1));

    let box1 = create_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box1 = RotateY::new(Rc::new(box1), -18.0);
    let box1 = Translate::new(Rc::new(box1), Vec3::new(130.0, 0.0, 65.0));
    let box1 = ConstantMedium::new_from_color(Rc::new(box1), 0.01, Color::new(1.0, 1.0, 1.0));
    world.add(Rc::new(box1));

    let cam = setup_camera(
        1.0,
        600,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        200,
        50,
        40.0,
        0.0,
        10.0,
    );
    (
        HittableList::new_from_node(Rc::new(BVHNode::new(world.into()))),
        cam,
    )
}

fn simple_light() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let perlin_surface = Rc::new(Lambertian::new(Box::new(NoiseTexture::new(4.0))));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_surface.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_surface,
    )));
    let difflight = Rc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));
    let cam = setup_camera(
        16.0 / 9.0,
        400,
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
        20.0,
        0.0,
        10.0,
    );
    (world, cam)
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

    let cam = setup_camera(
        1.0,
        400,
        Point3::new(0.0, 0.0, 9.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
        80.0,
        0.0,
        10.0,
    );
    (world, cam)
}

fn two_perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let perlin_surface = Rc::new(Lambertian::new(Box::new(NoiseTexture::new(4.0))));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_surface.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_surface,
    )));
    let cam = setup_camera(
        16.0 / 9.0,
        400,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
        20.0,
        0.6,
        10.0,
    );
    (world, cam)
}

fn two_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let checker =
        CheckerTexture::new_with_color(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9), 0.32);
    let mat_ground = Rc::new(Lambertian::new(Box::new(checker)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        mat_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        mat_ground,
    )));
    let cam = setup_camera(
        16.0 / 9.0,
        400,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
        20.0,
        0.6,
        10.0,
    );
    (world, cam)
}

fn earth() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let earth_texture = ImageTexture::new("earthmap.jpg");
    let earth_surface = Rc::new(Lambertian::new(Box::new(earth_texture)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    let cam = setup_camera(
        16.0 / 9.0,
        400,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
        20.0,
        0.6,
        10.0,
    );
    (world, cam)
}

fn random_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
    let mat_ground = Rc::new(Lambertian::new(Box::new(CheckerTexture::new_with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
        0.32,
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
                    let mat = Rc::new(Lambertian::new_from_color(
                        Color::random() * Color::random(),
                    ));
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
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat_1)));

    let mat_2: Rc<Lambertian> = Rc::new(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        mat_2,
    )));

    let mat_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat_3)));
    let cam = setup_camera(
        16.0 / 9.0,
        400,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        100,
        50,
        20.0,
        0.6,
        10.0,
    );
    (
        HittableList::new_from_node(Rc::new(BVHNode::new(world.into()))),
        cam,
    )
}

fn main() {
    // let cam = setup_camera_quad();
    // let world = random_spheres();
    // let world = two_spheres();
    // let world = earth();
    // let world = two_perlin_spheres();
    // let (world, cam) = quads();
    // let (world, cam) = simple_light();
    // let (world, cam) = cornell_box();
    let (world, cam) = final_scene();
    cam.render(&world);
}
