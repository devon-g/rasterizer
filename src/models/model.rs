use crate::color;
use crate::models::triangle::Triangle;
use nalgebra::{Matrix4, Point3, Rotation3, Scale3, Transform3, Translation3, Vector3};
use std::f32::consts::PI;
use std::rc::Rc;

pub struct Model {
    pub vertices: Vec<Point3<f32>>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance {
    pub model: Rc<Model>,
    scale_xyz: Vector3<f32>,
    rotation_xyz: Vector3<f32>,
    translation_xyz: Vector3<f32>,
    transformation: Transform3<f32>,
}

impl Instance {
    pub fn new(
        model: Rc<Model>,
        scale_xyz: Vector3<f32>,
        rotation_xyz: Vector3<f32>,
        translation_xyz: Vector3<f32>,
    ) -> Instance {
        let mut instance: Instance = Instance {
            model,
            scale_xyz,
            rotation_xyz,
            translation_xyz,
            transformation: Transform3::identity(),
        };

        return instance;
    }

    fn build_transform(&mut self) {
        Transform3::from_matrix_unchecked(
            Matrix4::new_translation(&self.translation_xyz)
                * Matrix4::from_axis_angle(&Vector3::<f32>::z_axis(), self.rotation_xyz[2])
                * Matrix4::from_axis_angle(&Vector3::<f32>::y_axis(), self.rotation_xyz[1])
                * Matrix4::from_axis_angle(&Vector3::<f32>::x_axis(), self.rotation_xyz[0])
                * Matrix4::new_nonuniform_scaling(&self.scale_xyz),
        );
    }

    pub fn get_scale(&self) -> Vector3<f32> {
        return self.scale_xyz;
    }

    pub fn set_scale(&mut self, new_scale: Vector3<f32> ) {
        self.scale_xyz = new_scale;
        self.build_transform();
    }

    pub fn get_rotation(&self) -> Vector3<f32>{
        return self.rotation_xyz;
    }

    pub fn set_rotation(&mut self, new_rotation: Mat4) {
        self.rotation = new_rotation;
    }

    pub fn get_translation(&self) -> Vec3 {
        return self.translation_xyz;
    }

    pub fn set_translation(&mut self, new_translation: Mat4) {
        self.translation = new_translation;
    }
}

pub fn default_cube() -> Model {
    Model {
        vertices: vec![
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-1.0, 1.0, 1.0),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, -1.0, -1.0),
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
