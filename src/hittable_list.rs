use crate::{Aabb, HitRecord, Hittable, Interval, Material, Point3, Quad, Vec3};
use std::rc::Rc;
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
    bbox: Aabb,
}
impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }
    pub fn new_from_node(node: Rc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(node);
        list
    }
    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
        self.objects.iter().for_each(|o| {
            self.bbox = Aabb::new_from_aabb(self.bbox.clone(), o.bounding_box());
        });
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn list(&self) -> &Vec<Rc<dyn Hittable>> {
        &self.objects
    }
}

impl From<HittableList> for Vec<Rc<dyn Hittable>> {
    fn from(list: HittableList) -> Vec<Rc<dyn Hittable>> {
        list.objects
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        interval: &Interval,
    ) -> Option<crate::hittable::HitRecord> {
        let mut close_so_far = interval.max;
        let mut result: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, &Interval::new(interval.min, close_so_far)) {
                close_so_far = record.t;
                result = Some(record);
            }
        }
        result
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

pub fn create_box(a: Point3, b: Point3, mat: Rc<dyn Material>) -> HittableList {
    let mut sides = HittableList::new();
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    )));
    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat,
    )));
    sides
}
