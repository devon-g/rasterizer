use crate::rendering::canvas::Canvas;
use nalgebra::{Point2, Point3, Rotation3, Translation3, Vector3};

pub struct Viewport {
    cw: f32,
    ch: f32,
    vw: f32,
    vh: f32,
    d: f32,
    translation: Translation3<f32>,
    rotation: Rotation3<f32>,
}

impl Viewport {
    pub fn new(width: f32, height: f32, depth: f32, canvas: &Canvas) -> Self {
        Viewport {
            cw: canvas.width as f32,
            ch: canvas.height as f32,
            vw: width,
            vh: height,
            d: depth,
            translation: Translation3::new(0.0, 0.0, 0.0),
            rotation: Rotation3::from_euler_angles(0.0, 0.0, 0.0),
        }
    }

    pub fn viewport_to_canvas(&self, p: Point2<f32>) -> Point2<f32> {
        return Point2::new(p.x * (self.cw / self.vw), p.y * (self.ch / self.vh));
    }

    pub fn project_vertex(&self, v: Point3<f32>) -> Point2<f32> {
        return self.viewport_to_canvas(Point2::new(v.x * self.d / v.z, v.y * self.d / v.z));
    }

    pub fn get_translation(&self) -> &Translation3<f32> {
        return &self.translation;
    }

    pub fn get_translation_values(&self) -> &Vector3<f32> {
        return &self.translation.vector;
    }

    pub fn set_translation(&mut self, new_translation: Vector3<f32>) {
        self.translation = Translation3::from(new_translation);
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
}
