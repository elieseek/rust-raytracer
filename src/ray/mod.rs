mod colour;
mod vec;

pub use colour::*;
pub use vec::NamedField;

use crate::hittable::Hittable;
use crate::utility::Random;
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

pub fn ray_colour(ray: &Ray, world: &dyn Hittable, depth: i64) -> Vector3<f64> {
    let mut rng = Random::new();
    if depth <= 0 {
        return vector![0.0, 0.0, 0.0];
    }
    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit) => {
            let target = rng.random_unit_vec().add(hit.normal.add(hit.point));

            ray_colour(
                &Ray {
                    origin: hit.point,
                    direction: Unit::new_normalize(target - hit.point),
                },
                world,
                depth - 1,
            )
            .mul(0.5)
        }
        None => {
            let t = 0.5 * (ray.direction().y() + 1.0);
            (1.0 - t) * vector![1.0, 1.0, 1.0] + t * vector![0.5, 0.7, 1.0]
        }
    }
}
