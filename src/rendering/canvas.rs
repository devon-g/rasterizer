use crate::color::Color;
use nalgebra_glm::Vec3;
use sdl2::rect::Point;
use sdl2::video::Window;

/// An SDL2 [`Canvas<Window>`](sdl2::render::Canvas<Window>) simplified for the book.
pub struct Canvas {
    canvas: sdl2::render::Canvas<Window>,
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
    pub fn put_pixel(&mut self, point: &Vec3, color: Color) {
        self.canvas.set_draw_color(color);
        // Convert from textbook screen space to sdl2 canvas space
        self.canvas
            .draw_point(Point::new(
                (self.width / 2) + point.x as i32,
                (self.height / 2) - point.y as i32,
            ))
            .unwrap();
    }

    /// Draws gradient triangle
    ///
    /// Uses interpolation to determine which pixels to draw and which color
    /// intensity to use for those pixels (between black and given color).
    pub fn draw_gradient_triangle(
        &mut self,
        mut p0: &Vec3,
        p0h: f32,
        mut p1: &Vec3,
        p1h: f32,
        mut p2: &Vec3,
        p2h: f32,
        color: Color,
    ) {
        // Organize points by y level. P0 <= P1 <= P2
        if p1.y < p0.y {
            (p0, p1) = (p1, p0);
        }
        if p2.y < p0.y {
            (p0, p2) = (p2, p0);
        }
        if p2.y < p1.y {
            (p1, p2) = (p2, p1);
        }

        // Compute x's for each row in the triangle
        let x01 = self.interpolate(p0.y, p0.x, p1.y, p1.x);
        let h01 = self.interpolate(p0.y, p0h, p1.y, p1h);

        let x12 = self.interpolate(p1.y, p1h, p2.y, p2h);
        let h12 = self.interpolate(p1.y, p1h, p2.y, p2h);

        let x02 = self.interpolate(p0.y, p0.x, p2.y, p2.x);
        let h02 = self.interpolate(p0.y, p0h, p2.y, p2h);

        // x01 and x12 have a common point so we remove one from x01 and join them
        let x012 = [&x01[..x01.len() - 1], &x12[..]].concat();
        let h012 = [&h01[..h01.len() - 1], &h12[..]].concat();

        // Figure out which array is left and which is right
        let x_left: Vec<f32>;
        let h_left: Vec<f32>;

        let x_right: Vec<f32>;
        let h_right: Vec<f32>;

        let m = x02.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            h_left = h02;

            x_right = x012;
            h_right = h012;
        } else {
            x_left = x012;
            h_left = h012;

            x_right = x02;
            h_right = h02;
        }

        for y in (p0.y as i32)..=(p2.y as i32) {
            let idx = (y - p0.y as i32) as usize;
            let x_l = x_left[idx];
            let x_r = x_right[idx];

            let h_segment = self.interpolate(x_l, h_left[idx], x_r, h_right[idx]);
            for x in (x_l as i32)..=(x_r as i32) {
                let shaded_color = color * h_segment[(x - x_l as i32) as usize];
                self.put_pixel(&Vec3::new(x as f32, y as f32, 1.0), shaded_color);
            }
        }
    }

    /// Draws filled triangle
    ///
    /// Uses interpolation to determine which pixels to draw inside the triangle
    pub fn draw_filled_triangle(
        &mut self,
        mut p0: &Vec3,
        mut p1: &Vec3,
        mut p2: &Vec3,
        color: Color,
    ) {
        // Organize points by y level. P0 <= P1 <= P2
        if p1.y < p0.y {
            (p0, p1) = (p1, p0);
        }
        if p2.y < p0.y {
            (p0, p2) = (p2, p0);
        }
        if p2.y < p1.y {
            (p1, p2) = (p2, p1);
        }

        // Compute x's for each row in the triangle
        let x01 = self.interpolate(p0.y, p0.x, p1.y, p1.x);
        let x12 = self.interpolate(p1.y, p1.x, p2.y, p2.x);
        let x02 = self.interpolate(p0.y, p0.x, p2.y, p2.x);

        // x01 and x12 have a common point so we remove one from x01 and join them
        let x012 = [&x01[..x01.len() - 1], &x12[..]].concat();

        // Figure out which array is left and which is right
        let x_left: Vec<f32>;
        let x_right: Vec<f32>;
        let m = x02.len() / 2;
        if x02[m] < x012[m] {
            x_left = x02;
            x_right = x012;
        } else {
            x_left = x012;
            x_right = x02;
        }

        for y in (p0.y as i32)..=(p2.y as i32) {
            let idx = (y - p0.y as i32) as usize;
            let x_l = x_left[idx];
            let x_r = x_right[idx];
            for x in (x_l as i32)..=(x_r as i32) {
                self.put_pixel(&Vec3::new(x as f32, y as f32, 1.0), color);
            }
        }
    }

    /// Draws wireframe triangle
    pub fn draw_wireframe_triangle(
        &mut self,
        p0: &Vec3,
        p1: &Vec3,
        p2: &Vec3,
        color: Color,
    ) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    /// Computes set of points between two points
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

            return values;
        }
    }

    /// Draws a line of given color between given points.
    ///
    /// Computes y coordinate for each x coordinate using parameterized
    /// line function.
    ///
    /// Uses floats throughout computation and converts to integer at the end.
    pub fn draw_line(&mut self, mut p0: &Vec3, mut p1: &Vec3, color: Color) {
        // Is there more rise than run?
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            // Compute y in terms of x so we can draw horizontal lines
            if p0.x > p1.x {
                (p0, p1) = (p1, p0);
            }
            let ys = self.interpolate(p0.x, p0.y, p1.x, p1.y);
            for x in (p0.x as i32)..=(p1.x as i32) {
                self.put_pixel(&Vec3::new(x as f32, ys[(x - p0.x as i32) as usize], 1.0), color);
            }
        } else {
            // Compute x in terms of y so we can draw vertical lines
            if p0.y > p1.y {
                (p0, p1) = (p1, p0);
            }
            let xs = self.interpolate(p0.y, p0.x, p1.y, p1.x);
            for y in (p0.y as i32)..=(p1.y as i32) {
                self.put_pixel(&Vec3::new(xs[(y - p0.y as i32) as usize], y as f32, 1.0), color);
            }
        }
    }
}
