use std::ops::Neg;

use crate::{rand, Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        Some((self.albedo, Ray::new(record.point, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.dir().unit_vector().reflect(&record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + self.fuzz * Vec3::random_unit_vector(),
        );
        if scattered.dir().dot(&record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cos: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = ray.dir().unit_vector();
        let cos_theta = unit_dir.neg().dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand()
        {
            Some((
                Color::new(1.0, 1.0, 1.0),
                Ray::new(record.point, unit_dir.reflect(&record.normal)),
            ))
        } else {
            Some((
                Color::new(1.0, 1.0, 1.0),
                Ray::new(
                    record.point,
                    unit_dir.refract(&record.normal, refraction_ratio),
                ),
            ))
        }
    }
}
