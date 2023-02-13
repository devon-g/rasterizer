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

        // Rotating square animation
        canvas.clear(Color::new(0, 0, 0));
        canvas.draw_line(
            Point::new(r * theta0.cos(), r * theta0.sin()),
            Point::new(r * theta1.cos(), r * theta1.sin()),
            Color::new(255, 255, 255),
        );
        canvas.draw_line(
            Point::new(r * theta1.cos(), r * theta1.sin()),
            Point::new(r * theta2.cos(), r * theta2.sin()),
            Color::new(255, 255, 255),
        );
        canvas.draw_line(
            Point::new(r * theta2.cos(), r * theta2.sin()),
            Point::new(r * theta3.cos(), r * theta3.sin()),
            Color::new(255, 255, 255),
        );
        canvas.draw_line(
            Point::new(r * theta3.cos(), r * theta3.sin()),
            Point::new(r * theta0.cos(), r * theta0.sin()),
            Color::new(255, 255, 255),
        );
        canvas.draw_line(
            Point::new(r * theta2.cos(), r * theta2.sin()),
            Point::new(r * theta0.cos(), r * theta0.sin()),
            Color::new(255, 255, 255),
        );
        canvas.draw_line(
            Point::new(r * theta3.cos(), r * theta3.sin()),
            Point::new(r * theta1.cos(), r * theta1.sin()),
            Color::new(255, 255, 255),
        );
        canvas.present();
        theta0 += dtheta;
        theta1 += dtheta;
        theta2 += dtheta;
        theta3 += dtheta;

        ::std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / 144));
    }
}
