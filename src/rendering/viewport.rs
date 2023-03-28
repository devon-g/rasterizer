use crate::models::geometry::{Point2, Point3};
use crate::rendering::canvas::Canvas;

pub struct Viewport {
    pub cw: f32,
    pub ch: f32,
    pub vw: f32,
    pub vh: f32,
    pub d: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32, depth: f32, canvas: &Canvas) -> Self {
        Viewport {
            cw: canvas.width as f32,
            ch: canvas.height as f32,
            vw: width,
            vh: height,
            d: depth,
        }
    }

    pub fn viewport_to_canvas(&self, p: Point2) -> Point2 {
        Point2::new(p.x * (self.cw / self.vw), p.y * (self.ch / self.vh), p.h)
    }

    pub fn project_vertex(&self, v: &Point3) -> Point2 {
        self.viewport_to_canvas(Point2::new(v.x * self.d / v.z, v.y * self.d / v.z, v.h))
    }
}
