mod hittable;
mod material;
mod ray;
mod renderer;
mod scene;
mod utility;

use hittable::{HittableList, Object, Sphere};
use material::{Lambertian, MaterialKind, Metal};
use na::vector;
use nalgebra as na;
use renderer::Renderer;
use scene::{Camera, Image, Scene};

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();

    let material_ground = MaterialKind::Diffuse(Lambertian {
        albedo: vector![0.8, 0.8, 0.0],
    });
    let material_centre = MaterialKind::Diffuse(Lambertian {
        albedo: vector![0.7, 0.3, 0.3],
    });
    let material_left = MaterialKind::Metallic(Metal {
        albedo: vector![0.8, 0.8, 0.8],
    });
    let material_right = MaterialKind::Metallic(Metal {
        albedo: vector![0.8, 0.6, 0.2],
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
    // camera
    let viewport_height = 2.0;
    let focal_length = 1.0;

    let scene = Scene {
        world: &Object::List(world),
        materials: &materials,
    };
    let cam = Camera::new(aspect_ratio, viewport_height, focal_length);
    let img = Image::new(aspect_ratio, image_width, samples, max_depth);
    let mut renderer = Renderer::new(cam, scene, img);
    for _ in 0..samples {
        renderer.render();
    }
    renderer.save_image("output/image.png");

    // Debug fallback
    // let mut img = image::ImageBuffer::new(image_width, image_height);
    // let mut rng = thread_rng();
    // for (x, y, pixel) in img.enumerate_pixels_mut() {
    //     // print!(
    //     //     "\rProgress: {:.2}%",
    //     //     100_f64 * (y * image_width + x) as f64 / ((image_width * image_height) as f64)
    //     // );

    //     let mut pixel_colour = vector![0.0, 0.0, 0.0];
    //     for _ in 0..samples {
    //         let u: f64 = (x as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
    //         let v: f64 = (y as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
    //         let ray = cam.get_ray(u, v);
    //         pixel_colour += ray_colour(&ray, &world, max_depth);
    //     }
    //     *pixel = ray::vec_to_rgb(pixel_colour, samples);
    // }

    // image::imageops::flip_vertical(&img)
    //     .save("output/image.png")
    //     .unwrap();
}
