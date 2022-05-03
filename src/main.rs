mod hittable;
mod material;
mod ray;
mod renderer;
mod scene;
mod utility;

use fltk::{app, enums, frame::Frame, prelude::*, window::Window};

use hittable::{HittableList, Object, Sphere};
use image::Rgb;
use material::{Lambertian, MaterialKind, Metal};
use na::vector;
use nalgebra as na;
use renderer::Renderer;
use scene::{Camera, Image, Scene};

use crate::material::Dielectric;

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u64;
    let samples = 4000;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let material_ground = MaterialKind::Diffuse(Lambertian {
        albedo: vector![0.8, 0.8, 0.0],
    });
    let material_centre = MaterialKind::Diffuse(Lambertian {
        albedo: vector![0.1, 0.2, 0.5],
    });
    let material_left = MaterialKind::Dielectric(Dielectric { ri: 1.5 });
    let material_right = MaterialKind::Metallic(Metal {
        albedo: vector![0.8, 0.6, 0.2],
        fuzz: 0.0,
    });

    let materials = vec![
        material_ground,
        material_centre,
        material_left,
        material_right,
    ];

    world.add(Object::Sphere(Sphere {
        centre: vector![0.0, -100.5, -1.0],
        radius: 100.0,
        material_handle: 0,
    }));
    world.add(Object::Sphere(Sphere {
        centre: vector![0.0, -0.0, -1.0],
        radius: 0.5,
        material_handle: 1,
    }));
    world.add(Object::Sphere(Sphere {
        centre: vector![-1.0, 0.0, -1.0],
        radius: 0.5,
        material_handle: 2,
    }));
    world.add(Object::Sphere(Sphere {
        centre: vector![1.0, 0.0, -1.0],
        radius: 0.5,
        material_handle: 3,
    }));

    let scene = Scene {
        world: Object::List(world),
        materials,
    };
    let cam = Camera::new(
        vector![-2.0, 2.0, 1.0],
        vector![0.0, 0.0, -1.0],
        vector![0.0, 1.0, 0.0],
        20.0,
        aspect_ratio,
    );
    let img = Image::new(aspect_ratio, image_width, samples, max_depth);
    let mut renderer = Renderer::new(cam, scene, img);

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(
            image_width.try_into().unwrap(),
            image_height.try_into().unwrap(),
        )
        .with_label("Raytracer");
    let mut frame = Frame::default().size_of(&wind);
    wind.make_resizable(true);
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    std::thread::spawn(move || {
        for _sample in 0..samples {
            renderer.render();
            renderer.set_output_buffer();
            let buffer = renderer.get_image_buffer();
            s.send(Message::Rendered(buffer));
        }
    });

    while app.wait() {
        if let Some(Message::Rendered(Some(buffer))) = r.recv() {
            let flipped_buffer = image::imageops::flip_vertical(&buffer);
            let mut image = fltk::image::RgbImage::new(
                &flipped_buffer,
                image_width.try_into().unwrap(),
                image_height.try_into().unwrap(),
                enums::ColorDepth::Rgb8,
            )
            .unwrap();
            frame.draw(move |f| {
                image.scale(f.width(), f.height(), false, true);
                image.draw(f.x(), f.y(), f.w(), f.height());
            });
            frame.redraw();
            wind.redraw();
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Rendered(Option<image::ImageBuffer<Rgb<u8>, Vec<u8>>>),
}
