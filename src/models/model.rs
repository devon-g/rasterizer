use crate::color;
use crate::models::geometry::Point3;
use crate::models::triangle::Triangle;

#[derive(Clone)]
pub struct Model {
    pub vertices: Vec<Point3>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance {
    pub model: Model,
    pub position: Point3,
}

impl Instance {
    pub fn new(model: &Model, position: Point3) -> Instance {
        Instance { model: model.clone(), position }
    }
}

pub fn default_cube() -> Model {
    Model {
        vertices: vec![
            Point3::new(1.0, 1.0, 1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0, 1.0),
            Point3::new(-1.0, -1.0, 1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0, 1.0),
            Point3::new(1.0, 1.0, -1.0, 1.0),
            Point3::new(-1.0, 1.0, -1.0, 1.0),
            Point3::new(-1.0, -1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, -1.0, 1.0),
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
