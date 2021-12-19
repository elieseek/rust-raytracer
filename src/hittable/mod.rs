pub mod hittable_list;
pub mod sphere;

pub use hittable_list::HittableList;
pub use sphere::Sphere;

use crate::material::Material;
use crate::ray::Ray;
use nalgebra::{Unit, Vector3};
use std::rc::Rc;

pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Unit<Vector3<f64>>,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from_ray(
        ray: &Ray,
        point: Vector3<f64>,
        material: Rc<dyn Material>,
        t: f64,
        outward_normal: &Unit<Vector3<f64>>,
    ) -> Self {
        let front_face = outward_normal.dot(&ray.direction()) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
