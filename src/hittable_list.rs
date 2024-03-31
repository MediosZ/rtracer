use crate::{HitRecord, Hittable, Interval, Aabb};
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
        Self { objects: vec![], bbox: Aabb::default()}
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
    fn hit(&self, ray: &crate::ray::Ray, interval: &Interval) -> Option<crate::hittable::HitRecord> {
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
