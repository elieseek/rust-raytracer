mod vec;

use std::f64::consts::PI;

pub use vec::*;

use nalgebra::{vector, Unit, Vector3};
use rand::{distributions::Uniform, prelude::ThreadRng, thread_rng, Rng};

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

pub fn degrees_to_radians(d: f64) -> f64 {
    d * PI / 180.0
}

pub struct Random {
    rng: ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Random { rng: thread_rng() }
    }

    pub fn random_vec(&mut self, min: f64, max: f64) -> Vector3<f64> {
        let range = Uniform::new(min, max);
        vector![
            self.rng.sample(range),
            self.rng.sample(range),
            self.rng.sample(range)
        ]
    }

    pub fn random_in_unit_sphere(&mut self) -> Vector3<f64> {
        loop {
            let p = self.random_vec(-1.0, 1.0);
            if p.dot(&p) >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vec(&mut self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.random_in_unit_sphere())
    }
}
