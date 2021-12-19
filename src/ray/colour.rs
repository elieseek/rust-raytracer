use crate::ray::NamedField;
use crate::utility::*;
use na::Vector3;
use nalgebra as na;

pub fn vec_to_rgb(c: Vector3<f64>, samples: i64) -> image::Rgb<u8> {
    let r = c.x();
    let g = c.y();
    let b = c.z();

    let scale = 1.0 / samples as f64;

    let ir = (256.0 * clamp(r * scale, 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp(g * scale, 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp(b * scale, 0.0, 0.999)) as u8;

    image::Rgb([ir, ig, ib])
}
