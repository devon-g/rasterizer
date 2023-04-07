use crate::color;
use crate::models::triangle::Triangle;
use nalgebra_glm::{Mat4, Vec3, Vec4};
use std::rc::Rc;

pub struct Model {
    pub vertices: Vec<Vec4>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance {
    model: Rc<Model>,
    scale: Mat4,
    rotation: Mat4,
    translation: Mat4,
}

impl Instance {
    pub fn new(model: Rc<Model>, scale: &Vec4, rotation: &Vec4, translation: &Vec4) -> Self {
        return Self {
            model,
            scale: Mat4::new_nonuniform_scaling(&scale.xyz()),
            rotation: Mat4::new_rotation(Vec3::z_axis().scale(rotation.z))
            * Mat4::new_rotation(Vec3::y_axis().scale(rotation.y))
            * Mat4::new_rotation(Vec3::x_axis().scale(rotation.x)),
            translation: Mat4::new_translation(&translation.xyz()),
        };
    }

    pub fn get_model(&self) -> &Model {
        return &self.model;
    }

    pub fn get_scale(&self) -> &Mat4 {
        return &self.scale;
    }

    pub fn set_scale(&mut self, scale: &Vec4) {
        self.scale = Mat4::new_nonuniform_scaling(&scale.xyz());
    }

    pub fn get_rotation(&self) -> &Mat4 {
        return &self.rotation;
    }

    pub fn set_rotation(&mut self, rotation: &Vec4) {
        self.rotation = Mat4::new_rotation(Vec3::z_axis().scale(rotation.z))
            * Mat4::new_rotation(Vec3::y_axis().scale(rotation.y))
            * Mat4::new_rotation(Vec3::x_axis().scale(rotation.x));
    }

    pub fn get_translation(&self) -> &Mat4 {
        return &self.translation;
    }

    pub fn set_translation(&mut self, translation: &Vec4) {
        self.translation = Mat4::new_translation(&translation.xyz());
    }
}

pub fn default_cube() -> Model {
    Model {
        vertices: vec![
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec4::new(-1.0, 1.0, 1.0, 1.0),
            Vec4::new(-1.0, -1.0, 1.0, 1.0),
            Vec4::new(1.0, -1.0, 1.0, 1.0),
            Vec4::new(1.0, 1.0, -1.0, 1.0),
            Vec4::new(-1.0, 1.0, -1.0, 1.0),
            Vec4::new(-1.0, -1.0, -1.0, 1.0),
            Vec4::new(1.0, -1.0, -1.0, 1.0),
        ],
        triangles: vec![
            Triangle::new(0, 1, 2, color::RED),
            Triangle::new(0, 2, 3, color::RED),
            Triangle::new(4, 0, 3, color::GREEN),
            Triangle::new(4, 3, 7, color::GREEN),
            Triangle::new(5, 4, 7, color::BLUE),
            Triangle::new(5, 7, 6, color::BLUE),
            Triangle::new(1, 5, 6, color::YELLOW),
            Triangle::new(1, 6, 2, color::YELLOW),
            Triangle::new(4, 5, 1, color::PURPLE),
            Triangle::new(4, 1, 0, color::PURPLE),
            Triangle::new(2, 6, 7, color::CYAN),
            Triangle::new(2, 7, 3, color::CYAN),
        ],
    }
}
