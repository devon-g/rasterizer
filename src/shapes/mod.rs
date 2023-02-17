pub mod geometry;
//pub mod rectangle;

use crate::canvas::Canvas;
use geometry::Color;
// use geometry::Point;

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas, color: Color);
    // fn get_points(&self) -> Vec<Point>;
}
