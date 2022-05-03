use crate::{ray::Ray, utility::degrees_to_radians};
use nalgebra::{Unit, Vector3};
use std::ops::Mul;

#[derive(Clone)]
pub struct Camera {
    pub origin: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub lower_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        v_up: Vector3<f64>,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = v_up.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - w;
        Camera {
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
                self.lower_left_corner + self.horizontal.mul(u) + self.vertical.mul(v)
                    - self.origin,
            ),
        }
    }
}
