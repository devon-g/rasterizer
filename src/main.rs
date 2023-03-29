extern crate nalgebra;
extern crate sdl2;

mod color;
mod models;
mod rendering;

use std::rc::Rc;
use crate::models::model::{Instance, Model};
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use crate::rendering::scene::Scene;
use crate::rendering::viewport::Viewport;
use crate::sdl2::event::Event;
use crate::sdl2::keyboard::Keycode;
use crate::sdl2::EventPump;
use nalgebra::{Point3, Rotation3, Scale3, Transform3, Translation3, Vector3};

/// Produces a [`Canvas`] and an [`EventPump`]
fn init_sdl(title: &str, width: u32, height: u32) -> (Canvas, EventPump) {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();
    (
        Canvas::new(window, width as i32, height as i32),
        context.event_pump().unwrap(),
    )
}

fn main() {
    let (canvas, mut event_pump) = init_sdl("Rust SDL2", 1280, 720);
    let vp = Viewport::new(12.80, 7.20, 12.0, &canvas);
    let mut renderer = Renderer::new(canvas, vp);

    // We need a scene to hold the models we want to render
    let mut scene: Scene = Scene::new();

    // Prepare canvas with black background
    renderer.canvas.clear(color::BLACK);

    // Generate default model
    let cube: Rc<Model> = Rc::new(models::model::default_cube());

    // Create an instance of our model
    let cube0: Instance = Instance::new(
        Rc::clone(&cube),
        Scale3::new(1.0, 1.0, 1.0),
        Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        Translation3::new(0.0, 0.0, 10.0),
    );
    let cube1: Instance = Instance::new(
        Rc::clone(&cube),
        Scale3::new(1.0, 1.0, 1.0),
        Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        Translation3::new(-3.0, 0.0, 10.0),
    );
    let cube2: Instance = Instance::new(
        Rc::clone(&cube),
        Scale3::new(1.0, 1.0, 1.0),
        Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        Translation3::new(0.0, 3.0, 10.0),
    );

    // Add my instance to the scene and render the scene
    scene.add_instance(cube0);
    scene.add_instance(cube1);
    scene.add_instance(cube2);
    renderer.render_scene(&scene);

    // Show latest buffer and cap at 144 fps
    renderer.canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / 144));
    }
}
