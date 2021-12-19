mod lambertian;
mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Vector3<f64>,
}
