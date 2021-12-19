use crate::utility::*;
use na::{vector, Vector3};
use nalgebra as na;

#[allow(dead_code)]
pub fn vec_to_rgb(c: Vector3<f64>, samples: i64) -> image::Rgb<u8> {
    let r = c.x();
    let g = c.y();
    let b = c.z();

    let scale = 1.0 / samples as f64;

    let ir = (256.0 * clamp((r * scale).sqrt(), 0.0, 0.999)) as u8;
    let ig = (256.0 * clamp((g * scale).sqrt(), 0.0, 0.999)) as u8;
    let ib = (256.0 * clamp((b * scale).sqrt(), 0.0, 0.999)) as u8;

    image::Rgb([ir, ig, ib])
}

pub fn vec_to_vec3(c: Vector3<f64>, samples: i64) -> Vector3<u64> {
    let r = c.x();
    let g = c.y();
    let b = c.z();

    let scale = 1.0 / samples as f64;

    let ir = (256.0 * clamp((r * scale).sqrt(), 0.0, 0.999)) as u64;
    let ig = (256.0 * clamp((g * scale).sqrt(), 0.0, 0.999)) as u64;
    let ib = (256.0 * clamp((b * scale).sqrt(), 0.0, 0.999)) as u64;

    vector![ir, ig, ib]
}
