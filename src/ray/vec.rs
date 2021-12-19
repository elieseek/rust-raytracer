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
