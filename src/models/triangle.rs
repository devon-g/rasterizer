use crate::color::Color;

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [i32; 3],
    pub color: Color,
}
impl Triangle {
    pub fn new(idx0: i32, idx1: i32, idx2: i32, color: Color) -> Triangle {
        Triangle {
            vertices: [idx0, idx1, idx2],
            color,
        }
    }
}
