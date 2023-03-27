extern crate sdl2;

mod rendering;
mod shapes;

use rendering::canvas::Canvas;
use rendering::renderer::Renderer;
use rendering::viewport::Viewport;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use shapes::geometry::Color;
use shapes::geometry::Point3;
use crate::shapes::triangle::Triangle;

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

    let black = Color::new(0, 0, 0);
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);
    let yellow = Color::new(255, 255, 0);
    let blue = Color::new(0, 0, 255);
    let cyan = Color::new(0, 255, 255);
    let _white = Color::new(255, 255, 255);
    let _grey = Color::new(128, 128, 128);
    let purple = Color::new(123, 50, 220);

    // Prepare canvas with black background
    renderer.canvas.clear(black);

    // Drawing box object
    let vertices: Vec<Point3> = vec![
        Point3::new(1.0, 1.0, 1.0, 1.0),
        Point3::new(-1.0, 1.0, 1.0, 1.0),
        Point3::new(-1.0, -1.0, 1.0, 1.0),
        Point3::new(1.0, -1.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, -1.0, 1.0),
        Point3::new(-1.0, 1.0, -1.0, 1.0),
        Point3::new(-1.0, -1.0, -1.0, 1.0),
        Point3::new(1.0, -1.0, -1.0, 1.0)
    ];

    let triangles: Vec<Triangle> = vec![
        Triangle::new(0, 1, 2, red),
        Triangle::new(0, 2, 3, red),
        Triangle::new(4, 0, 3, green),
        Triangle::new(4, 3, 7, green),
        Triangle::new(5, 4, 7, blue),
        Triangle::new(5, 7, 6, blue),
        Triangle::new(1, 5, 6, yellow),
        Triangle::new(1, 6, 2, yellow),
        Triangle::new(4, 5, 1, purple),
        Triangle::new(4, 1, 0, purple),
        Triangle::new(2, 6, 7, cyan),
        Triangle::new(2, 7, 3, cyan),
    ];

    renderer.render_object(vertices, triangles);

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
