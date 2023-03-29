use crate::models::model::{Instance, Model};
use crate::models::triangle::Triangle;
use crate::rendering::canvas::Canvas;
use crate::rendering::scene::Scene;
use crate::rendering::viewport::Viewport;
use nalgebra::{Point2, Point3};

pub struct Renderer {
    pub canvas: Canvas,
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(canvas: Canvas, viewport: Viewport) -> Renderer {
        Renderer { canvas, viewport }
    }

    pub fn render_object(&mut self, vertices: &Vec<Point3<f32>>, triangles: &Vec<Triangle>) {
        let mut projected: Vec<Point2<f32>> = Vec::new();
        // Convert all 3d points into 2d points
        for vertex in vertices {
            projected.push(self.viewport.project_vertex(*vertex));
        }
        // Render the triangles
        for triangle in triangles {
            self.render_triangle(triangle, &projected);
        }
    }

    pub fn render_triangle(&mut self, triangle: &Triangle, projected: &Vec<Point2<f32>>) {
        self.canvas.draw_wireframe_triangle(
            projected[triangle.vertices[0] as usize],
            projected[triangle.vertices[1] as usize],
            projected[triangle.vertices[2] as usize],
            triangle.color,
        );
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        for i in 0..scene.instances.len() {
            self.render_instance(&scene.instances[i]);
        }
    }

    pub fn render_instance(&mut self, instance: &Instance) {
        let mut projected: Vec<Point2<f32>> = Vec::new();
        // Convert all 3d points into 2d points
        for i in 0..instance.model.vertices.len() {
            let transformed_vertex = instance
                .translation
                .transform_point(&instance.model.vertices[i]);
            projected.push(self.viewport.project_vertex(transformed_vertex));
        }
        for i in 0..instance.model.triangles.len() {
            self.render_triangle(&instance.model.triangles[i], &projected);
        }
    }
}
