use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, interval: Interval) -> Option<crate::hittable::HitRecord> {
        let mut close_so_far = interval.max;
        let mut result: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, Interval::new(interval.min, close_so_far)) {
                close_so_far = record.t;
                result = Some(record);
            }
        }
        result
    }
}
