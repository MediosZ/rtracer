#![allow(dead_code)]
use crate::{
    deg2rad, rand, write_color, Color, Hittable, HittableList, Interval, Point3, Ray, Vec3, INF,
};

pub struct Camera {
    image_width: usize,
    image_height: usize,
    pixel00: Point3,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_per_pixel: usize,
    max_depth: usize,
    fov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        sample_per_pixel: usize,
        max_depth: usize,
        fov: f64,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let image_height = if image_height < 1 { 1 } else { image_height };
        let center = lookfrom;
        let focal_length = (lookfrom - lookat).length();
        let theta = deg2rad(fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = center - focal_length * w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Camera {
            image_height,
            image_width,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            center,
            sample_per_pixel,
            max_depth,
            fov: fov,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for i in 0..self.image_height {
            // eprintln!("Lines remaining: {}", height - i);
            for j in 0..self.image_width {
                let mut final_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let r = self.get_ray(i, j);
                    let color = self.ray_color(&r, self.max_depth, &world);
                    final_color += color;
                }
                write_color(&final_color, self.sample_per_pixel);
            }
        }
        eprintln!("DONE");
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00 + j as f64 * self.pixel_delta_u + i as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.sample_square();
        let ray_dir = pixel_sample - self.center;
        Ray::new(self.center, ray_dir)
    }

    fn sample_square(&self) -> Vec3 {
        (rand() - 0.5) * self.pixel_delta_u + (rand() - 0.5) * self.pixel_delta_v
    }

    fn ray_color(&self, ray: &Ray, depth: usize, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(record) = world.hit(ray, Interval::new(0.001, INF)) {
            if let Some((attenuation, scattered)) = record.mat.scatter(ray, &record) {
                attenuation * self.ray_color(&scattered, depth - 1, world)
            } else {
                Color::new(0.0, 0.0, 0.0)
            }
            // let direction = record.normal + Vec3::random_unit_vector();
            // 0.5 * self.ray_color(&Ray::new(record.point, direction), depth - 1, world)
        } else {
            let unit_dir = ray.dir().unit_vector();
            let a = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
