use crate::hittable::{HitRecord, Hittable};

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn add(&mut self, obj: T) {
        self.objects.push(obj);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let mut close_so_far = t_max;
        let mut result: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = T::hit(&object, ray, t_min, close_so_far) {
                close_so_far = record.t;
                result = Some(record);
            }
        }
        result
    }
}
