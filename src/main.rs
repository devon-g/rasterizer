extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::EventPump;
use std::time::Duration;

struct Canvas {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Canvas {
    fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(Point::new(x, y)).unwrap();
        self.canvas.present();
    }
}

fn init_sdl(title: &str, width: u32, height: u32) -> (Canvas, EventPump) {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video
        .window(title, width, height)
        .position_centered()
        .build()
        .unwrap();
    (
        Canvas {
            canvas: window.into_canvas().accelerated().build().unwrap(),
        },
        context.event_pump().unwrap(),
    )
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let (mut canvas, mut event_pump) = init_sdl("Rust SDL2", WIDTH, HEIGHT);

    let mut i = 0;
    'running: loop {
        canvas.put_pixel(
            i % WIDTH as i32,
            i % HEIGHT as i32,
            Color::RGB(255, 255, 255),
        );
        i += 1;

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
