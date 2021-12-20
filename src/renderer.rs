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

pub struct Renderer<'a> {
    accumulated_buffer: Vec<Vector3<u64>>,
    output_buffer: Vec<Vector3<u8>>,
    accumulated_samples: usize,
    camera: Camera,
    image: Image,
    scene: Scene<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(camera: Camera, scene: Scene<'a>, image: Image) -> Self {
        let accumulated_buffer = vec![vector![0, 0, 0]; (image.height * image.width) as usize];
        let output_buffer = vec![vector![0, 0, 0]; (image.height * image.width) as usize];
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

    fn set_output_buffer(&mut self) {
        self.output_buffer
            .par_iter_mut()
            .zip(self.accumulated_buffer.par_iter())
            .for_each(|(pixel, acc)| {
                let mut scaled = acc.cast::<f64>().div(self.accumulated_samples as f64);
                scaled.apply(|v| {
                    utility::clamp(255.0 * v.sqrt(), 0.0, 255.0);
                });
                *pixel = vector![scaled.x() as u8, scaled.y() as u8, scaled.z() as u8];
            });
    }

    pub fn get_image_buffer(&mut self) -> Option<image::ImageBuffer<Rgb<u8>, Vec<u8>>> {
        self.set_output_buffer();
        let mut buffer = Vec::new();
        self.output_buffer.iter().for_each(|v| {
            buffer.push(v.x());
            buffer.push(v.y());
            buffer.push(v.z());
        });
        image::ImageBuffer::from_raw(self.image.width as u32, self.image.height as u32, buffer)
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
