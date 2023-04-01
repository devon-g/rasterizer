#![allow(dead_code)]

extern crate nalgebra;
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
use nalgebra::{Rotation3, Scale3, Translation3, Vector3};
use std::cell::RefCell;
use std::collections::HashMap;
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

fn move_camera(viewport: &mut Viewport, dx: f32, dy: f32, dz: f32) {
    viewport.set_translation(Translation3::from(
        viewport.translation.vector
            + viewport
                .rotation
                .inverse()
                .transform_vector(&Vector3::new(-dx, dy, -dz)),
    ));
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
        Scale3::new(1.0, 1.0, 1.0),
        Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        Translation3::new(0.0, 0.0, 10.0),
    )));

    // Add my instance to the scene and render the scene
    scene.add_instance(Rc::clone(&cube0));

    let mut roll = 10.0;

    let mut pressed: HashMap<Keycode, bool> = HashMap::new();
    pressed.insert(Keycode::W, false);
    pressed.insert(Keycode::A, false);
    pressed.insert(Keycode::S, false);
    pressed.insert(Keycode::D, false);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::W | Keycode::A | Keycode::S | Keycode::D => {
                        *pressed.get_mut(&keycode.unwrap()).unwrap() = true;
                    }
                    Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {}
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode.unwrap() {
                    Keycode::W | Keycode::A | Keycode::S | Keycode::D => {
                        *pressed.get_mut(&keycode.unwrap()).unwrap() = false;
                    }
                    _ => {}
                },
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        if pressed[&Keycode::W] {
            move_camera(&mut renderer.viewport, 0.0, 0.0, 0.1);
        }
        if pressed[&Keycode::A] {
            move_camera(&mut renderer.viewport, -0.1, 0.0, 0.0);
        }
        if pressed[&Keycode::S] {
            move_camera(&mut renderer.viewport, 0.0, 0.0, -0.1);
        }
        if pressed[&Keycode::D] {
            move_camera(&mut renderer.viewport, 0.1, 0.0, 0.0);
        }

        // Animate things
        //cube0
        //    .borrow_mut()
        //    .set_rotation(Rotation3::from_euler_angles(roll, roll, 0.0));
        //roll += 0.01;

        // Turn camera left 30 degrees
        renderer
            .viewport
            .set_rotation(Rotation3::from_euler_angles(0.0, 0.0, 3.14 / 6.0));

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
