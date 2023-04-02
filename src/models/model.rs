use crate::color;
use crate::models::triangle::Triangle;
use nalgebra::{Point3, Rotation3, Scale3, Translation3, Vector3};
use std::rc::Rc;

pub struct Model {
    pub vertices: Vec<Point3<f32>>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance {
    model: Rc<Model>,
    scale: Scale3<f32>,
    rotation: Rotation3<f32>,
    translation: Translation3<f32>,
}

impl Instance {
    pub fn new(
        model: Rc<Model>,
        scale: Vector3<f32>,
        rotation: Vector3<f32>,
        translation: Vector3<f32>,
    ) -> Self {
        return Self {
            model,
            scale: Scale3::from(scale),
            rotation: Rotation3::from_euler_angles(rotation[0], rotation[1], rotation[2]),
            translation: Translation3::from(translation),
        };
    }

    pub fn get_model(&self) -> &Model {
        return &self.model;
    }

    pub fn get_scale(&self) -> &Scale3<f32> {
        return &self.scale;
    }

    pub fn get_scale_values(&self) -> &Vector3<f32> {
        return &self.scale.vector;
    }

    pub fn set_scale(&mut self, new_scale: Vector3<f32>) {
        self.scale = Scale3::from(new_scale);
    }

    pub fn get_rotation(&self) -> &Rotation3<f32> {
        return &self.rotation;
    }

    pub fn get_rotation_values(&self) -> Vector3<f32> {
        return Vector3::new(
            self.rotation.euler_angles().0,
            self.rotation.euler_angles().1,
            self.rotation.euler_angles().2,
        );
    }

    pub fn set_rotation(&mut self, new_rotation: Vector3<f32>) {
        self.rotation =
            Rotation3::from_euler_angles(new_rotation[0], new_rotation[1], new_rotation[2]);
    }

    pub fn get_translatioin(&self) -> &Translation3<f32> {
        return &self.translation;
    }

    pub fn get_translation_values(&self) -> &Vector3<f32> {
        return &self.translation.vector;
    }

    pub fn set_translation(&mut self, new_translation: Vector3<f32>) {
        self.translation = Translation3::from(new_translation);
    }
}

pub fn default_cube() -> Model {
    Model {
        vertices: vec![
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0),
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, 1.0, -1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
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
