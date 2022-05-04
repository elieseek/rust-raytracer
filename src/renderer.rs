use image::Rgb;
use nalgebra::{vector, Vector3};
use rand::Rng;
use rayon::prelude::*;
use std::{ops::Div, time::Duration};

use crate::{
    ray,
    scene::{Camera, Image, Scene},
    utility::{self, *},
};

#[derive(Debug, Clone)]
pub struct Renderer {
    accumulated_buffer: Vec<Vector3<u64>>,
    output_buffer: Vec<u8>,
    accumulated_samples: usize,
    camera: Camera,
    image: Image,
    scene: Scene,
}

impl Renderer {
    pub fn new(camera: Camera, scene: Scene, image: Image) -> Self {
        let accumulated_buffer = vec![vector![0, 0, 0]; (image.height * image.width) as usize];
        let output_buffer = vec![0u8; (3 * image.height * image.width) as usize];
        let accumulated_samples = 0;

        Self {
            accumulated_buffer,
            output_buffer,
            accumulated_samples,
            camera,
            scene,
            image,
        }
    }
    #[allow(dead_code)]
    pub fn trace_ray(&self, x: u64, y: u64, max_depth: u64) {
        let u = x as f64 / (self.image.width as f64 - 1.0);
        let v = (self.image.height - y) as f64 / (self.image.height as f64 - 1.0);
        let ray = self.camera.get_ray(u, v);
        self.scene.trace_ray(&ray, max_depth);
    }

    pub fn render(&mut self) -> Duration {
        let start = std::time::Instant::now();
        self.accumulated_samples += 1;
        self.accumulated_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let mut rng = rand::thread_rng();
                let x = i as u64 % self.image.width;
                let y = i as u64 / self.image.width;

                let u = (x as f64 + rng.gen::<f64>()) / (self.image.width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (self.image.height as f64 - 1.0);

                let ray = self.camera.get_ray(u, v);
                let pixel_colour = self.scene.ray_colour(&ray, self.image.max_depth);
                *pixel += ray::vec_to_vec3(pixel_colour, 1)
            });
        start.elapsed()
    }

    pub fn render_to_output_buffer(&mut self) {
        self.output_buffer
            .par_iter_mut()
            .chunks(3)
            .enumerate()
            .for_each(|(i, mut pixel)| {
                let mut rng = rand::thread_rng();
                let x = i as u64 % self.image.width;
                let y = i as u64 / self.image.width;

                let u = (x as f64 + rng.gen::<f64>()) / (self.image.width as f64 - 1.0);
                let v = (y as f64 + rng.gen::<f64>()) / (self.image.height as f64 - 1.0);

                let ray = self.camera.get_ray(u, v);
                let pixel_colour = self.scene.ray_colour(&ray, self.image.max_depth);

                let r = pixel_colour.x();
                let g = pixel_colour.x();
                let b = pixel_colour.x();

                *pixel[0] = (256.0 * clamp(r.sqrt(), 0.0, 0.999)) as u8;
                *pixel[1] = (256.0 * clamp(g.sqrt(), 0.0, 0.999)) as u8;
                *pixel[2] = (256.0 * clamp(b.sqrt(), 0.0, 0.999)) as u8;
            });
    }

    pub fn set_output_buffer(&mut self) {
        self.output_buffer
            .par_iter_mut()
            .chunks(3)
            .zip(self.accumulated_buffer.par_iter())
            .for_each(|(mut pixel, acc)| {
                let mut scaled = acc.cast::<f64>().div(self.accumulated_samples as f64);
                scaled.apply(|v| {
                    utility::clamp(255.0 * v.sqrt(), 0.0, 255.0);
                });
                *pixel[0] = scaled.x() as u8;
                *pixel[1] = scaled.y() as u8;
                *pixel[2] = scaled.z() as u8;
            });
    }

    pub fn get_image_buffer(&self) -> Option<image::ImageBuffer<Rgb<u8>, Vec<u8>>> {
        image::ImageBuffer::from_raw(
            self.image.width as u32,
            self.image.height as u32,
            self.output_buffer.clone(),
        )
    }

    pub fn get_raw_image_buffer(&self) -> Vec<u8> {
        self.output_buffer.clone()
    }

    pub fn save_image(&mut self, path: &str) {
        let img = self
            .get_image_buffer()
            .expect("Accumulated buffer was of incorrect size");
        image::imageops::flip_vertical(&img)
            .save(path)
            .expect("failed to save image");
    }
}
