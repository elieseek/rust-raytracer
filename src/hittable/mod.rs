pub mod hittable_list;
pub mod sphere;

use crate::ray::Ray;
use enum_dispatch::enum_dispatch;
pub use hittable_list::HittableList;
use nalgebra::{Unit, Vector3};
pub use sphere::Sphere;

#[derive(Debug)]
pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Unit<Vector3<f64>>,
    pub material_handle: usize,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn from_ray(
        ray: &Ray,
        point: Vector3<f64>,
        material_handle: usize,
        t: f64,
        outward_normal: &Unit<Vector3<f64>>,
    ) -> Self {
        let front_face = outward_normal.dot(&ray.direction()) < -1e-5;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        HitRecord {
            point,
            normal,
            material_handle,
            t,
            front_face,
        }
    }
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
#[enum_dispatch(Hittable)]
pub enum Object {
    Sphere(Sphere),
    List(HittableList),
}
