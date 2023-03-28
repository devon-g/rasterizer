use crate::models::geometry::{Point2, Point3};
use crate::models::triangle::Triangle;
use crate::rendering::canvas::Canvas;
use crate::rendering::viewport::Viewport;

pub struct Renderer {
    pub canvas: Canvas,
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(canvas: Canvas, viewport: Viewport) -> Renderer {
        Renderer { canvas, viewport }
    }

    pub fn render_object(&mut self, vertices: &Vec<Point3>, triangles: &Vec<Triangle>) {
        let mut projected: Vec<Point2> = Vec::new();
        // Convert all 3d points into 2d points
        for vertex in vertices {
            projected.push(self.viewport.project_vertex(&vertex));
        }
        // Render the triangles
        for triangle in triangles {
            self.render_triangle(triangle, &projected);
        }
    }

    pub fn render_triangle(&mut self, triangle: &Triangle, projected: &Vec<Point2>) {
        self.canvas.draw_wireframe_triangle(
            projected[triangle.vertices[0] as usize],
            projected[triangle.vertices[1] as usize],
            projected[triangle.vertices[2] as usize],
            triangle.color,
        );
    }
}
