use crate::models::model::Instance;
use crate::models::triangle::Triangle;
use crate::rendering::canvas::Canvas;
use crate::rendering::scene::Scene;
use crate::rendering::viewport::Viewport;
use nalgebra_glm::{Vec3, Vec4, Mat4};

pub struct Renderer {
    pub canvas: Canvas,
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(canvas: Canvas, viewport: Viewport) -> Renderer {
        Renderer { canvas, viewport }
    }

    pub fn render_object(&mut self, vertices: &Vec<Vec4>, triangles: &Vec<Triangle>) {
        let mut projected: Vec<Vec3> = Vec::new();
        // Convert all 3d points into 2d points
        for vertex in vertices {
            projected.push(self.viewport.project_vertex(vertex));
        }
        // Render the triangles
        for triangle in triangles {
            self.render_triangle(triangle, &projected);
        }
    }

    pub fn render_triangle(&mut self, triangle: &Triangle, projected: &Vec<Vec3>) {
        self.canvas.draw_wireframe_triangle(
            &projected[triangle.vertices[0] as usize],
            &projected[triangle.vertices[1] as usize],
            &projected[triangle.vertices[2] as usize],
            triangle.color,
        );
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        for i in 0..scene.instances.len() {
            self.render_instance(&scene.instances[i].borrow());
        }
    }

    pub fn render_instance(&mut self, instance: &Instance) {
        let mut projected: Vec<Vec3> = Vec::new();
        // Convert all 3d points into 2d points
        // TODO: Rebuild transform after converting to nalgebra glm
        let ultimate_transform: Mat4 = self.viewport.get_rotation().try_inverse().unwrap()
            * self.viewport.get_translation().try_inverse().unwrap()
            * instance.get_translation()
            * instance.get_rotation()
            * instance.get_scale();
        for i in 0..instance.get_model().vertices.len() {
            projected.push(self.viewport.project_vertex(
                    &(ultimate_transform * instance.get_model().vertices[i])
            ));
        }
        for i in 0..instance.get_model().triangles.len() {
            self.render_triangle(&instance.get_model().triangles[i], &projected);
        }
    }
}
