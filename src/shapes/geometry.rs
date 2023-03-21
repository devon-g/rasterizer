#[derive(Copy, Clone)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
    pub h: f32,
}

impl Point2 {
    pub fn new(x: f32, y: f32, h: f32) -> Point2 {
        Point2 {
            x,
            y,
            h: h.clamp(0.0, 1.0),
        }
    }
}

impl Into<sdl2::rect::Point> for Point2 {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x as i32, self.y as i32)
    }
}

pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub h: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32, h: f32) -> Point3 {
        Point3 {
            x,
            y,
            z,
            h: h.clamp(0.0, 1.0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r.clamp(0, 255),
            g: g.clamp(0, 255),
            b: b.clamp(0, 255),
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, color: Color) -> Color {
        Color::new(
            ((color.r as f32) * self) as u8,
            ((color.g as f32) * self) as u8,
            ((color.b as f32) * self) as u8,
        )
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, scalar: f32) -> Color {
        Color::new(
            ((self.r as f32) * scalar) as u8,
            ((self.g as f32) * scalar) as u8,
            ((self.b as f32) * scalar) as u8,
        )
    }
}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.r, self.g, self.b)
    }
}
