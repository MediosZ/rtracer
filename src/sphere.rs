use crate::{HitRecord, Hittable, Interval, Material, Point3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat: Rc::new(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, interval: Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(&ray.dir());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let mut t = (-half_b - discriminant.sqrt()) / a;
        if !interval.surrounds(t) {
            t = (-half_b + discriminant.sqrt()) / a;
            if !interval.surrounds(t) {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;
        Some(HitRecord::new(&ray, point, normal, t, self.mat.clone()))
    }
}
