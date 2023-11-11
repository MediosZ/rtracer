use crate::{Interval, Point3, Ray, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Point3, normal: Vec3, t: f64) -> Self {
        let front_face = ray.dir().dot(&normal) < 0.0;
        Self {
            point: point,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
