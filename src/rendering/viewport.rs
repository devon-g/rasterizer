use crate::rendering::canvas::Canvas;
use nalgebra_glm::{Vec3, Vec4, Mat4};

pub struct Viewport {
    cw: f32,
    ch: f32,
    vw: f32,
    vh: f32,
    d: f32,
    translation: Mat4,
    rotation: Mat4,
}

impl Viewport {
    pub fn new(width: f32, height: f32, depth: f32, canvas: &Canvas) -> Self {
        Viewport {
            cw: canvas.width as f32,
            ch: canvas.height as f32,
            vw: width,
            vh: height,
            d: depth,
            translation: Mat4::identity(),
            rotation: Mat4::identity(),
        }
    }

    pub fn viewport_to_canvas(&self, point: &Vec3) -> Vec3 {
        return Vec3::new(point.x * (self.cw / self.vw), point.y * (self.ch / self.vh), 1.0);
    }

    pub fn project_vertex(&self, vertex: &Vec4) -> Vec3 {
        return self.viewport_to_canvas(&Vec3::new(vertex.x * self.d / vertex.z, vertex.y * self.d / vertex.z, 1.0));
    }

    pub fn get_translation(&self) -> &Mat4 {
        return &self.translation;
    }

    pub fn set_translation(&mut self, translation: &Vec4) {
        self.translation = Mat4::new_translation(&translation.xyz());
    }

    pub fn get_rotation(&self) -> &Mat4 {
        return &self.rotation;
    }

    pub fn set_rotation(&mut self, rotation: &Vec4) {
        self.rotation = Mat4::new_rotation(Vec3::z_axis().scale(rotation.z))
            * Mat4::new_rotation(Vec3::y_axis().scale(rotation.y))
            * Mat4::new_rotation(Vec3::x_axis().scale(rotation.x));
    }
}
