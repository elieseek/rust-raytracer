use std::ops::Mul;

use nalgebra::{Unit, Vector3};

pub trait NamedField {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

impl NamedField for Vector3<f64> {
    fn x(&self) -> f64 {
        self[(0, 0)]
    }
    fn y(&self) -> f64 {
        self[(1, 0)]
    }
    fn z(&self) -> f64 {
        self[(2, 0)]
    }
}

impl NamedField for Unit<Vector3<f64>> {
    fn x(&self) -> f64 {
        self[(0, 0)]
    }
    fn y(&self) -> f64 {
        self[(1, 0)]
    }
    fn z(&self) -> f64 {
        self[(2, 0)]
    }
}

pub fn near_zero(v: &Vector3<f64>) -> bool {
    let s = 1e-8;

    v.x().abs() < s && v.y().abs() < s && v.z().abs() < s
}

pub fn reflect(v: Vector3<f64>, n: Unit<Vector3<f64>>) -> Vector3<f64> {
    v - n.mul(2.0 * v.dot(&n))
}
