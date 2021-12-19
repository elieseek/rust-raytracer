use crate::ray::Ray;
use nalgebra::{vector, Unit, Vector3};
use std::ops::Mul;

pub struct Camera {
    // aspect_ratio: f64,
    // viewport_height: f64,
    // viewport_width: f64,
    // focal_length: f64,
    pub origin: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub lower_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = vector![0.0, 0.0, 0.0];
        let horizontal = vector![viewport_width, 0.0, 0.0];
        let vertical = vector![0.0, viewport_height, 0.0];
        let lower_left_corner =
            origin - horizontal * 0.5 - vertical * 0.5 - vector![0.0, 0.0, focal_length];

        Camera {
            // aspect_ratio,
            // viewport_height,
            // viewport_width,
            // focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: Unit::new_normalize(
                self.lower_left_corner + self.horizontal.mul(u) + self.vertical.mul(v),
            ),
        }
    }
}
