use nalgebra::{Unit, Vector3};

use super::{Material, ScatterRecord};
use crate::{hittable::HitRecord, ray::Ray, utility, utility::Random};

pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let mut rng = Random::new();
        let mut scatter_direction = hit.normal.into_inner() + rng.random_unit_vec().into_inner();
        if utility::near_zero(&scatter_direction) {
            scatter_direction = hit.normal.into_inner();
        }
        let direction = Unit::new_normalize(scatter_direction);

        Some(ScatterRecord {
            ray: Ray {
                origin: hit.point,
                direction,
            },
            attenuation: self.albedo,
        })
    }
}
