extern crate sdl2;

pub mod canvas;
pub mod shapes;

use crate::shapes::geometry::Color;
use crate::shapes::geometry::Point;
use canvas::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

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
    let (mut canvas, mut event_pump) = init_sdl("Rust SDL2", 1280, 720);

    let mut theta0 = 0.0f32;
    let mut theta1 = std::f32::consts::FRAC_PI_2;
    let mut theta2 = 2.0 * std::f32::consts::FRAC_PI_2;
    let mut theta3 = 3.0 * std::f32::consts::FRAC_PI_2;
    let dtheta = 0.01f32;
    let r = 100.0f32;
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

        canvas.clear(Color::new(0, 0, 0));

        // Filled triangle outlined by wireframe triangle
        canvas.draw_filled_triangle(
            Point::new(200.0, -250.0, 1.0),
            Point::new(400.0, 50.0, 1.0),
            Point::new(20.0, 250.0, 1.0),
            Color::new(0, 255, 255),
        );
        canvas.draw_triangle(
            Point::new(200.0, -250.0, 1.0),
            Point::new(400.0, 50.0, 1.0),
            Point::new(20.0, 250.0, 1.0),
            Color::new(255, 255, 255),
        );

        // Gradiant triangle
        canvas.draw_gradient_triangle(
            Point::new(0.0, 300.0, 1.0),
            Point::new(-500.0, -300.0, 0.25),
            Point::new(500.0, -300.0, 0.0),
            Color::new(123, 50, 220),
        );

        // Rotating square animation
        canvas.draw_line(
            Point::new(r * theta0.cos(), r * theta0.sin(), 1.0),
            Point::new(r * theta1.cos(), r * theta1.sin(), 1.0),
            Color::new(255, 255, 0),
        );
        canvas.draw_line(
            Point::new(r * theta1.cos(), r * theta1.sin(), 1.0),
            Point::new(r * theta2.cos(), r * theta2.sin(), 1.0),
            Color::new(255, 0, 255),
        );
        canvas.draw_line(
            Point::new(r * theta2.cos(), r * theta2.sin(), 1.0),
            Point::new(r * theta3.cos(), r * theta3.sin(), 1.0),
            Color::new(0, 255, 255),
        );
        canvas.draw_line(
            Point::new(r * theta3.cos(), r * theta3.sin(), 1.0),
            Point::new(r * theta0.cos(), r * theta0.sin(), 1.0),
            Color::new(0, 0, 255),
        );
        canvas.draw_line(
            Point::new(r * theta2.cos(), r * theta2.sin(), 1.0),
            Point::new(r * theta0.cos(), r * theta0.sin(), 1.0),
            Color::new(255, 0, 0),
        );
        canvas.draw_line(
            Point::new(r * theta3.cos(), r * theta3.sin(), 1.0),
            Point::new(r * theta1.cos(), r * theta1.sin(), 1.0),
            Color::new(0, 255, 0),
        );

        canvas.present();
        theta0 += dtheta;
        theta1 += dtheta;
        theta2 += dtheta;
        theta3 += dtheta;

        ::std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / 144));
    }
}
