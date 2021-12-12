#[derive(Copy, Clone)]
pub enum Geometry {
    Rectangle,
    Circle,
}

pub struct Shape {
    pub geometry: Geometry,
    pub radius: f32,
}
