mod hittable;
mod material;
mod ray;
mod renderer;
mod scene;
mod utility;

use std::time::Duration;

use hittable::{HittableList, Object, Sphere};
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
    let mut runtime = Duration::new(0, 0);
    for s in 0..samples {
        runtime += renderer.render();
        print!(
            "\rProgress: {:.2}%",
            100.0 * (s as f64 / (samples as f64 - 1.0))
        );
    }
    println!(
        "\nTotal time: {}s\nAverage time per sample: {}ms",
        runtime.as_secs(),
        (runtime / (samples as u32)).as_millis()
    );
    renderer.save_image("output/image.png");
}
