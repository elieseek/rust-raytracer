use std::ops::Mul;

use nalgebra::{Unit, Vector3};

pub trait NamedField<T> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

impl<T> NamedField<T> for Vector3<T> {
    fn x(&self) -> T {
        self[(0, 0)]
    }
    fn y(&self) -> T {
        self[(1, 0)]
    }
    fn z(&self) -> T {
        self[(2, 0)]
    }
}

impl<T> NamedField<T> for Unit<Vector3<T>> {
    fn x(&self) -> T {
        self[(0, 0)]
    }
    fn y(&self) -> T {
        self[(1, 0)]
    }
    fn z(&self) -> T {
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
