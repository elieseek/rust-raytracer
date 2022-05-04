mod hittable;
mod material;
mod ray;
mod renderer;
mod scene;
mod utility;

use fltk::{
    app, button::Button, enums, frame::Frame, image::PngImage, misc::Progress, prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme, SchemeType, WidgetScheme};
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
    let image_width: i32 = 1920;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
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
    let img = Image::new(aspect_ratio, image_width as u64, samples, max_depth);
    let mut renderer = Renderer::new(cam, scene, img);

    let app = app::App::default();

    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();

    let colour_theme = ColorTheme::new(color_themes::DARK_THEME);
    colour_theme.apply();

    let mut wind = Window::default()
        .with_pos(100, 100)
        .with_size(1920, 100 + 1080)
        .with_label("Raytracer");

    let mut image_frame = Frame::default().with_size(1920, 1080);

    let controls_frame = Frame::default()
        .below_of(&image_frame, 25)
        .with_size(1920, 75);

    let mut save_button = Button::default()
        .with_size(100, 40)
        .left_of(&controls_frame, -150)
        .with_label("Save");
    save_button.set_color(fltk::enums::Color::Dark2);
    save_button.deactivate();

    let mut progress_bar = Progress::default()
        .with_size(300, 40)
        .right_of(&save_button, 40);
    progress_bar.set_selection_color(fltk::enums::Color::from_rgb(119, 130, 247));
    progress_bar.set_minimum(0.0);
    progress_bar.set_maximum((samples - 1) as f64);
    progress_bar.set_value(0.0);

    wind.set_icon(Some(PngImage::load("ico/ferris.png").unwrap()));
    wind.make_resizable(true);
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    std::thread::spawn(move || {
        for sample in 0..samples {
            renderer.render();
            renderer.set_output_buffer();
            let buffer = renderer.get_image_buffer().unwrap();
            s.send(Message::Rendered(RenderSample::new(buffer, sample as f64)));
        }
        s.send(Message::RenderCompleted(Box::new(renderer.clone())));
    });

    while app.wait() {
        match r.recv() {
            Some(Message::Rendered(render_sample)) => {
                let buffer = render_sample.render;
                let sample = render_sample.sample;
                let flipped_buffer = image::imageops::flip_vertical(&buffer);
                let mut image = fltk::image::RgbImage::new(
                    &flipped_buffer,
                    image_width,
                    image_height,
                    enums::ColorDepth::Rgb8,
                )
                .unwrap();
                progress_bar.set_value(sample);
                progress_bar.set_label(&format!("Sample: {}", sample));
                image_frame.draw(move |f| {
                    image.scale(f.width(), f.height(), false, true);
                    image.draw(f.x(), f.y(), f.w(), f.height());
                });
                image_frame.redraw();
            }

            Some(Message::RenderCompleted(mut renderer)) => {
                progress_bar.set_label("Done!");
                save_button.activate();
                save_button.set_callback(move |_ev| renderer.save_image("output/image.png"));
            }

            None => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Rendered(RenderSample),
    RenderCompleted(Box<Renderer>),
}

#[derive(Debug, Clone)]
pub struct RenderSample {
    render: image::ImageBuffer<Rgb<u8>, Vec<u8>>,
    sample: f64,
}

impl RenderSample {
    pub fn new(render: image::ImageBuffer<Rgb<u8>, Vec<u8>>, sample: f64) -> Self {
        RenderSample { render, sample }
    }
}
