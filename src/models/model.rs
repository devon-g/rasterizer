use crate::color;
use crate::models::triangle::Triangle;
use nalgebra::{Point3, Rotation3, Scale3, Transform3, Translation3};
use std::rc::Rc;

pub struct Model {
    pub vertices: Vec<Point3<f32>>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance {
    pub model: Rc<Model>,
    pub scale: Scale3<f32>,
    pub rotation: Rotation3<f32>,
    pub translation: Translation3<f32>,
    pub transformation: Transform3<f32>,
}

impl Instance {
    pub fn new(
        model: Rc<Model>,
        scale: Scale3<f32>,
        rotation: Rotation3<f32>,
        translation: Translation3<f32>,
    ) -> Instance {
        Instance {
            model,
            scale,
            rotation,
            translation,
            transformation: Transform3::from_matrix_unchecked(
                rotation
                    .inverse()
                    .to_homogeneous()
                    .prepend_nonuniform_scaling(&scale.vector)
                    .append_translation(&translation.vector),
            ),
        }
    }

    pub fn set_scale(&mut self, new_scale: Scale3<f32>) {
        self.scale = new_scale;
        self.transformation = Transform3::from_matrix_unchecked(
            self.rotation
                .inverse()
                .to_homogeneous()
                .prepend_nonuniform_scaling(&new_scale.vector)
                .append_translation(&self.translation.vector),
        );
    }

    pub fn set_rotation(&mut self, new_rotation: Rotation3<f32>) {
        self.rotation = new_rotation;
        self.rebuild_transformation();
    }

    pub fn set_translation(&mut self, new_translation: Translation3<f32>) {
        self.translation = new_translation;
        self.rebuild_transformation();
    }

    fn rebuild_transformation(&mut self) {
        self.transformation = Transform3::from_matrix_unchecked(
            self.rotation
                .inverse()
                .to_homogeneous()
                .prepend_nonuniform_scaling(&self.scale.vector)
                .append_translation(&self.translation.vector),
        )
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
