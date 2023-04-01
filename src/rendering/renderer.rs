use crate::models::model::Instance;
use crate::models::triangle::Triangle;
use crate::rendering::canvas::Canvas;
use crate::rendering::scene::Scene;
use crate::rendering::viewport::Viewport;
use nalgebra_glm::{Mat4, Vec2, Vec3};

pub struct Renderer {
    pub canvas: Canvas,
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(canvas: Canvas, viewport: Viewport) -> Renderer {
        Renderer { canvas, viewport }
    }

    pub fn render_object(&mut self, vertices: &Vec<Vec3>, triangles: &Vec<Triangle>) {
        let mut projected: Vec<Vec2> = Vec::new();
        // Convert all 3d points into 2d points
        for vertex in vertices {
            projected.push(self.viewport.project_vertex(*vertex));
        }
        // Render the triangles
        for triangle in triangles {
            self.render_triangle(triangle, &projected);
        }
    }

    pub fn render_triangle(&mut self, triangle: &Triangle, projected: &Vec<Vec2>) {
        self.canvas.draw_wireframe_triangle(
            projected[triangle.vertices[0] as usize],
            projected[triangle.vertices[1] as usize],
            projected[triangle.vertices[2] as usize],
            triangle.color,
        );
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        for i in 0..scene.instances.len() {
            self.render_instance(&scene.instances[i].borrow());
        }
    }

    pub fn render_instance(&mut self, instance: &Instance) {
        let mut projected: Vec<Vec2> = Vec::new();
        // Convert all 3d points into 2d points
        let ultimate_transform: Mat4 = self.viewport.rotation
            * self.viewport.translation.try_inverse().unwrap()
            * instance.translation
            * instance.rotation
            * instance.scale;
        for i in 0..instance.model.vertices.len() {
            projected.push(
                //self.viewport.project_vertex(Vec3::from(
                //    ultimate_transform
                //        .transform_point(&Point3::from(instance.model.vertices[i]))
                //        .coords,
                //)),
                self.viewport.project_vertex(
                    ultimate_transform.transform_vector(&instance.model.vertices[i]),
                ),
            );
        }
        for i in 0..instance.model.triangles.len() {
            self.render_triangle(&instance.model.triangles[i], &projected);
        }
    }
}
