mod camera;
mod image_data;

pub use camera::Camera;
pub use image_data::Image;
use nalgebra::{vector, Vector3};

use crate::{
    hittable::{Hittable, Object},
    material::{Material, MaterialKind},
    ray::Ray,
    utility::NamedField,
};

pub struct Scene<'a> {
    pub world: &'a Object,
    pub materials: &'a [MaterialKind],
}

impl Scene<'_> {
    pub fn ray_colour(&self, ray: &Ray, depth: u64) -> Vector3<f64> {
        if depth == 0 {
            return vector![0.0, 0.0, 0.0];
        }
        match self.world.hit(ray, 0.001, f64::INFINITY) {
            Some(hit) => {
                let scatter = self.materials[hit.material_handle].scatter(ray, &hit);
                if let Some(r) = scatter.ray {
                    self.ray_colour(&r, depth - 1)
                        .component_mul(&scatter.attenuation)
                } else {
                    vector![0.0, 0.0, 0.0]
                }
            }
            None => {
                let t = 0.5 * (ray.direction().y() + 1.0);
                (1.0 - t) * vector![1.0, 1.0, 1.0] + t * vector![0.5, 0.7, 1.0]
            }
        }
    }
}
