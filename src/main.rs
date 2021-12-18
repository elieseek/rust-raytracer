mod colour;

use na::vector;
use nalgebra as na;

fn main() {
    let imgx = 800;
    let imgy = 800;
    let mut img = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        print!(
            "\rProgress: {:.2}%",
            100_f64 * (y * imgx + x) as f64 / ((imgx * imgy) as f64)
        );

        let pixel_colour = vector![
            (x as f64) / ((imgx - 1) as f64),
            (imgy as f64 - y as f64) / ((imgy - 1) as f64),
            0.25_f64
        ];

        *pixel = colour::vec_to_rgb(pixel_colour);
    }

    img.save("image.png").unwrap();
}
