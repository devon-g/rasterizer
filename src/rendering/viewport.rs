use crate::rendering::canvas::Canvas;
use nalgebra_glm::{Mat4, Vec3, Vec4};

pub struct Plane {
    normal: Vec4,
    d: f32,
}

impl Plane {
    pub fn new(vector: &Vec4, distance: f32) -> Self {
        Self {
            normal: vector.normalize(),
            d: distance,
        }
    }

    pub fn signed_distance(&self, point: &Vec4) -> f32 {
        return self.normal.dot(point) + self.d;
    }

    pub fn intersection(&self, p0: &Vec4, p1: &Vec4) -> Vec4 {
        let t = (-self.d - self.normal.dot(p0)) / self.normal.dot(&(p1 - p0));
        return p0 + t*(p1 - p0);
    }
}

pub struct Viewport {
    cw: f32,
    ch: f32,
    vw: f32,
    vh: f32,
    d: f32,
    clipping_planes: [Plane; 5],
    translation: Mat4,
    rotation: Mat4,
    transformation: Mat4,
}

impl Viewport {
    pub fn new(width: f32, height: f32, depth: f32, canvas: &Canvas) -> Self {
        Self {
            cw: canvas.width as f32,
            ch: canvas.height as f32,
            vw: width,
            vh: height,
            d: depth,
            clipping_planes: [
                Plane::new(&Vec4::new(0.0, 0.0, 1.0, 0.0), depth), // Near
                Plane::new(&Vec4::new(depth, 0.0, width / 2.0, 0.0), 0.0), // Left
                Plane::new(&Vec4::new(depth, 0.0, -width / 2.0, 0.0), 0.0), // Right
                Plane::new(&Vec4::new(0.0, depth, -height / 2.0, 0.0), 0.0), // Top
                Plane::new(&Vec4::new(0.0, depth, height / 2.0, 0.0), 0.0), // Bottom
            ],
            translation: Mat4::identity(),
            rotation: Mat4::identity(),
            transformation: Mat4::identity(),
        }
    }

    pub fn viewport_to_canvas(&self, point: &Vec3) -> Vec3 {
        return Vec3::new(
            point.x * (self.cw / self.vw),
            point.y * (self.ch / self.vh),
            1.0,
        );
    }

    pub fn project_vertex(&self, vertex: &Vec4) -> Vec3 {
        return self.viewport_to_canvas(&Vec3::new(
            vertex.x * self.d / vertex.z,
            vertex.y * self.d / vertex.z,
            1.0,
        ));
    }

    pub fn get_translation(&self) -> &Mat4 {
        return &self.translation;
    }

    pub fn set_translation(&mut self, translation: &Vec4) {
        self.translation = Mat4::new_translation(&translation.xyz());
        self.generate_transform();
    }

    pub fn get_rotation(&self) -> &Mat4 {
        return &self.rotation;
    }

    pub fn set_rotation(&mut self, rotation: &Vec4) {
        self.rotation = Mat4::new_rotation(-Vec3::z_axis().scale(rotation.z))
            * Mat4::new_rotation(-Vec3::y_axis().scale(rotation.y))
            * Mat4::new_rotation(-Vec3::x_axis().scale(rotation.x));
        self.generate_transform();
    }

    pub fn generate_transform(&mut self) {
        self.transformation =
            self.rotation.try_inverse().unwrap() * self.translation.try_inverse().unwrap();
    }

    pub fn get_transform(&self) -> Mat4 {
        return self.transformation;
    }
}
