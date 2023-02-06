use sdl2::pixels::Color;
use sdl2::rect::Point;

/// An SDL2 [`Canvas<Window>`](sdl2::render::Canvas<sdl2::video::Window>) simplified for the book.
pub struct Canvas {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub width: i32,
    pub height: i32,
}

impl Canvas {
    pub fn new(
        canvas: sdl2::render::Canvas<sdl2::video::Window>,
        width: i32,
        height: i32,
    ) -> Canvas {
        Canvas {
            canvas,
            width,
            height,
        }
    }

    /// Draws a pixel of given color at given coordinates on the canvas.
    ///
    /// Converts given coordinates from
    /// "Screen Space" (origin at center, +x left, +y up) to
    /// "Canvas Space" (origin at top left, +x left, +y down)
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color);
        // Convert from textbook screen space to sdl2 canvas space
        self.canvas
            .draw_point(Point::new((self.width / 2) + x, (self.height / 2) - y))
            .unwrap();
        self.canvas.present();
    }
}
