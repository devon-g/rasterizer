#![allow(dead_code)]

extern crate nalgebra_glm as glm;
extern crate sdl2;

mod color;
mod models;
mod rendering;

use crate::models::model::{Instance, Model};
use crate::rendering::canvas::Canvas;
use crate::rendering::renderer::Renderer;
use crate::rendering::scene::Scene;
use crate::rendering::viewport::Viewport;
use crate::sdl2::event::Event;
use crate::sdl2::keyboard::Keycode;
use crate::sdl2::EventPump;
use nalgebra_glm::Vec4;
use std::cell::RefCell;
use std::rc::Rc;

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
    let vp = Viewport::new(12.80, 7.20, 6.0, &canvas);
    let mut renderer = Renderer::new(canvas, vp);

    // We need a scene to hold the models we want to render
    let mut scene: Scene = Scene::new();

    // Generate default model
    let cube: Rc<Model> = Rc::new(models::model::default_cube());

    // Create an instance of our model
    let cube0: Rc<RefCell<Instance>> = Rc::new(RefCell::new(Instance::new(
        Rc::clone(&cube),
        &Vec4::new(1.0, 1.0, 1.0, 0.0),
        &Vec4::new(0.0, 0.0, 0.0, 0.0),
        &Vec4::new(0.0, 0.0, 10.0, 0.0),
    )));

    // Add my instance to the scene and render the scene
    scene.add_instance(Rc::clone(&cube0));

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

        // Get rid of previous buffer
        renderer.canvas.clear(color::BLACK);
        // Choose scene to render
        renderer.render_scene(&scene);
        // Push latest buffer to the screen
        renderer.canvas.present();

        // Cap main loop roughly at 144 iterations per second
        std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / 144));
    }
}
