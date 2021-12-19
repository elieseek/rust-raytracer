use super::{Material, ScatterRecord};
use crate::{
    ray::Ray,
    utility::{self, Random},
};
use nalgebra::{Unit, Vector3};

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &crate::ray::Ray, hit: &crate::hittable::HitRecord) -> ScatterRecord {
        let mut rng = Random::new();
        let reflected = utility::reflect(ray_in.direction().into_inner(), hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: Unit::new_normalize(reflected + self.fuzz * rng.random_in_unit_sphere()),
        };
        ScatterRecord {
            ray: Some(scattered),
            attenuation: self.albedo,
        }
    }
}
