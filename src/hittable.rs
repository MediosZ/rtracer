use std::rc::Rc;

use crate::{Interval, Material, Point3, Ray, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Rc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Point3, normal: Vec3, t: f64, mat: Rc<Box<dyn Material>>) -> Self {
        let front_face = ray.dir().dot(&normal) < 0.0;
        Self {
            point: point,
            normal: if front_face { normal } else { -normal },
            t,
            front_face,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}
