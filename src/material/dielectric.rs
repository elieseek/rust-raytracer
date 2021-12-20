use super::Material;
use crate::{
    material::ScatterRecord,
    ray::Ray,
    utility::{reflect, refract},
};
use nalgebra::{vector, Unit};
use rand::Rng;

pub struct Dielectric {
    pub ri: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray,
        hit: &crate::hittable::HitRecord,
    ) -> super::ScatterRecord {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ri
        } else {
            self.ri
        };
        let unit_direction = ray_in.direction().normalize();
        let cos_theta = -hit.normal.dot(&unit_direction).clamp(-1.0, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || self.reflectance(cos_theta, refraction_ratio)
                > rand::thread_rng().gen_range(0.0..1.0)
        {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, refraction_ratio)
        };

        ScatterRecord {
            ray: Some(Ray {
                origin: hit.point,
                direction: Unit::new_normalize(direction),
            }),
            attenuation: vector![1.0, 1.0, 1.0],
        }
    }
}

impl Dielectric {
    fn reflectance(&self, cosine: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}
