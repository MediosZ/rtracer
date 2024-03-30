use crate::vec3::{Point3, Vec3};
#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self {
            origin,
            dir,
            time: 0.0,
        }
    }

    pub fn new_with_time(origin: Point3, dir: Vec3, time: f64) -> Self {
        Self { origin, dir, time }
    }
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
