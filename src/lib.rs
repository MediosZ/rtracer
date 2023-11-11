pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub const INF: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn deg2rad(deg: f64) -> f64 {
    deg / 180.0 * PI
}
