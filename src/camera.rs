use crate::{Color, Hittable, HittableList, Interval, Ray, INF};

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Camera {}
    }

    pub fn ray_color(ray: &Ray, world: &HittableList) -> Color {
        if let Some(record) = world.hit(ray, Interval::new(0.0, INF)) {
            0.5 * Color::new(
                record.normal.x() + 1.0,
                record.normal.y() + 1.0,
                record.normal.z() + 1.0,
            )
        } else {
            let unit_dir = ray.dir().unit_vector();
            let a = 0.5 * (unit_dir.y() + 1.0);

            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
