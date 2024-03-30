use crate::{Aabb, HitRecord, Hittable, Interval, Material, Point3, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<Rc<dyn Material>>,
    center_vec: Vec3,
    is_moving: bool,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::new_from_points(center - rvec, center + rvec);
        Self {
            center,
            radius,
            mat: Rc::new(mat),
            center_vec: Vec3::zero(),
            is_moving: false,
            bbox
        }
    }

    pub fn new_moving(
        center: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(center - rvec, center + rvec);
        let box2 = Aabb::new_from_points(center2 - rvec, center2 + rvec);
        let bbox = Aabb::new_from_aabb(box1, box2);
        Self {
            center,
            radius,
            mat: Rc::new(mat),
            center_vec: center2 - center,
            is_moving: true,
            bbox
        }
    }
    fn sphere_center(&self, time: f64) -> Point3 {
        self.center + self.center_vec * time
    }

    fn get_uv(&self, point: Point3) -> (f64, f64) {
        let theta = (-point.y()).acos();
        let phi = (-point.z()).atan2(point.x()) + std::f64::consts::PI;
        let u = phi / (2.0 * std::f64::consts::PI);
        let v = theta / std::f64::consts::PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<HitRecord> {
        let center = if self.is_moving {
            self.sphere_center(ray.time())
        } else {
            self.center
        };
        let oc = ray.origin() - center;
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
        let (u, v) = self.get_uv(normal);
        Some(HitRecord::new(&ray, point, normal, t, self.mat.clone(), u, v))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
