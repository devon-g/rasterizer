pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Into<sdl2::rect::Point> for Point {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
    }
}

pub struct Line {
    pub p0: Point,
    pub p1: Point,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.r, self.g, self.b)
    }
}
