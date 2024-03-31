use std::rc::Rc;

use crate::{rand, Aabb, Color, Interval, Isotropic, Material, Point3, Ray, Texture, Vec3};

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
    pub fn new(
        ray: &Ray,
        point: Point3,
        normal: Vec3,
        t: f64,
        mat: Rc<dyn Material>,
        u: f64,
        v: f64,
    ) -> Self {
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

pub struct Translate {
    offset: Vec3,
    obj: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl Translate {
    pub fn new(obj: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = obj.bounding_box() + offset;
        Self { obj, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new_with_time(ray.origin() - self.offset, ray.dir(), ray.time());
        if let Some(record) = self.obj.hit(&offset_r, interval) {
            Some(HitRecord::new(
                ray,
                record.point + self.offset,
                record.normal,
                record.t,
                record.mat,
                record.u,
                record.v,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

pub struct RotateY {
    obj: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(obj: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = obj.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Point3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        let bbox = Aabb::new_from_points(min, max);
        Self {
            obj,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let origin = ray.origin();
        let dir = ray.dir();
        let origin = Vec3::new(
            self.cos_theta * origin.x() - self.sin_theta * origin.z(),
            origin.y(),
            self.sin_theta * origin.x() + self.cos_theta * origin.z(),
        );
        let dir = Vec3::new(
            self.cos_theta * dir.x() - self.sin_theta * dir.z(),
            dir.y(),
            self.sin_theta * dir.x() + self.cos_theta * dir.z(),
        );

        let rotated_r = Ray::new_with_time(origin, dir, ray.time());

        if let Some(record) = self.obj.hit(&rotated_r, interval) {
            let point = Vec3::new(
                self.cos_theta * record.point.x() + self.sin_theta * record.point.z(),
                record.point.y(),
                -self.sin_theta * record.point.x() + self.cos_theta * record.point.z(),
            );

            let normal = Vec3::new(
                self.cos_theta * record.normal.x() + self.sin_theta * record.normal.z(),
                record.normal.y(),
                -self.sin_theta * record.normal.x() + self.cos_theta * record.normal.z(),
            );

            Some(HitRecord::new(
                ray, point, normal, record.t, record.mat, record.u, record.v,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, albedo: Box<dyn Texture>) -> Self {
        Self {
            boundary,
            phase_function: Rc::new(Isotropic::new(albedo)),
            neg_inv_density: -1.0 / density,
        }
    }
    pub fn new_from_color(boundary: Rc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            phase_function: Rc::new(Isotropic::new_from_color(color)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, &Interval::universe()) {
            if let Some(mut rec2) = self
                .boundary
                .hit(ray, &Interval::new(rec1.t + 0.0001, f64::INFINITY))
            {
                rec1.t = f64::max(rec1.t, interval.min);
                rec2.t = f64::min(rec2.t, interval.max);

                if rec1.t >= rec2.t {
                    return None;
                }

                rec1.t = f64::max(rec1.t, 0.0);

                let ray_length = ray.dir().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let point = ray.at(t);

                Some(HitRecord::new(
                    ray,
                    point,
                    Vec3::new(1.0, 0.0, 0.0),
                    t,
                    self.phase_function.clone(),
                    0.0,
                    0.0,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
