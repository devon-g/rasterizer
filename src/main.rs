extern crate sdl2;

pub mod canvas;

use canvas::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;
use std::time::Duration;

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
        Canvas::new(
            window.into_canvas().accelerated().build().unwrap(),
            width as i32,
            height as i32,
        ),
        context.event_pump().unwrap(),
    )
}

fn main() {
    let (mut canvas, mut event_pump) = init_sdl("Rust SDL2", 1280, 720);

    let mut x = -canvas.width / 2;
    let mut y = canvas.height / 2;
    'running: loop {
        canvas.put_pixel(x, y, Color::RGB(255, 255, 255));
        x += 1;
        y -= 1;

        if x >= canvas.width / 2 {
            x -= canvas.width;
        }
        if y < -canvas.height / 2 {
            y += canvas.height;
        }

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

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
