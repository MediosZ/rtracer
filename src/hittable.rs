use std::rc::Rc;

use crate::{Aabb, Interval, Material, Point3, Ray, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Point3, normal: Vec3, t: f64, mat: Rc<dyn Material>, u: f64, v: f64) -> Self {
        let front_face = ray.dir().dot(&normal) < 0.0;
        Self {
            point,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
            mat,
            u,
            v,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
