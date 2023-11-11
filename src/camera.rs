use crate::{write_color, Color, Hittable, HittableList, Interval, Point3, Ray, Vec3, INF};

pub struct Camera {
    image_width: i32,
    image_height: i32,
    pixel00: Point3,
    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * image_width as f64 / image_height as f64;
        let center = Point3::new(0.0, 0.0, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            image_height,
            image_width,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            center,
        }
    }
    pub fn render(&self, world: &HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for i in 0..self.image_height {
            // eprintln!("Lines remaining: {}", height - i);
            for j in 0..self.image_width {
                let pixel_center =
                    self.pixel00 + j as f64 * self.pixel_delta_u + i as f64 * self.pixel_delta_v;
                let ray_dir = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_dir);
                let color = self.ray_color(&ray, &world);
                write_color(&color);
            }
        }
        eprintln!("DONE");
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        if let Some(record) = world.hit(ray, Interval::new(0.0, INF)) {
            0.5 * (record.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_dir = ray.dir().unit_vector();
            let a = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}