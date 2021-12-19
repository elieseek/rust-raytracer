mod colour;
mod vec;

pub use colour::*;
pub use vec::NamedField;

use crate::hittable::Hittable;
use nalgebra::{vector, Unit, Vector3};
use std::ops::{Add, Mul};

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    pub fn origin(&self) -> Vector3<f64> {
        self.origin
    }

    pub fn direction(&self) -> Unit<Vector3<f64>> {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction.mul(t)
    }
}

pub fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Vector3<f64> {
    match world.hit(ray, 0.0, f64::INFINITY) {
        Some(hit) => hit.normal.add(vector![1.0, 1.0, 1.0]).mul(0.5),
        None => {
            let t = 0.5 * (ray.direction().y() + 1.0);
            (1.0 - t) * vector![1.0, 1.0, 1.0] + t * vector![0.5, 0.7, 1.0]
        }
    }
}
