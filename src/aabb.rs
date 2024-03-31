use std::ops::Add;

use crate::{Interval, Point3, Ray, Vec3};

#[derive(Debug, Default, Clone)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn new_from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(f64::min(a.x(), b.x()), f64::max(a.x(), b.x())),
            y: Interval::new(f64::min(a.y(), b.y()), f64::max(a.y(), b.y())),
            z: Interval::new(f64::min(a.z(), b.z()), f64::max(a.z(), b.z())),
        }
    }

    pub fn new_from_aabb(a: Aabb, b: Aabb) -> Self {
        Self {
            x: Interval::new_from_intervals(a.x, b.x),
            y: Interval::new_from_intervals(a.y, b.y),
            z: Interval::new_from_intervals(a.z, b.z),
        }
    }

    pub fn axis(&self, axis: usize) -> &Interval {
        match axis {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let x = if self.x.size() >= delta {
            self.x.clone()
        } else {
            self.x.expand(delta)
        };
        let y = if self.y.size() >= delta {
            self.y.clone()
        } else {
            self.y.expand(delta)
        };
        let z = if self.z.size() >= delta {
            self.z.clone()
        } else {
            self.z.expand(delta)
        };
        Self { x, y, z }
    }

    pub fn hit(&self, ray: &Ray, interval: &Interval) -> bool {
        let mut t_min = interval.min;
        let mut t_max = interval.max;
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir()[i];
            let origin = ray.origin()[i];
            let axis = self.axis(i);
            let mut t0 = (axis.min - origin) * inv_d;
            let mut t1 = (axis.max - origin) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;
    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(
            Interval::new(self.x.min + rhs.x(), self.x.max + rhs.x()),
            Interval::new(self.y.min + rhs.y(), self.y.max + rhs.y()),
            Interval::new(self.z.min + rhs.z(), self.z.max + rhs.z()),
        )
    }
}

impl Add<&Vec3> for &Aabb {
    type Output = Aabb;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Aabb::new(
            Interval::new(self.x.min + rhs.x(), self.x.max + rhs.x()),
            Interval::new(self.y.min + rhs.y(), self.y.max + rhs.y()),
            Interval::new(self.z.min + rhs.z(), self.z.max + rhs.z()),
        )
    }
}

impl Add<&Aabb> for &Vec3 {
    type Output = Aabb;
    fn add(self, rhs: &Aabb) -> Self::Output {
        rhs + self
    }
}
