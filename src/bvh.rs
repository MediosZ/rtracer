use std::rc::Rc;
use crate::{rand_i32, Aabb, HitRecord, Hittable, Interval, Ray};
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}


impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval) {
            return None;
        }
        let hit_left = self.left.hit(ray, interval);
        let max_time = if let Some(ref hit) = hit_left {
            hit.t
        } else {
            interval.max
        };
        let hit_right = self.right.hit(ray, &Interval::new(interval.min, max_time));
        // let right first
        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

impl BVHNode {
    pub fn new(src_objects: Vec<Rc<dyn Hittable>>) -> Self {
        let (left, right) = Self::generate(src_objects);
        let bbox = Aabb::new_from_aabb(left.bounding_box(), right.bounding_box());
        // dbg!(&bbox);
        Self {
            left,
            right,
            bbox
        }
    }

    fn generate(src_objects: Vec<Rc<dyn Hittable>>) -> (Rc<dyn Hittable>, Rc<dyn Hittable>) {
        let mut objects = src_objects;
        let axis = rand_i32(0, 3);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };
        let object_span = objects.len();
        if object_span == 1 {
            let node = objects[0].clone();
            (node.clone(), node)
        } else if object_span == 2 {
            let node1: Rc<dyn Hittable> = objects[0].clone();
            let node2: Rc<dyn Hittable> = objects[1].clone(); 
            if comparator(&node1, &node2) {
                (node1.clone(), node2)
            } else {
                (node2.clone(), node1)
            }
        } else {
            objects.sort_by(|a, b| {
                if comparator(a, b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            let mid = object_span / 2;
            let left = BVHNode::new(objects.drain(0..mid).collect());
            let right = BVHNode::new(objects);
            // (Rc::from(left), Rc::from(right))
            // unimplemented!()
            (Rc::from(left), Rc::from(right))
        }
    }

    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> bool {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();
        box_a.axis(axis).min < box_b.axis(axis).min
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> bool {
        Self::box_compare(a, b, 2)
    }
}
