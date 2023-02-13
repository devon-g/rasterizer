use crate::shapes::geometry::Color;
use crate::shapes::geometry::Point;
use sdl2::video::Window;

/// An SDL2 [`Canvas<Window>`](sdl2::render::Canvas<sdl2::video::Window>) simplified for the book.
pub struct Canvas {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub width: i32,
    pub height: i32,
}

impl Canvas {
    pub fn new(window: Window, width: i32, height: i32) -> Self {
        Self {
            canvas: window.into_canvas().accelerated().build().unwrap(),
            width,
            height,
        }
    }

    /// Swap the presented and currently drawn to buffers
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Clears the canvas with given color.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Draws a pixel of given color at given coordinates on the canvas.
    ///
    /// Converts given coordinates from
    /// "Screen Space" (origin at center, +x left, +y up) to
    /// "Canvas Space" (origin at top left, +x left, +y down)
    pub fn put_pixel(&mut self, p: Point, color: Color) {
        self.canvas.set_draw_color(color);
        // Convert from textbook screen space to sdl2 canvas space
        self.canvas
            .draw_point(Point::new(
                (self.width / 2) as f32 + p.x,
                (self.height / 2) as f32 - p.y,
            ))
            .unwrap();
    }

    pub fn interpolate(&mut self, i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
        if i0 == i1 {
            vec![d0]
        } else {
            let mut values = Vec::new();
            let a = (d1 - d0) / (i1 - i0);
            let mut d = d0;
            for _ in (i0 as i32)..=(i1 as i32) {
                values.push(d);
                d = d + a;
            }

            values
        }
    }

    /// Draws a line of given color between given points.
    ///
    /// Computes y coordinate for each x cordinate using parameterized
    /// line function.
    ///
    /// Uses floats throughout computation and converts to integer at the end.
    pub fn draw_line(&mut self, mut p0: Point, mut p1: Point, color: Color) {
        // Is there more rise than run?
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            // Compute y in terms of x so we can draw horizontal lines
            if p0.x > p1.x {
                (p0, p1) = (p1, p0);
            }
            let ys = self.interpolate(p0.x, p0.y, p1.x, p1.y);
            for x in (p0.x as i32)..=(p1.x as i32) {
                self.put_pixel(Point::new(x as f32, ys[(x - p0.x as i32) as usize]), color);
            }
        } else {
            // Compute x in terms of y so we can draw vertical lines
            if p0.y > p1.y {
                (p0, p1) = (p1, p0);
            }
            let xs = self.interpolate(p0.y, p0.x, p1.y, p1.x);
            for y in (p0.y as i32)..=(p1.y as i32) {
                self.put_pixel(Point::new(xs[(y - p0.y as i32) as usize], y as f32), color);
            }
        }
    }
}
