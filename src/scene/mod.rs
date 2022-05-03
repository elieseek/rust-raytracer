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

#[derive(Clone)]
pub struct Scene {
    pub world: Object,
    pub materials: Vec<MaterialKind>,
}

impl Scene {
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
    #[allow(dead_code)]
    pub fn trace_ray(&self, ray: &Ray, depth: u64) -> Vector3<f64> {
        if depth == 0 {
            println!("Ray hit max depth.");
            return vector![0.0, 0.0, 0.0];
        }
        println!(
            "Tracing Ray {} with o: {}, d: {}",
            depth,
            ray.origin(),
            ray.direction().into_inner()
        );
        match self.world.hit(ray, 0.001, f64::INFINITY) {
            Some(hit) => {
                println!("Ray {} hit {:?}.", depth, hit);
                let scatter = self.materials[hit.material_handle].scatter(ray, &hit);
                if let Some(r) = scatter.ray {
                    self.trace_ray(&r, depth - 1)
                        .component_mul(&scatter.attenuation)
                } else {
                    vector![0.0, 0.0, 0.0]
                }
            }
            None => {
                println!("No hit.");
                let t = 0.5 * (ray.direction().y() + 1.0);
                (1.0 - t) * vector![1.0, 1.0, 1.0] + t * vector![0.5, 0.7, 1.0]
            }
        }
    }
}
