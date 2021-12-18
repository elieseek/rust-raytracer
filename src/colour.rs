use na::Vector3;
use nalgebra as na;

pub fn vec_to_rgb(c: Vector3<f64>) -> image::Rgb<u8> {
    let r = (255.999 * c[(0, 0)]) as u8;
    let g = (255.999 * c[(1, 0)]) as u8;
    let b = (255.999 * c[(2, 0)]) as u8;

    image::Rgb([r, g, b])
}
