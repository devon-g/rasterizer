use crate::rendering::canvas::Canvas;
use nalgebra_glm::{Mat4, Vec2, Vec3};

pub struct Viewport {
    cw: f32,
    ch: f32,
    vw: f32,
    vh: f32,
    d: f32,
    translation_xyz: Vec3,
    rotation_xyz: Vec3,
    pub translation: Mat4,
    pub rotation: Mat4,
}

impl Viewport {
    pub fn new(width: f32, height: f32, depth: f32, canvas: &Canvas) -> Self {
        Viewport {
            cw: canvas.width as f32,
            ch: canvas.height as f32,
            vw: width,
            vh: height,
            d: depth,
            translation_xyz: Vec3::new(0.0, 0.0, 0.0),
            translation: Mat4::identity(),
            rotation_xyz: Vec3::new(0.0, 0.0, 0.0),
            rotation: Mat4::identity(),
        }
    }

    pub fn viewport_to_canvas(&self, p: Vec2) -> Vec2 {
        return Vec2::new(p.x * (self.cw / self.vw), p.y * (self.ch / self.vh));
    }

    pub fn project_vertex(&self, v: Vec3) -> Vec2 {
        return self.viewport_to_canvas(Vec2::new(v.x * self.d / v.z, v.y * self.d / v.z));
    }

    pub fn get_translation(&self) -> Vec3 {
        return self.translation_xyz;
    }

    pub fn set_translation(&mut self, new_translation: Vec3) {
        self.translation_xyz = new_translation;
        self.translation = nalgebra_glm::translation(&self.translation_xyz);
    }

    pub fn get_rotation(&self) -> Vec3 {
        return self.rotation_xyz;
    }
    pub fn set_rotation(&mut self, new_rotation: Vec3) {
        self.rotation_xyz = new_rotation;
        self.rotation = nalgebra_glm::rotation(self.rotation_xyz[2], &Vec3::z_axis())
            * nalgebra_glm::rotation(self.rotation_xyz[1], &Vec3::y_axis())
            * nalgebra_glm::rotation(self.rotation_xyz[0], &Vec3::x_axis());
    }
}
