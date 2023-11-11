use crate::{
    hittable::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.dir().length_squared();
        let half_b = oc.dot(&ray.dir());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + discriminant.sqrt()) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }
        Some(HitRecord::new(
            &ray,
            ray.at(root),
            (ray.at(root) - self.center) / self.radius,
            root,
        ))
    }
}
