extern crate sdl2;

pub mod canvas;
pub mod shapes;
pub mod viewport;

use crate::shapes::geometry::Color;
use crate::shapes::geometry::Point2;
use crate::shapes::geometry::Point3;
use canvas::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use viewport::Viewport;

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
    let vp = Viewport::new(16.0, 9.0, 10.0, &canvas);

    // Prepare canvas with black background
    canvas.clear(Color::new(0, 0, 0));

    // Filled triangle outlined by wireframe triangle
    canvas.draw_filled_triangle(
        Point2::new(200.0, -250.0, 1.0),
        Point2::new(400.0, 50.0, 1.0),
        Point2::new(20.0, 250.0, 1.0),
        Color::new(50, 50, 50),
    );
    canvas.draw_triangle(
        Point2::new(200.0, -250.0, 1.0),
        Point2::new(400.0, 50.0, 1.0),
        Point2::new(20.0, 250.0, 1.0),
        Color::new(255, 255, 255),
    );

    // Gradiant triangle
    canvas.draw_gradient_triangle(
        Point2::new(0.0, 300.0, 1.0),
        Point2::new(-500.0, -300.0, 0.25),
        Point2::new(500.0, -300.0, 0.0),
        Color::new(123, 50, 220),
    );

    // Front vertices of square
    let v0 = Point3::new(-3.0, -0.5, 5.0, 1.0);
    let v1 = Point3::new(-3.0, 0.5, 5.0, 1.0);
    let v2 = Point3::new(-2.0, 0.5, 5.0, 1.0);
    let v3 = Point3::new(-2.0, -0.5, 5.0, 1.0);

    // Back vertices of square
    let v4 = Point3::new(-3.0, -0.5, 6.0, 1.0);
    let v5 = Point3::new(-3.0, 0.5, 6.0, 1.0);
    let v6 = Point3::new(-2.0, 0.5, 6.0, 1.0);
    let v7 = Point3::new(-2.0, -0.5, 6.0, 1.0);

    // Draw front face
    canvas.draw_line(
        vp.project_vertex(&v0),
        vp.project_vertex(&v1),
        Color::new(0, 0, 255),
    );
    canvas.draw_line(
        vp.project_vertex(&v1),
        vp.project_vertex(&v2),
        Color::new(0, 0, 255),
    );
    canvas.draw_line(
        vp.project_vertex(&v2),
        vp.project_vertex(&v3),
        Color::new(0, 0, 255),
    );
    canvas.draw_line(
        vp.project_vertex(&v3),
        vp.project_vertex(&v0),
        Color::new(0, 0, 255),
    );

    // Back face
    canvas.draw_line(
        vp.project_vertex(&v4),
        vp.project_vertex(&v5),
        Color::new(255, 0, 0),
    );
    canvas.draw_line(
        vp.project_vertex(&v5),
        vp.project_vertex(&v6),
        Color::new(255, 0, 0),
    );
    canvas.draw_line(
        vp.project_vertex(&v6),
        vp.project_vertex(&v7),
        Color::new(255, 0, 0),
    );
    canvas.draw_line(
        vp.project_vertex(&v7),
        vp.project_vertex(&v4),
        Color::new(255, 0, 0),
    );

    // Front to back edges
    canvas.draw_line(
        vp.project_vertex(&v0),
        vp.project_vertex(&v4),
        Color::new(0, 255, 0),
    );
    canvas.draw_line(
        vp.project_vertex(&v1),
        vp.project_vertex(&v5),
        Color::new(0, 255, 0),
    );
    canvas.draw_line(
        vp.project_vertex(&v2),
        vp.project_vertex(&v6),
        Color::new(0, 255, 0),
    );
    canvas.draw_line(
        vp.project_vertex(&v3),
        vp.project_vertex(&v7),
        Color::new(0, 255, 0),
    );

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
