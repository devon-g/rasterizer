use super::viewport::Plane;
use crate::models::triangle::Triangle;
use nalgebra_glm::Vec4;

pub fn clip_triangle(triangle: Triangle, plane: Plane) -> Vec<Triangle> {
    let d0 = plane.signed_distance(triangle.vertices[0]);
    let d1 = plane.signed_distance(triangle.vertices[1]);
    let d2 = plane.signed_distance(triangle.vertices[2]);

    if d0.is_sign_positive() && d1.is_sign_positive() && d2.is_sign_positive() {
        return [triangle].to_vec();
    } else if d0.is_sign_negative() && d1.is_sign_negative() && d2.is_sign_negative() {
        return [].to_vec();
    } else if d0.is_sign_positive() ^ d1.is_sign_positive() ^ d2.is_sign_positive() {
        let mut a: i32;
        let mut b: i32;
        let mut c: i32;
        if d0.is_sign_positive() {
            a = triangle.vertices[0];
            b = triangle.vertices[1];
            c = triangle.vertices[2];
        } else if d1.is_sign_positive() {
            a = triangle.vertices[1];
            b = triangle.vertices[0];
            c = triangle.vertices[2]
        } else {
            a = triangle.vertices[2];
            b = triangle.vertices[1];
            c = triangle.vertices[0];
        }
        let bp = plane.intersection(a, b);
        let cp = plane.intersection(a, c);
        todo!("Need triangle object to contain vertices");
    } else {
        ..
    }
}
