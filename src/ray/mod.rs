mod colour;

pub use colour::*;

use nalgebra::{Unit, Vector3};
use std::ops::Mul;

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
