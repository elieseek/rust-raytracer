mod lambertian;
mod metal;

use enum_dispatch::enum_dispatch;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use nalgebra::Vector3;

#[enum_dispatch]
pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> ScatterRecord;
}

pub struct ScatterRecord {
    pub ray: Option<Ray>,
    pub attenuation: Vector3<f64>,
}

#[enum_dispatch(Material)]
pub enum MaterialKind {
    Diffuse(Lambertian),
    Metallic(Metal),
}
