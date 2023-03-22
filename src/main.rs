extern crate sdl2;

mod shapes;
mod rendering;

use rendering::canvas::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use shapes::geometry::Color;
use shapes::geometry::Point2;
use shapes::geometry::Point3;
use rendering::viewport::Viewport;

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
    let vp = Viewport::new(12.80, 7.20, 12.0, &canvas);

    let black = Color::new(0, 0, 0);
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);
    let yellow = Color::new(255, 255, 0);
    let blue = Color::new(0, 0, 255);
    let cyan = Color::new(0, 255, 255);
    let white = Color::new(255, 255, 255);
    let grey = Color::new(128, 128, 128);
    let purple = Color::new(123, 50, 220);

    // Prepare canvas with black background
    canvas.clear(black);

    // Filled triangle outlined by wireframe triangle
    canvas.draw_filled_triangle(
        Point2::new(200.0, -250.0, 1.0),
        Point2::new(400.0, 50.0, 1.0),
        Point2::new(20.0, 250.0, 1.0),
        grey,
    );
    canvas.draw_wireframe_triangle(
        Point2::new(200.0, -250.0, 1.0),
        Point2::new(400.0, 50.0, 1.0),
        Point2::new(20.0, 250.0, 1.0),
        white,
    );

    // Gradiant triangle
    canvas.draw_gradient_triangle(
        Point2::new(0.0, 300.0, 1.0),
        Point2::new(-500.0, -300.0, 0.25),
        Point2::new(500.0, -300.0, 0.0),
        purple,
    );

    // Front vertices of square
    let v0 = Point3::new(-2.0, -2.0, 12.0, 1.0);
    let v1 = Point3::new(-2.0, 2.0, 12.0, 1.0);
    let v2 = Point3::new(-6.0, 2.0, 12.0, 1.0);
    let v3 = Point3::new(-6.0, -2.0, 12.0, 1.0);

    // Back vertices of square
    let v4 = Point3::new(-2.0, -2.0, 14.0, 1.0);
    let v5 = Point3::new(-2.0, 2.0, 14.0, 1.0);
    let v6 = Point3::new(-6.0, 2.0, 14.0, 1.0);
    let v7 = Point3::new(-6.0, -2.0, 14.0, 1.0);

    // Draw front face
    canvas.draw_line(vp.project_vertex(&v0), vp.project_vertex(&v1), blue);
    canvas.draw_line(vp.project_vertex(&v1), vp.project_vertex(&v2), blue);
    canvas.draw_line(vp.project_vertex(&v2), vp.project_vertex(&v3), blue);
    canvas.draw_line(vp.project_vertex(&v3), vp.project_vertex(&v0), blue);

    // Back face
    canvas.draw_line(vp.project_vertex(&v4), vp.project_vertex(&v5), red);
    canvas.draw_line(vp.project_vertex(&v5), vp.project_vertex(&v6), red);
    canvas.draw_line(vp.project_vertex(&v6), vp.project_vertex(&v7), red);
    canvas.draw_line(vp.project_vertex(&v7), vp.project_vertex(&v4), red);

    // Front to back edges
    canvas.draw_line(vp.project_vertex(&v0), vp.project_vertex(&v4), green);
    canvas.draw_line(vp.project_vertex(&v1), vp.project_vertex(&v5), green);
    canvas.draw_line(vp.project_vertex(&v2), vp.project_vertex(&v6), green);
    canvas.draw_line(vp.project_vertex(&v3), vp.project_vertex(&v7), green);

    // Show latest buffer and cap at 144 fps
    canvas.present();

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

        ::std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000u64 / 144));
    }
}
