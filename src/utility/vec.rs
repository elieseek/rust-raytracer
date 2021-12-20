use std::ops::Mul;

use nalgebra::{Unit, Vector3};

pub trait NamedField<T: Copy> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn z(&self) -> T;
}

impl<T: Copy> NamedField<T> for Vector3<T> {
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

impl<T: Copy> NamedField<T> for Unit<Vector3<T>> {
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

pub fn refract(uv: Vector3<f64>, n: Unit<Vector3<f64>>, rri: f64) -> Vector3<f64> {
    let cos_theta = (-n.dot(&uv)).clamp(-1.0, 1.0);
    let r_out_perp = rri * (uv + n.mul(cos_theta));
    let r_out_parallel = n.mul(-((1.0 - r_out_perp.norm_squared()).abs().sqrt()));
    r_out_perp + r_out_parallel
}
