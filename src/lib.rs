pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod image;
pub mod perlin;
pub mod quad;

pub use camera::Camera;
pub use color::{write_color, Color};
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use interval::Interval;
pub use material::{Dielectric, Lambertian, Material, Metal};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Point3, Vec3};
pub use aabb::Aabb;
pub use bvh::BVHNode;
pub use texture::{CheckerTexture, SolidColor, NoiseTexture, Texture};
pub use image::Image;
pub use perlin::Perlin;
pub use quad::Quad;

pub const INF: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;


pub fn deg2rad(deg: f64) -> f64 {
    deg / 180.0 * PI
}

pub fn rand() -> f64 {
    rand::random::<f64>()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand()
}

pub fn rand_i32(min: i32, max: i32) -> i32 {
    rand_range(min as f64, max as f64 + 1.0) as i32
}