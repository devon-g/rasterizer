use crate::rendering::canvas::Canvas;
use nalgebra::{Point2, Point3, Rotation3, Transform3, Translation3};

pub struct Viewport {
    pub cw: f32,
    pub ch: f32,
    pub vw: f32,
    pub vh: f32,
    pub d: f32,
    pub translation: Translation3<f32>,
    pub rotation: Rotation3<f32>,
    pub transformation: Transform3<f32>,
}

impl Viewport {
    pub fn new(width: f32, height: f32, depth: f32, canvas: &Canvas) -> Self {
        Viewport {
            cw: canvas.width as f32,
            ch: canvas.height as f32,
            vw: width,
            vh: height,
            d: depth,
            translation: Translation3::identity(),
            rotation: Rotation3::identity(),
            transformation: Transform3::identity(),
        }
    }

    pub fn viewport_to_canvas(&self, p: Point2<f32>) -> Point2<f32> {
        Point2::new(p.x * (self.cw / self.vw), p.y * (self.ch / self.vh))
    }

    pub fn project_vertex(&self, v: Point3<f32>) -> Point2<f32> {
        self.viewport_to_canvas(Point2::new(v.x * self.d / v.z, v.y * self.d / v.z))
    }

    pub fn set_translation(&mut self, new_translation: Translation3<f32>) {
        self.translation = new_translation;
        self.rebuild_transformation();
    }

    pub fn set_rotation(&mut self, new_rotation: Rotation3<f32>) {
        self.rotation = new_rotation;
        self.rebuild_transformation();
    }

    fn rebuild_transformation(&mut self) {
        self.transformation = Transform3::from_matrix_unchecked(
            self.rotation
                .to_homogeneous()
                .prepend_translation(&self.translation.vector),
        );
    }
}
