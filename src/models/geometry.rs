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
