use super::geometry::{Color, Point};
use super::Drawable;
use crate::canvas::Canvas;

#[derive(Copy, Clone)]
struct Rectangle {
    points: [Point; 4],
    center: Point,
    theta: f32,
}

impl Rectangle {
    pub fn new(width: f32, height: f32, center: Point, theta: f32) -> Self {
        Self {
            points: [
                Point::new(-width / 2.0, height / 2.0, 1.0),
                Point::new(-width / 2.0, -height / 2.0, 1.0),
                Point::new(width / 2.0, height / 2.0, 1.0),
                Point::new(width / 2.0, -height / 2.0, 1.0),
            ],
            center,
            theta,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, canvas: &mut Canvas, color: Color) {
        canvas.draw_line(self.points[0], self.points[1], color);
        canvas.draw_line(self.points[1], self.points[2], color);
        canvas.draw_line(self.points[2], self.points[3], color);
        canvas.draw_line(self.points[3], self.points[0], color);
    }
    // fn get_points(&self) -> Vec<Point> {
    //     self.points.to_vec()
    // }
}
