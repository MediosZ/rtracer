use crate::{Aabb, HitRecord, Hittable, Material, Point3, Vec3};
use std::rc::Rc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = Vec3::cross(&u, &v);
        let normal = n.unit_vector();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);
        let bbox = Aabb::new_from_points(q, q + u + v).pad();
        Self {
            q,
            u,
            v,
            mat,
            bbox,
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        interval: &crate::Interval,
    ) -> Option<crate::hittable::HitRecord> {
        let denom = self.normal.dot(&ray.dir());
        if denom.abs() < 1e-8 {
            return None;
        };
        let t = (self.d - self.normal.dot(&ray.origin())) / denom;
        if !interval.contains(t) {
            return None;
        };

        let interection = ray.at(t);
        let planar_hitpt_vector = interection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if alpha < 0.0 || beta < 0.0 || alpha > 1.0 || beta > 1.0 {
            return None;
        };

        Some(HitRecord::new(
            ray,
            interection,
            self.normal,
            t,
            self.mat.clone(),
            alpha,
            beta,
        ))
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
