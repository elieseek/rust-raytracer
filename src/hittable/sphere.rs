use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{Unit, Vector3};
use std::ops::Mul;
use std::rc::Rc;

pub struct Sphere {
    pub centre: Vector3<f64>,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.centre;
        let half_b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - c;

        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();
            let mut root = -half_b - sqrtd;

            if t_min < root && root < t_max {
                return Some(HitRecord::from_ray(
                    ray,
                    ray.at(root),
                    Rc::clone(&self.material),
                    root,
                    &Unit::new_normalize((ray.at(root) - self.centre).mul(1.0 / self.radius)),
                ));
            }
            root = -half_b + sqrtd;
            if t_min < root && root < t_min {
                return Some(HitRecord::from_ray(
                    ray,
                    ray.at(root),
                    Rc::clone(&self.material),
                    root,
                    &Unit::new_normalize((ray.at(root) - self.centre).mul(1.0 / self.radius)),
                ));
            }
        }
        None
    }
}
